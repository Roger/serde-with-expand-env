use envmnt::{ExpandOptions, ExpansionType};
use serde::{self, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

/// Deserializes a field expanding all the environment variables before the conversion
///
/// # Example:
///
/// ```rust
/// use serde_json;
/// use serde::Deserialize;
/// use serde_with_expand_env::with_expand_envs;
///
/// #[derive(Deserialize, Debug)]
/// struct Test {
///     #[serde(deserialize_with="with_expand_envs")]
///     number: usize,
///     #[serde(deserialize_with="with_expand_envs")]
///     string: String,
/// }
///
/// fn main() {
///     let serialized = r#"{"number": "$NUMBER", "string": "my string: $STRING"}"#;
///
///     envmnt::set("NUMBER", "42");
///     envmnt::set("STRING", "hacker");
///     let deserialized: Test = serde_json::from_str(&serialized).unwrap();
///
///     assert_eq!(deserialized.number, 42);
///     assert_eq!(deserialized.string, "my string: hacker");
///
///     // Invalid number
///     envmnt::set("NUMBER", "cuarentaydos");
///     envmnt::set("STRING", "42");
///
///     assert_eq!(serde_json::from_str::<Test>(&serialized).is_err(), true);
/// }
/// ```
pub fn with_expand_envs<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrAnything<T> {
        String(String),
        Anything(T),
    }

    match StringOrAnything::<T>::deserialize(deserializer)? {
        StringOrAnything::String(s) => {
            let mut options = ExpandOptions::new();
            options.expansion_type = Some(ExpansionType::Unix);
            let value = envmnt::expand(&s, Some(options));

            value.parse::<T>().map_err(serde::de::Error::custom)
        }
        StringOrAnything::Anything(anything) => Ok(anything),
    }
}
