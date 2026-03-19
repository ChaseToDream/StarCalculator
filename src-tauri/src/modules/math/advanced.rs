pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn ln(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        Err("对数的自变量必须大于零".to_string())
    } else {
        Ok(x.ln())
    }
}

pub fn log10(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        Err("对数的自变量必须大于零".to_string())
    } else {
        Ok(x.log10())
    }
}

pub fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

pub fn sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("平方根的被开方数不能为负数".to_string())
    } else {
        Ok(x.sqrt())
    }
}

pub fn factorial(n: u64) -> Result<u64, String> {
    if n > 20 {
        Err("阶乘数值过大，超出计算范围".to_string())
    } else {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        Ok(result)
    }
}
