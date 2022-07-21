use std::{time::Duration, collections::HashSet};

// TODO this should take a struct containing all relevant information
pub fn print_conclusion(num_requests: usize, time: Duration, reqs_pr_second: u64) {
    println!("\n\n+---------------------------------");
    println!("|");
    println!("|  Requests sent: {}", num_requests);
    println!("|  Time elapsed: {} seconds", time.as_secs());
    println!("|  {reqs_pr_second} requests pr. second");
    println!("|");
    println!("+---------------------------------\n\n");
}

pub fn clear_terminal() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print_errormessages(error_messages: HashSet<String>) {
    if error_messages.len() == 0 { return }
    
    println!("Encountered following errors:");
    error_messages.iter().for_each(|msg| {
        println!("\x1b[93m[X] {}\x1b[0m", msg);
    });
}