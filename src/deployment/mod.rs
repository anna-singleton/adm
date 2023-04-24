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

/// The flattened collection for this configuration, with variables and modules
/// flattened into single lists, ready to be templated out.
pub struct Deployment {
    modules: Vec<Module>,
    variables: HashMap<String, String>,
}

impl Deployment {
    pub fn new(cfg_str: &str, top_level_collection: &str) -> Result<Self, String> {
        let parsed = toml::from_str::<Config>(cfg_str);
        let Ok(cfg) = parsed else {
            return Err(parsed.unwrap_err().to_string());
        };
        println!("{:#?}", cfg);
        todo!()
    }
}
