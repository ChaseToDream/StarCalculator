pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn convert_number_base(value: &str, from: NumberBase, to: NumberBase) -> Result<String, String> {
    let radix_from = match from {
        NumberBase::Binary => 2,
        NumberBase::Octal => 8,
        NumberBase::Decimal => 10,
        NumberBase::Hexadecimal => 16,
    };

    let num = u64::from_str_radix(value, radix_from)
        .map_err(|_| "无效的数字格式".to_string())?;

    let result = match to {
        NumberBase::Binary => format!("{:b}", num),
        NumberBase::Octal => format!("{:o}", num),
        NumberBase::Decimal => format!("{}", num),
        NumberBase::Hexadecimal => format!("{:X}", num),
    };

    Ok(result)
}
