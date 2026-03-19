use std::f64::consts::PI;

pub fn cube_volume(side: f64) -> f64 {
    side * side * side
}

pub fn rectangular_prism_volume(length: f64, width: f64, height: f64) -> f64 {
    length * width * height
}

pub fn sphere_volume(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius * radius * radius
}

pub fn cylinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

pub fn cone_volume(radius: f64, height: f64) -> f64 {
    (1.0 / 3.0) * PI * radius * radius * height
}
