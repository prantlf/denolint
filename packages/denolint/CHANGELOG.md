## [1.0.2](https://github.com/prantlf/denolint/compare/v1.0.1...v1.0.2) (2022-09-29)

### Bug Fixes

* **denolint:** Ensure symlink to the executable in .bin ([192ac24](https://github.com/prantlf/denolint/commit/192ac24d42377411946beda6eb6bff8938ba87f2))

## 1.0.1 (2022-09-29)

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
