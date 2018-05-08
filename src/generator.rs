use cli::Params;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;
use chrono::Utc;
use std::time::Instant;
use field::Field;
use field::DataType;
use serde_json::Number;
use serde_json::de::ParserNumber;
use rand::Rng;
use rand::thread_rng;
use seed_data;
use inflector::Inflector;

//{"registered":"2017-02-18 10:38:46","ipv6":"9afb:4093:c6b0:9928:8fbb:368a:f9af:3398","ipv4":"192.168.173.69","about":"tempor irure mollit ipum velit sint minim.","company":"Suwanee Insurance","lastName":"Velasquez","firstName":"Ernest","eyeColor":"green","age":18,"balance_raw":"$6.455,05","balance":6455.05,"isActive":true,"index":0,"location":"50.995782,30.470266","id":"0"}

pub fn generate_and_write(config: Vec<Field>, params: &Params) {
    let start = Instant::now();
    println!("Generating {} records.", params.number_of_records);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&params.output)
        .unwrap();
    writeln!(file, "[");

    for iteration in 0..params.number_of_records {
        let mut entry_map: BTreeMap<&String, Value> = BTreeMap::new();
        for field in config.iter() {
            let data = match field.data_type {
                DataType::Text => generate_text(field.text_config.words),
                DataType::Date => generate_date(),
                DataType::Number => generate_number(field.number_config.min, field.number_config.max),
                _ => Value::Null
            };

            entry_map.insert(&field.name, data);
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

fn generate_text(words: u32) -> Value {
    let mut data = "".to_string();
    for i in 0..words {
        let mut word = seed_data::WORDS[thread_rng().gen_range(0, seed_data::WORDS.len())].to_string();
        if i == 0 {
            word = word.to_title_case();
        } else {
            data.push_str(" ");
        }
        data.push_str(word.as_ref());
    }
    data.push_str(".");
    Value::String(data)
}

fn generate_date() -> Value {
    let data = Utc::now();
    Value::String(data.to_string())
}

fn generate_number(min: i64, max: i64) -> Value {
    let data = thread_rng().gen_range(min, max);
    Value::Number(Number::from(ParserNumber::I64(data)))
}