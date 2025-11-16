use std::f64::consts::E;
use crate::models::*;

pub fn present_value_discrete(
    future_value: f64,
    rate: f64,
    periods: f64
) -> f64 {
    future_value / (1.0 + rate).powf(periods)
}

pub fn present_value_cts(
    future_value: f64,
    rate: f64,
    time: f64
) -> f64 {
    future_value * E.powf(-rate * time)
}

pub fn present_value(
    future_value: f64,
    rate: f64,
    time: f64,
    compounding_type: CompoundingType
) -> f64 {
    match compounding_type {
        CompoundingType::Discrete => present_value_discrete(future_value, rate, time),
        CompoundingType::Continuous => present_value_cts(future_value, rate, time)
    }
}

pub fn present_value_stream(
    cash_flows: Vec<CashFlow>,
    rate: f64,
) -> f64 {
    cash_flows
        .iter()
        .fold(
            0.0,
            |acc, cash_flow| acc + present_value(
                cash_flow.amount, 
                rate, 
                cash_flow.time, 
                cash_flow.compounding_type
            )
        )
}

pub fn future_value_discrete(
    present_value: f64,
    rate: f64,
    periods: f64
) -> f64 {
    present_value * (1.0 + rate).powf(periods)
}

pub fn real_rate(
    nominal_rate: f64,
    inflation_rate: f64
) -> f64 {
    (1.0 + nominal_rate) / (1.0 + inflation_rate) - 1.0
}