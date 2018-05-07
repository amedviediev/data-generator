use cli::Params;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;
use chrono::Utc;
use std::time::Instant;

//{"registered":"2017-02-18 10:38:46","ipv6":"9afb:4093:c6b0:9928:8fbb:368a:f9af:3398","ipv4":"192.168.173.69","about":"tempor irure mollit ipum velit sint minim.","company":"Suwanee Insurance","lastName":"Velasquez","firstName":"Ernest","eyeColor":"green","age":18,"balance_raw":"$6.455,05","balance":6455.05,"isActive":true,"index":0,"location":"50.995782,30.470266","id":"0"}

pub fn generate_and_write(config: &HashMap<String, String>, params: &Params) {
    let start = Instant::now();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&params.output)
        .unwrap();
    writeln!(file, "[");

    for iteration in 0..params.number_of_records {
        let mut entry_map: BTreeMap<&String, Value> = BTreeMap::new();
        for (key, value) in config.iter() {
            let data = match value.as_ref() {
                "string" => generate_string(),
                "date" => generate_date(),
                _ => Value::Null
            };

            entry_map.insert(key, data);
        }

        let record = serde_json::to_writer(&file, &entry_map);

        if iteration < params.number_of_records - 1 {
            writeln!(file, ",");
        }
    }

    writeln!(file, "");
    writeln!(file, "]");

    let elapsed = start.elapsed();
    println!("Generated {} records in {}.{} seconds", params.number_of_records, elapsed.as_secs(), elapsed.subsec_millis());
}

fn generate_string() -> Value {
    let data = "this is a string".to_string();
    Value::String(data)
}

fn generate_date() -> Value {
    let data = Utc::now();
    Value::String(data.to_string())
}