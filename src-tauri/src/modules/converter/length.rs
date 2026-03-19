pub enum LengthUnit {
    Meter,
    Kilometer,
    Centimeter,
    Millimeter,
    Inch,
    Foot,
    Yard,
    Mile,
}

pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    let meters = match from {
        LengthUnit::Meter => value,
        LengthUnit::Kilometer => value * 1000.0,
        LengthUnit::Centimeter => value / 100.0,
        LengthUnit::Millimeter => value / 1000.0,
        LengthUnit::Inch => value * 0.0254,
        LengthUnit::Foot => value * 0.3048,
        LengthUnit::Yard => value * 0.9144,
        LengthUnit::Mile => value * 1609.344,
    };

    match to {
        LengthUnit::Meter => meters,
        LengthUnit::Kilometer => meters / 1000.0,
        LengthUnit::Centimeter => meters * 100.0,
        LengthUnit::Millimeter => meters * 1000.0,
        LengthUnit::Inch => meters / 0.0254,
        LengthUnit::Foot => meters / 0.3048,
        LengthUnit::Yard => meters / 0.9144,
        LengthUnit::Mile => meters / 1609.344,
    }
}
