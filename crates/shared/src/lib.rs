use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::mem;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deno_lint::linter::LinterBuilder;
use deno_lint::rules::get_recommended_rules;
use deno_lint::rules::LintRule;
use ignore::overrides::OverrideBuilder;
use ignore::types::TypesBuilder;
use ignore::WalkBuilder;

pub mod config;
pub mod diagnostics;
pub mod media;

fn lint_file(
  p: &Path,
  base: &Path,
  rules: Vec<Arc<dyn LintRule>>,
  format: Option<String>,
) -> Result<bool, Error> {
  let file_content = fs::read_to_string(&p).map_err(|e| {
    Error::new(
      ErrorKind::Other,
      format!("Reading file {:?} failed: {}", &p, e),
    )
  })?;

  let linter = LinterBuilder::default()
    .rules(rules)
    .media_type(media::get_media_type(p))
    .ignore_file_directive("eslint-disable")
    .ignore_diagnostic_directive("eslint-disable-next-line")
    .build();
  let name = media::make_relative(p, base);
  let (s, file_diagnostics) = linter
    .lint(
      name
        .to_str()
        .ok_or_else(|| {
          Error::new(
            ErrorKind::Other,
            format!("Converting path to string failed: {:?}", &name),
          )
        })?
        .to_owned(),
      file_content,
    )
    .map_err(|e| {
      let suffix = if e.to_string().contains(name.to_str().unwrap()) {
        "".to_owned()
      } else {
        format!(", at: {:?}", &name)
      };
      Error::new(ErrorKind::Other, format!("Lint failed: {}{}", e, &suffix))
    })?;
  for issue in diagnostics::display_diagnostics(
    &file_diagnostics,
    s.text_info(),
    name.to_str().unwrap(),
    format.as_deref(),
  )
  .map_err(|err| Error::new(ErrorKind::Other, format!("{err}")))?
  {
    eprintln!("{issue}")
  }
  Ok(file_diagnostics.is_empty())
}

