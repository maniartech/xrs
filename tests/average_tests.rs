use std::time::Instant;
use xrs::helpers::TestTuple;
use xrs::statistics::{avedev, average, average_if};

#[cfg(test)]
mod tests {
    use xrs::helpers::round_to_decimal;

    use super::*;

    #[test]
    fn test_average() {
        let test_data: [TestTuple; 10] = [
            (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0),
            (vec![2.0, 4.0, 6.0, 8.0, 10.0], 6.0),
            (vec![1.0, 1.0, 1.0, 1.0, 1.0], 1.0),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3.5),
            (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], 4.0),
            // List with fractional numbers
            (vec![1.4, 2.3, 3.2, 4.1, 5.0], 3.2),
            // List with negative numbers
            (vec![-1.0, -2.0, -3.0, -4.0, -5.0], -3.0),
            // List with negative and positive numbers
            (vec![-1.0, 2.0, -3.0, 4.0, -5.0], -0.6),
            // List with negative and positive fractional numbers
            (vec![-1.4, 2.3, -3.2, 4.1, -5.0], -0.64),
            // Empty list
            (vec![], 0.0),
        ];

        for (data, expected) in &test_data {
            let avg = round_to_decimal(average(data), 2);
            assert_eq!(avg, *expected);
        }
    }

    #[test]
    fn test_avedev() {
        let test_data: [TestTuple; 2] = [
            (vec![11.0, 6.0, 6.0, 12.0, 12.0, 7.0, 7.0, 9.0], 2.25),
            // Empty list
            (vec![], 0.0),
        ];

        for (data, expected) in &test_data {
            let now = Instant::now();
            let result = round_to_decimal(avedev(data), 2);
            let elapsed = now.elapsed();
            println!("avedev({:?}) = {} [{:?}]", data, result, elapsed);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_average_if() {
        let test_data: [TestTuple; 2] = [
            (vec![-5.0, -14.0, 1.0, 2.0, 3.0, 4.0, 5.0], 3.0),
            // Empty list
            (vec![], 0.0),
        ];

        for (data, expected) in &test_data {
            let avg = round_to_decimal(average_if(data, |x| *x > 0.0), 2);
            assert_eq!(avg, *expected);
        }
    }
}
