extern crate napi_derive;
extern crate shared;

use std::path::Path;
use std::str;

use deno_lint::linter::LinterBuilder;
use napi::bindgen_prelude::*;
use napi::{Env, Result, Task};
use napi_derive::*;

use shared::config;
use shared::diagnostics;
use shared::media;

struct AsyncLint {
  file_name: String,
  source_code: Either<String, Buffer>,
  all_rules: Option<bool>,
  exclude_rules: Option<Vec<String>>,
  include_rules: Option<Vec<String>>,
}

impl Task for AsyncLint {
  type Output = Vec<String>;
  type JsValue = Vec<String>;

  fn compute(&mut self) -> Result<Self::Output> {
    let source: Either<String, Buffer> = match &self.source_code {
      Either::A(s) => Either::A(s.clone()),
      Either::B(b) => Either::B(Buffer::from(b.as_ref().to_vec())),
    };
    lint_sync(
      self.file_name.clone(),
      source,
      self.all_rules,
      self.exclude_rules.clone(),
      self.include_rules.clone(),
    )
  }

  fn resolve(&mut self, _env: Env, output: Vec<String>) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn lint(
  file_name: String,
  source_code: Either<String, Buffer>,
  all_rules: Option<bool>,
  exclude_rules: Option<Vec<String>>,
  include_rules: Option<Vec<String>>,
  signal: Option<AbortSignal>,
) -> AsyncTask<AsyncLint> {
  AsyncTask::with_optional_signal(
    AsyncLint {
      file_name,
      source_code,
      all_rules,
      exclude_rules,
      include_rules,
    },
    signal,
  )
}

#[napi]
fn lint_sync(
  file_name: String,
  source_code: Either<String, Buffer>,
  all_rules: Option<bool>,
  exclude_rules: Option<Vec<String>>,
  include_rules: Option<Vec<String>>,
) -> Result<Vec<String>> {
  let linter = LinterBuilder::default()
    .rules(config::filter_rules(
      all_rules.unwrap_or(false),
      exclude_rules,
      include_rules,
    ))
    .media_type(media::get_media_type(Path::new(file_name.as_str())))
    .ignore_file_directive("eslint-disable")
    .ignore_diagnostic_directive("eslint-disable-next-line")
    .build();

  let source_string = match &source_code {
    Either::A(s) => s,
    Either::B(b) => str::from_utf8(b.as_ref()).map_err(|e| {
      Error::new(
        Status::StringExpected,
        format!("Input source is not valid utf8 string {}", e),
      )
    })?,
  };

  let (s, file_diagnostics) = linter
    .lint(file_name.clone(), source_string.to_owned())
    .map_err(|e| {
      let suffix = if e.to_string().contains(&file_name) {
        "".to_string()
      } else {
        format!(", at: {:?}", &file_name)
      };
      Error::new(
        Status::GenericFailure,
        format!("Lint failed: {}{}", e, &suffix),
      )
    })?;

  diagnostics::display_diagnostics(&file_diagnostics, s.text_info(), &file_name)
    .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
}
