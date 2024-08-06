use core::panic;

// Roman function converts arabic numbers to roman numerals
//
// Arguments
//
// * `number` - The number to convert.
//
// Returns
//
// * A string representing the roman numeral.
//
// Note
//
// * The number should be between 1 and 3999
//
// Examples
//
// ```
// use xrs::calc::roman;
//
// let number = 10;
// let result = roman(number);
// assert_eq!(result, "X");
// ```
pub fn roman(number: i32) -> String {
    if number < 1 || number > 3999 {
        panic!("The number should be between 1 and 3999");
    }

    let mut num: i32 = number;
    let mut result: String = String::new();
    let roman_numerals: Vec<(&str, i32)> = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for (numeral, value) in roman_numerals.iter() {
        while num >= *value {
            result.push_str(*numeral);
            num -= value;
        }
    }

    result
}
