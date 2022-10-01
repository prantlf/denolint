extern crate global_alloc;
extern crate shared;

use std::env;
use std::process::ExitCode;

fn help() {
  println!(
    "Checks syntax of JavaScript and TypeScript source files.

Usage: denolint [options] [directory...]

Options:
  -i|--ignore-pattern <pattern>  file patterns to ignore during scanning
  -p|--project <directory>       project root directory (default: cwd)
  -c|--config <file>             config file (default: .denolint.json)
  --no-config                    disable searching for the default config
  -V|--version                   print version number and exit
  -h|--help                      print usage instructions and exit

The project directory will be scanned if no directory is specified either on
the command line or in the config file. Directories on the command line take
precedence over the directories in the config file.

Examples:
  $ denolint
  $ denolint -c .denolint-tests.json tests"
  );
}

fn main() -> ExitCode {
  let mut proj_dir: Option<String> = None;
  let mut config_path: Option<String> = None;
  let mut dirs = vec![];
  let mut ignore_patterns = vec![];

  let args: Vec<String> = env::args().collect();
  let mut i = 1;
  let l = args.len();
  while i < l {
    let arg = &args[i];
    match arg.as_str() {
      "-i" | "--ignore-pattern" => {
        i += 1;
        if i == l {
          eprintln!("missing ignore pattern");
          return ExitCode::from(1);
        }
        ignore_patterns.push(args[i].clone());
      }
      "-p" | "--project" => {
        i += 1;
        if i == l {
          eprintln!("missing project directory");
          return ExitCode::from(1);
        }
        proj_dir = Some(args[i].clone());
      }
      "-c" | "--config" => {
        i += 1;
        if i == l {
          eprintln!("missing config file path");
          return ExitCode::from(1);
        }
        config_path = Some(args[i].clone());
      }
      "--no-config" => {
        config_path = Some("".to_string());
      }
      "-h" | "--help" => {
        help();
        return ExitCode::from(0);
      }
      "-V" | "--version" => {
        println!("1.1.4");
        return ExitCode::from(0);
      }
      &_ => dirs.push(arg.clone()),
    }
    i += 1;
  }

  match shared::denolint(proj_dir, config_path, Some(dirs), Some(ignore_patterns)) {
    Ok(ok) => ExitCode::from(if ok { 0 } else { 1 }),
    Err(e) => {
      eprintln!("{e}");
      ExitCode::from(1)
    }
  }
}
