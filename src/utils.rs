use std::fs::remove_dir_all;
use std::io::Error;
use ansi_term::Colour::Blue;

pub fn clear_workspace(path: &String, directories: &Vec<String>) -> Result<(), Error> {
  for dir in directories {
    println!("removing: {}",Blue.paint(dir));
    remove_dir_all(format!("{}/{}", path, dir))?
  }
    Ok(())
}
