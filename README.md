# denolint

> Monorepo for calling `deno lint` in Node.js projects using the Rust package [deno_lint].

This project started as fork of a [customised @node-rs/deno-lint], adding the flexibility of `eslint`:

* Scan specific directories ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Support including and excluding rules in the `lint` method ([631], [643])
* Execute the command-line tool `denolint` without loading the Node.js VM ([648])
* Offer both synchronous and asynchronous methods ([650])
* Allow specifying directories, files and patterns as input for checking

## Synopsis

Scan sources in two directories on the command line:

    npx denolint src test

Check one source file programmatically:

```js
import { readFile } from 'fs/promises'
import { lint } from 'libdenolint'

const filepath = 'src/index.js'
const source = await readFile(filepath)

const warnings = await lint(filepath, source)
for (const warning of warnings) console.warn(warning)
```

## Packages

See more information about the packages and their compatibility with [@node-rs/deno-lint]:

| Project       | Package                                                             | Description                                   |
| ------------- | ------------------------------------------------------------------- | --------------------------------------------- |
| [denolint]    | [![denolint](https://img.shields.io/npm/v/denolint.svg)][cmd]       | Deno lint command-line executable for Node.js |
| [libdenolint] | [![libdenolint](https://img.shields.io/npm/v/libdenolint.svg)][lib] | Deno lint library binding for Node.js         |

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

[deno_lint]: https://github.com/denoland/deno_lint
[customised @node-rs/deno-lint]: https://github.com/prantlf/node-rs/commits/combined
[@node-rs/deno-lint]: https://github.com/napi-rs/node-rs/tree/main/packages/deno-lint
[denolint]: https://github.com/prantlf/denolint/tree/master/packages/denolint
[cmd]: https://www.npmjs.com/package/denolint
[libdenolint]: https://github.com/prantlf/denolint/tree/master/packages/libdenolint
[lib]: https://www.npmjs.com/package/libdenolint
[630]: https://github.com/napi-rs/node-rs/issues/630
[631]: https://github.com/napi-rs/node-rs/issues/631
[635]: https://github.com/napi-rs/node-rs/issues/635
[642]: https://github.com/napi-rs/node-rs/pull/642
[643]: https://github.com/napi-rs/node-rs/pull/643
[645]: https://github.com/napi-rs/node-rs/pull/645
[646]: https://github.com/napi-rs/node-rs/pull/646
[647]: https://github.com/napi-rs/node-rs/pull/647
[648]: https://github.com/napi-rs/node-rs/issues/648
[650]: https://github.com/napi-rs/node-rs/issues/650