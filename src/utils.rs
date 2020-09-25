use std::fs::remove_dir_all;
use std::io::Error;
use ansi_term::Colour::Blue;
use std::path::Path;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateConfig {
  pub tables: Vec<String>,
  pub output_dir: String
}

pub fn get_project_ascii_art() -> &'static str {
  let ascii_art =
  "________________________________________________
  |___|___|___|___|___|___|___|___|___|___|___|___|
  |_|___ TS TESTS TEMPLATES GENERATOR |___|___|___|
  |___|___|___|___|___|___|___|___|___|___|___|___|";
  ascii_art
}

pub fn clear_workspace(path: &String, directories: &Vec<String>) -> Result<(), Error> {
  for dir in directories {
    println!("removing: {}",Blue.paint(dir));
    remove_dir_all(format!("{}/{}", path, dir))?
  }
    Ok(())
}

// Get config obj
pub fn get_config(filename: &str) -> PrivateConfig {
  let config_path = Path::new(filename);
  let config_file = File::open(config_path).expect("file not found");
  return serde_json::from_reader(config_file).expect("error while reading json");
}