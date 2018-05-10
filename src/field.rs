use chrono::DateTime;
use chrono::Utc;
use chrono::NaiveDateTime;
use chrono::naive::MIN_DATE;
use chrono::NaiveTime;
use chrono::naive::MAX_DATE;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,

    #[serde(default)]
    pub text_config: TextConfig,
    #[serde(default)]
    pub date_config: DateConfig,
    #[serde(default)]
    pub number_config: NumberConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Text,
    Date,
    Number,
    Name,
    Email,
    IPv4
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextConfig {
    pub words: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateConfig {
    #[serde(with = "config_date_format")]
    pub min_date: DateTime<Utc>,
    #[serde(with = "config_date_format")]
    pub max_date: DateTime<Utc>,
}

impl Default for DateConfig {
    fn default() -> DateConfig {
        DateConfig {
            min_date: DateTime::from_utc(NaiveDateTime::new(MIN_DATE, NaiveTime::from_hms(0, 0, 0)), Utc),
            max_date: DateTime::from_utc(NaiveDateTime::new(MAX_DATE, NaiveTime::from_hms(23, 59, 59)), Utc)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NumberConfig {
    pub min: i64,
    pub max: i64
}

mod config_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}