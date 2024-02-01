# serde_default_utils

[![v](https://img.shields.io/badge/v-0.2.1-blueviolet)]()

## Overview
This is a simple set of functions to make your life easier while working with defaults in serde.
Based on [const generic parameters](https://doc.rust-lang.org/reference/items/generics.html#const-generics).
Heavily inspired by discussions on issues about serde defaults, but mostly [this one](https://github.com/serde-rs/serde/issues/368)

## Kudos
- JohnTheCoolingFan posted this solution, I just made it available as crate and a macro that
helps to generate another const generic function for any const generic type.
- bytedream         made a more powerful version of it, and although I still see const generic approach as more readable,
I have to admit that for strings it's superior, hence - included under the feature


## Example
```rust
    use serde_default_utils::*;
    use serde::{Deserialize, Serialize};

        const JSON: &str =
            r#"{
                "yes_or_no":false,
                "max":60,
                "delta":-77,
                "delimeter":"☀",
                "motto":"I matter"
            }"#;
        const INLINE_JSON: &str =
            r#"{
                "inline_motto":"I matter even more",
                "slice":["I", "matter", "even", "more"],
                "slice_u64":[9000, 8000, 10000, 7000]
            }"#;
    const EMPTY_JSON: &str = r#"{}"#;
    const MAX: u32 = 7;

    serde_default!(motto, "You matter");

    #[derive(Serialize, Deserialize, Default)]
    struct Config {
        #[serde(default = "default_bool::<true>")]
        yes_or_no: bool,
        #[serde(default = "default_i16::<-3>")]
        delta: i16,
        // you can even use consts right here
        #[serde(default = "default_u32::<MAX>")]
        max: u32,
        #[serde(default = "default_char::<'☀'>")]
        delimeter: char,
        #[serde(default = "default_motto")]
        motto: &'static str,
    }

    #[cfg(feature = "inline")]
    #[serde_inline_default]
    #[derive(Serialize, Deserialize, Default)]
    struct InlineConfig {
        #[serde_inline_default("You matter even more".to_string())]
        inline_motto: String,
        #[serde_inline_default(vec!["You", "matter", "even", "more"])]
        slice: Vec<&'static str>,
        #[serde_inline_default(vec![98712, 12346, 129389, 102937])]
        slice_u64: Vec<u64>,
    }

    fn main() {
        // existing json fields are not changed
        let config: Config = serde_json::from_str(JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":false,"delta":-77,"max":60,"delimeter":"☀","motto":"I matter"}"#, &s);
        // if the field is not present - it is substituted with defaults
        let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":true,"delta":-3,"max":7,"delimeter":"☀","motto":"You matter"}"#, &s);
        // the default impl is just calling underlying type defaults unless you have a custom impl Default
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":false,"delta":0,"max":0,"delimeter":"\u0000","motto":""}"#, &s);

        // Inline
        #[cfg(feature = "inline")]
        {
        // existing json fields are not changed
        let config: InlineConfig = serde_json::from_str(INLINE_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"inline_motto":"I matter even more","slice":["I","matter","even","more"],"slice_u64":[9000,8000,10000,7000]}"#, &s);
        // if the field is not present - it is substituted with defaults
        let config: InlineConfig = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"inline_motto":"You matter even more","slice":["You","matter","even","more"],"slice_u64":[98712,12346,129389,102937]}"#, &s);
        // the default impl is just calling underlying type defaults unless you have a custom impl Default
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"inline_motto":"","slice":[],"slice_u64":[]}"#, &s);
        }
    }

```

License: MIT OR Apache-2.0
