extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::{Env, Result, Task};
use napi_derive::*;

struct AsyncDenoLint {
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_dirs: Option<Vec<String>>,
}

impl Task for AsyncDenoLint {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    denolint_sync(
      self.proj_dir.clone(),
      self.config_path.clone(),
      self.scan_dirs.clone(),
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
  scan_dirs: Option<Vec<String>>,
  signal: Option<AbortSignal>,
) -> AsyncTask<AsyncDenoLint> {
  AsyncTask::with_optional_signal(
    AsyncDenoLint {
      proj_dir,
      config_path,
      scan_dirs,
    },
    signal,
  )
}

#[napi]
fn denolint_sync(
  proj_dir: Option<String>,
  config_path: Option<String>,
  scan_dirs: Option<Vec<String>>,
) -> Result<bool> {
  match shared::denolint(proj_dir, config_path, scan_dirs) {
    Ok(s) => Ok(s),
    Err(e) => Err(Error::new(Status::GenericFailure, format!("{e}"))),
  }
}
