# simple_env_load
[![Documentation][docs_badge]][docs]
[![Crates][crates_badge]][crates]
[![Actions][actions_badge]][actions]

A simple ***.env*** file loader

## Description
Giving a sequence of env files from most general to most specific.

## Operation
Parse each file for key val remove any comments blank lines and extra whitespace.

## Syntax
```rust
TEST_DATA=bar       # spaces are optional
## this is a comment
TEST_baz = "baz"    # double quotes are removed
## above line was left intentionally blank
```
will produce:

|Key|Value|
|---|---|
`TEST_DATA`|`bar`
`TEST_baz`|`baz`

License: 0BSD

[docs_badge]: https://docs.rs/simple_env_load/badge.svg
[docs]: https://docs.rs/simple_env_load
[crates_badge]: https://img.shields.io/crates/v/simple_env_load.svg
[crates]: https://crates.io/crates/simple_env_load
[actions_badge]: https://github.com/museun/simple_env_load/workflows/Rust/badge.svg
[actions]: https://github.com/museun/simple_env_load/actions
