use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_files as fs;
use actix_files::Files;

#[derive(Debug, Serialize, Deserialize)]
struct TestObj {
    name: String,
    number: i32,
}

#[get("/foos")]
async fn test_route() -> Result<HttpResponse, ()> {
    let test_obj: TestObj = TestObj {
        name: String::from("foo"),
        number: 32
    };
    Ok(HttpResponse::Ok().json(test_obj))
}

fn static_files() -> Files {
    fs::Files::new("/", "static/index.html")
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(test_route);
    config.service(static_files());
}
