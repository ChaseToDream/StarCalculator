use crate::modules::math::*;

#[tauri::command]
pub fn math_add(a: f64, b: f64) -> f64 {
    add(a, b)
}

#[tauri::command]
pub fn math_subtract(a: f64, b: f64) -> f64 {
    subtract(a, b)
}

#[tauri::command]
pub fn math_multiply(a: f64, b: f64) -> f64 {
    multiply(a, b)
}

#[tauri::command]
pub fn math_divide(a: f64, b: f64) -> Result<f64, String> {
    divide(a, b)
}

#[tauri::command]
pub fn math_sin(x: f64) -> f64 {
    sin(x)
}

#[tauri::command]
pub fn math_cos(x: f64) -> f64 {
    cos(x)
}

#[tauri::command]
pub fn math_tan(x: f64) -> f64 {
    tan(x)
}

#[tauri::command]
pub fn math_ln(x: f64) -> Result<f64, String> {
    ln(x)
}

#[tauri::command]
pub fn math_log10(x: f64) -> Result<f64, String> {
    log10(x)
}

#[tauri::command]
pub fn math_pow(base: f64, exponent: f64) -> f64 {
    pow(base, exponent)
}

#[tauri::command]
pub fn math_sqrt(x: f64) -> Result<f64, String> {
    sqrt(x)
}

#[tauri::command]
pub fn math_factorial(n: u64) -> Result<u64, String> {
    factorial(n)
}
