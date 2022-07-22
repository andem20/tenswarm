use std::{collections::HashSet, time::Duration};

// TODO this should take a struct containing all relevant information
pub fn print_conclusion(num_requests: usize, time: Duration, reqs_pr_second: f32) {
    println!("\n\n+---------------------------------");
    println!("|");
    println!("|  Requests sent: {}", num_requests);
    println!("|  Time elapsed: {} seconds", time.as_secs_f32());
    println!("|  {reqs_pr_second} requests pr. second");
    println!("|");
    println!("+---------------------------------\n\n");
}

pub fn clear_terminal() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print_errormessages(error_messages: HashSet<String>) {
    if error_messages.len() == 0 {
        return;
    }

    println!("Encountered following errors:");
    error_messages.iter().for_each(|msg| {
        println!("\x1b[93m[X] {}\x1b[0m", msg);
    });
}

pub fn print_progress(progress: f32, response_time: u128, response_count: usize, error_count: usize, num_requests: usize) {
    let progress_percent = (progress * 100.0) as usize;

    let mut characters: String = std::iter::repeat("=").take(progress_percent).collect();

    if progress < 1.0 {
        characters.push('>');
    }

    let avg_response_time = response_time as f32 / response_count as f32;
    let error_rate = (error_count as f32 / num_requests as f32) * 100.0;

    clear_terminal();
    println!(
        "Avg. response time: {} ms, Error rate: {:>3}%, Responses received: {}",
        avg_response_time, error_rate, response_count
    );

    println!("[{:<100}] {:>3}%", characters, progress_percent);
}
