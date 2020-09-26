mod test_gen;
mod utils;

extern crate serde;
extern crate serde_json;

use ansi_term::Colour::{ Green};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = utils::get_config_async("config.json").await?;
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    // Removes the previous templates so be careful not to override anything!
    utils::clear_workspace(&config.output_dir, &config.tables).await?;

    // Iterates the given suite tables (as json files) and generate Typescript suite files
    // containing empty test templates
    let total_test_templates = test_gen::generate_all_suites(config.tables, config.output_dir).await?;
    println!("{}", Green.paint("Done!"));
    println!(" ⭐  Generated {} test templates ⭐ ", total_test_templates);
    Ok(())
}
