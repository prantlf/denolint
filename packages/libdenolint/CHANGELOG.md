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
