use crate::modules::converter::*;

fn parse_length_unit(unit: &str) -> Result<LengthUnit, String> {
    match unit.to_lowercase().as_str() {
        "meter" | "米" => Ok(LengthUnit::Meter),
        "kilometer" | "千米" | "公里" => Ok(LengthUnit::Kilometer),
        "centimeter" | "厘米" => Ok(LengthUnit::Centimeter),
        "millimeter" | "毫米" => Ok(LengthUnit::Millimeter),
        "inch" | "英寸" => Ok(LengthUnit::Inch),
        "foot" | "英尺" => Ok(LengthUnit::Foot),
        "yard" | "码" => Ok(LengthUnit::Yard),
        "mile" | "英里" => Ok(LengthUnit::Mile),
        _ => Err("不支持的长度单位".to_string()),
    }
}

fn parse_weight_unit(unit: &str) -> Result<WeightUnit, String> {
    match unit.to_lowercase().as_str() {
        "kilogram" | "千克" | "公斤" => Ok(WeightUnit::Kilogram),
        "gram" | "克" => Ok(WeightUnit::Gram),
        "milligram" | "毫克" => Ok(WeightUnit::Milligram),
        "ton" | "吨" => Ok(WeightUnit::Ton),
        "pound" | "磅" => Ok(WeightUnit::Pound),
        "ounce" | "盎司" => Ok(WeightUnit::Ounce),
        _ => Err("不支持的重量单位".to_string()),
    }
}

fn parse_temperature_unit(unit: &str) -> Result<TemperatureUnit, String> {
    match unit.to_lowercase().as_str() {
        "celsius" | "摄氏度" | "c" => Ok(TemperatureUnit::Celsius),
        "fahrenheit" | "华氏度" | "f" => Ok(TemperatureUnit::Fahrenheit),
        "kelvin" | "开尔文" | "k" => Ok(TemperatureUnit::Kelvin),
        _ => Err("不支持的温度单位".to_string()),
    }
}

fn parse_number_base(unit: &str) -> Result<NumberBase, String> {
    match unit.to_lowercase().as_str() {
        "binary" | "bin" | "二进制" | "2" => Ok(NumberBase::Binary),
        "octal" | "oct" | "八进制" | "8" => Ok(NumberBase::Octal),
        "decimal" | "dec" | "十进制" | "10" => Ok(NumberBase::Decimal),
        "hexadecimal" | "hex" | "十六进制" | "16" => Ok(NumberBase::Hexadecimal),
        _ => Err("不支持的进制".to_string()),
    }
}

#[tauri::command]
pub fn converter_length(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = parse_length_unit(&from)?;
    let to_unit = parse_length_unit(&to)?;
    Ok(convert_length(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_weight(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = parse_weight_unit(&from)?;
    let to_unit = parse_weight_unit(&to)?;
    Ok(convert_weight(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_temperature(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = parse_temperature_unit(&from)?;
    let to_unit = parse_temperature_unit(&to)?;
    Ok(convert_temperature(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_number_base(value: String, from: String, to: String) -> Result<String, String> {
    let from_base = parse_number_base(&from)?;
    let to_base = parse_number_base(&to)?;
    convert_number_base(&value, from_base, to_base)
}
