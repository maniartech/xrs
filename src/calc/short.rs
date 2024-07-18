// Returns the absolute value of a number. The absolute value of a number is the number without its sign.
//
// Arguments
//
// * `number` - The number for which to calculate the absolute value.
//
// Returns
//
// * A number representing the absolute value of the input number.
//
// Examples
//
// ```
// use xrs::calc::short;
//
// let number = -5.0;
// let result = short::abs(number);
// assert_eq!(result, 5.0);
// ```
pub fn abs(number: f64) -> f64 {
    number.abs()
}
