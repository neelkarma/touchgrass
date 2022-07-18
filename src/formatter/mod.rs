pub mod default;
pub mod json;

#[derive(Debug)]
pub enum Formatter {
    JSON,
    Default,
}
