use regex::Regex;

pub fn concatenate_string(firstname: &str, lastname: &str) -> String {
    format!("{}{}", firstname, lastname)
}

pub fn string_to_binary_sum(msg: &str) -> u64 {
    let binary_vec: Vec<u8> = msg.bytes().collect();
    let binary_sum = binary_vec.iter().map(|&x| x as u64).sum();
    binary_sum
}

pub fn remove_symbol(input: &str) -> Vec<&str> {
    let symbol_removed = Regex::new(r"[\/\-\s]+").unwrap(); // Match symbols and spaces
    let parts: Vec<&str> = symbol_removed.split(input).collect();
    parts
}

pub fn remove_space(input: &str) -> String {
    input.chars().filter(|&c| c != ' ').collect()
}

pub fn add_numbers(parts: Vec<&str>) -> u64 {
    let mut total: u64 = 0;
    for part in parts {
        if !part.is_empty() {
            let number: u64 = part.parse().unwrap();
            total += number;
        }
    }
    total
}

pub fn string_slice(string: &str) -> String {
    let input: String = string.chars().rev().collect();
    input[..2].to_string()
}
