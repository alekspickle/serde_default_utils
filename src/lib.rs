//! [![v](https://img.shields.io/badge/v-0.2.0-blueviolet)]()
//!
//! # Overview
//! This is a simple set of functions to make your life easier while working with defaults in serde.
//! Based on [const generic parameters](https://doc.rust-lang.org/reference/items/generics.html#const-generics).
//! Heavily inspired by discussions on issues about serde defaults, but mostly [this one](https://github.com/serde-rs/serde/issues/368)
//!
//! # Kudos
//! This is just a @JohnTheCoolingFan's proposed solution but available as crate and a macro that
//! helps to generate another const generic function for any const generic type.
//!
//! # Example
//! ```rust
//!     use serde_default_utils::*;
//!     use serde::{Deserialize, Serialize};
//!
//!     const JSON: &str = r#"{"yes_or_no":false,"max":60,"delta":-77,"delimeter":"☀"}"#;
//!     const EMPTY_JSON: &str = r#"{}"#;
//!     const MAX: u32 = 7;
//!
//!     #[derive(Serialize, Deserialize, Default)]
//!     struct Config {
//!         #[serde(default = "default_bool::<true>")]
//!         yes_or_no: bool,
//!         #[serde(default = "default_i16::<-3>")]
//!         delta: i16,
//!         // you can even use consts right here
//!         #[serde(default = "default_u32::<MAX>")]
//!         max: u32,
//!         #[serde(default = "default_char::<'☀'>")]
//!         delimeter: char,
//!     }
//!
//!     fn main() {
//!         // existing json fields are not changed
//!         let config: Config = serde_json::from_str(JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":false,"delta":-77,"max":60,"delimeter":"☀"}"#, &s);
//!         // if the field is not present - it is substituted with defaults
//!         let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":true,"delta":-3,"max":7,"delimeter":"☀"}"#, &s);
//!         // the default impl is just calling underlying type defaults unless you have a custom impl Default
//!         let config = Config::default();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":false,"delta":0,"max":0,"delimeter":"\u0000"}"#, &s);
//!     }
//!
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

/// Generates a function for a type provided or a custom default function
/// This is not supposed to be used outside since const generic parameter approach
/// is [pretty limited](https://doc.rust-lang.org/reference/items/generics.html#const-generics) at the moment
///
/// # Limitations
/// Slices are limited to only `&'static [u8]` and parcing it from JSON
/// using `serde_json::from_str`` will not work, only `serde_json::from_str`.
///
/// # Output
/// Generates something like
/// ```rust
/// use serde_default_utils::*;
///
/// // Generates
/// // pub const fn default_u8<const V: u8>() -> u8 {
/// //     V
/// // }
/// serde_default!(u8);
///
/// // !Experimental!
/// // Generates
/// // pub const fn default_hey<const V: &'static u8>() -> &'static [u8] {
/// //     &[1, 2, 3, 4]
/// // }
/// serde_default!(hey, &'static str, "hey");
///
/// assert!(default_u8::<6>() == 6u8);
/// assert_eq!(default_hey(), "hey");
///
/// ```
#[macro_export]
macro_rules! serde_default {
    ($kind:ty) => {
        ::paste::paste! {
            pub const fn [<default_$kind:lower>]<const V: $kind>() -> $kind {
                V
            }
        }
    };
    ($name:ident,$ty:ty,$val:expr) => {
        ::paste::paste! {
            pub const fn [<default_$name:lower>]() -> $ty {
                $val
            }
        }
    };
}

serde_default!(bool);
serde_default!(char);
serde_default!(usize);
serde_default!(u8);
serde_default!(u16);
serde_default!(u32);
serde_default!(u64);
serde_default!(u128);
serde_default!(isize);
serde_default!(i8);
serde_default!(i16);
serde_default!(i32);
serde_default!(i64);
serde_default!(i128);

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default)]
    struct Config {
        #[serde(default = "default_bool::<true>")]
        yes_or_no: bool,
        #[serde(default = "default_i32::<-3>")]
        delta: i32,
        #[serde(default = "default_u32::<7>")]
        max: u32,
        #[serde(default = "default_char::<'☀'>")]
        delimeter: char,
    }

    const JSON: &str = r#"{"yes_or_no":false,"max":60,"delta":-77,"delimeter":"☀"}"#;
    const EMPTY_JSON: &str = r#"{}"#;

    #[test]
    fn deserialization_works() {
        let config: Config = serde_json::from_str(JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"delta":-77,"max":60,"delimeter":"☀"}"#]].assert_eq(&s);
        let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":true,"delta":-3,"max":7,"delimeter":"☀"}"#]].assert_eq(&s);
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"delta":0,"max":0,"delimeter":"\u0000"}"#]].assert_eq(&s);
    }
}
