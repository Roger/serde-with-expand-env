use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;

#[derive(Deserialize, Debug)]
struct Inner {
    #[serde(deserialize_with="with_expand_envs")]
    number: usize,
    #[serde(deserialize_with="with_expand_envs")]
    string: String,
}

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(deserialize_with="with_expand_envs")]
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

    envmnt::set("NAME", "Bruce");
    envmnt::set("NUMBER1", "42");
    envmnt::set("STRING1", "life, universe and everyhing");

    envmnt::set("NUMBER2", "1337");
    envmnt::set("STRING2", "leet");

    let deserialized: Test = toml::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);
}
