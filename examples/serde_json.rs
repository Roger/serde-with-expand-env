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

    envmnt::set("NUMBER", "1337");
    envmnt::set("STRING", "hacker");
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);

    envmnt::set("NUMBER", "42");
    envmnt::set("STRING", "life, the universe and everyhing");
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);
}
