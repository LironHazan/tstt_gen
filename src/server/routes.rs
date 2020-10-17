use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_files as fs;
use actix_files::Files;

pub struct AppState {
    pub results_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestObj {
    name: String,
    number: usize,
}

#[get("/results")]
async fn test_route(data: web::Data<AppState>) -> Result<HttpResponse, ()> {
    let test_obj: TestObj = TestObj {
        name: String::from("foo"),
        number: data.results_count
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
