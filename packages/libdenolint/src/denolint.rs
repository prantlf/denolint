extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::{Env, Result, Task};
use napi_derive::*;

#[napi(object)]
pub struct DenoLintOptions {
  pub scan_dirs: Option<Vec<String>>,
  pub ignore_patterns: Option<Vec<String>>,
  pub format: Option<String>,
  pub dry_run: Option<bool>,
}

struct AsyncDenoLint {
  proj_dir: Option<String>,
  config_path: Option<String>,
  options: Option<DenoLintOptions>,
}

impl Task for AsyncDenoLint {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    denolint_sync(
      self.proj_dir.clone(),
      self.config_path.clone(),
      self.options.as_ref().map(|o| DenoLintOptions {
        scan_dirs: o.scan_dirs.clone(),
        ignore_patterns: o.ignore_patterns.clone(),
        format: o.format.clone(),
        dry_run: o.dry_run,
      }),
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
  options: Option<DenoLintOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<AsyncDenoLint> {
  AsyncTask::with_optional_signal(
    AsyncDenoLint {
      proj_dir,
      config_path,
      options,
    },
    signal,
  )
}

#[napi]
fn denolint_sync(
  proj_dir: Option<String>,
  config_path: Option<String>,
  options: Option<DenoLintOptions>,
) -> Result<bool> {
  let scan_dirs: Option<Vec<String>>;
  let ignore_patterns: Option<Vec<String>>;
  let format: Option<String>;
  let dry_run: Option<bool>;
  match options {
    Some(o) => {
      scan_dirs = o.scan_dirs;
      ignore_patterns = o.ignore_patterns;
      format = o.format;
      dry_run = o.dry_run;
    }
    None => {
      scan_dirs = None;
      ignore_patterns = None;
      format = None;
      dry_run = None;
    }
  };
  match shared::denolint(
    proj_dir,
    config_path,
    scan_dirs,
    ignore_patterns,
    format,
    dry_run,
  ) {
    Ok(s) => Ok(s),
    Err(e) => Err(Error::new(Status::GenericFailure, format!("{e}"))),
  }
}
