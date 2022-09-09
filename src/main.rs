mod clients;
mod utils;
mod scenario;
mod test_clients;


use std::{error::Error, sync::Arc};

use scenario::test_scenario::Scenario;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scenario = Arc::new(Scenario::new("testcase.http"));
    scenario.execute().await;

    Ok(())
}