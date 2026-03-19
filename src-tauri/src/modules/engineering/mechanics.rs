pub fn force(mass: f64, acceleration: f64) -> f64 {
    mass * acceleration
}

pub fn pressure(force: f64, area: f64) -> Result<f64, String> {
    if area <= 0.0 {
        Err("面积必须大于零".to_string())
    } else {
        Ok(force / area)
    }
}

pub fn work(force: f64, distance: f64) -> f64 {
    force * distance
}

pub fn power(work: f64, time: f64) -> Result<f64, String> {
    if time <= 0.0 {
        Err("时间必须大于零".to_string())
    } else {
        Ok(work / time)
    }
}

pub fn ohms_law_voltage(current: f64, resistance: f64) -> f64 {
    current * resistance
}

pub fn ohms_law_current(voltage: f64, resistance: f64) -> Result<f64, String> {
    if resistance <= 0.0 {
        Err("电阻必须大于零".to_string())
    } else {
        Ok(voltage / resistance)
    }
}

pub fn electrical_power(voltage: f64, current: f64) -> f64 {
    voltage * current
}
