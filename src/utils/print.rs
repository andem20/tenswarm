use std::{collections::HashSet, time::{Duration, Instant}};

const PROGRESS_BAR_SIZE: usize = 40;

// TODO this should take a struct containing all relevant information
pub fn print_conclusion(total_start_time: Instant, total_response_count: u32, total_response_time: u128) {
    println!("\n\n+---------------------------------");
    println!("|");
    println!("|  Time elapsed: {:.2}s", total_start_time.elapsed().as_secs_f32());
    println!("|  Response Count: {total_response_count}");
    println!("|  Requests pr. second: {:.2}", total_response_count as f32 / total_start_time.elapsed().as_secs_f32());
    println!("|  Avg. response time: {:.2}ms", total_response_time as f32 / total_response_count as f32);
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

pub fn print_progress(progress: f32) {
    let progress_percent = (progress * 100.0) as usize;

    let mut characters: String = "=".repeat((progress * PROGRESS_BAR_SIZE as f32) as usize);

    if progress < 1.0 {
        characters.push('>');
    }

    clear_terminal();
    println!("[{characters:<size$}] {progress_percent:>3}%", size = PROGRESS_BAR_SIZE);
}
