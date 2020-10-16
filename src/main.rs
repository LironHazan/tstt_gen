mod test_gen;
mod utils;
use tokio;
extern crate serde;
extern crate serde_json;
use test_gen::TGenerator;
use ansi_term::Colour::{ Green};
use actix_files as fs;
use actix_web::{App, HttpServer, web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use actix_web::http::header;
use serde::{Deserialize, Serialize};

async fn run_tsttgen() -> Result<(), std::io::Error> {
    let config = utils::get_config_async("config.json").await?;
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    // Removes the previous templates so be careful not to override anything!
    utils::clear_workspace(&config.output_dir, &config.tables).await?;

    // Iterates the given suite tables (as json files) and generate Typescript suite files
    // containing empty test templates
    let total_test_templates = TGenerator::new(HashMap::new())
        .generate_all_suites(config.tables, config.output_dir).await?;
    println!("{}", Green.paint("Done!"));
    println!(" ⭐  Generated {} test templates ⭐ ", total_test_templates);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct TestObj {
    name: String,
    number: i32,
}

async fn test_route(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    let test_obj: TestObj = TestObj {
        name: String::from("foo"),
        number: 32
    };

    HttpResponse::Ok()
        .content_type("text/plain")
        .json(test_obj)
}

async fn serve_report() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080"; //todo: move to config.json
    let server =  HttpServer::new(|| {
        App::new()
            .service(web::resource("/results").route(web::get().to(test_route)))
            .service(fs::Files::new("/", "static/index.html"))
    })
        .bind(server_address)?
        .run();
    tokio::join!(server, utils::open_web_app(server_address));

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_tsttgen().await?;
    serve_report().await
}


