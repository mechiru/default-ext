//! This library provides extension methods for the `Default` trait.
//!
//! ## Example
//! case1:
//! ```
//! # use default_ext::DefaultExt;
//! assert!(false.is_default());
//! ```
//!
//! case2:
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct Object {
//!    #[serde(
//!        default,
//!        skip_serializing_if = "default_ext::DefaultExt::is_default",
//!    )]
//!    is_test: bool,
//! }
//! ```

pub trait DefaultExt {
    fn is_default(&self) -> bool;
}

impl<T> DefaultExt for T
where
    T: Default + PartialEq,
{
    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
