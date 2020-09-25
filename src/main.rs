mod test_gen;
mod utils;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use tokio::fs::*;
use crate::test_gen::Suite;
use ansi_term::Colour::{ Green};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = utils::get_config("config.json");
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    utils::clear_workspace(&config.output_dir, &config.tables)?;

    for table in config.tables {
        create_dir(&table).await?;
        let suite = get_parsed_tables(&table);
        test_gen::generate_test_suite(suite, format!("{}/{}", config.output_dir,table)).await?;
    }
    println!("{}", Green.paint("Done!"));
    Ok(())
}

// Get the parsed tables data based on the tests sheets
fn get_parsed_tables(table: &str) -> Vec<Suite> {
    let path = format!("sheets/tables/{}.json", table);
    let json_file_path = Path::new(&path);
    let json_file = File::open(json_file_path).expect("file not found");
    return serde_json::from_reader(json_file).expect("error while reading json");
}
