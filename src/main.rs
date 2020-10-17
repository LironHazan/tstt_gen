mod test_gen;
mod server;

use tokio;
use crate::server::server::serve_report;
use crate::test_gen::test_gen::run_tsttgen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let results = run_tsttgen().await?;
    serve_report(results).await
}


