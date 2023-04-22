extern crate napi_derive;
extern crate shared;

use std::path::Path;
use std::str;

use deno_lint::linter::LinterBuilder;
use napi::bindgen_prelude::*;
use napi::{Env, Result, Task};
use napi_derive::*;

use shared::config;
use shared::diagnostics::{display_diagnostics, is_compact};
use shared::media;

#[napi(object)]
pub struct LintOptions {
  pub all_rules: Option<bool>,
  pub exclude_rules: Option<Vec<String>>,
  pub include_rules: Option<Vec<String>>,
  pub format: Option<String>,
}

struct AsyncLint {
  file_name: String,
  source_code: Either<String, Buffer>,
  options: Option<LintOptions>,
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
      self.options.as_ref().map(|o| LintOptions {
        all_rules: o.all_rules,
        exclude_rules: o.exclude_rules.clone(),
        include_rules: o.include_rules.clone(),
        format: o.format.clone(),
      }),
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
  options: Option<LintOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<AsyncLint> {
  AsyncTask::with_optional_signal(
    AsyncLint {
      file_name,
      source_code,
      options,
    },
    signal,
  )
}

#[napi]
fn lint_sync(
  file_name: String,
  source_code: Either<String, Buffer>,
  options: Option<LintOptions>,
) -> Result<Vec<String>> {
  let all_rules: Option<bool>;
  let exclude_rules: Option<Vec<String>>;
  let include_rules: Option<Vec<String>>;
  let format: Option<String>;
  match options {
    Some(o) => {
      all_rules = o.all_rules;
      exclude_rules = o.exclude_rules;
      include_rules = o.include_rules;
      format = o.format;
    }
    None => {
      all_rules = None;
      exclude_rules = None;
      include_rules = None;
      format = None;
    }
  };

  let compact_result = is_compact(format);
  if let Err(err) = compact_result {
    return Err(Error::new(Status::InvalidArg, err.to_string()));
  }
  let compact = compact_result.unwrap();

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
        format!("Input source is not valid utf8 string {e}"),
      )
    })?,
  };

  let (s, file_diagnostics) = linter
    .lint(file_name.clone(), source_string.to_owned())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        media::format_error(compact, &e.to_string(), &file_name),
      )
    })?;

  display_diagnostics(&file_diagnostics, s.text_info(), &file_name, compact)
    .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
}
