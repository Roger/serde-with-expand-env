# serde-with-expand-env
[![Build Status](https://travis-ci.org/Roger/serde-with-expand-env.svg?branch=master)](https://travis-ci.org/Roger/serde-with-expand-env) [![](http://meritbadge.herokuapp.com/serde-with-expand-env)](https://crates.io/crates/serde-with-expand-env) [![](https://docs.rs/serde-with-expand-env/badge.svg)](https://docs.rs/serde-with-expand-env) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Parse environment variables in [`serde`](https://github.com/serde-rs/serde) crate using `deserialize_with`.

# Example:

```rust
use serde_json;
use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(deserialize_with="with_expand_envs")]
    number: usize,
    #[serde(deserialize_with="with_expand_envs")]
    string: String,
}

fn main() {
    let serialized = r#"{"number": "$NUMBER", "string": "my string: $STRING"}"#;

    envmnt::set("NUMBER", "42");
    envmnt::set("STRING", "hacker");
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.number, 42);
    assert_eq!(deserialized.string, "my string: hacker");

    // Invalid number
    envmnt::set("NUMBER", "cuarentaydos");
    envmnt::set("STRING", "42");

    assert_eq!(serde_json::from_str::<Test>(&serialized).is_err(), true);
}
```
