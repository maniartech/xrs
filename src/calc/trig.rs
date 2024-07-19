// Trigonometric functions
pub fn sin(number: f64) -> f64 {
    number.sin()
}

pub fn csc(number: f64) -> f64 {
    1.0 / number.sin()
}

pub fn asin(number: f64) -> f64 {
    number.asin()
}

pub fn asinh(number: f64) -> f64 {
    number.asinh()
}

pub fn sinh(number: f64) -> f64 {
    number.sinh()
}
pub fn cos(number: f64) -> f64 {
    number.cos()
}

pub fn acos(number: f64) -> f64 {
    number.acos()
}

pub fn acosh(number: f64) -> f64 {
    number.acosh()
}

pub fn cosh(number: f64) -> f64 {
    number.cosh()
}

pub fn tan(number: f64) -> f64 {
    number.tan()
}

pub fn atan(number: f64) -> f64 {
    number.atan()
}

pub fn atan2(number1: f64, number2: f64) -> f64 {
    number1.atan2(number2)
}

pub fn atanh(number: f64) -> f64 {
    number.atanh()
}

pub fn tanh(number: f64) -> f64 {
    number.tanh()
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees.to_radians()
}

pub fn rad_to_deg(radians: f64) -> f64 {
    radians.to_degrees()
}
