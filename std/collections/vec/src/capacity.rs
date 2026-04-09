pub mod capacity_tests {
    use std::assert_eq;

    pub fn run_all() {
        test_with_capacity();
        test_capacity();
        test_reserve();
        test_reserve_exact();
        test_shrink_to_fit();
        test_resize();
        test_resize_with();
        test_truncate();
        test_clear();
        test_extend();
        test_split_off();
        test_append();
        test_drain();
        test_retain();
        test_dedup();
        test_dedup_by();
        test_dedup_by_key();
    }

    pub fn test_with_capacity() {
        let v: Vec<i32> = Vec::with_capacity(10);
        assert!(v.capacity() >= 10);
        assert!(v.is_empty());

        let v: Vec<i32> = Vec::with_capacity(0);

        println!("test_with_capacity passed");
    }

    pub fn test_capacity() {
        let v: Vec<i32> = Vec::new();

        let v = vec![1, 2, 3];
        assert_eq!(v.capacity(), 3);

        let mut v = vec![1, 2, 3];
        v.push(4);
        assert!(v.capacity() >= 4);

        println!("test_capacity passed");
    }

    pub fn test_reserve() {
        let mut v = vec![1, 2, 3];
        let cap = v.capacity();
        v.reserve(10);
        assert!(v.capacity() >= cap + 10);

        let mut v = vec![1, 2, 3];
        let cap = v.capacity();
        v.reserve(0);
        assert!(v.capacity() >= cap);

        println!("test_reserve passed");
    }

    pub fn test_reserve_exact() {
        let mut v = vec![1, 2, 3];
        let len = v.len();
        v.reserve_exact(5);
        assert_eq!(v.capacity(), len + 5);

        let mut v = vec![1, 2, 3];
        v.reserve_exact(0);
        assert!(v.capacity() >= 3);

        println!("test_reserve_exact passed");
    }

    pub fn test_shrink_to_fit() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.reserve(100);
        assert!(v.capacity() > 5);
        v.shrink_to_fit();
        assert_eq!(v.capacity(), 5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v: Vec<i32> = vec![];
        v.shrink_to_fit();

        println!("test_shrink_to_fit passed");
    }

    pub fn test_resize() {
        let mut v = vec![1, 2, 3];
        v.resize(5, 0);
        assert_eq!(v, vec![1, 2, 3, 0, 0]);

        let mut v = vec![1, 2, 3, 4, 5];
        v.resize(2, 0);
        assert_eq!(v, vec![1, 2]);

        let mut v: Vec<i32> = vec![];
        v.resize(3, 5);
        assert_eq!(v, vec![5, 5, 5]);

        println!("test_resize passed");
    }

    pub fn test_resize_with() {
        let mut v = vec![1, 2, 3];
        let mut counter = 4;
        v.resize_with(6, || {
            let curr = counter;
            counter += 1;
            curr
        });
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);

        let mut v: Vec<i32> = vec![];
        v.resize_with(3, || 42);
        assert_eq!(v, vec![42, 42, 42]);

        println!("test_resize_with passed");
    }

    pub fn test_truncate() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.truncate(3);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![1, 2, 3];
        v.truncate(10);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![1, 2, 3];
        v.truncate(0);
        assert!(v.is_empty());

        println!("test_truncate passed");
    }

    pub fn test_clear() {
        let mut v = vec![1, 2, 3];
        v.clear();
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);

        let mut v: Vec<i32> = vec![];
        v.clear();
        assert!(v.is_empty());

        println!("test_clear passed");
    }

    pub fn test_extend() {
        let mut v = vec![1, 2, 3];
        v.extend(vec![4, 5, 6]);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);

        let mut v = vec![1];
        v.extend(2..=5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3];
        v.extend(&[4, 5][..]);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        println!("test_extend passed");
    }

    pub fn test_split_off() {
        let mut v = vec![1, 2, 3, 4, 5];
        let split = v.split_off(0);
        assert_eq!(v, vec![]);
        assert_eq!(split, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        let split = v.split_off(2);
        assert_eq!(v, vec![1, 2]);
        assert_eq!(split, vec![3, 4, 5]);

        let mut v = vec![1, 2, 3];
        let split = v.split_off(3);
        assert_eq!(v, vec![1, 2, 3]);
        assert!(split.is_empty());

        println!("test_split_off passed");
    }

    pub fn test_append() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        v1.append(&mut v2);
        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6]);
        assert!(v2.is_empty());

        let mut v1 = vec![1];
        let mut v2: Vec<i32> = vec![];
        v1.append(&mut v2);
        assert_eq!(v1, vec![1]);

        println!("test_append passed");
    }

    pub fn test_drain() {
        let mut v = vec![1, 2, 3, 4, 5];
        let drained: Vec<i32> = v.drain(1..4).collect();
        assert_eq!(drained, vec![2, 3, 4]);
        assert_eq!(v, vec![1, 5]);

        let mut v = vec![1, 2, 3];
        let drained: Vec<i32> = v.drain(..).collect();
        assert_eq!(drained, vec![1, 2, 3]);
        assert!(v.is_empty());

        let mut v = vec![1, 2, 3, 4, 5];
        let drained: Vec<i32> = v.drain(..=2).collect();
        assert_eq!(drained, vec![1, 2, 3]);
        assert_eq!(v, vec![4, 5]);

        println!("test_drain passed");
    }

    pub fn test_retain() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|x| x % 2 == 0);
        assert_eq!(v, vec![2, 4]);

        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|_| true);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|_| false);
        assert!(v.is_empty());

        println!("test_retain passed");
    }

    pub fn test_dedup() {
        let mut v = vec![1, 1, 2, 2, 3];
        v.dedup();
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![1, 1, 1];
        v.dedup();
        assert_eq!(v, vec![1]);

        let mut v = vec![1, 2, 3];
        v.dedup();
        assert_eq!(v, vec![1, 2, 3]);

        let mut v: Vec<i32> = vec![];
        v.dedup();
        assert!(v.is_empty());

        let mut v = vec![1, 2, 2, 1, 1, 2];
        v.dedup();
        assert_eq!(v, vec![1, 2, 1, 2]);

        println!("test_dedup passed");
    }

    pub fn test_dedup_by() {
        let mut v = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
        v.dedup_by(|a, b| a == b);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec!["a", "b", "b", "c"];
        v.dedup_by(|a, b| a == b);
        assert_eq!(v, vec!["a", "b", "c"]);

        println!("test_dedup_by passed");
    }

    pub fn test_dedup_by_key() {
        let mut v = vec![1, 2, 2, 3, 3, 3];
        v.dedup_by_key(|x| *x % 2);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v: Vec<i32> = vec![-1, 1, -2, 2, -3, 3];
        v.dedup_by_key(|x| x.abs() % 2);
        assert_eq!(v, vec![-1, -2, -3]);

        println!("test_dedup_by_key passed");
    }
}

