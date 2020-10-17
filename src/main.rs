mod test_gen;
mod server;

use tokio;
extern crate serde;
extern crate serde_json;
use test_gen::test_gen::{TGenerator};
use ansi_term::Colour::{ Green};
use actix_web::{App, HttpServer, web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use actix_web::http::header;
use serde::{Deserialize, Serialize};
use crate::test_gen::utils;
use crate::server::routes::AppState;

async fn run_tsttgen() -> Result<usize, std::io::Error> {
    let config = utils::get_config_async("config.json").await?;
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    // Removes the previous templates so be careful not to override anything!
    utils::clear_workspace(&config.output_dir, &config.tables).await?;

    // Iterates the given suite tables (as json files) and generate Typescript suite files
    // containing empty test templates
    let mut generator =  TGenerator::new(HashMap::new());
    let total_test_templates = generator.generate_all_suites(config.tables, config.output_dir).await?;
    let total_suites = TGenerator::get_suites_state(&generator).count();

    println!("{}", Green.paint("Done!"));
    println!(" ⭐  Generated {} test templates from {} test suites", total_test_templates, total_suites);
    Ok(total_suites)
}

async fn serve_report(results_count: usize) -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080"; //todo: move to config.json
    let server =  HttpServer::new(move || {
        App::new().data(AppState {
            results_count,
        })
        .configure(server::routes::init_routes)})
        .bind(server_address)?
        .run();
    tokio::join!(server, utils::open_web_app(server_address));
    Ok(())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let results = run_tsttgen().await?;
    serve_report(results).await
}


