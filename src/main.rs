use std::{sync::Arc, time::Duration};

use futures::StreamExt; // 0.3.5
use reqwest::Client; // 0.10.6

const NUM_REQUESTS: usize = 10000;
const REQ_PR_SECOND: u64 = 1000;
const DELAY_MILLIS: u64 = 1000 / REQ_PR_SECOND;
const URL: &'static str = "http://localhost:9090";

#[tokio::main]
async fn main() {
    create_requests(NUM_REQUESTS).await;
}

async fn create_requests(num_requests: usize) {
    let client = Arc::new(Client::new());

    let mut requests = futures::stream::FuturesUnordered::new();

    for i in 0..num_requests {
        let client = client.clone();
        let request = tokio::spawn(async move {

            tokio::time::sleep(std::time::Duration::from_millis(DELAY_MILLIS * i as u64)).await;

            // println!("Sending Request #{i}");
            let start_time = std::time::Instant::now();

            let response = client.get(URL).send().await;

            let time = start_time.elapsed().as_millis();

            let result = match response {
                Ok(r)  => (i, r.status().as_str().to_owned(), time),
                Err(e) => return Err(e),
            };

            Ok(result)
        });

        requests.push(request);
    }

    let start_time = std::time::Instant::now();

    let mut count = 0.0;
    let mut response_time: u128 = 0;
    let mut errors = 0;

    while let Some(response) = requests.next().await {
        match response {
            Ok(response) => {
                count += 1.0;
                match response {
                    Ok(r) => {
                        response_time += r.2;
                        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        let progress = ((count / NUM_REQUESTS as f32) * 100.0) as usize;
                        let mut characters = std::iter::repeat("=").take(progress).collect::<String>();
                        characters.push('>');
                        println!("Avg. response time: {} ms, Error rate: {:>3}%, Sent requests: {}", response_time as f32 / count, (errors as f32 / NUM_REQUESTS as f32) * 100.0, count);
                        println!("[ {:<101} {:>3}% ]", characters, progress);
                    },
                    Err(e) => {
                        errors += 1;
                        panic!("{}", e);
                        continue;
                    }
                }
            },
            Err(e) => {
                errors += 1;
                panic!("{}", e);
                continue;
            }
        }
    }

    let time = start_time.elapsed().as_secs_f64();

    println!("\n\n+------------------------------------------------");
    println!("|");
    println!("|  Sent {} requests in:", num_requests);
    println!("|  {time} seconds");
    println!("|  {REQ_PR_SECOND} requests pr. second");
    println!("|");
    println!("+------------------------------------------------\n\n");
}