use std::fs;
use std::{env, path::PathBuf};

fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    let key = "USERPROFILE";

    #[cfg(not(windows))]
    let key = "HOME";

    env::var_os(key).map(PathBuf::from)
}

pub fn read_savings() -> String {
    if let Some(home) = home_dir() {
        fs::read_to_string(
            format!("{}/savings.txt", home.display())
        ).unwrap_or_else(|_| write_savings("0"))
    } else {
        String::from("Couldn't read savings from home directory")
    }
}

pub fn write_savings(savings: &str) -> String {
    if let Some(home) = home_dir() {
        let path = home.join("savings.txt");
        match std::fs::write(&path, savings) {
            Ok(()) => format!("Saved to {}", path.display()),
            Err(_) => String::from("Couldn't write savings to home directory"),
        }
    } else {
        String::from("Couldn't write savings to home directory")
    }
}

pub fn adjust_savings(by: f64) {
    let current: f64 = read_savings().trim().parse().unwrap_or(0.0);
    let updated = current + by;
    write_savings(&format!("{}", updated));
}