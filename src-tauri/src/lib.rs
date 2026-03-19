mod modules;
mod commands;

use commands::math::*;
use commands::converter::*;
use commands::engineering::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            math_add,
            math_subtract,
            math_multiply,
            math_divide,
            math_sin,
            math_cos,
            math_tan,
            math_ln,
            math_log10,
            math_pow,
            math_sqrt,
            math_factorial,
            converter_length,
            converter_weight,
            converter_temperature,
            converter_number_base,
            engineering_rectangle_area,
            engineering_square_area,
            engineering_circle_area,
            engineering_triangle_area,
            engineering_trapezoid_area,
            engineering_cube_volume,
            engineering_rectangular_prism_volume,
            engineering_sphere_volume,
            engineering_cylinder_volume,
            engineering_cone_volume,
            engineering_force,
            engineering_pressure,
            engineering_work,
            engineering_power,
            engineering_ohms_law_voltage,
            engineering_ohms_law_current,
            engineering_electrical_power
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
