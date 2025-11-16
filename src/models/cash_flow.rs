use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum CompoundingType {
    Discrete,
    Continuous
}

impl From<&str> for CompoundingType {
    fn from(s: &str) -> Self {
        match s {
            "discrete" => CompoundingType::Discrete,
            "continuous" => CompoundingType::Continuous,
            _ => CompoundingType::Discrete
        }
    }
}

impl From<String> for CompoundingType {
    fn from(s: String) -> Self { CompoundingType::from(&s[..]) }
}

impl Display for CompoundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CompoundingType::Discrete => "discrete".to_string(),
            CompoundingType::Continuous => "continuous".to_string()
        };
        write!(f, "{}", str)
    }
}

impl Default for CompoundingType {
    fn default() -> Self { CompoundingType::Discrete }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CashFlow {
    pub amount: f64,
    pub time: f64,
    pub compounding_type: CompoundingType
}

impl CashFlow {
    pub fn new(
        amount: f64,
        time: f64,
        compounding_type: CompoundingType
    ) -> Self {
        Self { amount, time, compounding_type }
    }
}

impl Default for CashFlow {
    fn default() -> Self {
        CashFlow {
            amount: 0.0, 
            time: 0.0, 
            compounding_type: CompoundingType::default()
        }
    }
}
