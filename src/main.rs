mod test_gen;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use crate::test_gen::Suite;


#[derive(Serialize, Deserialize, Debug)]
struct PrivateConfig {
    tables: Vec<String>,
    output_dir: String
}

#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    let config = get_config("config.json");

    for table in config.tables {
        fs::create_dir(&table)?;
        let suite = parsed_tables(&table);
        test_gen::generate_test_suite(suite, format!("{}/{}", config.output_dir,table)).await?;
    }

    Ok(())
}
// Get config obj
fn get_config(filename: &str) -> PrivateConfig {
    let config_path = Path::new(filename);
    let config_file = File::open(config_path).expect("file not found");
    return serde_json::from_reader(config_file).expect("error while reading json");
}
// Get the parsed tables data based on the tests sheets
fn parsed_tables(table: &str) -> Vec<Suite> {
    let path = format!("sheets/tables/{}.json", table);
    let json_file_path = Path::new(&path);
    let json_file = File::open(json_file_path).expect("file not found");
    return serde_json::from_reader(json_file).expect("error while reading json");
}
