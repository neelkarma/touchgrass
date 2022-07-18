use self::{default::DefaultFormatter, json::JSONFormatter};

pub mod default;
pub mod json;

#[derive(Debug)]
pub enum Formatter {
    JSON,
    Default,
}
