mod config;
mod utils;

use std::{sync::Arc, collections::HashSet};

use futures::StreamExt;
use reqwest::{Client, Error};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    create_requests(
        config::URL,
        config::NUM_REQUESTS,
        config::DELAY_NANOS,
        config::REQS_PR_SECOND,
    )
    .await;
}

async fn create_requests(
    url: &'static str,
    num_requests: usize,
    delay_nanos: u64,
    reqs_pr_second: u64,
) {
    let client = Arc::new(Client::new());

    let mut requests = futures::stream::FuturesUnordered::new();

    for i in 0..num_requests {
        let request = send_request(url, &client, i, delay_nanos);
        requests.push(request);
    }

    let start_time = std::time::Instant::now();

    let mut count = 0.0;
    let mut response_time: u128 = 0;
    let mut errors = 0;
    let mut error_messages = HashSet::new();

    while let Some(response) = requests.next().await {
        match response {
            Ok(response) => {
                count += 1.0;
                
                match response {
                    Ok(r) => {
                        response_time += r.2;

                        
                        let progress = ((count / num_requests as f32) * 100.0) as usize;
                        let mut characters =
                        std::iter::repeat("=").take(progress).collect::<String>();
                        
                        if progress < 100 {
                            characters.push('>');
                        }
                        
                        let avg_response_time = response_time as f32 / count;
                        let error_rate = (errors as f32 / num_requests as f32) * 100.0;
                        

                        utils::clear_terminal();
                        println!(
                            "Avg. response time: {} ms, Error rate: {:>3}%, Responses received: {}",
                            avg_response_time, error_rate, count
                        );

                        println!("[{:<101}{:>3}% ]", characters, progress);
                    }
                    Err(e) => {
                        errors += 1;
                        error_messages.insert(e.to_string());
                        continue;
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    let time = start_time.elapsed();

    utils::print_conclusion(num_requests, time, reqs_pr_second);
    utils::print_errormessages(error_messages);
}


fn send_request(url: &'static str, client: &Arc<Client>, i: usize, delay_nanos: u64) -> JoinHandle<Result<(usize, String, u128), Error>> {
    let client = client.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_nanos(delay_nanos * (i as u64))).await;

        let start_time = std::time::Instant::now();
        let response = client.get(url).send().await;
        let time = start_time.elapsed().as_millis();

        let result = match response {
            Ok(r) => (i, r.status().as_str().to_owned(), time),
            Err(e) => return Err(e),
        };

        Ok(result)
    })
}