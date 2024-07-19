// Arabic converts roman numerals to arabic numbers
pub fn arabic(roman: &str) -> i32 {
    let mut result: i32 = 0;
    for i in 0..roman.len() {
        let current = roman.chars().nth(i).unwrap();
        let next = roman.chars().nth(i + 1).unwrap_or(' ');
        let current_value = match current {
            'I' => {
                if next == 'V' || next == 'X' {
                    -1
                } else {
                    1
                }
            }
            'V' => 5,
            'X' => {
                if next == 'L' || next == 'C' {
                    -10
                } else {
                    10
                }
            }
            'L' => 50,
            'C' => {
                if next == 'D' || next == 'M' {
                    -100
                } else {
                    100
                }
            }
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        result += current_value;
    }
    result
}
