// This file contains functions for calculating averages similar the functions
// found in the excel AVERAGE family of functions.

/// Calculates the average of a slice of numbers.
///
/// # Arguments
///
/// * `list` - A slice of numbers for which to calculate the average.
///
/// # Returns
///
/// * A number representing the average of the input slice. If the slice is empty, the function will return NaN.
///
/// # Examples
///
/// ```
/// use xrs::statistics::average;
///
/// let list = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let avg = average(&list);
/// assert_eq!(avg, 3.0);
/// ```
pub fn average(list: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    for &num in list {
        sum += num;
        count += 1.0;
    }

    if count == 0.0 {
        return 0.0;
    }

    sum / count
}

// avedev returns the average of the absolute deviations of data points from their mean.
//
// Arguments
//
// * `list` - A slice of numbers for which to calculate the average deviation.
//
// Returns
//
// * A number representing the average deviation of the input slice. If the slice is empty, the function will return NaN.
//
// Examples
//
// ```
// use xrs::statistics::avedev;
//
// let list = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = avedev(&list);
// assert_eq!(result, 1.0);
// ```
pub fn avedev(list: &[f64]) -> f64 {
    let avg = average(list);
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    for &num in list {
        sum += (num - avg).abs();
        count += 1.0;
    }

    if count == 0.0 {
        return 0.0;
    }

    sum / count
}

// average_if returns the average of a slice of numbers that meet a certain condition.
//
// Arguments
//
// * `list` - A slice of numbers for which to calculate the average.
// * `condition` - A closure that takes a reference to a number and returns a boolean.
//
// Returns
//
// * A number representing the average of the input slice that meets the condition. If the slice is empty, the function will return NaN.
//
// Examples
//
// ```
// use xrs::statistics::average_if;
//
// let list = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = average_if(&list, |&x| x > 3.0);
// assert_eq!(result, 4.5);
// ```
pub fn average_if<F>(list: &[f64], condition: F) -> f64
where
    F: Fn(&f64) -> bool,
{
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    for &num in list {
        if condition(&num) {
            sum += num;
            count += 1.0;
        }
    }

    if count == 0.0 {
        return 0.0;
    }

    sum / count
}
