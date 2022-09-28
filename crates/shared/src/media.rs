use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deno_ast::MediaType;

pub fn get_media_type(p: &Path) -> MediaType {
  match p.extension().and_then(|e| e.to_str()) {
    Some("tsx") => MediaType::Tsx,
    Some("jsx") => MediaType::Jsx,
    Some("js") | Some("mjs") => MediaType::JavaScript,
    Some("ts") => MediaType::TypeScript,
    _ => MediaType::Tsx,
  }
}

pub fn make_absolute(p: &String, proj: &Path) -> PathBuf {
  let path = Path::new(p);
  if path.is_absolute() {
    return PathBuf::from(path);
  }
  let mut buf = proj.to_path_buf();
  buf.push(path);
  match fs::canonicalize(buf) {
    Ok(p) => {
      // workaround for UNC path see https://github.com/rust-lang/rust/issues/42869
      if p.starts_with(r"\\?\") {
        match p.to_str() {
          Some(s) => PathBuf::from(&s[4..]),
          None => p,
        }
      } else {
        p
      }
    }
    Err(_) => PathBuf::from(p),
  }
}

pub fn classify_paths(paths: &Vec<String>, proj: &Path) -> (Vec<String>, Vec<String>, Vec<String>) {
  let mut dirs: Vec<String> = vec![];
  let mut files: Vec<String> = vec![];
  let mut patterns: Vec<String> = vec![];
  for p in paths {
    let path = make_absolute(p, proj);
    match fs::metadata(&path) {
      Ok(m) => {
        if m.is_dir() {
          match path.as_path().to_str() {
            Some(s) => dirs.push(s.to_string()),
            None => panic!("Convert path to string failed: {:?}", p),
          }
        } else {
          match path.as_path().to_str() {
            Some(s) => files.push(s.to_string()),
            None => panic!("Convert path to string failed: {:?}", p),
          }
        }
      }
      Err(_) => {
        if p.contains('*') {
          patterns.push(p.clone())
        } else {
          files.push(p.clone())
        }
      }
    }
  }
  (dirs, files, patterns)
}
