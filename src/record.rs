use crate::{Format, NamedLogLevel};
use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use itertools::Itertools;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Write;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LogRecord<'a> {
    /// This is the bunyan log format version. The log version is a single integer.
    /// It is meant to be 0 until version "1.0.0" of `node-bunyan` is released.
    /// Thereafter, starting with 1, this will be incremented if there is any backward incompatible
    /// change to the log record format.
    #[serde(rename = "v")]
    #[allow(dead_code)]
    pub version: u8,
    /// Name of the service/application emitting logs in bunyan format.
    pub name: &'a str,
    /// Log message.
    #[serde(rename = "msg")]
    pub message: Cow<'a, str>,
    /// See `LogLevel`
    pub level: u8,
    /// Name of the operating system host.
    pub hostname: &'a str,
    /// Process identifier.
    #[serde(rename = "pid")]
    pub process_identifier: u32,
    /// The time of the event captured by the log in [ISO 8601 extended format](http://en.wikipedia.org/wiki/ISO_8601).
    pub time: DateTime<Utc>,
    /// Any extra contextual piece of information in the log record.
    #[serde(flatten)]
    pub extras: serde_json::Map<String, serde_json::Value>,
}

impl LogRecord<'_> {
    pub fn format(&self, format: &Format, log: &mut String) {
        let level = format_level(self.level);
        log.clear();
        match format {
            Format::Long => {
                if write!(
                    log,
                    "[{}] {}: {}/{} on {}: {}{}",
                    self.time.to_rfc3339_opts(SecondsFormat::Millis, true),
                    level,
                    self.name,
                    self.process_identifier,
                    self.hostname,
                    self.message.cyan(),
                    format_extras(&self.extras)
                )
                .is_err()
                {
                    log.clear();
                    *log = format!(
                        "[{}] {}: {}/{} on {}: {}{}",
                        self.time.to_rfc3339_opts(SecondsFormat::Millis, true),
                        level,
                        self.name,
                        self.process_identifier,
                        self.hostname,
                        self.message.cyan(),
                        format_extras(&self.extras)
                    );
                };
            }
            Format::Short => {
                if write!(
                    log,
                    "{} {} {}: {}{}",
                    self.time.format("%H:%M:%S%.3fZ"),
                    level,
                    self.name,
                    self.message.cyan(),
                    format_extras(&self.extras)
                )
                .is_err()
                {
                    log.clear();
                    *log = format!(
                        "{} {} {}: {}{}",
                        self.time.format("%H:%M:%S%.3fZ"),
                        level,
                        self.name,
                        self.message.cyan(),
                        format_extras(&self.extras)
                    );
                };
            }
            Format::Json => {
                *log = serde_json::to_string_pretty(&self).expect("This should not happen")
            }
            Format::JsonN(l) => {
                let indent = " ".repeat(<u8 as Into<usize>>::into(*l));
                let value = serde_json::to_value(self).expect("This should not happen");
                *log = json_to_indented_string(&value, &indent);
            }
            Format::Bunyan => {
                if writeln!(
                    log,
                    "{}",
                    serde_json::to_string(&self).expect("This should not happen")
                )
                .is_err()
                {
                    log.clear();
                    *log = format!(
                        "{}\n",
                        serde_json::to_string(&self).expect("This should not happen")
                    )
                };
            }
        }
    }
}

pub fn format_level(level: u8) -> String {
    if let Ok(level) = NamedLogLevel::try_from(level) {
        match level {
            // Making sure all levels are 5 characters
            NamedLogLevel::Fatal => "FATAL".reversed(),
            NamedLogLevel::Error => "ERROR".red(),
            NamedLogLevel::Warn => " WARN".magenta(),
            NamedLogLevel::Info => " INFO".cyan(),
            NamedLogLevel::Debug => "DEBUG".yellow(),
            NamedLogLevel::Trace => "TRACE".white(),
        }
        .to_string()
    } else {
        format!("LVL{}", level)
    }
}

pub fn format_extras(extra_fields: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut details = String::new();
    let mut extras = String::new();
    let mut details_first_iter = true;
    let mut stringfied = String::new();
    let mut extras_first_iter = true;

    for (key, value) in extra_fields {
        if let serde_json::Value::String(s) = value {
            // Preserve strings unless they contain whitespaces/are empty
            // In that case, we want surrounding quotes.
            if s.contains(' ') || s.is_empty() {
                if write!(&mut stringfied, "\"{}\"", s).is_err() {
                    stringfied = format!("\"{}\"", s);
                }
            } else if write!(&mut stringfied, "{}", s).is_err() {
                stringfied = s.to_owned();
            }
        } else {
            stringfied = json_to_indented_string(value, "  ");
        }

        if stringfied.contains('\n') || stringfied.len() > 50 {
            if details_first_iter {
                details_first_iter = false;
            } else {
                details.push_str("\n    --\n");
            }
            if let serde_json::Value::String(s) = value {
                details.push_str(&indent(&format!("{}: {}", key.bold(), s)));
            } else {
                details.push_str(&indent(&format!("{}: {}", key.bold(), stringfied)));
            }
        } else {
            if extras_first_iter {
                extras.push_str(" (");
                extras_first_iter = false;
            } else {
                extras.push(',');
            }

            extras.push_str(&key.bold().to_string());
            extras.push('=');
            extras.push_str(&stringfied);
        }
        stringfied.clear();
    }
    if !details_first_iter {
        details.push('\n');
    }
    if !extras.is_empty() {
        extras.push_str(")\n");
        extras.push_str(&details);
    } else {
        extras.push('\n');
        extras.push_str(&details);
    }
    extras
}

/// Serialize a JSON value to a string using the specified indentation.
///
/// It mimics the implementation of `serde_json::to_string_pretty`.
fn json_to_indented_string(value: &serde_json::Value, indent: &str) -> String {
    let mut writer = Vec::with_capacity(128);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer).unwrap();
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

pub fn indent(s: &str) -> String {
    format!("    {}", s.lines().join("\n    "))
}
