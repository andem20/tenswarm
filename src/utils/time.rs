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