# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes

- Update nix flake using dependabot ([#22](https://github.com/vbrandl/merging-iterator/pull/22))

### Chores

- Update dependencies, cargo edition, CI pipelines, dev environment, nix flake, audit, semver compatibility check ([#6](https://github.com/vbrandl/merging-iterator/pull/6))

### Dependencies
- Bump `actions/checkout` from 5 to 6 ([#7](https://github.com/vbrandl/merging-iterator/pull/7))
- Bump `rust-toolchain` from 1.92.0 to 1.94.1 ([#9](https://github.com/vbrandl/merging-iterator/pull/9), [#15](https://github.com/vbrandl/merging-iterator/pull/15), [#17](https://github.com/vbrandl/merging-iterator/pull/17), [#19](https://github.com/vbrandl/merging-iterator/pull/19))
- Bump `actions/checkout` from 5 to 6 ([#7](https://github.com/vbrandl/merging-iterator/pull/7))
- Bump `proptest` from 1.9.0 to 1.11.0 ([#14](https://github.com/vbrandl/merging-iterator/pull/14), [#18](https://github.com/vbrandl/merging-iterator/pull/18))

## [1.3.0] 2020-11-18

- Remove unnecessary trait requirement on `Ord` for `Iterator` implementation.
  Thanks to @Tamschi for the fix.

## [1.2.0] 2019-09-15

- Get rid of unnecessary heap allocation by using `fn` instead of `Fn`
