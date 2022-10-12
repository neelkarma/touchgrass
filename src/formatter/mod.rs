use crate::{context::Context, provider::Weather};

use self::{default::DefaultFormatter, json::JSONFormatter};

pub mod default;
pub mod json;

#[derive(Debug)]
pub enum Formatters {
    JSON,
    Default,
}

impl Formatters {
    pub fn into_formatter(&self) -> &dyn Formatter {
        match self {
            Self::Default => &DefaultFormatter,
            Self::JSON => &JSONFormatter,
        }
    }
}

pub trait Formatter {
    fn format(&self, ctx: &Context, weather: &Weather) -> String;
}
