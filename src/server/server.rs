use crate::server::routes::AppState;
use crate::server;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};

// Starts the server with wanted configuration and shared state
async fn start_server(results_count: usize, server_address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().data(AppState {
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
