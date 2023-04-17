use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    module: HashMap<String, Module>,
    collection: HashMap<String, Collection>
}

#[derive(Serialize, Deserialize, Debug)]
struct Module {
    src: String,
    dest: String,
    hook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
    include: Vec<String>,
    modules: Vec<String>,
    variables: HashMap<String, String>
}

fn main() {
    let s = include_str!("test.toml");
    let val:Result<Config, toml::de::Error> = toml::from_str(s);
    match val {
        Ok(conf) => println!("{:#?}", conf),
        Err(err) => println!("{}", err),
    }
}
