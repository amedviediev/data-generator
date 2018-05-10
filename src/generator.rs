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
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use names::Generator;
use names::Name;
use chrono::DateTime;
use chrono::NaiveDateTime;

//{"registered":"2017-02-18 10:38:46","ipv6":"9afb:4093:c6b0:9928:8fbb:368a:f9af:3398","ipv4":"192.168.173.69","about":"tempor irure mollit ipum velit sint minim.","company":"Suwanee Insurance","lastName":"Velasquez","firstName":"Ernest","eyeColor":"green","age":18,"balance_raw":"$6.455,05","balance":6455.05,"isActive":true,"index":0,"location":"50.995782,30.470266","id":"0"}

pub fn generate_and_write(config: Vec<Field>, params: &Params) {
    let start = Instant::now();
    println!("Generating {} records.", params.number_of_records);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&params.output)
        .unwrap();
    write!(file, "[");

    let bar = ProgressBar::new(params.number_of_records);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    for iteration in 0..params.number_of_records {
        let mut entry_map: BTreeMap<&String, Value> = BTreeMap::new();
        for field in config.iter() {
            let data = match field.data_type {
                DataType::Text => generate_text(field.text_config.words),
                DataType::Date => generate_date(field.date_config.min_date, field.date_config.max_date),
                DataType::Number => generate_number(field.number_config.min, field.number_config.max),
                DataType::Name => generate_name(),
                DataType::Email => generate_email(),
                DataType::IPv4 => generate_ipv4()
            };

            entry_map.insert(&field.name, data);
        }

        if params.pretty {
            let mut string = serde_json::to_string_pretty(&entry_map).unwrap();
            file.write(string.as_bytes());
        } else {
            let mut string = serde_json::to_string(&entry_map).unwrap();
            file.write(string.as_bytes());
        }

        if iteration < params.number_of_records - 1 {
            write!(file, ",");
        }
        bar.inc(1);
    }
    bar.finish();

    write!(file, "]");

    let elapsed = start.elapsed();
    println!("Generated {} records in {}.{} seconds", params.number_of_records, elapsed.as_secs(), elapsed.subsec_millis());
}

fn generate_text(words: u32) -> Value {
    let mut text = "".to_string();
    for i in 0..words {
        let mut word = seed_data::WORDS[thread_rng().gen_range(0, seed_data::WORDS.len())].to_string();
        if i == 0 {
            word = word.to_title_case();
        } else {
            text.push_str(" ");
        }
        text.push_str(word.as_ref());
    }
    text.push_str(".");
    Value::String(text)
}

fn generate_date(min_date: DateTime<Utc>, max_date: DateTime<Utc>) -> Value {
    let min_timestamp = min_date.timestamp();
    let max_timestamp = max_date.timestamp();
    let random_timestamp = thread_rng().gen_range(min_timestamp, max_timestamp);
    let naive_datetime = NaiveDateTime::from_timestamp(random_timestamp, 0);

    let date: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    Value::String(date.to_string())
}

fn generate_number(min: i64, max: i64) -> Value {
    let data = thread_rng().gen_range(min, max);
    Value::Number(Number::from(ParserNumber::I64(data)))
}

fn generate_name() -> Value {
    let mut name = "".to_string();
    let first_name = seed_data::FIRST_NAMES[thread_rng().gen_range(0, seed_data::FIRST_NAMES.len())];
    let last_name = seed_data::LAST_NAMES[thread_rng().gen_range(0, seed_data::LAST_NAMES.len())];
    name.push_str(first_name);
    name.push_str(" ");
    name.push_str(last_name);
    Value::String(name)
}

fn generate_email() -> Value {
    let adress = Generator::with_naming(Name::Plain).next().unwrap();
    let host = seed_data::EMAILS_HOSTS[thread_rng().gen_range(0, seed_data::EMAILS_HOSTS.len())];
    let tld = seed_data::TLDS[thread_rng().gen_range(0, seed_data::TLDS.len())];
    let email = format!("{}@{}.{}", adress, host, tld);
    Value::String(email)
}

fn generate_ipv4() -> Value {
    let one: u8 = thread_rng().gen_range(0, 255);
    let two: u8 = thread_rng().gen_range(0, 255);
    let three: u8 = thread_rng().gen_range(0, 255);
    let four: u8 = thread_rng().gen_range(0, 255);
    let ip = format!("{}.{}.{}.{}", one, two, three, four);
    Value::String(ip)
}