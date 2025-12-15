use wasm_bindgen::prelude::*;

pub mod mcc;
pub mod geometry;

#[wasm_bindgen]
pub fn calculate_mcc(sequence: &str) -> f64 {
    mcc::calculate_mcc(sequence)
}

#[wasm_bindgen]
pub fn get_move_count(algorithm: &str, metric: &str) -> u32 {
    mcc::get_move_count(algorithm, metric)
}
