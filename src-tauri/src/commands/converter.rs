use crate::modules::converter::*;

#[tauri::command]
pub fn converter_length(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = match from.to_lowercase().as_str() {
        "meter" | "米" => LengthUnit::Meter,
        "kilometer" | "千米" | "公里" => LengthUnit::Kilometer,
        "centimeter" | "厘米" => LengthUnit::Centimeter,
        "millimeter" | "毫米" => LengthUnit::Millimeter,
        "inch" | "英寸" => LengthUnit::Inch,
        "foot" | "英尺" => LengthUnit::Foot,
        "yard" | "码" => LengthUnit::Yard,
        "mile" | "英里" => LengthUnit::Mile,
        _ => return Err("不支持的长度单位".to_string()),
    };

    let to_unit = match to.to_lowercase().as_str() {
        "meter" | "米" => LengthUnit::Meter,
        "kilometer" | "千米" | "公里" => LengthUnit::Kilometer,
        "centimeter" | "厘米" => LengthUnit::Centimeter,
        "millimeter" | "毫米" => LengthUnit::Millimeter,
        "inch" | "英寸" => LengthUnit::Inch,
        "foot" | "英尺" => LengthUnit::Foot,
        "yard" | "码" => LengthUnit::Yard,
        "mile" | "英里" => LengthUnit::Mile,
        _ => return Err("不支持的长度单位".to_string()),
    };

    Ok(convert_length(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_weight(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = match from.to_lowercase().as_str() {
        "kilogram" | "千克" | "公斤" => WeightUnit::Kilogram,
        "gram" | "克" => WeightUnit::Gram,
        "milligram" | "毫克" => WeightUnit::Milligram,
        "ton" | "吨" => WeightUnit::Ton,
        "pound" | "磅" => WeightUnit::Pound,
        "ounce" | "盎司" => WeightUnit::Ounce,
        _ => return Err("不支持的重量单位".to_string()),
    };

    let to_unit = match to.to_lowercase().as_str() {
        "kilogram" | "千克" | "公斤" => WeightUnit::Kilogram,
        "gram" | "克" => WeightUnit::Gram,
        "milligram" | "毫克" => WeightUnit::Milligram,
        "ton" | "吨" => WeightUnit::Ton,
        "pound" | "磅" => WeightUnit::Pound,
        "ounce" | "盎司" => WeightUnit::Ounce,
        _ => return Err("不支持的重量单位".to_string()),
    };

    Ok(convert_weight(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_temperature(value: f64, from: String, to: String) -> Result<f64, String> {
    let from_unit = match from.to_lowercase().as_str() {
        "celsius" | "摄氏度" | "c" => TemperatureUnit::Celsius,
        "fahrenheit" | "华氏度" | "f" => TemperatureUnit::Fahrenheit,
        "kelvin" | "开尔文" | "k" => TemperatureUnit::Kelvin,
        _ => return Err("不支持的温度单位".to_string()),
    };

    let to_unit = match to.to_lowercase().as_str() {
        "celsius" | "摄氏度" | "c" => TemperatureUnit::Celsius,
        "fahrenheit" | "华氏度" | "f" => TemperatureUnit::Fahrenheit,
        "kelvin" | "开尔文" | "k" => TemperatureUnit::Kelvin,
        _ => return Err("不支持的温度单位".to_string()),
    };

    Ok(convert_temperature(value, from_unit, to_unit))
}

#[tauri::command]
pub fn converter_number_base(value: String, from: String, to: String) -> Result<String, String> {
    let from_base = match from.to_lowercase().as_str() {
        "binary" | "bin" | "二进制" | "2" => NumberBase::Binary,
        "octal" | "oct" | "八进制" | "8" => NumberBase::Octal,
        "decimal" | "dec" | "十进制" | "10" => NumberBase::Decimal,
        "hexadecimal" | "hex" | "十六进制" | "16" => NumberBase::Hexadecimal,
        _ => return Err("不支持的进制".to_string()),
    };

    let to_base = match to.to_lowercase().as_str() {
        "binary" | "bin" | "二进制" | "2" => NumberBase::Binary,
        "octal" | "oct" | "八进制" | "8" => NumberBase::Octal,
        "decimal" | "dec" | "十进制" | "10" => NumberBase::Decimal,
        "hexadecimal" | "hex" | "十六进制" | "16" => NumberBase::Hexadecimal,
        _ => return Err("不支持的进制".to_string()),
    };

    convert_number_base(&value, from_base, to_base)
}
