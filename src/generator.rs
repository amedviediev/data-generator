use cli::Params;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;
use chrono::Utc;

pub fn generate_and_write(config: &HashMap<String, String>, params: &Params) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&params.output)
        .unwrap();
    writeln!(file, "[");

    let mut entry_map: BTreeMap<&String, Value> = BTreeMap::new();
    for (key, value) in config.iter() {
        let data = match value.as_ref() {
            "string" => generate_string(),
            "date" => generate_date(),
            _ => Value::Null
        };

        entry_map.insert(key, data);
    }

    let v = serde_json::to_writer(&file, &entry_map);

    writeln!(file, "");
    writeln!(file, "]");
}

fn generate_string() -> Value {
    let data = "this is a string".to_string();
    Value::String(data)
}

fn generate_date() -> Value {
    let data = Utc::now();
    Value::String(data.to_string())
}