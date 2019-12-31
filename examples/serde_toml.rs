use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;
use std::env;

#[derive(Deserialize, Debug)]
struct Inner {
    #[serde(deserialize_with = "with_expand_envs")]
    number: usize,
    #[serde(deserialize_with = "with_expand_envs")]
    string: String,
}

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(deserialize_with = "with_expand_envs")]
    name: String,
    inner: Vec<Inner>,
}

fn main() {
    let serialized = r#"
      name = "Your name is: $NAME"
      [[inner]]
      number = "$NUMBER1"
      string = "my string: $STRING1"

      [[inner]]
      number = "$NUMBER2"
      string = "my string: $STRING2"
    "#;

    env::set_var("NAME", "Bruce");
    env::set_var("NUMBER1", "42");
    env::set_var("STRING1", "life, universe and everyhing");

    env::set_var("NUMBER2", "1337");
    env::set_var("STRING2", "leet");

    let deserialized: Test = toml::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);
}
