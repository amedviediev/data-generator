extern crate serde;
extern crate serde_json;
extern crate data_generator;

use std::process;
use std::error::Error;
use std::fs::File;
use data_generator::cli;
use data_generator::generator;
use data_generator::field::Field;


fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<Error>> {
    let params: cli::Params = cli::parse();

    let config_file = File::open(&params.config)?;
    let config: Vec<Field> = serde_json::from_reader(config_file)?;

    generator::generate_and_write(config, &params);
    Ok(())
}