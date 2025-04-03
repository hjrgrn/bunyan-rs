use crate::record::LogRecord;
use crate::Format;
use std::io::BufRead;

pub fn process_input<T: BufRead>(buffer: T, format: Format, level_filter: u8, strict: bool) {
    let mut log = String::new();
    for line in buffer.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                if !strict {
                    eprintln!("{}", e);
                }
                continue;
            }
        };
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => {
                if r.level >= level_filter {
                    // TODO: not sure about &format
                    r.format(&format, &mut log);
                    print!("{}", log);
                }
            }
            Err(_) => {
                if !strict {
                    println!("{}", line)
                }
            }
        }
    }
}
