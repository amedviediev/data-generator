#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate inflector;
extern crate indicatif;

pub mod cli;
pub mod generator;
pub mod field;
pub mod seed_data;