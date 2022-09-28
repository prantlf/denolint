# denolint

![denolint](https://img.shields.io/npm/v/denolint.svg)

> [Deno lint] command-line executable for Node.js

This project started as fork of [customised @node-rs/deno-lint], adding the flexibility of `eslint`:

* Scan specific directories ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Execute the command-line tool `denolint` without loading the Node.js VM ([648])
* Allow specifying dirdctories, files and patterns as input for checking

## Synopsis

Scan sources in the current directory:

    npx denolint

Scan sources in the test subdirectory using an alternative config file:

    npx denolint -c .denolint-tests.json tests

## Usage

    npx denolint [options] [dirs...]

The project directory will be scanned if no directory is specified either on the command line or in the config file. Directories on the command line take precedence over the directories in the config file.

### `--project`, `-p`

Root directory of the project, which contains files `.denolintignore` or `.eslintignore`. Defaults to the current directory.

### `--config`, `-c`

Config path relative to the lint path. If not provided, a `.denolint.json` in the project directory will be tried to load.

Config file must be a JSON file:

Example:

```json
{
  "rules": {
    "tags": ["recommended"],
    "exclude": [
      "no-explicit-any",
      "ban-unknown-rule-code",
      "no-window-prefix",
      "no-empty-interface",
      "ban-types",
      "ban-untagged-todo",
      "no-unused-vars",
      "ban-ts-comment",
      "no-case-declarations",
      "no-this-alias"
    ]
  }
}
```

Check out [deno_lint rules] for all rules.

### `--no-config`

Disables searching for the default config file if no specific config file is provided.

### `--help`, `-h`

Prints the usage instructions and exits.

### `--version`, `-V`

Prints the version of the package and exits.

## Migration from `@node-rs/deno-lint`

If you specified the project root directory, do it using the `-p, --project` argument:

```diff
- npx denolint package/cmd
+ npx denolint -p package/cmd
```

The argument `--check-only` is not supported. If you needed to ignore the exit code, use an additional executable or shell alias, which exits successfully, for example:

```diff
- npx denolint--check-only
+ npx denolint || true
```

## Support matrix

| Platform         | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| MacOS x64        | ✓      | ✓      | ✓      |
| MacOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

[deno lint]: https://github.com/denoland/deno_lint
[customised @node-rs/deno-lint]: https://github.com/prantlf/node-rs/commits/combined
[deno_lint rules]: https://github.com/denoland/deno_lint/tree/main/docs/rules
[630]: https://github.com/napi-rs/node-rs/issues/630
[631]: https://github.com/napi-rs/node-rs/issues/631
[635]: https://github.com/napi-rs/node-rs/issues/635
[642]: https://github.com/napi-rs/node-rs/pull/642
[645]: https://github.com/napi-rs/node-rs/pull/645
[646]: https://github.com/napi-rs/node-rs/pull/646
[647]: https://github.com/napi-rs/node-rs/pull/647
[648]: https://github.com/napi-rs/node-rs/issues/648
