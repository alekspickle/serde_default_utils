# serde_default_utils

[![v](https://img.shields.io/badge/v-0.1.0-blueviolet)]()

## Overview
This is a simple set of functions to make your life easier while working with defaults in serde.
Heavily inspired by discussions on issues about serde defaults, but mostly [this one](https://github.com/serde-rs/serde/issues/368)

## Example
```rust
    use serde_default_utils::*;
    use serde::{Deserialize, Serialize};

    const JSON: &str = r#"{"yes_or_no":false,"max":60,"delta":-77}"#;
    const EMPTY_JSON: &str = r#"{}"#;
    const MAX: u32 = 7;

    #[derive(Serialize, Deserialize, Default)]
    struct Config {
        #[serde(default = "default_bool::<true>")]
        yes_or_no: bool,
        #[serde(default = "default_i16::<-3>")]
        delta: i16,
        // you can even use consts right here
        #[serde(default = "default_u32::<MAX>")]
        max: u32,
    }

    fn main() {
        // existing json fields are not changed
        let config: Config = serde_json::from_str(JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":false,"delta":-77,"max":60}"#, &s);
        // if the field is not present - it is substituted with defaults
        let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":true,"delta":-3,"max":7}"#, &s);
        // the default impl is just calling underlying type defaults unless you have a custom impl Default
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        assert_eq!(r#"{"yes_or_no":false,"delta":0,"max":0}"#, &s);
    }

```

License: MIT OR Apache-2.0
