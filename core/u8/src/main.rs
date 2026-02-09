#![feature(ascii_char)]
#![feature(is_ascii_octdigit)]

fn main() {
    println!("Running u8 tests...");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_ascii() {
        let ascii = 97u8; // 'a'
        let non_ascii = 150u8;
        let chinese_byte = 0xE4u8; // First byte of '中'

        assert!(ascii.is_ascii());
        assert!(!non_ascii.is_ascii());
        assert!(!chinese_byte.is_ascii());

        // Boundary check
        assert!(127u8.is_ascii());
        assert!(!128u8.is_ascii());
        assert!(0u8.is_ascii());
        assert!(!255u8.is_ascii());
    }

    #[test]
    fn test_as_ascii() {
        let byte = 97u8;
        let non_ascii = 150u8;

        assert!(byte.as_ascii().is_some());
        assert!(non_ascii.as_ascii().is_none());
    }

    #[test]
    fn test_as_ascii_unchecked() {
        let byte = 97u8;
        unsafe {
            let char = byte.as_ascii_unchecked();
            // Cast to u8 to verify
            assert_eq!(char as u8, 97);
        }
    }

    #[test]
    fn test_to_ascii_uppercase() {
        let lowercase = b'a';
        let uppercase = b'A';
        let non_ascii = 0xE4u8;

        assert_eq!(lowercase.to_ascii_uppercase(), b'A');
        assert_eq!(uppercase.to_ascii_uppercase(), b'A');
        assert_eq!(non_ascii.to_ascii_uppercase(), 0xE4);
    }

    #[test]
    fn test_to_ascii_lowercase() {
        let uppercase = b'A';
        let lowercase = b'a';
        let non_ascii = 0xE4u8;

        assert_eq!(uppercase.to_ascii_lowercase(), b'a');
        assert_eq!(lowercase.to_ascii_lowercase(), b'a');
        assert_eq!(non_ascii.to_ascii_lowercase(), 0xE4);
    }

    #[test]
    fn test_eq_ignore_ascii_case() {
        assert!(b'a'.eq_ignore_ascii_case(&b'A'));
        assert!(b'A'.eq_ignore_ascii_case(&b'a'));
        assert!(!b'a'.eq_ignore_ascii_case(&b'b'));
        // Non-ascii should be equal only to itself
        assert!(0xE4u8.eq_ignore_ascii_case(&0xE4u8));
        assert!(!0xE4u8.eq_ignore_ascii_case(&0xE5u8));
    }

    #[test]
    fn test_make_ascii_uppercase() {
        let mut byte = b'a';
        byte.make_ascii_uppercase();
        assert_eq!(byte, b'A');

        let mut non_ascii = 0xE4u8;
        non_ascii.make_ascii_uppercase();
        assert_eq!(non_ascii, 0xE4);
    }

    #[test]
    fn test_make_ascii_lowercase() {
        let mut byte = b'A';
        byte.make_ascii_lowercase();
        assert_eq!(byte, b'a');

        let mut non_ascii = 0xE4u8;
        non_ascii.make_ascii_lowercase();
        assert_eq!(non_ascii, 0xE4);
    }

    #[test]
    fn test_is_ascii_alphabetic() {
        assert!(b'a'.is_ascii_alphabetic());
        assert!(b'A'.is_ascii_alphabetic());
        assert!(!b'0'.is_ascii_alphabetic());
        assert!(!0xE4u8.is_ascii_alphabetic());
    }

    #[test]
    fn test_is_ascii_uppercase() {
        assert!(b'A'.is_ascii_uppercase());
        assert!(!b'a'.is_ascii_uppercase());
        assert!(!0xE4u8.is_ascii_uppercase());
    }

    #[test]
    fn test_is_ascii_lowercase() {
        assert!(b'a'.is_ascii_lowercase());
        assert!(!b'A'.is_ascii_lowercase());
        assert!(!0xE4u8.is_ascii_lowercase());
    }

    #[test]
    fn test_is_ascii_alphanumeric() {
        assert!(b'a'.is_ascii_alphanumeric());
        assert!(b'0'.is_ascii_alphanumeric());
        assert!(!b'.'.is_ascii_alphanumeric());
        assert!(!0xE4u8.is_ascii_alphanumeric());
    }

    #[test]
    fn test_is_ascii_digit() {
        assert!(b'0'.is_ascii_digit());
        assert!(b'9'.is_ascii_digit());
        assert!(!b'a'.is_ascii_digit());
    }

    #[test]
    fn test_is_ascii_octdigit() {
        assert!(b'0'.is_ascii_octdigit());
        assert!(b'7'.is_ascii_octdigit());
        assert!(!b'8'.is_ascii_octdigit());
        assert!(!b'9'.is_ascii_octdigit());
        assert!(!b'a'.is_ascii_octdigit());
    }

    #[test]
    fn test_is_ascii_hexdigit() {
        assert!(b'0'.is_ascii_hexdigit());
        assert!(b'9'.is_ascii_hexdigit());
        assert!(b'a'.is_ascii_hexdigit());
        assert!(b'f'.is_ascii_hexdigit());
        assert!(b'A'.is_ascii_hexdigit());
        assert!(b'F'.is_ascii_hexdigit());
        assert!(!b'g'.is_ascii_hexdigit());
    }

    #[test]
    fn test_is_ascii_punctuation() {
        assert!(b'!'.is_ascii_punctuation());
        assert!(b'.'.is_ascii_punctuation());
        assert!(!b'a'.is_ascii_punctuation());
        assert!(!b'0'.is_ascii_punctuation());
    }

    #[test]
    fn test_is_ascii_graphic() {
        assert!(b'a'.is_ascii_graphic());
        assert!(b'!'.is_ascii_graphic());
        assert!(!b' '.is_ascii_graphic()); // space is not graphic
        assert!(!b'\n'.is_ascii_graphic());
    }

    #[test]
    fn test_is_ascii_whitespace() {
        assert!(b' '.is_ascii_whitespace());
        assert!(b'\t'.is_ascii_whitespace());
        assert!(b'\n'.is_ascii_whitespace());
        assert!(!b'a'.is_ascii_whitespace());
    }

    #[test]
    fn test_is_ascii_control() {
        assert!(b'\0'.is_ascii_control());
        assert!(b'\n'.is_ascii_control()); // \n is control (0x0A)
        assert!(b'\x1F'.is_ascii_control());
        assert!(b'\x7F'.is_ascii_control()); // DEL
        assert!(!b' '.is_ascii_control()); // Space is not control
        assert!(!b'a'.is_ascii_control());
    }

    #[test]
    fn test_escape_ascii() {
        let escaped: String = b'\n'.escape_ascii().map(|b| b as char).collect();
        assert_eq!(escaped, "\\n");

        let escaped: String = b'a'.escape_ascii().map(|b| b as char).collect();
        assert_eq!(escaped, "a");

        let escaped: String = 0xE4u8.escape_ascii().map(|b| b as char).collect();
        assert_eq!(escaped, "\\xe4"); // Default escape for non-ascii
    }
}
