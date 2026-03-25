pub mod swap_tests {
    use std::assert_eq;

    pub fn run_all() {
        test_swap();
        test_swap_with_slice();
        test_swap_remove();
    }

    pub fn test_swap() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.swap(0, 4);
        assert_eq!(v, vec![5, 2, 3, 4, 1]);

        let mut arr = [1, 2, 3];
        arr.swap(0, 2);
        assert_eq!(arr, [3, 2, 1]);

        let mut v = vec![1];
        v.swap(0, 0);
        assert_eq!(v, vec![1]);

        println!("test_swap passed");
    }

    pub fn test_swap_with_slice() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        v1.swap_with_slice(&mut v2);
        assert_eq!(v1, vec![4, 5, 6]);
        assert_eq!(v2, vec![1, 2, 3]);

        let mut arr1 = [1, 2, 3];
        let mut arr2 = [4, 5, 6];
        arr1.swap_with_slice(&mut arr2);
        assert_eq!(&arr1[..], &[4, 5, 6][..]);
        assert_eq!(&arr2[..], &[1, 2, 3][..]);

        let mut v1: Vec<i32> = vec![];
        let mut v2: Vec<i32> = vec![];
        v1.swap_with_slice(&mut v2);
        assert!(v1.is_empty());
        assert!(v2.is_empty());

        println!("test_swap_with_slice passed");
    }

    pub fn test_swap_remove() {
        let mut v = vec![1, 2, 3, 4, 5];
        let removed = v.swap_remove(0);
        assert_eq!(removed, 1);
        assert_eq!(v, vec![5, 2, 3, 4]);

        let mut v = vec![1, 2, 3, 4, 5];
        let removed = v.swap_remove(2);
        assert_eq!(removed, 3);
        assert_eq!(v, vec![1, 2, 5, 4]);

        let mut v = vec![1];
        let removed = v.swap_remove(0);
        assert_eq!(removed, 1);
        assert!(v.is_empty());

        let mut v = vec![1, 2, 3];
        let removed = v.swap_remove(v.len() - 1);
        assert_eq!(removed, 3);
        assert_eq!(v, vec![1, 2]);

        println!("test_swap_remove passed");
    }
}

#[cfg(test)]
mod tests {
    use super::swap_tests::*;

    #[test]
    fn run_test_swap() {
        test_swap();
    }

    #[test]
    fn run_test_swap_with_slice() {
        test_swap_with_slice();
    }

    #[test]
    fn run_test_swap_remove() {
        test_swap_remove();
    }
}
