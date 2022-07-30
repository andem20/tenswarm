mod http_client;
mod config;
mod print_utils;
mod test_scenario;

use test_scenario::Scenario;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scenario = Scenario::new("testcase.example");
    scenario.execute().await;

    Ok(())
}