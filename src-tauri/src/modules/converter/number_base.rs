pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn convert_number_base(value: &str, from: NumberBase, to: NumberBase) -> Result<String, String> {
    let trimmed_value = value.trim();
    
    if trimmed_value.is_empty() {
        return Err("输入值不能为空".to_string());
    }

    let radix_from = match from {
        NumberBase::Binary => 2,
        NumberBase::Octal => 8,
        NumberBase::Decimal => 10,
        NumberBase::Hexadecimal => 16,
    };

    let num = u64::from_str_radix(trimmed_value, radix_from)
        .map_err(|_| {
            match from {
                NumberBase::Binary => "无效的二进制格式，请仅使用 0 和 1".to_string(),
                NumberBase::Octal => "无效的八进制格式，请仅使用 0-7".to_string(),
                NumberBase::Decimal => "无效的十进制格式，请仅使用 0-9".to_string(),
                NumberBase::Hexadecimal => "无效的十六进制格式，请仅使用 0-9 和 A-F".to_string(),
            }
        })?;

    let result = match to {
        NumberBase::Binary => format!("{:b}", num),
        NumberBase::Octal => format!("{:o}", num),
        NumberBase::Decimal => format!("{}", num),
        NumberBase::Hexadecimal => format!("{:X}", num),
    };

    Ok(result)
}
