# default-ext

[![ci](https://github.com/mechiru/default-ext/workflows/ci/badge.svg)](https://github.com/mechiru/default-ext/actions?query=workflow:ci)
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
