# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Allow more expressions in `construct_with` attribute ([4873afb](https://github.com/df5602/adhoc_derive/commit/4873afb4c4c4eb61fb2065a91294414b6ed65c78)).
This includes array syntax, tuples, binary and unary operations, literals as function arguments, if/else expressions, nested function calls, etc.
Refer to [tests/construct_with.rs](https://github.com/df5602/adhoc_derive/blob/master/tests/construct_with.rs) for a number of examples.
- Allow type ascription in `construct_with` attribute to help with type inference ([54df5ac](https://github.com/df5602/adhoc_derive/commit/54df5acea53caea7620037bbdb7c1dc5f5f33c42)).

## [0.1.0] - 2019-01-03
Initial release
