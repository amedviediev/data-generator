use cli::Params;
use std::collections::HashMap;

pub fn generate(config: &HashMap<String, String>, params: &Params) -> String {
    for (key, value) in config.iter() {
        println!("{} : {}", key, value);
    }
    "Test".to_string()
}