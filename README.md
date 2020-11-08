# default-ext

[![ci](https://github.com/mechiru/default-ext/workflows/ci/badge.svg)](https://github.com/mechiru/default-ext/actions?query=workflow:ci)
[![Rust Documentation](https://docs.rs/default-ext/badge.svg)](https://docs.rs/default-ext)
[![Latest Version](https://img.shields.io/crates/v/default-ext.svg)](https://crates.io/crates/default-ext)

This library provides extension methods for the `Default` trait.

## Example
case1:
```rust
use default_ext::DefaultExt;
assert!(false.is_default());
```

case2:
```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Object {
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
    )]
    is_test: bool,
}
```

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
