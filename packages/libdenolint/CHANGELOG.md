## [2.0.4](https://github.com/prantlf/denolint/compare/v2.0.2...v2.0.4) (2022-10-29)

### Bug Fixes

* Upgrade dependencies ([2b89960](https://github.com/prantlf/denolint/commit/2b8996029dfdca64ab784fc50c781377d90ca055))

## [2.0.2](https://github.com/prantlf/denolint/compare/v2.0.0...v2.0.2) (2022-10-09)

### Bug Fixes

* Use the compact format known from unix tools ([335cd8a](https://github.com/prantlf/denolint/commit/335cd8a42270c326af17c406edcdec51cf60f26a))

# [2.0.0](https://github.com/prantlf/denolint/compare/v1.1.6...v2.0.0) (2022-10-09)

### Features

* Add dry-run option to print found file names without checking ([5030b2a](https://github.com/prantlf/denolint/commit/5030b2a75a21a5f8efad96fa0f7831ef4dff8f29))
* **libdenolint:** Add an option to print warnings in compact format ([c812d3e](https://github.com/prantlf/denolint/commit/c812d3e5ac3035d2d2fc5321a1435db36e1bcbea))

### BREAKING CHANGES

* **libdenolint:** All public methods require the optional parameters
to be specified as an object. The alternative listing the options
as separate function arguments has been removed. (Making the `signal`
argument alternating is not possible using the current NAPI.)

How to update the source code:

```diff
  denolint(projectDir, configPath,
-   scanDirs, ignorePatterns, signal)
+   { scanDirs, ignorePatterns }, signal)

  denolintSync(projectDir, configPath,
-   scanDirs, ignorePatterns)
+   { scanDirs, ignorePatterns })

  lint(fileName, sourceCode,
-   allRules, excludeRules, includeRules, signal)
+   { allRules, excludeRules, includeRules }, signal)

  lintSync(fileName, sourceCode,
-   allRules, excludeRules, includeRules)
+   { allRules, excludeRules, includeRules })
```

The interface of the command-line tool was not affected by this change.

## [1.1.6](https://github.com/prantlf/denolint/compare/v1.1.5...v1.1.6) (2022-10-09)

### Bug Fixes

* Add installation debug logging ([f7f8410](https://github.com/prantlf/denolint/commit/f7f8410ddc545ebb4c3445491322df9e9b5e8ea1))
* Remove publishing settings from package.json ([fb13c61](https://github.com/prantlf/denolint/commit/fb13c615d9cd499c67527d4b3bd644d58b30f724))

## [1.1.5](https://github.com/prantlf/denolint/compare/v1.1.3...v1.1.5) (2022-10-01)

### Bug Fixes

* **libdenolint:** Fix upgrade of an already installed package ([9533373](https://github.com/prantlf/denolint/commit/95333731dd9f3c0e3dcd6e73a1456fce19706e04))

## [1.1.3](https://github.com/prantlf/denolint/compare/v1.1.1...v1.1.3) (2022-10-01)

### Bug Fixes

* **libdenolint:** Remove the sel-package link from dev dependencies ([f2cc0fe](https://github.com/prantlf/denolint/commit/f2cc0fe01f9392c95b30477cb3e6f3c8f5caeb10))
* Separate fatal parser errors with an extra line break ([7ece377](https://github.com/prantlf/denolint/commit/7ece377891fa3039b9c074ba6b481f685e9752e0))

# [1.1.1](https://github.com/prantlf/denolint/compare/v1.1.0...v1.1.1) (2022-10-01)

Publish missing native packages.

# [1.1.0](https://github.com/prantlf/denolint/compare/v1.0.82...v1.1.0) (2022-09-30)

### Bug Fixes

* Separate fatal parser errors with an extra line break ([7ece377](https://github.com/prantlf/denolint/commit/7ece377891fa3039b9c074ba6b481f685e9752e0))

### Features

* Add platforms win32-ia32, linux-arm, linux-arm64, linux-musl and freebsd ([046a336](https://github.com/prantlf/denolint/commit/046a336fa403df8875a7015e797bfa98ea74bbb9))
* Allow specifying ignore patterns on the command line ([217b756](https://github.com/prantlf/denolint/commit/217b7565e179c3958dbcadd619c015dd8eb239b3))

## [1.0.82](https://github.com/prantlf/denolint/compare/v1.0.3...v1.0.82) (2022-09-29)

### Bug Fixes

* Work around malfunctioning creation of symlinks in .bin ([a6a3d22](https://github.com/prantlf/denolint/commit/a6a3d22b2bc0997072e26d1b2b33321fb01e5c94))

## [1.0.3](https://github.com/prantlf/denolint/compare/v1.0.2...v1.0.3) (2022-09-29)

### Bug Fixes

* Fix incomplete publishing of native packages ([7065c24](https://github.com/prantlf/denolint/commit/7065c2466f62733369e364bd3edc007e4acc6d97))

## [1.0.2](https://github.com/prantlf/denolint/compare/v1.0.1...v1.0.2) (2022-09-29)

### Bug Fixes

* **libdenolint:** Fix automatic publishing of native packages ([e49a42e](https://github.com/prantlf/denolint/commit/e49a42ecf462dc1992b19955ded88f966c14391c))

## [1.0.1](https://github.com/prantlf/denolint/compare/v1.0.0...v1.0.1) (2022-09-29)

### Bug Fixes

* Install platform packages using optional dependencies ([bfaf001](https://github.com/prantlf/denolint/commit/bfaf00136b5462fc216c8cb56d9c95224ecd056f))

# 1.0.0 (2022-09-28)

This is the first version released after forking the [customisation] of the [original project].

* Scan specific directories ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Support including and excluding rules in the `lint` method ([631], [643])
* Offer both synchronous and asynchronous methods ([650])
* Allow specifying directories, files and patterns as input for checking

[customisation]: https://github.com/prantlf/node-rs/commits/combined
[original project]: https://github.com/napi-rs/node-rs/tree/main/packages/deno-lint
[630]: https://github.com/napi-rs/node-rs/issues/630
[631]: https://github.com/napi-rs/node-rs/issues/631
[635]: https://github.com/napi-rs/node-rs/issues/635
[642]: https://github.com/napi-rs/node-rs/pull/642
[643]: https://github.com/napi-rs/node-rs/pull/643
[645]: https://github.com/napi-rs/node-rs/pull/645
[646]: https://github.com/napi-rs/node-rs/pull/646
[647]: https://github.com/napi-rs/node-rs/pull/647
[650]: https://github.com/napi-rs/node-rs/issues/650
