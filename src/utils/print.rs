use std::{collections::HashSet, time::Duration};

const PROGRESS_BAR_SIZE: usize = 40;

// TODO this should take a struct containing all relevant information
pub fn print_conclusion(num_requests: usize, time: Duration, reqs_pr_second: f32) {
    println!("\n\n+---------------------------------");
    println!("|");
    println!("|  Requests sent: {}", num_requests);
    println!("|  Time elapsed: {:.2} seconds", time.as_secs_f32());
    println!("|  {:.2} requests pr. second", reqs_pr_second);
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

    let mut characters: String = std::iter::repeat("=").take((progress * PROGRESS_BAR_SIZE as f32) as usize).collect();

    if progress < 1.0 {
        characters.push('>');
    }

    clear_terminal();
    println!("[{characters:<size$}] {progress_percent:>3}%", size = PROGRESS_BAR_SIZE);
}
