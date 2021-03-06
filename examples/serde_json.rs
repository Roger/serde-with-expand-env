use serde::Deserialize;
use serde_json;
use serde_with_expand_env::with_expand_envs;
use std::env;

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(deserialize_with = "with_expand_envs")]
    number: usize,
    #[serde(deserialize_with = "with_expand_envs")]
    string: String,
    #[serde(deserialize_with = "with_expand_envs")]
    with_defaut: String,
}

fn main() {
    let serialized = r#"{
        "number": "$NUMBER",
        "string": "my string: $STRING",
        "with_defaut": "${DEFAULT:-default_value}"
    }"#;

    env::set_var("NUMBER", "1337");
    env::set_var("STRING", "hacker");
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);

    env::set_var("NUMBER", "42");
    env::set_var("STRING", "life, the universe and everyhing");
    env::set_var("DEFAULT", "marvin");
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);
}
