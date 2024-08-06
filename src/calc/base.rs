// Base returns the string representation of the number in the specified base
//
// Arguments
//
// * `number` - The number to convert.
//
// * `base` - The base to convert the number to.
//
// * `minL` - minimum length of the string representation of the number
//
// Note
//
// * The base should be between 2 and 36
//
// * minL should be between 0 and 255
//
// Examples
//
// ```
// use xrs::calc::base;
//
// let number = 10;
// let base = 2;
// let minL = 0;
// let result = base(number, base, minL);
// assert_eq!(result, "1010");
// ```
pub fn base(number: &str, base: u32, min_l: u8) -> String {
    if base < 2 || base > 36 {
        panic!("The base should be between 2 and 36");
    }

    let mut num: i32 = number.parse().unwrap();
    let mut result: String = String::new();
    let mut negative: bool = false;

    if num < 0 {
        negative = true;
        num = -num;
    }

    while num > 0 {
        let rem: i32 = num % base as i32;
        result.push(if rem < 10 {
            (rem + 48) as u8 as char
        } else {
            (rem + 55) as u8 as char
        });
        num /= base as i32;
    }

    if negative {
        result.push('-');
    }

    while result.len() < min_l as usize {
        result.push('0');
    }

    result.chars().rev().collect::<String>()
}
