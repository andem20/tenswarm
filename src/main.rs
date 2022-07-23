mod tcp_client;
mod config;
mod utils;

use tcp_client::TcpClient;

use std::{error::Error, sync::Arc, time::Instant};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    validate_url(config::URL);

    let mut tasks = Vec::with_capacity(config::NUM_CLIENTS);

    let headers = Arc::new(vec![format!("GET {} HTTP/1.1", "/").as_str(), "\r\n"].join("\r\n"));

    let total_response_time: Arc<Mutex<u128>> = Arc::new(Mutex::new(0));
    let total_response_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    let total_time = Arc::new(Instant::now());

    for _client in 0..config::NUM_CLIENTS {
        let task = create_client(
            &headers,
            &total_time,
            &total_response_count,
            &total_response_time,
        );

        tasks.push(task);
    }

    futures::future::join_all(tasks).await;

    let elapsed_time = total_time.elapsed();
    let total_response_count = *total_response_count.lock().await;

    let reqs_pr_second = total_response_count as f32 / elapsed_time.as_secs_f32();

    utils::print_conclusion(total_response_count, elapsed_time, reqs_pr_second);

    Ok(())
}

fn validate_url(url: &'static str) {
    match std::net::TcpStream::connect(url) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}

fn create_client(
    headers: &Arc<String>,
    total_time: &Arc<Instant>,
    total_response_count: &Arc<Mutex<usize>>,
    total_response_time: &Arc<Mutex<u128>>,
) -> tokio::task::JoinHandle<Result<(), std::io::Error>> {
    let headers = headers.clone();
    let total_response_count = total_response_count.clone();
    let total_response_time = total_response_time.clone();
    let total_time = total_time.clone();

    tokio::task::spawn(async move {
        let mut client = TcpClient::new();
        let client = client.connect(config::URL).await;

        while total_time.elapsed().as_millis() < config::PEAK_DURATION {
            let start_time = Instant::now();

            client
                .get(config::URL.to_owned(), headers.as_bytes())
                .await?;

            let response_time = start_time.elapsed().as_millis();

            let mut total_response_time = total_response_time.lock().await;
            let mut total_response_count = total_response_count.lock().await;

            *total_response_time += response_time;
            *total_response_count += 1;

            let progress = total_time.elapsed().as_millis() as f32 / config::PEAK_DURATION as f32;
            utils::print_progress(progress, *total_response_time, *total_response_count);
        }

        Ok::<(), std::io::Error>(())
    })
}
