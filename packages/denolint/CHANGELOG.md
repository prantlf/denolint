## [1.1.2](https://github.com/prantlf/denolint/compare/v1.0.84...v1.1.2) (2022-10-01)

### Bug Fixes

* **denolint:** Support PNPM ([bf50084](https://github.com/prantlf/denolint/commit/bf50084395ee4c0609ba7bedb15fc500cc2d2e7c))
* Separate fatal parser errors with an extra line break ([7ece377](https://github.com/prantlf/denolint/commit/7ece377891fa3039b9c074ba6b481f685e9752e0))

# [1.1.1](https://github.com/prantlf/denolint/compare/v1.1.0...v1.1.1) (2022-10-01)

Publish missing native packages.

# [1.1.0](https://github.com/prantlf/denolint/compare/v1.0.84...v1.1.0) (2022-09-30)

### Bug Fixes

* Separate fatal parser errors with an extra line break ([7ece377](https://github.com/prantlf/denolint/commit/7ece377891fa3039b9c074ba6b481f685e9752e0))

### Features

* Add platforms win32-ia32, linux-arm, linux-arm64, linux-musl and freebsd ([046a336](https://github.com/prantlf/denolint/commit/046a336fa403df8875a7015e797bfa98ea74bbb9))
* Allow specifying ignore patterns on the command line ([217b756](https://github.com/prantlf/denolint/commit/217b7565e179c3958dbcadd619c015dd8eb239b3))

## [1.0.84](https://github.com/prantlf/denolint/compare/v1.0.83...v1.0.84) (2022-09-29)

### Bug Fixes

* **denolint:** Fix native package publishing ([df93d3f](https://github.com/prantlf/denolint/commit/df93d3fdb5f7660361dd835f5acbe3edd028d665))

## [1.0.83](https://github.com/prantlf/denolint/compare/v1.0.82...v1.0.83) (2022-09-29)

### Bug Fixes

* **denolint:** Work around for missing executable permissions ([190ec5a](https://github.com/prantlf/denolint/commit/190ec5a53e7397be4eb928c8509a08403d00b25a))

## [1.0.82](https://github.com/prantlf/denolint/compare/v1.0.3...v1.0.82) (2022-09-29)

### Bug Fixes

* Work around malfunctioning creation of symlinks in .bin ([a6a3d22](https://github.com/prantlf/denolint/commit/a6a3d22b2bc0997072e26d1b2b33321fb01e5c94))

## [1.0.3](https://github.com/prantlf/denolint/compare/v1.0.2...v1.0.3) (2022-09-29)

### Bug Fixes

* Fix incomplete publishing of native packages ([7065c24](https://github.com/prantlf/denolint/commit/7065c2466f62733369e364bd3edc007e4acc6d97))

## [1.0.2](https://github.com/prantlf/denolint/compare/v1.0.1...v1.0.2) (2022-09-29)

### Bug Fixes

* **denolint:** Ensure symlink to the executable in .bin ([192ac24](https://github.com/prantlf/denolint/commit/192ac24d42377411946beda6eb6bff8938ba87f2))

## [1.0.1](https://github.com/prantlf/denolint/compare/v1.0.0...v1.0.1) (2022-09-29)

### Bug Fixes

* **denolint:** Do not include all platforms in the main package ([b0c4f37](https://github.com/prantlf/denolint/commit/b0c4f37b5d3ffc51208826bd1f906a61de986ba0))
* Install platform packages using optional dependencies ([bfaf001](https://github.com/prantlf/denolint/commit/bfaf00136b5462fc216c8cb56d9c95224ecd056f))

# 1.0.0 (2022-09-28)

This is the first version released after forking the [customisation] of the [original project].

* Scan specific directories ([631], [647])
* Scan directories configured by `files.include` ([635], [645])
* Fix handling of the configuration `files.exclude` ([635], [646])
* Support disabling rules in souces using `eslint-disable` ([630], [642])
* Support including and excluding rules in the `lint` method ([631], [643])
* Execute the command-line tool `denolint` without loading the Node.js VM ([648])
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
[648]: https://github.com/napi-rs/node-rs/issues/648
