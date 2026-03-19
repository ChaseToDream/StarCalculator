pub enum WeightUnit {
    Kilogram,
    Gram,
    Milligram,
    Ton,
    Pound,
    Ounce,
}

pub fn convert_weight(value: f64, from: WeightUnit, to: WeightUnit) -> f64 {
    let kilograms = match from {
        WeightUnit::Kilogram => value,
        WeightUnit::Gram => value / 1000.0,
        WeightUnit::Milligram => value / 1_000_000.0,
        WeightUnit::Ton => value * 1000.0,
        WeightUnit::Pound => value * 0.45359237,
        WeightUnit::Ounce => value * 0.0283495,
    };

    match to {
        WeightUnit::Kilogram => kilograms,
        WeightUnit::Gram => kilograms * 1000.0,
        WeightUnit::Milligram => kilograms * 1_000_000.0,
        WeightUnit::Ton => kilograms / 1000.0,
        WeightUnit::Pound => kilograms / 0.45359237,
        WeightUnit::Ounce => kilograms / 0.0283495,
    }
}
