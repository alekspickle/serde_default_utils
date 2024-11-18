#[macro_export]
macro_rules! serde_default {
    ($kind:ty) => {
        ::paste::paste! {
            pub const fn [<default_$kind>]<const V: $kind>() -> $kind {
                V
            }
        }
    };
}
serde_default!(usize);
serde_default!(bool);
serde_default!(u8);
serde_default!(u16);
serde_default!(u32);
serde_default!(u64);
serde_default!(u128);
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
    }

    const JSON: &str = r#"{"yes_or_no":false,"max":60,"delta":-77}"#;
    const EMPTY_JSON: &str = r#"{}"#;

    #[test]
    fn deserialization_works() {
        let config: Config = serde_json::from_str(JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"max":60,"delta":-77}"#]].assert_eq(&s);
        let config: Config = serde_json::from_str(EMPTY_JSON).unwrap();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":true,"max":7,"delta":-3}"#]].assert_eq(&s);
        let config = Config::default();
        let s = serde_json::to_string(&config).unwrap();
        expect![[r#"{"yes_or_no":false,"max":0,"delta":0}"#]].assert_eq(&s);
    }
}
