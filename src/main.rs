mod test_gen;
mod utils;

extern crate serde;
extern crate serde_json;

use ansi_term::Colour::{ Green};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = utils::get_config("config.json");
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    utils::clear_workspace(&config.output_dir, &config.tables)?;
    test_gen::generate_all_suites(config.tables, config.output_dir).await?;
    println!("{}", Green.paint("Done!"));
    Ok(())
}
