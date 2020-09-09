use tokio::fs::File;
use tokio::prelude::*; // for write_all()
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    // id ?
    name: String,
    objective: String,
    steps: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
    name: String,
    tests: Vec<Test>
}

pub async fn generate_test_suite(test_suites: Vec<Suite>, filepath: &str) -> Result<(), std::io::Error> {
    for suite in test_suites.iter() {
        // Create file per suite ** todo - support different OS types
        let mut test_file = File::create(format!("{}.ts", suite.name)).await?;
        test_file.write_all(format!("// Test Suite was generated for: {}\n", suite.name).as_bytes()).await?;
        let logger = "logger";
        test_file.write_all(format!("export const {} = ({}) => {{", suite.name, logger).as_bytes()).await?;

        for test in suite.tests.iter() {
            let test_template = generate_test_template(test, logger);
            test_file.write_all(test_template.as_bytes()).await?;
        }

        // Closing suite main fn
        test_file.write_all(b"\n\n};").await?;
    }
    Ok(())
}

fn generate_test_template(test: &Test, logger: &str) -> String {
    // Test template defaults
    let annotation_start = String::from("\n\n/** Test Flow:");
    let annotation_end = String::from("\n**/");
    let step_prefix = String::from("\n*");
    let test_template = format!("\ntest.requestHooks({})('{}', async () => {{}});", logger, test.objective);

    let test_template =
        annotation_start
            + &step_prefix
            + &test.steps
            + &annotation_end
            + &test_template;
    test_template
}
