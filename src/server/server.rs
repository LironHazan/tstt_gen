use crate::server::routes::AppState;
use crate::server;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};

pub async fn start_server(results_count: usize) -> Result<Server, std::io::Error> {
    let server_address = "127.0.0.1:8080"; //todo: move to config.json
    let server = HttpServer::new(move || {
        App::new().data(AppState {
            results_count,
        }).configure(server::routes::init_routes)})
        .bind(server_address)?
        .run();
    Ok(server)
}
