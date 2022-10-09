# denolint

[![Latest version](https://img.shields.io/npm/v/denolint)
 ![Dependency status](https://img.shields.io/librariesio/release/npm/denolint)
](https://www.npmjs.com/package/denolint)

> [Deno lint] command-line executable for Node.js

A lot faster than [eslint], handling both JavaScript and TypeScript sources.

This project started as fork of [customised @node-rs/deno-lint], adding the flexibility of `eslint`:

* Scan specific directories with specific ignore patterns ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Execute the command-line tool `denolint` without loading the Node.js VM ([648])
* Allow specifying dirdctories, files and patterns as input for checking

## Synopsis

Scan sources in the current directory:

    $ npx denolint

    no-var

      × `var` keyword is not allowed.
      ╭─[src/index.js:3:3]
    3 │ export function answer() {
    4 │   var answer = 42
      ·   ───────────────
    5 │   return answer
      ╰────
      help: https://lint.deno.land/#no-var

    Lint failed: Unexpected token `return`. Expected this, import, async, function,
    [ for array literal, { for object literal, @ for decorator, function, class,
    null, true, false, number, bigint, string, regexp, ` for template literal, (,
    or an identifier at test/index.js:4:3

Scan sources in the test subdirectory using an alternative config file:

    npx denolint -c .denolint-tests.json tests

## Usage

    npx denolint [options] [dirs...]

The project directory will be scanned if no directory is specified either on the command line or in the config file. Directories on the command line take precedence over the directories in the config file.

### `--format`, `-f`

Format of warnings printed on stderr. Either `compact` or `pretty` (default).

### `--project`, `-p`

Root directory of the project, which contains files `.denolintignore` or `.eslintignore`. Defaults to the current directory.

### `--config`, `-c`

Config path relative to the lint path. If not provided, a `.denolint.json` in the project directory will be tried to load.

Config file must be a JSON file:

Example:

```json
{
  "files": {
    "include": ["src", "test"],
    "exclude": ["examples"]
  },
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

### `--dry-run`

Only lists the files names, which would be processed, without checking their syntax. It can be used to learn what files will be processed.

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

If you used the `-r, --root` argument from the [customised @node-rs/deno-lint], change it to the `-p, --project` argument:

```diff
- denolint -r package/cmd
+ denolint -p package/cmd
```

The argument `--check-only` known from [@node-rs/deno-lint] is not supported. If you needed to ignore the exit code, use an additional executable or shell alias, which exits successfully, for example:

```diff
- npx denolint--check-only
+ npx denolint || true
```

## Troubleshooting

If the installation fails, set the environment variable `DENOLINT_DEBUG` to `true` and watch the debugging output on the console.

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
| Linux arm64 musl | ×      | ×      | ×      |
| Android arm64    | ×      | ×      | ×      |
| Android armv7    | ×      | ×      | ×      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## Contributing

In lieu of a formal styleguide, take care to maintain the existing coding style. Add unit tests for any new or changed functionality. Lint and test your code.

## License

Copyright (c) 2020-2022 LongYinan<br>
Copyright (c) 2022 Ferdinand Prantl

Licensed under the [MIT] license.

[deno lint]: https://github.com/denoland/deno_lint#readme
[customised @node-rs/deno-lint]: https://github.com/prantlf/node-rs/commits/combined
[@node-rs/deno-lint]: https://github.com/napi-rs/node-rs/tree/main/packages/deno-lint#readme
[deno_lint rules]: https://github.com/denoland/deno_lint/tree/main/docs/rules
[eslint]: https://eslint.org/
[630]: https://github.com/napi-rs/node-rs/issues/630
[631]: https://github.com/napi-rs/node-rs/issues/631
[635]: https://github.com/napi-rs/node-rs/issues/635
[642]: https://github.com/napi-rs/node-rs/pull/642
[645]: https://github.com/napi-rs/node-rs/pull/645
[646]: https://github.com/napi-rs/node-rs/pull/646
[647]: https://github.com/napi-rs/node-rs/pull/647
[648]: https://github.com/napi-rs/node-rs/issues/648
[MIT]: https://github.com/prantlf/denolint/blob/master/LICENSE
