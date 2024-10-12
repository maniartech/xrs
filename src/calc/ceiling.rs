// use crate::helpers::math::modf;

pub fn ceiling(number: f64, significance: f64) -> f64 {
    let round: f64;
    // do mod from the math module and get the decimal part
    let decimal_part: f64 = number % significance;

    if decimal_part >= 0.0 {
        round = (number / significance).ceil();
    } else {
        round = (number / significance).floor() + 1.0;
    }

    round * significance // result
}

pub fn ceiling_math(number: f64, significance: f64, mode: f64) -> f64 {
    let round: f64;

    let decimal_part = number % significance;

    // Depending on the mode value the number is rounded up or down
    if mode >= 0.0 {
        if decimal_part >= 0.0 {
            round = (number / significance).ceil();
        } else {
            round = (number / significance).floor() + 1.0;
        }
    } else {
        if decimal_part >= 0.0 {
            round = (number / significance).floor();
        } else {
            round = (number / significance).ceil() - 1.0;
        }
    }

    round * significance
}
