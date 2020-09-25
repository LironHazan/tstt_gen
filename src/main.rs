mod test_gen;
mod utils;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use tokio::fs::*;
use serde_derive::{Deserialize, Serialize};
use crate::test_gen::Suite;
use ansi_term::Colour::{ Yellow, Green};


#[derive(Serialize, Deserialize, Debug)]
struct PrivateConfig {
    tables: Vec<String>,
    output_dir: String
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config("config.json");
    println!("{}", Yellow.paint("Generating test suites into: "));
    println!("{}", Yellow.paint(&config.output_dir));

    utils::clear_workspace(&config.output_dir, &config.tables)?;

    for table in config.tables {
        create_dir(&table).await?;
        let suite = get_parsed_tables(&table);
        test_gen::generate_test_suite(suite, format!("{}/{}", config.output_dir,table)).await?;
    }
    println!("{}", Green.paint("Done!"));
    Ok(())
}
// Get config obj
fn get_config(filename: &str) -> PrivateConfig {
    let config_path = Path::new(filename);
    let config_file = File::open(config_path).expect("file not found");
    return serde_json::from_reader(config_file).expect("error while reading json");
}
// Get the parsed tables data based on the tests sheets
fn get_parsed_tables(table: &str) -> Vec<Suite> {
    let path = format!("sheets/tables/{}.json", table);
    let json_file_path = Path::new(&path);
    let json_file = File::open(json_file_path).expect("file not found");
    return serde_json::from_reader(json_file).expect("error while reading json");
}
