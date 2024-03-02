use xrs::helpers::TestTuple;
use xrs::statistics::{count, count_if};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let test_data: [TestTuple; 2] = [
            (vec![1.0, 2.0, 3.0, 4.0, 5.0], 5.0),
            // Empty list
            (vec![], 0.0),
        ];

        for (data, expected) in &test_data {
            let result = count(data);
            assert_eq!(result, *expected as usize);
        }
    }

    #[test]
    fn test_count_if() {
        let test_data: [TestTuple; 2] = [
            (vec![-5.0, -14.0, 1.0, 2.0, 3.0, 4.0, 5.0], 5.0),
            // Empty list
            (vec![], 0.0),
        ];

        for (data, expected) in &test_data {
            let result = count_if(data, |x| *x > 0.0);
            assert_eq!(result, *expected as usize);
        }
    }
}
