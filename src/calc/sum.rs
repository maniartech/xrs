//Sum function sums all the elements in the array
//
// Arguments
//
// * `numbers` - The array of numbers to sum.
//
// Returns
//
// * A number representing the sum of the input numbers.
//
// Examples
//
// ```
// use xrs::calc::sum;
//
// let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum(numbers);
// assert_eq!(result, 15.0);
// ```
pub fn sum(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

// sump_product function returns the sum of the products of corresponding arrays.
//
// Arguments
//
// * `numbers1` - The first array of numbers.
//
// * `numbers2` - The second array of numbers.
//
// * `numbersn` - The nth array of numbers.
//
// Returns
//
// * A number representing the sum of the products of the corresponding arrays.
//
// Examples
//
// ```
// use xrs::calc::sum_product;
//
// let numbers1 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let numbers2 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum_product(&numbers1, &numbers2);
// assert_eq!(result, 55.0);
// ```
pub fn sum_product(numbers1: Vec<f64>, numbersn: Vec<Vec<f64>>) -> f64 {
    let mut sumprod: f64 = 0.0;
    for (i, &num1) in numbers1.iter().enumerate() {
        let mut prod: f64 = num1;
        for num2 in &numbersn {
            prod *= num2[i];
        }
        sumprod += prod;
    }

    sumprod
}

// SumSq function returns the sum of the squares of the elements in the array
//
// Arguments
//
// * `numbers` - The array of numbers to sum.
//
// Returns
//
// * A number representing the sum of the squares of the input numbers.
//
// Examples
//
// ```
// use xrs::calc::sum_sq;
//
// let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum_sq(numbers);
// assert_eq!(result, 55.0);
// ```
pub fn sum_sq(numbers: &[f64]) -> f64 {
    numbers.iter().map(|x| x.powi(2)).sum()
}

// sum_x2_m_y2 function returns the sum of the squares of the elements in the first array minus the sum of the squares of the elements in the second array
//
// Arguments
//
// * `numbers1` - The first array of numbers.
//
// * `numbers2` - The second array of numbers.
//
// Returns
//
// * A number representing the sum of the squares of the first array minus the sum of the squares of the second array.
//
// Note:
//
// * Length of numbers1 and numbers2 should be equal
//
// Examples
//
// ```
// use xrs::calc::sum_x2_m_y2;
//
// let numbers1 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let numbers2 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum_x2_m_y2(&numbers1, &numbers2);
// assert_eq!(result, 0.0);
// ```
pub fn sum_x2_m_y2(numbers1: &[f64], numbers2: &[f64]) -> f64 {
    if numbers1.len() != numbers2.len() {
        panic!("Length of numbers1 and numbers2 should be equal");
    }
    let mut result: f64 = 0.0;

    for (i, num1) in numbers1.iter().enumerate() {
        result += (num1 * num1) - (numbers2[i] * numbers2[i]);
    }

    result
}

// sum_x2_p_y2 function returns the sum of the squares of the elements in the first array plus the sum of the squares of the elements in the second array
//
// Arguments
//
// * `numbers1` - The first array of numbers.
//
// * `numbers2` - The second array of numbers.
//
// Returns
//
// * A number representing the sum of the squares of the first array plus the sum of the squares of the second array.
//
// Note:
//
// * Length of numbers1 and numbers2 should be equal
//
// Examples
//
// ```
// use xrs::calc::sum_x2_p_y2;
//
// let numbers1 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let numbers2 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum_x2_p_y2(&numbers1, &numbers2);
// assert_eq!(result, 110.0);
// ```
pub fn sum_x2_p_y2(numbers1: &[f64], numbers2: &[f64]) -> f64 {
    if numbers1.len() != numbers2.len() {
        panic!("Length of numbers1 and numbers2 should be equal");
    }
    let mut result: f64 = 0.0;

    for (i, num1) in numbers1.iter().enumerate() {
        result += (num1 * num1) + (numbers2[i] * numbers2[i]);
    }

    result
}

// sum_x_m_y2 function returns the sum of squares of differences of corresponding values in two arrays.
//
// Arguments
//
// * `numbers1` - The first array of numbers.
//
// * `numbers2` - The second array of numbers.
//
// Returns
//
// * A number representing the sum of the squares of the first array plus the sum of the squares of the second array.
//
// Note:
//
// * Length of numbers1 and numbers2 should be equal
//
// Examples
//
// ```
// use xrs::calc::sum_x_m_y2;
//
// let numbers1 = [1.0, 2.0, 3.0, 4.0, 5.0];
// let numbers2 = [2.0, 3.0, 4.0, 5.0, 6.0];
// let result = sum_x_m_y2(&numbers1, &numbers2);
// assert_eq!(result, 5.0);
// ```
pub fn sum_x_m_y2(numbers1: &[f64], numbers2: &[f64]) -> f64 {
    if numbers1.len() != numbers2.len() {
        panic!("Length of numbers1 and numbers2 should be equal");
    }
    let mut result: f64 = 0.0;

    for (i, num1) in numbers1.iter().enumerate() {
        result += (num1 - numbers2[i]) * (num1 - numbers2[i]);
    }

    result
}

// sum_if function returns the sum of the elements in the array that satisfy the condition.
//
// Arguments
//
// * `numbers` - The array of numbers to sum.
//
// * `condition` - The condition to satisfy. The function should return true if the element satisfies the condition.
//
// Returns
//
// * A number representing the sum of the input numbers that satisfy the condition.
//
// Examples
//
// ```
// use xrs::calc::sum_if;
//
// let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
// let result = sum_if(&numbers, |x| *x > 3.0);
// assert_eq!(result, 9.0);
// ```
pub fn sum_if(numbers: &[f64], condition: fn(&f64) -> bool) -> f64 {
    numbers.iter().filter(|&x| condition(x)).sum()
}
