extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::{Env, Result, Task};
use napi_derive::*;

#[napi(object)]
pub struct DenoLintOptions {
  pub scan_dirs: Option<Vec<String>>,
  pub ignore_patterns: Option<Vec<String>>,
}

struct AsyncDenoLint {
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_dirs: Option<Vec<String>>,
  ignore_patterns: Option<Vec<String>>,
}

impl Task for AsyncDenoLint {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    denolint_sync(
      self.proj_dir.clone(),
      self.config_path.clone(),
      Some(Either::B(DenoLintOptions {
        scan_dirs: self.scan_dirs.clone(),
        ignore_patterns: self.ignore_patterns.clone(),
      })),
    )
  }

  fn resolve(&mut self, _env: Env, output: bool) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn denolint(
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_or_options: Option<Either<Vec<String>, DenoLintOptions>>,
  signal: Option<AbortSignal>,
) -> AsyncTask<AsyncDenoLint> {
  let scan_dirs: Option<Vec<String>>;
  let ignore_patterns: Option<Vec<String>>;
  match scan_or_options {
    Some(e) => match e {
      Either::A(v) => {
        scan_dirs = Some(v);
        ignore_patterns = None;
      }
      Either::B(o) => {
        scan_dirs = o.scan_dirs;
        ignore_patterns = o.ignore_patterns;
      }
    },
    None => {
      scan_dirs = None;
      ignore_patterns = None;
    }
  };
  AsyncTask::with_optional_signal(
    AsyncDenoLint {
      proj_dir,
      config_path,
      scan_dirs,
      ignore_patterns,
    },
    signal,
  )
}

#[napi]
fn denolint_sync(
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_or_options: Option<Either<Vec<String>, DenoLintOptions>>,
) -> Result<bool> {
  let scan_dirs: Option<Vec<String>>;
  let ignore_patterns: Option<Vec<String>>;
  match scan_or_options {
    Some(e) => match e {
      Either::A(v) => {
        scan_dirs = Some(v);
        ignore_patterns = None;
      }
      Either::B(o) => {
        scan_dirs = o.scan_dirs;
        ignore_patterns = o.ignore_patterns;
      }
    },
    None => {
      scan_dirs = None;
      ignore_patterns = None;
    }
  };
  match shared::denolint(proj_dir, config_path, scan_dirs, ignore_patterns) {
    Ok(s) => Ok(s),
    Err(e) => Err(Error::new(Status::GenericFailure, format!("{e}"))),
  }
}
