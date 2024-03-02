pub fn round_to_decimal(value: f64, decimal: u32) -> f64 {
    let factor = 10_f64.powi(decimal as i32);
    (value * factor).round() / factor
}
