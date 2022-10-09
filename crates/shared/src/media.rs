use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deno_ast::MediaType;
use pathdiff;

pub fn get_media_type(p: &Path) -> MediaType {
  match p.extension().and_then(|e| e.to_str()) {
    Some("tsx") => MediaType::Tsx,
    Some("jsx") => MediaType::Jsx,
    Some("js") | Some("mjs") => MediaType::JavaScript,
    Some("ts") => MediaType::TypeScript,
    _ => MediaType::Tsx,
  }
}

pub fn make_absolute(p: &String, base: &Path) -> PathBuf {
  let path = Path::new(p);
  if path.is_absolute() {
    return PathBuf::from(path);
  }
  let mut buf = base.to_path_buf();
  buf.push(path);
  match fs::canonicalize(buf) {
    Ok(c) => {
      // workaround for UNC path see https://github.com/rust-lang/rust/issues/42869
      if c.starts_with(r"\\?\") {
        match c.to_str() {
          Some(s) => PathBuf::from(&s[4..]),
          None => c,
        }
      } else {
        c
      }
    }
    Err(_) => PathBuf::from(p),
  }
}

pub fn make_absolute_string(p: &String, base: &Path) -> String {
  let path = Path::new(p);
  if path.is_absolute() {
    return p.to_string();
  }
  let mut buf = base.to_path_buf();
  buf.push(path);
  match fs::canonicalize(buf) {
    Ok(c) => {
      match c.as_path().to_str() {
        Some(s) => {
          // workaround for UNC path see https://github.com/rust-lang/rust/issues/42869
          if let Some(t) = s.strip_prefix(r"\\?\") {
            t.to_string()
          } else {
            s.to_string()
          }
        }
        None => panic!("Converting path to string failed: {:?}", c),
      }
    }
    Err(_) => p.to_string(),
  }
}

pub fn make_relative(p: &Path, base: &Path) -> PathBuf {
  let r = pathdiff::diff_paths(p, base).unwrap();
  match r.as_path().to_str() {
    Some(s) => {
      // workaround for UNC path see https://github.com/rust-lang/rust/issues/42869
      if let Some(t) = s.strip_prefix(r"\\?\") {
        PathBuf::from(&t)
      } else {
        PathBuf::from(s)
      }
    }
    None => panic!("Converting path to string failed: {:?}", r),
  }
}

pub fn make_relative_string(p: &String, base: &Path) -> String {
  match pathdiff::diff_paths(p, base) {
    Some(r) => match r.as_path().to_str() {
      Some(s) => {
        // workaround for UNC path see https://github.com/rust-lang/rust/issues/42869
        if let Some(t) = s.strip_prefix(r"\\?\") {
          t.to_string()
        } else {
          s.to_string()
        }
      }
      None => panic!("Converting path to string failed: {:?}", r),
    },
    None => p.to_string(),
  }
}

pub fn classify_paths(paths: &Vec<String>, base: &Path) -> (Vec<String>, Vec<String>, Vec<String>) {
  let mut dirs: Vec<String> = vec![];
  let mut files: Vec<String> = vec![];
  let mut patterns: Vec<String> = vec![];
  for p in paths {
    if p.contains('*') {
      patterns.push(p.clone())
    } else {
      let path = make_absolute(p, base);
      if let Ok(m) = fs::metadata(&path) {
        if m.is_dir() {
          match path.as_path().to_str() {
            Some(s) => dirs.push(s.to_string()),
            None => panic!("Converting path to string failed: {:?}", p),
          }
        } else {
          match path.as_path().to_str() {
            Some(s) => files.push(s.to_string()),
            None => panic!("Converting path to string failed: {:?}", p),
          }
        }
      }
    }
  }
  (dirs, files, patterns)
}
