pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub fn convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    let celsius = match from {
        TemperatureUnit::Celsius => value,
        TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TemperatureUnit::Kelvin => value - 273.15,
    };

    match to {
        TemperatureUnit::Celsius => celsius,
        TemperatureUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Kelvin => celsius + 273.15,
    }
}
