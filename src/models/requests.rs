use serde::{Deserialize, Serialize};
use crate::models::CashFlow;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresentValueRequest {
    pub cash_flows: Vec<CashFlow>,
    pub nominal_rate: f64,
    pub inflation_rate: f64,
}