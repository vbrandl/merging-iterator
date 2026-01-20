# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Chores

- Update dependencies, cargo edition, CI pipelines, dev environment ([#6](https://github.com/vbrandl/merging-iterator/pull/6))

## [1.3.0] 2020-11-18

- Remove unnecessary trait requirement on `Ord` for `Iterator` implementation.
  Thanks to @Tamschi for the fix.

## [1.2.0] 2019-09-15

- Get rid of unnecessary heap allocation by using `fn` instead of `Fn`
