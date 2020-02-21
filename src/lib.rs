use serde::{de::Error, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

/// Deserializes a field expanding all the environment variables before the conversion
///
/// # Example:
///
/// ```rust
/// use std::env;
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
///     #[serde(deserialize_with="with_expand_envs")]
///     default: usize,
/// }
///
/// let serialized = r#"{"number": "$NUMBER",
///                      "string": "my string: $STRING",
///                      "default": "${DEFAULT:-42}"
///                   }"#;
///
/// // No envs set will fail with enviroment variable not found
/// assert_eq!(serde_json::from_str::<Test>(&serialized).is_err(), true);
///
/// env::set_var("NUMBER", "42");
/// env::set_var("STRING", "hacker");
/// let deserialized: Test = serde_json::from_str(&serialized).unwrap();
///
/// assert_eq!(deserialized.number, 42);
/// assert_eq!(deserialized.string, "my string: hacker");
/// assert_eq!(deserialized.default, 42);
///
/// env::set_var("DEFAULT", "4200");
/// let deserialized: Test = serde_json::from_str(&serialized).unwrap();
/// assert_eq!(deserialized.default, 4200);
///
/// // Invalid number
/// env::set_var("NUMBER", "cuarentaydos");
/// env::set_var("STRING", "42");
///
/// assert_eq!(serde_json::from_str::<Test>(&serialized).is_err(), true);
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
        StringOrAnything::String(s) => match shellexpand::env(&s) {
            Ok(value) => value.parse::<T>().map_err(Error::custom),
            Err(err) => Err(Error::custom(err)),
        },
        StringOrAnything::Anything(anything) => Ok(anything),
    }
}
