mod test_gen;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct PrivateConfig {
    tables: Vec<String>
}

#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    let config_path = Path::new("private_config.json");
    let config_file = File::open(config_path).expect("file not found");
    let config: PrivateConfig = serde_json::from_reader(config_file).expect("error while reading json");

    for table in config.tables {
        fs::create_dir(&table)?;

        let path = format!("sheets/tables/{}.json", table);
        let json_file_path = Path::new(&path);
        let json_file = File::open(json_file_path).expect("file not found");
        let suite = serde_json::from_reader(json_file).expect("error while reading json");

        test_gen::generate_test_suite(suite, table).await?;
    }

    Ok(())
}
