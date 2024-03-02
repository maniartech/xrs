// This file replicats the functionality of the excel COUNT family of functions.

// Counts the number of cells in a range that contain numbers.
//
// Arguments
//
// * `list` - A slice of numbers for which to count the number of cells.
//
// Returns
//
// * A number representing the number of cells in the input slice that contain numbers.
//
// Examples
//
// ```
// use xrs::statistics::count;
//
// let list = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = count(&list);
// assert_eq!(result, 5);
// ```
pub fn count(list: &[f64]) -> usize {
    list.len()
}

// Counts the number of items if they meet a certain condition.
//
// Arguments
//
// * `list` - A slice of numbers for which to count the number of cells.
// * `condition` - A closure that takes a reference to a number and returns a boolean.
//
// Returns
//
// * A number representing the number of cells in the input slice that meet the condition.
//
// Examples
//
// ```
// use xrs::statistics::count_if;
//
// let list = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = count_if(&list, |&x| x > 3.0);
// assert_eq!(result, 2);
// ```
pub fn count_if<F>(list: &[f64], condition: F) -> usize
where
    F: Fn(&f64) -> bool,
{
    list.iter().filter(|&x| condition(x)).count()
}
