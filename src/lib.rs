//! [![v](https://img.shields.io/badge/v-0.2.1-blueviolet)]()
//!
//! # Overview
//! This is a simple set of functions to make your life easier while working with defaults in serde.
//! Based on [const generic parameters](https://doc.rust-lang.org/reference/items/generics.html#const-generics).
//! Heavily inspired by discussions on issues about serde defaults, but mostly [this one](https://github.com/serde-rs/serde/issues/368)
//!
//! # Kudos
//! - JohnTheCoolingFan posted this solution, I just made it available as crate and a macro that
//! helps to generate another const generic function for any const generic type.
//! - bytedream         made a more powerful version of it, and although I still see const generic approach as more readable,
//! I have to admit that for strings it's superior, hence - included under the feature
//!
//!
//! # Example
//! ```rust
//!     use serde_default_utils::*;
//!     use serde::{Deserialize, Serialize};
//!
//!         const JSON: &str =
//!             r#"{
//!                 "yes_or_no":false,
//!                 "max":60,
//!                 "delta":-77,
//!                 "delimeter":"☀",
//!                 "motto":"I matter"
//!             }"#;
//!         const INLINE_JSON: &str =
//!             r#"{
//!                 "inline_motto":"I matter even more",
//!                 "slice":["I", "matter", "even", "more"],
//!                 "slice_u64":[9000, 8000, 10000, 7000]
//!             }"#;
//!     const EMPTY_JSON: &str = r#"{}"#;
//!     const MAX: u32 = 7;
//!
//!     serde_default!(motto, "You matter");
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
//!         #[serde(default = "default_motto")]
//!         motto: &'static str,
//!     }
//!
//!     #[cfg(feature = "inline")]
//!     #[serde_inline_default]
//!     #[derive(Serialize, Deserialize, Default)]
//!     struct InlineConfig {
//!         #[serde_inline_default("You matter even more".to_string())]
//!         inline_motto: String,
//!         #[serde_inline_default(vec!["You", "matter", "even", "more"])]
//!         slice: Vec<&'static str>,
//!         #[serde_inline_default(vec![98712, 12346, 129389, 102937])]
//!         slice_u64: Vec<u64>,
//!     }
//!
//!     fn main() {
//!         // existing json fields are not changed
//!         let config: Config = serde_json::from_str(JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":false,"delta":-77,"max":60,"delimeter":"☀","motto":"I matter"}"#, &s);
//!         // if the field is not present - it is substituted with defaults
//!         let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":true,"delta":-3,"max":7,"delimeter":"☀","motto":"You matter"}"#, &s);
//!         // the default impl is just calling underlying type defaults unless you have a custom impl Default
//!         let config = Config::default();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"yes_or_no":false,"delta":0,"max":0,"delimeter":"\u0000","motto":""}"#, &s);
//!         
//!         // Inline
//!         #[cfg(feature = "inline")]
//!         {
//!         // existing json fields are not changed
//!         let config: InlineConfig = serde_json::from_str(INLINE_JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"inline_motto":"I matter even more","slice":["I","matter","even","more"],"slice_u64":[9000,8000,10000,7000]}"#, &s);
//!         // if the field is not present - it is substituted with defaults
//!         let config: InlineConfig = serde_json::from_str(EMPTY_JSON).unwrap();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"inline_motto":"You matter even more","slice":["You","matter","even","more"],"slice_u64":[98712,12346,129389,102937]}"#, &s);
//!         // the default impl is just calling underlying type defaults unless you have a custom impl Default
//!         let config = Config::default();
//!         let s = serde_json::to_string(&config).unwrap();
//!         assert_eq!(r#"{"inline_motto":"","slice":[],"slice_u64":[]}"#, &s);
//!         }
//!     }
//!
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "inline-derive")]
pub use serde_inline_default::serde_inline_default;

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
/// // Generates
/// // pub const fn default_hey() -> &'static ::core::primitive::str {
/// //     "hey"
/// // }
/// serde_default!(hey, "hey");
///
/// // !Experimental!
/// // (not sure if it's even useful, but this allows you to store raw bytes)
/// // Generates
/// // pub const fn default_arr() -> &'static [::core::primitive::u8] {
/// //     &[1,2,3,4,5]
/// // }
/// serde_default!(arr, &[1,2,3,4,5]);
///
/// assert!(default_u8::<6>() == 6u8);
/// assert_eq!(default_hey(), "hey");
/// assert_eq!(default_arr(), &[1,2,3,4,5]);
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
    ($name:ident,$text:literal) => {
        ::paste::paste! {
            pub const fn [<default_$name:lower>]() -> &'static ::core::primitive::str {
                $text
            }
        }
    };
    ($name:ident, &[ $($value:expr),* $(,)? ]) => {
        ::paste::paste! {
            pub const fn [<default_$name:lower>]() -> &'static [::core::primitive::u8] {
                &[$($value,)*]
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

    serde_default!(hey, "You matter");

    #[serde_inline_default]
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
        #[serde(default = "default_hey")]
        motto: &'static str,
        #[serde_inline_default("You matter even more".to_string())]
        inline_motto: String,
        #[serde_inline_default(vec!["You", "matter", "even", "more"])]
        slice: Vec<&'static str>,
        #[serde_inline_default(vec![98712, 12346, 129389, 102937])]
        slice_u64: Vec<u64>,
    }

    const JSON: &str = r#"{
            "yes_or_no":false,
            "max":60,
            "delta":-77,
            "delimeter":"☀",
            "motto":"I matter",
            "inline_motto":"I matter even more",
            "slice":["I", "matter", "even", "more"],
            "slice_u64":[9000, 8000, 10000, 7000]
        }"#;
    const EMPTY_JSON: &str = r#"{}"#;

    #[test]
    fn deserialization_works() {
        let config: Config = serde_json::from_str(JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"delta":-77,"max":60,"delimeter":"☀","motto":"I matter","inline_motto":"I matter even more","slice":["I","matter","even","more"],"slice_u64":[9000,8000,10000,7000]}"#]]
        .assert_eq(&s);
        let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":true,"delta":-3,"max":7,"delimeter":"☀","motto":"You matter","inline_motto":"You matter even more","slice":["You","matter","even","more"],"slice_u64":[98712,12346,129389,102937]}"#]]
            .assert_eq(&s);
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"delta":0,"max":0,"delimeter":"\u0000","motto":"","inline_motto":"","slice":[],"slice_u64":[]}"#]]
            .assert_eq(&s);
    }
}
