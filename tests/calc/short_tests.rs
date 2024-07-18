use std::time::Instant;
use xrs::calc::short::abs;

#[cfg(test)]
mod tests {
    use super::*;

    type TestTuple = (f64, f64);

    #[test]
    fn test_abs() {
        let test_data: [TestTuple; 2] = [
            (-5.0, 5.0),
            (5.0, 5.0),
        ];

        for (data, expected) in &test_data {
            let now = Instant::now();
            let result = abs(*data);
            let elapsed = now.elapsed();
            println!("abs({}) = {} [{:?}]", data, result, elapsed);
            assert_eq!(result, *expected);
        }
    }
}