use regex::Regex;

pub fn string_to_binary(msg: &str) -> Vec<u32> {
    msg.bytes().map(|x| x as u32).collect()
}

pub fn remove_symbol(str: &str) -> Result<Vec<u16>, &'static str> {

    let symbol_removed = Regex::new(r"[/]").unwrap();
    let parts: Vec<&str> = symbol_removed.split(str).collect();

    let mut integers = Vec::new();

    for part in parts {
        if !part.is_empty() {
            let number: u16 = part.parse().map_err(|_| "Invalid Number!")?;
            integers.push(number);
        }
    }

    Ok(integers)
}