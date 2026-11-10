use std::fs;

pub fn read(css_path: &str) -> String {
    fs::read_to_string(format!("assets/{}", css_path)).unwrap_or_else(|_| String::from(""))
}
