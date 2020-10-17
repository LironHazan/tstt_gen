use crate::server::routes::AppState;
use crate::server;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;


// Starts the server with wanted configuration and shared state
async fn start_server(results_count: usize, server_address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().wrap(
            Cors::new()
                .allowed_origin("http://localhost:3000") // The local client I'm developing in Elm is currently using port 3000
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
                .finish(),
        )
            .data(AppState {
            results_count,
        }).configure(server::routes::init_routes)})
        .bind(server_address)?
        .run();
    Ok(server)
}

// Opens chrome once the server starts
async fn open_web_app(url: &str) -> () {
    use std::process::Command;
    let mut cmd = Command::new("open");
    cmd.arg("-a")
        .arg("Google Chrome")
        .arg(format!("http://{}", url))
        .spawn()
        .expect("failed to spawn child");
    println!("{}", url)
}

pub async fn serve_report(results_count: usize) -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080"; //todo: move to config.json
    let server = start_server(results_count, server_address).await?;
    tokio::join!(server, open_web_app(server_address));
    Ok(())
}
