use std::fs::{metadata, File};
use std::io::Error;
use ansi_term::Colour::Blue;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use crate::test_gen::Suite;
use tokio::fs::{remove_dir_all};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateConfig {
  pub tables: Vec<String>,
  pub output_dir: String
}

pub fn get_project_ascii_art() -> &'static str {
  let ascii_art =
  "👻|________________________________________________
  |___|___|___|___|___|___|___|___|___|___|___|___|
  |_|___ TS TESTS TEMPLATES GENERATOR |___|___|___|
  |___|___|___|___|___|___|___|___|___|___|___|___|👻";
  ascii_art
}

pub async fn clear_workspace(path: &String, directories: &Vec<String>) -> Result<(), Error> {
  for dir in directories {
    let _path = format!("{}/{}", path, dir);
    if metadata(&_path).is_ok() {
      println!("removing older entry of: {}",Blue.paint(dir));
      remove_dir_all(&_path).await?
    }
  }
    Ok(())
}

// Get config obj
fn get_config(filename: &str) -> PrivateConfig {
  let config_path = Path::new(filename);
  let config_file = File::open(config_path).expect("file not found");
  serde_json::from_reader(config_file).expect("error while reading json")
}

pub async fn get_config_async(filename: &str) -> Result<PrivateConfig, Error> {
  Ok(get_config(filename))
}

// Get the parsed tables data based on the tests sheets
fn get_parsed_tables(path: String) -> Vec<Suite> {
  let json_file_path = Path::new(&path);
  let json_file = File::open(json_file_path).expect("file not found");
  serde_json::from_reader(json_file).expect("error while reading json")
}
pub async fn get_parser_tables_async(path: String) -> Result<Vec<Suite>, Error> {
  Ok(get_parsed_tables(path))
}
