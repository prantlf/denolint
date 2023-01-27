# libdenolint

[![Latest version](https://img.shields.io/npm/v/libdenolint)
 ![Dependency status](https://img.shields.io/librariesio/release/npm/libdenolint)
](https://www.npmjs.com/package/libdenolint)

> [Deno lint] library binding for Node.js

Used in bundler plugins: [rollup-plugin-denolint] for [Rollup], [esbuild-plugin-denolint] for [esbuild], [webpack-loader-denolint] for [Webpack].

This project started as fork of [customised @node-rs/deno-lint], adding the flexibility of `eslint`:

* Scan specific directories with specific ignore patterns ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Support including and excluding rules in the `lint` method ([631], [643])
* Offer both synchronous and asynchronous methods ([650])
* Allow specifying directories, files and patterns as input for checking
* Print warning messages in pretty or compact formats

## Synopsis

Scan sources in the current directory:

```js
import { denolint } from 'libdenolint'

process.exitCode = await denolint() ? 0 : 1
```

Check one source file:

```js
import { readFile } from 'fs/promises'
import { lint } from 'libdenolint'

const filepath = 'src/index.js'
const source = await readFile(filepath)

try {
  const warnings = await lint(filepath, source)
  for (const warning of warnings) console.warn(warning)
} catch ({ message }) {
  console.error(message)
}
```

## Usage of `denolint`

```ts
import { denolint, denolintSync } from 'libdenolint'

interface DenoLintOptions {
  scanDirs?: string[],
  ignorePatterns?: string[]
}

function denolint(
  projectDir?: string, configPath?: string,
  options?: DenoLintOptions, signal?: AbortSignal
): Promise<boolean>

function denolintSync(
  projectDir?: string, configPath?: string, options?: DenoLintOptions
): boolean
```

The project directory will be scanned if no directory is specified either on the command line or in the config file. Directories from `scanDirs` take precedence over the directories in the config file.

Returns `true` if all files are valid, otherwise `false`.

### `projectDir`

- Type: `String`
- Default: `current directory`

Root directory of the project, which contains files `.denolintignore` or `.eslintignore`. Defaults to the current directory.

### `configPath`

- Type: `String`
- Default: `'.denolint.json'`

Config path relative to the lint path. If not provided, a `.denolint.json` in the project directory will be tried to load. If an empty string is specified explicitly, it will disable searching for the default config file.

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

### `scanDirs`

- Type: `String[]`
- Default: `[]`

Directories to scan for sources. The project directory will be scanned if none specified.

### `ignorePatterns`

- Type: `String[]`
- Default: `[]`

File patterns to ignore when the source directories are scanned. I applies only to directories specified on the command line; not to the directories specified by `files.include` from the config file.

### `dryRun`

- Type: `Boolean`
- Default: `false`

Only lists the files names, which would be processed, without checking their syntax. It can be used to learn what files will be processed.

### `signal`

- Type: `AbortSignal`
- Default: `undefined`

A signal from an `AbortController` to abort the operation. Optional.

## Usage of `lint`

```ts
import { lint, lintSync } from 'libdenolint'

function lint(
  fileName: string, sourceCode: string | Buffer,
  allRules?: boolean, excludeRules?: string[], includeRules?: string[],
  signal?: AbortSignal
): Promise<Array<string>>

function lintSync(
  fileName: string, sourceCode: string | Buffer,
  allRules?: boolean, excludeRules?: string[], includeRules?: string[]
): Array<string>
```

Returns an empty array if the files is valid, otherwise the array will contain printable warnings about the problems found. May throw an error in case of a fatal parser failure.

### `fileName`

- Type: `String`

The name of the source file for diagnostic messages.

### `sourceCode`

- Type: `String | Buffer`

The contents of the source file to be checked.

### `allRules`

- Type: `Boolean`
- Default: `false`

Whether to enable all rules. If `false`, `denolint` will enable only the recommend rules.

### `excludeRules`

- Type: `String[]`
- Default: `[]`

Rules to exclude from all or recommended ones chosen by `enableAllRules`.

### `includeRules`

- Type: `String[]`
- Default: `[]`

Rules to include in addition to the recommended ones chosen by `enableAllRules` set to `false`.

### `signal`

- Type: `AbortSignal`
- Default: `undefined`

A signal from an `AbortController` to abort the operation. Optional.

## Migration from `@node-rs/deno-lint`

Methods `denolint` and `lint` are synchronous. If you want to retain the synchronous behaviour, call the methods with the `Sync` suffix:

```diff
- denolint(process.cwd(), '.denolint.json')
+ denolintSync(process.cwd(), '.denolint.json')

- lint(fileName, fileContents)
+ lintSync(fileName, fileContents)
```

Methods `denolint` and `denolintSync` return `boolean` (or a `Promise` to a `boolean`) with the result of the syntax check. The value `true` means success, the value `false` means a failure. Pay attention, the `boolean` retuurned by the method `denolint` from [@node-rs/deno-lint] has the meaning the other way round!

## Troubleshooting

If the installation fails, set the environment variable `LIBDENOLINT_DEBUG` to `true` and watch the debugging output on the console.

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
Copyright (c) 2023 Ferdinand Prantl

Licensed under the [MIT] license.

[deno lint]: https://github.com/denoland/deno_lint#readme
[customised @node-rs/deno-lint]: https://github.com/prantlf/node-rs/commits/combined
[@node-rs/deno-lint]: https://github.com/napi-rs/node-rs/tree/main/packages/deno-lint#readme
[deno_lint rules]: https://github.com/denoland/deno_lint/tree/main/docs/rules
[esbuild-plugin-denolint]: https://github.com/prantlf/esbuild-plugin-denolint#readme
[esbuild]: https://esbuild.github.io/
[rollup-plugin-denolint]: https://github.com/prantlf/rollup-plugin-denolint#readme
[Rollup]: https://rollupjs.org/
[webpack-loader-denolint]: https://github.com/prantlf/webpack-loader-denolint#readme
[Webpack]: https://webpack.js.org/
[630]: https://github.com/napi-rs/node-rs/issues/630
[631]: https://github.com/napi-rs/node-rs/issues/631
[635]: https://github.com/napi-rs/node-rs/issues/635
[642]: https://github.com/napi-rs/node-rs/pull/642
[643]: https://github.com/napi-rs/node-rs/pull/643
[645]: https://github.com/napi-rs/node-rs/pull/645
[646]: https://github.com/napi-rs/node-rs/pull/646
[647]: https://github.com/napi-rs/node-rs/pull/647
[650]: https://github.com/napi-rs/node-rs/issues/650
[MIT]: https://github.com/prantlf/denolint/blob/master/LICENSE
