// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    }
}

use snazzy::items;
/// Returns a large shirt of the specified color
pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt: items::Shirt = items::Shirt {
        color,
        ..Default::default()
    };
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::HashSet};

    use prost::Message;

    use super::items::{Shirt, shirt::Size};

    #[test]
    fn test_shirt_all() {
        // Test size getters and setters
        test_shirt_size_getter();
        test_shirt_size_setter();
        test_shirt_clear();
        test_shirt_encode_decode();
        test_shirt_encoded_len();
        // derive std methods
        test_shirt_default();
        test_shirt_clone();
        test_shirt_partial_eq();
        test_shirt_debug();
        // Test not_equal and hash
        test_shirt_not_equal();
        test_shirt_hash();
    }

    #[test]
    // Test Size enum methods
    fn test_size_all() {
        //  assert!(Size::is_valid(0));
        test_size_is_valid();
        //  assert_eq!(Size::try_from(0i32), Ok(Size::Small));
        test_size_try_from();
        // let size: i32 = Size::Medium.into();
        test_size_into();
        // assert_eq!(Size::Small.as_str_name(), "SMALL");
        test_size_as_str_name();
        //  assert_eq!(Size::from_str_name("SMALL"), Some(Size::Small));
        test_size_from_str_name();
        // std
        test_size_default();
        test_size_clone();
        test_size_copy();
        test_size_debug();
        test_size_partial_eq();
        test_size_hash();
        test_size_partial_ord();
        test_size_ord();
        // deprecated
        test_size_from_i32();
    }

    fn test_shirt_default() {
        let shirt = Shirt::default();
        assert_eq!(shirt.color, "");
        assert_eq!(shirt.size(), Size::Small);
    }

    fn test_shirt_clone() {
        let shirt = Shirt {
            color: "blue".to_string(),
            size: Size::Medium as i32,
            ..Default::default()
        };
        let cloned = shirt.clone();
        assert_eq!(cloned.color, "blue");
        assert_eq!(cloned.size(), Size::Medium);
    }

    fn test_shirt_partial_eq() {
        let shirt1 = Shirt {
            color: "red".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        let shirt2 = Shirt {
            color: "red".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        assert_eq!(shirt1, shirt2);
    }

    fn test_shirt_not_equal() {
        let shirt1 = Shirt {
            color: "red".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        let shirt2 = Shirt {
            color: "blue".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        assert_ne!(shirt1, shirt2);
    }

    fn test_shirt_hash() {
        let shirt1 = Shirt {
            color: "red".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        let shirt2 = Shirt {
            color: "red".to_string(),
            size: Size::Large as i32,
            ..Default::default()
        };

        let mut hash_set = HashSet::new();
        assert!(hash_set.insert(shirt1.clone()));
        assert!(!hash_set.insert(shirt2));
    }

    fn test_shirt_debug() {
        let shirt = Shirt {
            color: "blue".to_string(),
            size: Size::Medium as i32,
            ..Default::default()
        };
        let debug_str = format!("{:?}", shirt);
        assert!(debug_str.contains("Shirt"));
        assert!(debug_str.contains("color"));
        assert!(debug_str.contains("Medium"));
    }

    fn test_shirt_size_getter() {
        let shirt = Shirt::default();
        assert_eq!(shirt.size(), Size::Small);

        let mut shirt = shirt;
        shirt.set_size(Size::Large);
        assert_eq!(shirt.size(), Size::Large);

        shirt.size = Size::Medium as i32;
        assert_eq!(shirt.size(), Size::Medium);
    }

    fn test_shirt_size_setter() {
        let shirt = Shirt::default();
        let mut shirt = shirt;
        shirt.set_size(Size::Large);
        assert_eq!(shirt.size, Size::Large as i32);
    }

    fn test_shirt_clear() {
        let shirt = Shirt {
            color: "red".to_string(),

            ..Default::default()
        };
        let mut shirt = shirt;
        shirt.set_size(Size::Large);
        shirt.clear();
        assert_eq!(shirt.color, "");
        assert_eq!(shirt.size(), Size::Small);
    }

    fn test_shirt_encode_decode() {
        let shirt = Shirt {
            color: "red".to_string(),
            ..Default::default()
        };
        let mut shirt = shirt;
        shirt.set_size(Size::Large);

        let buf = shirt.encode_to_vec();
        assert!(!buf.is_empty());

        let decoded_shirt = Shirt::decode(buf.as_slice()).unwrap();
        assert_eq!(decoded_shirt.color, "red");
        assert_eq!(decoded_shirt.size(), Size::Large);
    }

    fn test_shirt_encoded_len() {
        let shirt = Shirt::default();
        assert_eq!(shirt.encoded_len(), 0);

        let shirt = Shirt {
            color: "red".to_string(),
            ..Default::default()
        };
        assert!(shirt.encoded_len() > 0);

        let mut shirt = shirt;
        shirt.set_size(Size::Large);
        assert!(shirt.encoded_len() > 0);
    }

    fn test_size_default() {
        assert_eq!(Size::default(), Size::Small);
    }

    fn test_size_clone() {
        let size = Size::Medium;
        let cloned = size;
        assert_eq!(cloned, Size::Medium);
    }

    fn test_size_copy() {
        let size = Size::Large;
        let copied = size;
        assert_eq!(copied, Size::Large);
    }

    fn test_size_debug() {
        assert_eq!(format!("{:?}", Size::Small), "Small");
        assert_eq!(format!("{:?}", Size::Medium), "Medium");
        assert_eq!(format!("{:?}", Size::Large), "Large");
    }

    fn test_size_partial_eq() {
        assert_ne!(Size::Small, Size::Medium);
        let s = Size::Medium;
        assert_eq!(s, Size::Medium);
    }

    fn test_size_hash() {
        let mut hash_set = HashSet::new();
        assert!(hash_set.insert(Size::Small));
        assert!(hash_set.insert(Size::Medium));
        assert!(hash_set.insert(Size::Large));
        assert_eq!(hash_set.len(), 3);
    }

    fn test_size_partial_ord() {
        assert!(Size::Small < Size::Medium);
        assert!(Size::Medium < Size::Large);
        assert!(Size::Large > Size::Small);
    }

    fn test_size_ord() {
        assert_eq!(Size::Small.cmp(&Size::Small), Ordering::Equal);
        assert_eq!(Size::Small.cmp(&Size::Medium), Ordering::Less);
        assert_eq!(Size::Large.cmp(&Size::Medium), Ordering::Greater);
    }

    fn test_size_is_valid() {
        assert!(Size::is_valid(0));
        assert!(Size::is_valid(1));
        assert!(Size::is_valid(2));
        assert!(!Size::is_valid(3));
        assert!(!Size::is_valid(-1));
    }

    fn test_size_from_i32() {
        #[allow(deprecated)]
        {
            assert_eq!(Size::from_i32(0), Some(Size::Small));
            assert_eq!(Size::from_i32(1), Some(Size::Medium));
            assert_eq!(Size::from_i32(2), Some(Size::Large));
            assert_eq!(Size::from_i32(3), None);
        }
    }

    fn test_size_into() {
        let size: i32 = Size::Medium.into();
        assert_eq!(size, 1);

        let size: i32 = Size::Large.into();
        assert_eq!(size, 2);
    }

    fn test_size_try_from() {
        assert_eq!(Size::try_from(0i32), Ok(Size::Small));
        assert_eq!(Size::try_from(1i32), Ok(Size::Medium));
        assert_eq!(Size::try_from(2i32), Ok(Size::Large));
        assert_eq!(Size::try_from(3i32), Err(prost::UnknownEnumValue(3)));
    }

    fn test_size_as_str_name() {
        assert_eq!(Size::Small.as_str_name(), "SMALL");
        assert_eq!(Size::Medium.as_str_name(), "MEDIUM");
        assert_eq!(Size::Large.as_str_name(), "LARGE");
    }

    fn test_size_from_str_name() {
        assert_eq!(Size::from_str_name("SMALL"), Some(Size::Small));
        assert_eq!(Size::from_str_name("MEDIUM"), Some(Size::Medium));
        assert_eq!(Size::from_str_name("LARGE"), Some(Size::Large));
        assert_eq!(Size::from_str_name("INVALID"), None);
    }
}
