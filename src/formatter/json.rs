use crate::{context::Context, provider::Weather};

use super::Formatter;

#[derive(Debug)]
pub struct JSONFormatter;

impl Formatter for JSONFormatter {
    fn format(&self, _ctx: &Context, weather: &Weather) -> String {
        match serde_json::to_string(weather) {
            Ok(json) => json,
            Err(_) => panic!("Couldn't serialize Weather struct to JSON"),
        }
    }
}
