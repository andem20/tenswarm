mod clients;
mod utils;
mod scenario;


use std::error::Error;

use scenario::test_scenario::Scenario;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scenario = Scenario::new("testcase.example");
    scenario.execute().await;

    Ok(())
}