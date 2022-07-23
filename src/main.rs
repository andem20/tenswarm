mod config;
mod utils;

use std::{sync::Arc, error::Error};

use tokio::{net::TcpStream, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, sync::Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match TcpStream::connect(config::URL).await {
        Ok(stream) => stream,
        Err(e) => panic!("{e}")
    };

    let mut tasks = Vec::with_capacity(config::NUM_CLIENTS);

    let headers = Arc::new(vec![
        format!("GET {} HTTP/1.1", "/").as_str(),
        "\r\n",
    ].join("\r\n"));

    let total_response_time = Arc::new(Mutex::new(0));
    let total_response_count = Arc::new(Mutex::new(0));

    let overall_time = Arc::new(std::time::Instant::now());
        
    for _client in 0..config::NUM_CLIENTS {
        let headers = headers.clone();
        let total_response_count = total_response_count.clone();
        let total_response_time = total_response_time.clone();

        let overall_time = overall_time.clone();
        
        let task = tokio::task::spawn(async move {
            let stream = TcpStream::connect(config::URL).await?;

            let mut stream = BufReader::new(stream);
            let mut buffer = String::new();
            
            while overall_time.elapsed().as_millis() < config::PEAK_DURATION {

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

                let progress = overall_time.elapsed().as_millis() as f32 / config::PEAK_DURATION as f32;
                utils::print_progress(
                    progress,
                    *total_response_time,
                    *total_response_count,
                );
            }

            Ok::<(), std::io::Error>(())
        });

        tasks.push(task);
    }

    futures::future::join_all(tasks).await;

    let elapsed_time = overall_time.elapsed();
    let total_response_count = *total_response_count.lock().await;

    let reqs_pr_second = total_response_count as f32 / elapsed_time.as_secs_f32();

    utils::print_conclusion(total_response_count, elapsed_time, reqs_pr_second);

    Ok(())
}