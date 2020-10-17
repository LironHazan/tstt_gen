use tokio::fs::{File, create_dir};
use tokio::prelude::*; // for write_all()
use serde_derive::{Deserialize, Serialize};
use crate::test_gen::utils;
use ansi_term::Colour::{ Green, Blue};
use std::collections::HashMap;
use std::collections::hash_map::{Values};
extern crate serde;
extern crate serde_json;

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

pub struct TGenerator {
    suites_map: HashMap<String, Vec<String>>
}

impl TGenerator  {
    pub fn new(suites_map: HashMap<String, Vec<String>>) -> Self { // builder
        Self { suites_map }
    }

    pub fn get_suites_state(&self) -> Values<'_, String, Vec<String>> {
        self.suites_map.values()
    }

    async fn generate_test_suite(&mut self, test_suites: Vec<Suite>, path: String) -> Result<u32, std::io::Error> {
        let mut tests_count = 0;
        for suite in test_suites.iter() {
            self.suites_map.entry((*suite.name).parse().unwrap()).or_insert(vec![]);
            // Create file per suite
            let mut test_file = File::create(format!("{}/{}.ts", path, suite.name)).await?;
            test_file.write_all(format!("// Test Suite was generated for: {}\n", suite.name).as_bytes()).await?;
            let logger = "logger";
            test_file.write_all(format!("export const {} = ({}) => {{", suite.name, logger).as_bytes()).await?;

            for test in suite.tests.iter() {
                // self.suites_map.get(&*suite.name).unwrap().push((*test.name).parse().unwrap());
                tests_count = tests_count + 1;
                let test_template = self.generate_test_template(test, logger);
                test_file.write_all(test_template.as_bytes()).await?;
            }

            // Closing suite main fn
            test_file.write_all(b"\n\n};").await?;
        }
        println!("Total suites per table: {:?}", test_suites.len());
        println!("Total Test templates per table: {:?} ✔️  ", tests_count);
        Ok(tests_count)
    }

    fn generate_test_template(&mut self, test: &Test, logger: &str) -> String {
        // Test template defaults
        let annotation_start = String::from("\n\n/** @Steps:");
        let annotation_end = String::from("\n**/");
        let step_prefix = String::from("\n");
        let expected = String::from("* @Expected:");

        let test_template = format!("\ntest.requestHooks({})('{}', async () => {{}});", logger, test.objective);
        let steps = Self::normalize_test_steps(&test.steps);

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

    pub async fn generate_all_suites(&mut self, tables: Vec<String>, output_dir: String) -> Result<u32, std::io::Error> {
        let mut tests_count = 0;
        for table in tables {
            println!("⚙️⚙️⚙️⚙️⚙️⚙️⚙️⚙️⚙️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️️⚙️");
            println!("Generating test suites from: {}",Blue.paint(&table));
            create_dir(&table).await?;
            let path = format!("sheets/tables/{}.json", table);
            let suites = utils::get_parser_tables_async(path).await?;
            let count = self.generate_test_suite(suites, format!("{}/{}", output_dir,table)).await?;
            tests_count = count + tests_count;
        }
        Ok(tests_count)
    }
}

pub async fn run_tsttgen() -> Result<usize, std::io::Error> {
    let config = utils::get_config_async("config.json").await?;
    println!("{}", utils::get_project_ascii_art());
    println!("{}", Green.paint("Generating test suites into: "));
    println!("{}", Green.paint(&config.output_dir));

    // Removes the previous templates so be careful not to override anything!
    utils::clear_workspace(&config.output_dir, &config.tables).await?;

    // Iterates the given suite tables (as json files) and generate Typescript suite files
    // containing empty test templates
    let mut generator =  TGenerator::new(HashMap::new());
    let total_test_templates = generator.generate_all_suites(config.tables, config.output_dir).await?;
    let total_suites = TGenerator::get_suites_state(&generator).count();

    println!("{}", Green.paint("Done!"));
    println!(" ⭐  Generated {} test templates from {} test suites", total_test_templates, total_suites);
    Ok(total_suites)
}

