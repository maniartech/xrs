pub fn gcd(v: Vec<f64>) -> f64 {
    let mut result: i64 = 0;

    for x in v {
        if x == 0.0 {
            return 0.0;
        }

        let mut number: i64 = x as i64;
        while number != 0 {
            result = number;
            number = number % result;
        }
    }

    result as f64
}

pub fn lcm(v: Vec<f64>) -> f64 {
    let mut a: i64 = v[0] as i64;
    let mut b: i64 = v[1] as i64;

    while a != 0 {
        let temp: i64 = a;
        a = b % a;
        b = temp;
    }
    if b == 0 {
        return 0.0;
    }

    let mut result: i64 = ((v[0] * v[1]) as i64) / b;

    for num in v[2..].iter() {
        let x: i64 = *num as i64;
        // Find the gcd of the current result and the next number
        a = x;
        b = result;

        while a != 0 {
            let temp: i64 = a;
            a = b % a;
            b = temp;
        }
        if b == 0 {
            return 0.0;
        }

        result = x * result / b;
    }

    result as f64
}
