pub fn series_sum(x: f64, n: f64, m: f64, c: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..c.len() {
        let power = n + i as f64 * m;
        sum += c[i] * x.powf(power);
    }

    sum
}

pub fn sequence(rows: f64, columns: f64, start: f64, step: f64) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for i in 0..rows as usize {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..columns as usize {
            row.push(start + (i as f64 * columns + j as f64) * step);
        }
        result.push(row);
    }

    result
}