#[cfg(test)]
mod tests {
    use super::capacity_tests::*;

    #[test]
    fn run_test_with_capacity() {
        test_with_capacity();
    }

    #[test]
    fn run_test_capacity() {
        test_capacity();
    }

    #[test]
    fn run_test_reserve() {
        test_reserve();
    }

    #[test]
    fn run_test_reserve_exact() {
        test_reserve_exact();
    }

    #[test]
    fn run_test_shrink_to_fit() {
        test_shrink_to_fit();
    }

    #[test]
    fn run_test_resize() {
        test_resize();
    }

    #[test]
    fn run_test_resize_with() {
        test_resize_with();
    }

    #[test]
    fn run_test_truncate() {
        test_truncate();
    }

    #[test]
    fn run_test_clear() {
        test_clear();
    }

    #[test]
    fn run_test_extend() {
        test_extend();
    }

    #[test]
    fn run_test_split_off() {
        test_split_off();
    }

    #[test]
    fn run_test_append() {
        test_append();
    }

    #[test]
    fn run_test_drain() {
        test_drain();
    }

    #[test]
    fn run_test_retain() {
        test_retain();
    }

    #[test]
    fn run_test_dedup() {
        test_dedup();
    }

    #[test]
    fn run_test_dedup_by() {
        test_dedup_by();
    }

    #[test]
    fn run_test_dedup_by_key() {
        test_dedup_by_key();
    }
}
