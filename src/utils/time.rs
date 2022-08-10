use std::time::Duration;

use super::print::print_progress;

pub fn string_to_millis_u128(time: &str) -> u128 {
    let unit: String = time.chars().filter(|c| !c.is_digit(10)).collect();
    let unit = unit.as_str();
    let time: u128 = time
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let factor = match unit {
        "t" => 3600000,
        "m" => 60000,
        "s" => 1000,
        "ms" => 1,
        _ => 1,
    };

    time * factor
}

pub fn create_timer(duration_millis: u128) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let start_time = tokio::time::Instant::now();
        let mut interval = tokio::time::interval(Duration::from_millis(1000));

        loop {
            let instant = interval.tick().await;

            let progress =
                instant.duration_since(start_time).as_millis() as f32 / duration_millis as f32;
            print_progress(progress);

            if instant.duration_since(start_time).as_millis() >= duration_millis {
                break;
            }
        }
    })
}
