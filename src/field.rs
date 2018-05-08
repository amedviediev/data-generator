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
    pub string_config: StringConfig,
    #[serde(default)]
    pub date_config: DateConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    String,
    Date,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StringConfig {
    pub length: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateConfig {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Default for DateConfig {
    fn default() -> DateConfig {
        DateConfig {
            start: DateTime::from_utc(NaiveDateTime::new(MIN_DATE, NaiveTime::from_hms(0, 0, 0)), Utc),
            end: DateTime::from_utc(NaiveDateTime::new(MAX_DATE, NaiveTime::from_hms(23, 59, 59)), Utc)
        }
    }
}