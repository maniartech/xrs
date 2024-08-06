
#[cfg(test)]
mod tests {
  use xrs::calc::base;
  

    #[test]
    fn test_base_binary() {
        assert_eq!(base("10", 2, 0), "1010"); // 10 in decimal is 1010 in binary
    }

    #[test]
    fn test_base_binary_with_padding() {
        assert_eq!(base("10", 2, 8), "00001010"); // 10 in decimal is 1010 in binary with padding to 8 digits
    }

    #[test]
    fn test_base_hexadecimal() {
        assert_eq!(base("255", 16, 0), "FF"); // 255 in decimal is FF in hexadecimal
    }

    #[test]
    fn test_base_hexadecimal_with_padding() {
        assert_eq!(base("255", 16, 4), "00FF"); // 255 in decimal is FF in hexadecimal with padding to 4 digits
    }

    #[test]
    fn test_base_negative_number() {
        assert_eq!(base("-255", 16, 0), "-FF"); // -255 in decimal is -FF in hexadecimal
    }

    #[test]
    fn test_base_negative_number_with_padding() {
        assert_eq!(base("-255", 16, 4), "-00FF"); // -255 in decimal is -FF in hexadecimal with padding to 4 digits
    }

    #[test]
    #[should_panic(expected = "The base should be between 2 and 36")]
    fn test_invalid_base_low() {
        base("10", 1, 0);
    }

    #[test]
    #[should_panic(expected = "The base should be between 2 and 36")]
    fn test_invalid_base_high() {
        base("10", 37, 0);
    }
}
