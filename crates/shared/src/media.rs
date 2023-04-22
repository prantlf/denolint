use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deno_ast::MediaType;
use pathdiff;

pub fn format_error(compact: bool, msg: &String, path: &String) -> String {
  if compact {
    compact_error(msg, path)
  } else {
    pretty_error(msg, path)
  }
}

pub fn compact_error(msg: &String, path: &String) -> String {
  if let Some(loc) = msg.find(format!(" at {path}").as_str()) {
    let from_loc = &msg[(loc + 4)..];
    let parts = from_loc.lines().collect::<Vec<_>>();
    if parts.len() == 1 {
      // Single-line message with the location at the end, for example:
      // Unexpected token `return`. Expected this... at test/samples/fail/ultimate.txt:4:3
      // Output:
      // test/samples/fail/ultimate.txt:4:3: Expected this...
      format!("{}: {}", &parts[0], &msg[0..loc])
    } else {
      // Multi-line message with the location at the firsts line, for example:
      // Expression expected at test/samples/fail/ultimate.txt:4:3
      //
      //     return answer
      //     ~~~~~~
      // Output:
      // test/samples/fail/ultimate.txt:4:3: Expression expected: return answer
      let path_with_loc = parts[0];
      let code_lines = &parts[1..];
      let mut code = String::new();
      for line in code_lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.chars().all(|x| x == '~') {
          if code.is_empty() {
            code.push(':');
          }
          code.push(' ');
          code.push_str(trimmed);
        }
      }
      format!("{}: {}{}", path_with_loc, &msg[0..loc], &code)
    }
  } else {
    format!("{path}: {msg}")
  }
}

pub fn pretty_error(msg: &String, path: &String) -> String {
  if msg.contains(path) {
    msg.clone()
  } else {
    format!("{msg} at {path}")
  }
}

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
        None => panic!("Converting path to string failed: {c:?}"),
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
    None => panic!("Converting path to string failed: {r:?}"),
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
      None => panic!("Converting path to string failed: {r:?}"),
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
            None => panic!("Converting path to string failed: {p:?}"),
          }
        } else {
          match path.as_path().to_str() {
            Some(s) => files.push(s.to_string()),
            None => panic!("Converting path to string failed: {p:?}"),
          }
        }
      }
    }
  }
  (dirs, files, patterns)
}
