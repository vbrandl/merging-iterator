# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes

- Update nix flake using dependabot ([#22](https://github.com/vbrandl/merging-iterator/pull/22))

### Chores

- Update dependencies, cargo edition, CI pipelines, dev environment, nix flake, audit, semver compatibility check ([#6](https://github.com/vbrandl/merging-iterator/pull/6))

### Dependencies
- Bump `actions/checkout` from 5 to 7 ([#7](https://github.com/vbrandl/merging-iterator/pull/7), [#53](https://github.com/vbrandl/merging-iterator/pull/53))
- Bump `rust-toolchain` from 1.92.0 to 1.97.1 ([#9](https://github.com/vbrandl/merging-iterator/pull/9), [#15](https://github.com/vbrandl/merging-iterator/pull/15), [#17](https://github.com/vbrandl/merging-iterator/pull/17), [#19](https://github.com/vbrandl/merging-iterator/pull/19), [#28](https://github.com/vbrandl/merging-iterator/pull/28), [#45](https://github.com/vbrandl/merging-iterator/pull/45), [#57](https://github.com/vbrandl/merging-iterator/pull/57), [#60](https://github.com/vbrandl/merging-iterator/pull/60), [#63](https://github.com/vbrandl/merging-iterator/pull/63))
- Bump `actions/checkout` from 5 to 6 ([#7](https://github.com/vbrandl/merging-iterator/pull/7))
- Bump `proptest` from 1.9.0 to 1.11.0 ([#14](https://github.com/vbrandl/merging-iterator/pull/14), [#18](https://github.com/vbrandl/merging-iterator/pull/18))
- Bump `rust-overlay` from `a1ab5e8` to `a794b72` ([#23](https://github.com/vbrandl/merging-iterator/pull/23), [#27](https://github.com/vbrandl/merging-iterator/pull/27), [#30](https://github.com/vbrandl/merging-iterator/pull/30), [#31](https://github.com/vbrandl/merging-iterator/pull/31), [#34](https://github.com/vbrandl/merging-iterator/pull/34), [#37](https://github.com/vbrandl/merging-iterator/pull/37), [#40](https://github.com/vbrandl/merging-iterator/pull/40), [#42](https://github.com/vbrandl/merging-iterator/pull/42), [#44](https://github.com/vbrandl/merging-iterator/pull/44), [#46](https://github.com/vbrandl/merging-iterator/pull/46), [#51](https://github.com/vbrandl/merging-iterator/pull/51), [#52](https://github.com/vbrandl/merging-iterator/pull/52), [#55](https://github.com/vbrandl/merging-iterator/pull/55), [#58](https://github.com/vbrandl/merging-iterator/pull/58), [#59](https://github.com/vbrandl/merging-iterator/pull/59), [#62](https://github.com/vbrandl/merging-iterator/pull/62))
- Bump `flake-parts` from `5792860` to `17c9d6c` ([#24](https://github.com/vbrandl/merging-iterator/pull/24), [#35](https://github.com/vbrandl/merging-iterator/pull/35), [#39](https://github.com/vbrandl/merging-iterator/pull/39), [#59](https://github.com/vbrandl/merging-iterator/pull/59))
- Bump `nixpkgs` from `dd9b079` to `18b9261` ([#25](https://github.com/vbrandl/merging-iterator/pull/25), [#29](https://github.com/vbrandl/merging-iterator/pull/29), [#32](https://github.com/vbrandl/merging-iterator/pull/32), [#33](https://github.com/vbrandl/merging-iterator/pull/33), [#36](https://github.com/vbrandl/merging-iterator/pull/36), [#38](https://github.com/vbrandl/merging-iterator/pull/38), [#43](https://github.com/vbrandl/merging-iterator/pull/43), [#47](https://github.com/vbrandl/merging-iterator/pull/47), [#51](https://github.com/vbrandl/merging-iterator/pull/51), [#52](https://github.com/vbrandl/merging-iterator/pull/52), [#55](https://github.com/vbrandl/merging-iterator/pull/55), [#58](https://github.com/vbrandl/merging-iterator/pull/58), [#59](https://github.com/vbrandl/merging-iterator/pull/59), [#62](https://github.com/vbrandl/merging-iterator/pull/62))
- Bump `actions/cache` from 5 to 6 ([#54](https://github.com/vbrandl/merging-iterator/pull/54))

## [1.3.0] 2020-11-18

- Remove unnecessary trait requirement on `Ord` for `Iterator` implementation.
  Thanks to @Tamschi for the fix.

## [1.2.0] 2019-09-15

- Get rid of unnecessary heap allocation by using `fn` instead of `Fn`
