mod test_gen;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main()  -> Result<(), std::io::Error> {
    let json_file_path = Path::new("test.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let suite = serde_json::from_reader(json_file).expect("error while reading json");

    test_gen::generate_test_suite(suite, "").await?;
    Ok(())
}
