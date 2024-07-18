use std::time::Instant;
use xrs::helpers::TestTuple;
use xrs::calc::sum;
use xrs::calc::sum_product;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_sum() {
        let test_data: [TestTuple; 2] = [
            (vec![1.0,2.0,3.0], 6.0),
            (vec![1.0,2.0,3.0,4.0,5.0], 15.0),
        ];

        for (data, expected) in &test_data {
            let now = Instant::now();
            let result = sum(&data);
            let elapsed = now.elapsed();
            println!("sum({:?}) = {} [{:?}]", data, result, elapsed);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_sum_product() {
      type TestTuple = ((Vec<f64>, Vec<Vec<f64>>), f64);
      let test_data: [TestTuple; 2] = [
          ((vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]]), 55.0),
          ((vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![1.0, 2.0, 3.0, 4.0, 5.0]]), 225.0),
        ];

        for (data1, data2) in &test_data {
            let now = Instant::now();
            let result = sum_product(data1.0.clone(), data1.1.clone());
            let elapsed = now.elapsed();
            println!("sum_product({:?}) = {} [{:?}]", data1, result, elapsed);
            assert_eq!(result, *data2);
        }
    }
}