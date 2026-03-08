use crate::reporter::Reporter;

pub enum NumberIn {
    Literal(f64),
    Reporter(Reporter),
}

pub enum StringIn {
    Literal(String),
    Reporter(Reporter),
}
