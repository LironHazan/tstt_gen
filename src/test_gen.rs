use tokio::fs::*;
use tokio::prelude::*; // for write_all()
use serde_derive::{Deserialize, Serialize};
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    // id ?
    name: String,
    objective: String,
    steps: String,
    expected: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
    name: String,
    tests: Vec<Test>
}

async fn generate_test_suite(test_suites: Vec<Suite>, path: String) -> Result<(), std::io::Error> {
    for suite in test_suites.iter() {
        // Create file per suite
        let mut test_file = File::create(format!("{}/{}.ts", path, suite.name)).await?;
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
    let annotation_start = String::from("\n\n/** @Steps:");
    let annotation_end = String::from("\n**/");
    let step_prefix = String::from("\n");
    let expected = String::from("* @Expected:");

    let test_template = format!("\ntest.requestHooks({})('{}', async () => {{}});", logger, test.objective);
    let steps = normalize_test_steps(&test.steps);

    let test_template =
        annotation_start
            + &step_prefix
            + &steps
            + &*expected
            + &test.expected
            + &annotation_end
            + &test_template;
    test_template
}

fn normalize_test_steps(steps: &String) -> String {
    // Adding the astrix before each test for preserving the Typescript comment look
    let test_steps: String = steps.lines()
        .map(|step| format!("* {} \n", step) ).collect();
    String::from(test_steps)
}

pub async fn generate_all_suites(tables: Vec<String>, output_dir: String) -> Result<(), std::io::Error> {
    for table in tables {
        create_dir(&table).await?;
        let path = format!("sheets/tables/{}.json", table);
        let suite = utils::get_parsed_tables(path);
        generate_test_suite(suite, format!("{}/{}", output_dir,table)).await?;
    }
    Ok(())
}
