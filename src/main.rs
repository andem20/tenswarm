mod config;
mod utils;

use std::{sync::Arc, error::Error};

use tokio::{net::TcpStream, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, sync::Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks = Vec::with_capacity(config::NUM_CLIENTS);

    let headers = Arc::new(vec![
        format!("GET {} HTTP/1.1", "/").as_str(),
        "\r\n",
    ].join("\r\n"));

    let total_response_time = Arc::new(Mutex::new(0));
    let total_response_count = Arc::new(Mutex::new(0));

    let start_time = std::time::Instant::now();
        
    for _client in 0..config::NUM_CLIENTS {
        let headers = headers.clone();
        let total_response_count = total_response_count.clone();
        let total_response_time = total_response_time.clone();

        let task = tokio::task::spawn(async move {
            let stream = TcpStream::connect(config::URL).await?;
            let mut stream = BufReader::new(stream);
            
            for i in 0..config::NUM_REQUESTS {
                let mut buffer = String::new();

                let start_time = std::time::Instant::now();

                stream.write_all(headers.as_bytes()).await?;
                
                stream.read_line(&mut buffer).await?;
                let len = stream.fill_buf().await.unwrap().len();
                stream.consume(len);

                let response_time = start_time.elapsed().as_millis();

                let mut total_response_time = total_response_time.lock().await;
                let mut total_response_count = total_response_count.lock().await;
                *total_response_time += response_time;
                *total_response_count += 1;

                let progress = *total_response_count as f32 / (config::NUM_REQUESTS * config::NUM_CLIENTS) as f32;
                utils::print_progress(
                    progress,
                    *total_response_time,
                    *total_response_count,
                    0, // TODO
                    config::NUM_REQUESTS,
                );
            }

            Ok::<(), std::io::Error>(())
        });

        tasks.push(task);
    }

    futures::future::join_all(tasks).await;

    let elapsed_time = start_time.elapsed();

    let reqs_pr_second = (config::NUM_CLIENTS * config::NUM_REQUESTS) as f32 / elapsed_time.as_secs_f32();

    utils::print_conclusion(config::NUM_CLIENTS * config::NUM_REQUESTS, elapsed_time, reqs_pr_second);

    Ok(())
}