#[cfg(test)]
mod tests {
    use xrs::calc::roman;

    #[test]
    fn test_roman_simple() {
        assert_eq!(roman(1), "I");
        assert_eq!(roman(5), "V");
        assert_eq!(roman(10), "X");
        assert_eq!(roman(50), "L");
        assert_eq!(roman(100), "C");
        assert_eq!(roman(500), "D");
        assert_eq!(roman(1000), "M");
    }

    #[test]
    fn test_roman_combinations() {
        assert_eq!(roman(2), "II");
        assert_eq!(roman(3), "III");
        assert_eq!(roman(4), "IV");
        assert_eq!(roman(6), "VI");
        assert_eq!(roman(7), "VII");
        assert_eq!(roman(8), "VIII");
        assert_eq!(roman(9), "IX");
    }

    #[test]
    fn test_roman_complex() {
        assert_eq!(roman(12), "XII");
        assert_eq!(roman(14), "XIV");
        assert_eq!(roman(20), "XX");
        assert_eq!(roman(29), "XXIX");
        assert_eq!(roman(40), "XL");
        assert_eq!(roman(54), "LIV");
        assert_eq!(roman(90), "XC");
        assert_eq!(roman(99), "XCIX");
        assert_eq!(roman(140), "CXL");
        assert_eq!(roman(400), "CD");
        assert_eq!(roman(800), "DCCC");
        assert_eq!(roman(900), "CM");
        assert_eq!(roman(1994), "MCMXCIV");
    }

    #[test]
    #[should_panic(expected = "The number should be between 1 and 3999")]
    fn test_roman_invalid_low() {
        roman(0);
    }

    #[test]
    #[should_panic(expected = "The number should be between 1 and 3999")]
    fn test_roman_invalid_high() {
        roman(4000);
    }
}
