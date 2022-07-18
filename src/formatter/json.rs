use crate::{context::Context, provider::Weather};

#[derive(Debug)]
pub struct JSONFormatter;

impl JSONFormatter {
    pub fn format(_ctx: &Context, weather: &Weather) -> String {
        match serde_json::to_string(weather) {
            Ok(json) => json,
            Err(_) => panic!("Couldn't serialize Weather struct to JSON"),
        }
    }
}