pub fn denolint(
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_dirs: Option<Vec<String>>,
  ignore_patterns: Option<Vec<String>>,
  format: Option<String>,
  dry_run: Option<bool>,
) -> Result<bool, Error> {
  let mut ok = true;
  let has_proj: bool;

  let cwd = env::current_dir().map_err(|e| {
    Error::new(
      ErrorKind::Other,
      format!("Getting current_dir failed {}", e),
    )
  })?;
  let proj = match proj_dir {
    Some(s) => {
      has_proj = true;
      media::make_absolute(&s, &cwd)
    }
    None => {
      has_proj = false;
      cwd.clone()
    }
  };

  let config = config_path.unwrap_or_else(|| ".denolint.json".to_owned());
  let config_existed =
    !config.is_empty() && fs::metadata(&config).map(|m| m.is_file()).unwrap_or(false);

  let (rules, cfg_ignore_files, cfg_add_files) = if config_existed {
    let cfg = config::load_from_json(path::Path::new(&config))?;
    (cfg.get_rules(), cfg.files.exclude, cfg.files.include)
  } else {
    (get_recommended_rules(), vec![], vec![])
  };

  let mut eslint_ignore_file = proj.clone();
  eslint_ignore_file.push(".eslintignore");

  let mut denolint_ignore_file = proj.clone();
  denolint_ignore_file.push(".denolintignore");

  let mut type_builder = TypesBuilder::new();

  type_builder
    .add("typescript", "*.ts")
    .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;
  type_builder
    .add("typescript", "*.tsx")
    .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;

  let types = type_builder
    .add_defaults()
    .select("typescript")
    .select("js")
    .build()
    .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;

  let ignore_file_path = match fs::File::open(&denolint_ignore_file) {
    Ok(_) => denolint_ignore_file.as_path().to_str().ok_or_else(|| {
      Error::new(
        ErrorKind::Other,
        format!(
          "Converting path to string failed: {:?}",
          &denolint_ignore_file
        ),
      )
    })?,
    Err(_) => match fs::File::open(&eslint_ignore_file) {
      Ok(_) => eslint_ignore_file.as_path().to_str().ok_or_else(|| {
        Error::new(
          ErrorKind::Other,
          format!(
            "Converting path to string failed: {:?}",
            &eslint_ignore_file
          ),
        )
      })?,
      Err(_) => "",
    },
  };

  let mut dirs: Vec<String>;
  let files: Vec<String>;
  let mut patterns: Vec<String>;
  let mut ignore: Vec<String>;
  let scan = scan_dirs.unwrap_or_default();
  if !scan.is_empty() {
    (dirs, files, patterns) = media::classify_paths(&scan, cwd.as_path());
  } else if !cfg_add_files.is_empty() {
    (dirs, files, patterns) = media::classify_paths(&cfg_add_files, &proj);
  } else {
    dirs = vec![];
    files = vec![];
    patterns = vec![];
  }
  ignore = ignore_patterns.unwrap_or_default();
  if !ignore.is_empty() {
    ignore = ignore
      .iter()
      .map(|p| media::make_absolute_string(p, &cwd))
      .collect();
  } else if !cfg_ignore_files.is_empty() {
    ignore = cfg_ignore_files
      .iter()
      .map(|p| media::make_absolute_string(p, &proj))
      .collect();
  };
  #[allow(clippy::needless_range_loop)]
  for i in 0..patterns.len() {
    let pattern = &patterns[i];
    match pattern.chars().position(|c| c == '*') {
      Some(p) => {
        dirs.push(pattern.chars().take(p).collect());
        let s: String = pattern.chars().skip(p).collect();
        let _ = mem::replace(&mut patterns[i], s);
      }
      None => {}
    }
  }

  let no_lint = dry_run.unwrap_or_default();

  if !dirs.is_empty() || scan.is_empty() && cfg_add_files.is_empty() {
    let root = if has_proj || dirs.is_empty() || scan.is_empty() {
      proj.clone()
    } else {
      PathBuf::from(&dirs[0])
    };
    let mut dir_walker = WalkBuilder::new(&root);
    dir_walker.types(types).follow_links(true);
    if !ignore_file_path.is_empty() {
      dir_walker.add_custom_ignore_filename(ignore_file_path);
    }
    if !ignore.is_empty() {
      let mut overrides = OverrideBuilder::new(&root);
      for i in &ignore {
        let mut p = "!".to_owned();
        p.push_str(media::make_relative_string(i, root.as_path()).as_str());
        overrides
          .add(&p)
          .unwrap_or_else(|e| panic!("Adding exclude pattern {:?} failed: {}", i, e));
      }
      let o = overrides
        .build()
        .unwrap_or_else(|e| panic!("Applying files.exclude from {:?} failed: {}", config, e));
      dir_walker.overrides(o);
    }
    for i in dirs.into_iter().skip(1) {
      dir_walker.add(media::make_relative_string(&i, &root));
    }
    if !patterns.is_empty() {
      let mut overrides = OverrideBuilder::new(&root);
      for i in &patterns {
        overrides
          .add(media::make_relative_string(i, &root).as_str())
          .unwrap_or_else(|e| panic!("Adding include pattern {:?} failed: {}", i, e));
      }
      let o = overrides
        .build()
        .unwrap_or_else(|e| panic!("Applying files.include from {:?} failed: {}", config, e));
      dir_walker.overrides(o);
    }
    for entry in dir_walker.build().filter_map(|v| v.ok()) {
      let p = entry.path();
      if p.is_file() {
        if no_lint {
          let name = media::make_relative(p, &proj);
          println!(
            "{}",
            name
              .as_path()
              .to_str()
              .unwrap_or_else(|| { panic!("Converting path to string failed: {:?}", p) })
          )
        } else {
          match lint_file(p, &proj, rules.clone(), format.clone()) {
            Ok(b) => ok = ok && b,
            Err(e) => {
              eprintln!("{e}\n");
              ok = false
            }
          }
        }
      }
    }
  }

  if no_lint {
    for i in &files {
      let name = media::make_relative(Path::new(i), &proj);
      println!(
        "{}",
        name
          .as_path()
          .to_str()
          .unwrap_or_else(|| { panic!("Converting path to string failed: {:?}", i) })
      );
    }
  } else {
    for i in &files {
      match lint_file(Path::new(i), &proj, rules.clone(), format.clone()) {
        Ok(b) => ok = ok && b,
        Err(e) => {
          eprintln!("{e}\n");
          ok = false
        }
      }
    }
  }

  Ok(ok)
}
