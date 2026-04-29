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
- Bump `rust-toolchain` from 1.92.0 to 1.95.0 ([#9](https://github.com/vbrandl/merging-iterator/pull/9), [#15](https://github.com/vbrandl/merging-iterator/pull/15), [#17](https://github.com/vbrandl/merging-iterator/pull/17), [#19](https://github.com/vbrandl/merging-iterator/pull/19), [#28](https://github.com/vbrandl/merging-iterator/pull/28))
- Bump `actions/checkout` from 5 to 6 ([#7](https://github.com/vbrandl/merging-iterator/pull/7))
- Bump `proptest` from 1.9.0 to 1.11.0 ([#14](https://github.com/vbrandl/merging-iterator/pull/14), [#18](https://github.com/vbrandl/merging-iterator/pull/18))
- Bump `rust-overlay` from `a1ab5e8` to `0206130` ([#23](https://github.com/vbrandl/merging-iterator/pull/23), [#27](https://github.com/vbrandl/merging-iterator/pull/27), [#30](https://github.com/vbrandl/merging-iterator/pull/30))
- Bump `flake-parts` from `5792860` to `3107b77` ([#24](https://github.com/vbrandl/merging-iterator/pull/24))
- Bump `nixpkgs` from `dd9b079` to `1c3fe55` ([#25](https://github.com/vbrandl/merging-iterator/pull/25), [#29](https://github.com/vbrandl/merging-iterator/pull/29), [#32](https://github.com/vbrandl/merging-iterator/pull/32))

## [1.3.0] 2020-11-18

- Remove unnecessary trait requirement on `Ord` for `Iterator` implementation.
  Thanks to @Tamschi for the fix.

## [1.2.0] 2019-09-15

- Get rid of unnecessary heap allocation by using `fn` instead of `Fn`
