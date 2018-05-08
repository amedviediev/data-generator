use clap::{Arg, App};

pub struct Params {
    pub config: String,
    pub number_of_records: i32,
    pub output: String,
    pub pretty: bool,
}

pub fn parse() -> Params {
    let matches = App::new("Data generator")
        .version("0.1.0")
        .author("amedvedev <amedvyedyev@gmail.com>")
        .about("Generates random json data")
        .arg(Arg::with_name("CONFIG")
            .short("c")
            .long("config")
            .help("Sets a custom config file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("NUMBER")
            .short("n")
            .long("number")
            .help("Number of records to generate")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("Name of output file")
            .takes_value(true))
        .arg(Arg::with_name("PRETTY")
            .short("p")
            .long("pretty")
            .help("Pretty-print output json"))
        .get_matches();

    let config_path = matches.value_of("CONFIG").expect("File not found");
    let number_of_records: i32 = matches.value_of("NUMBER").unwrap().parse().unwrap_or(0);
    let output_path = matches.value_of("OUTPUT").unwrap_or("data.json");
    let pretty = matches.is_present("PRETTY");

    let params = Params {
        config: config_path.to_string(),
        number_of_records,
        output: output_path.to_string(),
        pretty,
    };
    params
}