use std::f64::consts::PI;

pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

pub fn square_area(side: f64) -> f64 {
    side * side
}

pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

pub fn triangle_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}

pub fn trapezoid_area(base1: f64, base2: f64, height: f64) -> f64 {
    0.5 * (base1 + base2) * height
}
