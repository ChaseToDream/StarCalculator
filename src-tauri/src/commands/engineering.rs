use crate::modules::engineering::*;

#[tauri::command]
pub fn engineering_rectangle_area(width: f64, height: f64) -> f64 {
    rectangle_area(width, height)
}

#[tauri::command]
pub fn engineering_square_area(side: f64) -> f64 {
    square_area(side)
}

#[tauri::command]
pub fn engineering_circle_area(radius: f64) -> f64 {
    circle_area(radius)
}

#[tauri::command]
pub fn engineering_triangle_area(base: f64, height: f64) -> f64 {
    triangle_area(base, height)
}

#[tauri::command]
pub fn engineering_trapezoid_area(base1: f64, base2: f64, height: f64) -> f64 {
    trapezoid_area(base1, base2, height)
}

#[tauri::command]
pub fn engineering_cube_volume(side: f64) -> f64 {
    cube_volume(side)
}

#[tauri::command]
pub fn engineering_rectangular_prism_volume(length: f64, width: f64, height: f64) -> f64 {
    rectangular_prism_volume(length, width, height)
}

#[tauri::command]
pub fn engineering_sphere_volume(radius: f64) -> f64 {
    sphere_volume(radius)
}

#[tauri::command]
pub fn engineering_cylinder_volume(radius: f64, height: f64) -> f64 {
    cylinder_volume(radius, height)
}

#[tauri::command]
pub fn engineering_cone_volume(radius: f64, height: f64) -> f64 {
    cone_volume(radius, height)
}

#[tauri::command]
pub fn engineering_force(mass: f64, acceleration: f64) -> f64 {
    force(mass, acceleration)
}

#[tauri::command]
pub fn engineering_pressure(force_val: f64, area: f64) -> Result<f64, String> {
    pressure(force_val, area)
}

#[tauri::command]
pub fn engineering_work(force_val: f64, distance: f64) -> f64 {
    work(force_val, distance)
}

#[tauri::command]
pub fn engineering_power(work_val: f64, time: f64) -> Result<f64, String> {
    power(work_val, time)
}

#[tauri::command]
pub fn engineering_ohms_law_voltage(current: f64, resistance: f64) -> f64 {
    ohms_law_voltage(current, resistance)
}

#[tauri::command]
pub fn engineering_ohms_law_current(voltage: f64, resistance: f64) -> Result<f64, String> {
    ohms_law_current(voltage, resistance)
}

#[tauri::command]
pub fn engineering_electrical_power(voltage: f64, current: f64) -> f64 {
    electrical_power(voltage, current)
}
