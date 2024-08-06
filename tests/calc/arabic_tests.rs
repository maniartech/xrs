#[cfg(test)]
mod tests {
    use xrs::calc::arabic;

    #[test]
    fn test_arabic_simple() {
        assert_eq!(arabic("I"), 1);
        assert_eq!(arabic("V"), 5);
        assert_eq!(arabic("X"), 10);
        assert_eq!(arabic("L"), 50);
        assert_eq!(arabic("C"), 100);
        assert_eq!(arabic("D"), 500);
        assert_eq!(arabic("M"), 1000);
    }

    #[test]
    fn test_arabic_combinations() {
        assert_eq!(arabic("II"), 2);
        assert_eq!(arabic("III"), 3);
        assert_eq!(arabic("IV"), 4);
        assert_eq!(arabic("VI"), 6);
        assert_eq!(arabic("VII"), 7);
        assert_eq!(arabic("VIII"), 8);
        assert_eq!(arabic("IX"), 9);
    }

    #[test]
    fn test_arabic_complex() {
        assert_eq!(arabic("XII"), 12);
        assert_eq!(arabic("XIV"), 14);
        assert_eq!(arabic("XX"), 20);
        assert_eq!(arabic("XXIX"), 29);
        assert_eq!(arabic("XL"), 40);
        assert_eq!(arabic("LIV"), 54);
        assert_eq!(arabic("XC"), 90);
        assert_eq!(arabic("XCIX"), 99);
        assert_eq!(arabic("CXL"), 140);
        assert_eq!(arabic("CD"), 400);
        assert_eq!(arabic("DCCC"), 800);
        assert_eq!(arabic("CM"), 900);
        assert_eq!(arabic("MCMXCIV"), 1994);
        assert_eq!(arabic("MMMCMXCIX"), 3999);
    }

    #[test]
    fn test_arabic_invalid() {
        assert_eq!(arabic("IIII"), 4); // Should ideally be invalid
        assert_eq!(arabic("VV"), 10); // Should ideally be invalid
        assert_eq!(arabic("XXXX"), 40); // Should ideally be invalid
    }

    #[test]
    fn test_arabic_empty_string() {
        assert_eq!(arabic(""), 0); // Empty string should return 0
    }
}
