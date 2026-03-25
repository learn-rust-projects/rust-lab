pub mod split_tests {
    use std::assert_eq;

    pub fn run_all() {
        test_split_at();
        test_split_at_mut();
        test_split_first();
        test_split_first_mut();
        test_split_last();
        test_split_last_mut();
        test_split();
        test_split_mut();
        test_split_inclusive();
        test_split_inclusive_mut();
        test_rsplit();
        test_rsplit_mut();
        test_splitn();
        test_splitn_mut();
        test_rsplitn();
        test_rsplitn_mut();
        test_chunks();
        test_chunks_mut();
        test_rchunks();
        test_rchunks_mut();
        test_chunks_exact();
        test_chunks_exact_mut();
        test_rchunks_exact();
        test_rchunks_exact_mut();
        test_windows();
    }

    pub fn test_split_at() {
        let v = vec![1, 2, 3, 4, 5];
        let (left, right) = v.split_at(0);
        assert_eq!(left, &[]);
        assert_eq!(right, &[1, 2, 3, 4, 5]);

        let v = vec![1, 2, 3, 4, 5];
        let (left, right) = v.split_at(2);
        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4, 5]);

        let v = vec![1, 2, 3, 4, 5];
        let (left, right) = v.split_at(5);
        assert_eq!(left, &[1, 2, 3, 4, 5]);
        assert_eq!(right, &[]);

        let arr = [1, 2, 3, 4, 5];
        let (left, right) = arr.split_at(3);
        assert_eq!(left, &[1, 2, 3]);
        assert_eq!(right, &[4, 5]);

        println!("test_split_at passed");
    }

    pub fn test_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        let (left, right) = v.split_at_mut(2);
        left[0] = 10;
        right[0] = 20;
        assert_eq!(v, vec![10, 2, 20, 4, 5]);

        let mut arr = [1, 2, 3, 4, 5];
        let (left, right) = arr.split_at_mut(3);
        left[0] = 100;
        right[0] = 200;
        assert_eq!(arr, [100, 2, 3, 200, 5]);

        println!("test_split_at_mut passed");
    }

    pub fn test_split_first() {
        let v = vec![1, 2, 3];
        let (first, rest) = v.split_first().unwrap();
        assert_eq!(*first, 1);
        assert_eq!(rest, &[2, 3]);

        let v: Vec<i32> = vec![];
        assert!(v.split_first().is_none());

        let arr = [1, 2, 3, 4];
        let (first, rest) = arr.split_first().unwrap();
        assert_eq!(*first, 1);
        assert_eq!(rest, &[2, 3, 4]);

        println!("test_split_first passed");
    }

    pub fn test_split_first_mut() {
        let mut v = vec![1, 2, 3];
        let (first, rest) = v.split_first_mut().unwrap();
        *first = 10;
        rest[0] = 20;
        assert_eq!(v, vec![10, 20, 3]);

        let mut arr = [1, 2, 3];
        let (first, rest) = arr.split_first_mut().unwrap();
        *first = 100;
        rest[0] = 200;
        assert_eq!(arr, [100, 200, 3]);

        println!("test_split_first_mut passed");
    }

    pub fn test_split_last() {
        let v = vec![1, 2, 3];
        let (last, rest) = v.split_last().unwrap();
        assert_eq!(*last, 3);
        assert_eq!(rest, &[1, 2]);

        let v: Vec<i32> = vec![];
        assert!(v.split_last().is_none());

        let arr = [1, 2, 3, 4];
        let (last, rest) = arr.split_last().unwrap();
        assert_eq!(*last, 4);
        assert_eq!(rest, &[1, 2, 3]);

        println!("test_split_last passed");
    }

    pub fn test_split_last_mut() {
        let mut v = vec![1, 2, 3];
        let (last, rest) = v.split_last_mut().unwrap();
        *last = 30;
        rest[0] = 10;
        assert_eq!(v, vec![10, 2, 30]);

        let mut arr = [1, 2, 3];
        let (last, rest) = arr.split_last_mut().unwrap();
        *last = 300;
        rest[0] = 100;
        assert_eq!(arr, [100, 2, 300]);

        println!("test_split_last_mut passed");
    }

    pub fn test_split() {
        let v = vec![1, 2, 0, 3, 4, 0, 5];
        let parts: Vec<&[i32]> = v.split(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[1, 2][..], &[3, 4][..], &[5][..]]);

        let v = vec![0, 1, 2, 0];
        let parts: Vec<&[i32]> = v.split(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[][..], &[1, 2][..], &[][..]]);

        let v = vec![1, 2, 3];
        let parts: Vec<&[i32]> = v.split(|&x| x > 10).collect();
        assert_eq!(parts, vec![&[1, 2, 3][..]]);

        let arr = [1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = arr.split(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[1][..], &[2][..], &[3][..]]);

        println!("test_split passed");
    }

    pub fn test_split_mut() {
        let mut v = vec![1, 2, 0, 3, 4];
        for part in v.split_mut(|&x| x == 0) {
            for elem in part.iter_mut() {
                *elem *= 2;
            }
        }
        assert_eq!(v, vec![2, 4, 0, 6, 8]);

        let mut arr = [1, 0, 2, 0, 3];
        for part in arr.split_mut(|&x| x == 0) {
            part[0] = 99;
        }
        assert_eq!(arr, [99, 0, 99, 0, 99]);

        println!("test_split_mut passed");
    }

    pub fn test_split_inclusive() {
        let v = vec![1, 2, 0, 3, 4, 0, 5];
        let parts: Vec<&[i32]> = v.split_inclusive(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[1, 2, 0][..], &[3, 4, 0][..], &[5][..]]);

        let arr = [1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = arr.split_inclusive(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[1, 0][..], &[2, 0][..], &[3][..]]);

        println!("test_split_inclusive passed");
    }

    pub fn test_split_inclusive_mut() {
        let mut v = vec![1, 2, 0, 3, 4, 0, 5];
        for part in v.split_inclusive_mut(|&x| x == 0) {
            if let Some(last) = part.last_mut() {
                *last = 9;
            }
        }
        assert_eq!(v, vec![1, 2, 9, 3, 4, 9, 9]);

        println!("test_split_inclusive_mut passed");
    }

    pub fn test_rsplit() {
        let v = vec![1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = v.rsplit(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[3][..], &[2][..], &[1][..]]);

        let v = vec![0, 1, 2, 0];
        let parts: Vec<&[i32]> = v.rsplit(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[][..], &[1, 2][..], &[][..]]);

        println!("test_rsplit passed");
    }

    pub fn test_rsplit_mut() {
        let mut v = vec![1, 0, 2, 0, 3];
        for part in v.rsplit_mut(|&x| x == 0) {
            if let Some(first) = part.first_mut() {
                *first = 99;
            }
        }
        assert_eq!(v, vec![99, 0, 99, 0, 99]);

        println!("test_rsplit_mut passed");
    }

    pub fn test_splitn() {
        let v = vec![1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = v.splitn(3, |&x| x == 0).collect();
        assert_eq!(parts, vec![&[1][..], &[2][..], &[3][..]]);

        let v = vec![1, 2, 3, 4, 5];
        let parts: Vec<&[i32]> = v.splitn(2, |&x| x > 10).collect();
        assert_eq!(parts, vec![&[1, 2, 3, 4, 5][..]]);

        let arr = [1, 0, 2, 0, 3, 0, 4];
        let parts: Vec<&[i32]> = arr.splitn(2, |&x| x == 0).collect();
        assert_eq!(parts, vec![&[1][..], &[2, 0, 3, 0, 4][..]]);

        println!("test_splitn passed");
    }

    pub fn test_splitn_mut() {
        let mut v = vec![1, 0, 2, 0, 3];
        {
            let mut iter = v.splitn_mut(2, |&x| x == 0);
            if let Some(part) = iter.next() {
                part[0] = 99;
            }
        }
        assert_eq!(v, vec![99, 0, 2, 0, 3]);

        println!("test_splitn_mut passed");
    }

    pub fn test_rsplitn() {
        let v = vec![1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = v.rsplitn(3, |&x| x == 0).collect();
        assert_eq!(parts, vec![&[3][..], &[2][..], &[1][..]]);

        let arr = [1, 0, 2, 0, 3];
        let parts: Vec<&[i32]> = arr.rsplitn(2, |&x| x == 0).collect();
        assert_eq!(parts, vec![&[3][..], &[1, 0, 2][..]]);

        println!("test_rsplitn passed");
    }

    pub fn test_rsplitn_mut() {
        let mut v = vec![1, 0, 2, 0, 3];
        {
            let mut iter = v.rsplitn_mut(2, |&x| x == 0);
            if let Some(part) = iter.next() {
                part[0] = 99;
            }
        }
        assert_eq!(v, vec![1, 0, 2, 0, 99]);

        println!("test_rsplitn_mut passed");
    }

    pub fn test_chunks() {
        let v = vec![1, 2, 3, 4, 5];
        let chunks: Vec<&[i32]> = v.chunks(2).collect();
        assert_eq!(chunks, vec![&[1, 2][..], &[3, 4][..], &[5][..]]);

        let v = vec![1, 2, 3, 4, 5, 6];
        let chunks: Vec<&[i32]> = v.chunks(2).collect();
        assert_eq!(chunks, vec![&[1, 2][..], &[3, 4][..], &[5, 6][..]]);

        let arr = [1, 2, 3, 4, 5];
        let chunks: Vec<&[i32]> = arr.chunks(3).collect();
        assert_eq!(chunks, vec![&[1, 2, 3][..], &[4, 5][..]]);

        let v = vec![1];
        let chunks: Vec<&[i32]> = v.chunks(2).collect();
        assert_eq!(chunks, vec![&[1][..]]);

        println!("test_chunks passed");
    }

    pub fn test_chunks_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        for chunk in v.chunks_mut(2) {
            for elem in chunk.iter_mut() {
                *elem *= 2;
            }
        }
        assert_eq!(v, vec![2, 4, 6, 8, 10]);

        let mut arr = [1, 2, 3, 4];
        for chunk in arr.chunks_mut(2) {
            chunk[0] = 99;
        }
        assert_eq!(arr, [99, 2, 99, 4]);

        println!("test_chunks_mut passed");
    }

    pub fn test_rchunks() {
        let v = vec![1, 2, 3, 4, 5];
        let chunks: Vec<&[i32]> = v.rchunks(2).collect();
        assert_eq!(chunks, vec![&[5][..], &[3, 4][..], &[1, 2][..]]);

        let arr = [1, 2, 3, 4, 5];
        let chunks: Vec<&[i32]> = arr.rchunks(3).collect();
        assert_eq!(chunks, vec![&[3, 4, 5][..], &[1, 2][..]]);

        println!("test_rchunks passed");
    }

    pub fn test_rchunks_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        for chunk in v.rchunks_mut(2) {
            chunk[0] = 99;
        }
        assert_eq!(v, vec![1, 2, 99, 4, 99]);

        println!("test_rchunks_mut passed");
    }

    pub fn test_chunks_exact() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut chunks = v.chunks_exact(2);
        assert_eq!(chunks.next().unwrap(), &[1, 2]);
        assert_eq!(chunks.next().unwrap(), &[3, 4]);
        assert_eq!(chunks.next().unwrap(), &[5, 6]);
        assert!(chunks.next().is_none());
        assert_eq!(chunks.remainder(), &[5]);

        let v = vec![1, 2, 3, 4];
        let mut chunks = v.chunks_exact(2);
        assert_eq!(chunks.next().unwrap(), &[1, 2]);
        assert_eq!(chunks.next().unwrap(), &[3, 4]);
        assert!(chunks.next().is_none());
        assert!(chunks.remainder().is_empty());

        println!("test_chunks_exact passed");
    }

    pub fn test_chunks_exact_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        {
            let mut chunks = v.chunks_exact_mut(2);
            if let Some(chunk) = chunks.next() {
                chunk[0] = 99;
            }
        }
        assert_eq!(v, vec![99, 2, 3, 4, 5]);

        println!("test_chunks_exact_mut passed");
    }

    pub fn test_rchunks_exact() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut chunks = v.rchunks_exact(2);
        assert_eq!(chunks.next().unwrap(), &[5, 6]);
        assert_eq!(chunks.next().unwrap(), &[3, 4]);
        assert_eq!(chunks.next().unwrap(), &[1, 2]);
        assert!(chunks.next().is_none());
        assert_eq!(chunks.remainder(), &[1]);

        println!("test_rchunks_exact passed");
    }

    pub fn test_rchunks_exact_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        {
            let mut chunks = v.rchunks_exact_mut(2);
            if let Some(chunk) = chunks.next() {
                chunk[0] = 99;
            }
        }
        assert_eq!(v, vec![1, 2, 3, 99, 5]);

        println!("test_rchunks_exact_mut passed");
    }

    pub fn test_windows() {
        let v = vec![1, 2, 3, 4, 5];
        let windows: Vec<&[i32]> = v.windows(2).collect();
        assert_eq!(windows, vec![&[1, 2], &[2, 3], &[3, 4], &[4, 5]]);

        let v = vec![1, 2, 3, 4, 5];
        let windows: Vec<&[i32]> = v.windows(3).collect();
        assert_eq!(windows, vec![&[1, 2, 3], &[2, 3, 4], &[3, 4, 5]]);

        let v = vec![1];
        let windows: Vec<&[i32]> = v.windows(2).collect();
        assert!(windows.is_empty());

        let v = vec![1, 2, 3];
        let windows: Vec<&[i32]> = v.windows(1).collect();
        assert_eq!(windows, vec![&[1], &[2], &[3]]);

        let arr = [1, 2, 3, 4];
        let windows: Vec<&[i32]> = arr.windows(2).collect();
        assert_eq!(windows, vec![&[1, 2], &[2, 3], &[3, 4]]);

        let daily_temps = vec![70, 72, 75, 73, 71, 69, 68];
        let changes: Vec<i32> = daily_temps.windows(2).map(|w| w[1] - w[0]).collect();
        assert_eq!(changes, vec![2, 3, -2, -2, -2, -1]);

        println!("test_windows passed");
    }
}

#[cfg(test)]
mod tests {
    use super::split_tests::*;

    #[test]
    fn run_test_split_at() {
        test_split_at();
    }

    #[test]
    fn run_test_split_at_mut() {
        test_split_at_mut();
    }

    #[test]
    fn run_test_split_first() {
        test_split_first();
    }

    #[test]
    fn run_test_split_first_mut() {
        test_split_first_mut();
    }

    #[test]
    fn run_test_split_last() {
        test_split_last();
    }

    #[test]
    fn run_test_split_last_mut() {
        test_split_last_mut();
    }

    #[test]
    fn run_test_split() {
        test_split();
    }

    #[test]
    fn run_test_split_mut() {
        test_split_mut();
    }

    #[test]
    fn run_test_split_inclusive() {
        test_split_inclusive();
    }

    #[test]
    fn run_test_split_inclusive_mut() {
        test_split_inclusive_mut();
    }

    #[test]
    fn run_test_rsplit() {
        test_rsplit();
    }

    #[test]
    fn run_test_rsplit_mut() {
        test_rsplit_mut();
    }

    #[test]
    fn run_test_splitn() {
        test_splitn();
    }

    #[test]
    fn run_test_splitn_mut() {
        test_splitn_mut();
    }

    #[test]
    fn run_test_rsplitn() {
        test_rsplitn();
    }

    #[test]
    fn run_test_rsplitn_mut() {
        test_rsplitn_mut();
    }

    #[test]
    fn run_test_chunks() {
        test_chunks();
    }

    #[test]
    fn run_test_chunks_mut() {
        test_chunks_mut();
    }

    #[test]
    fn run_test_rchunks() {
        test_rchunks();
    }

    #[test]
    fn run_test_rchunks_mut() {
        test_rchunks_mut();
    }

    #[test]
    fn run_test_chunks_exact() {
        test_chunks_exact();
    }

    #[test]
    fn run_test_chunks_exact_mut() {
        test_chunks_exact_mut();
    }

    #[test]
    fn run_test_rchunks_exact() {
        test_rchunks_exact();
    }

    #[test]
    fn run_test_rchunks_exact_mut() {
        test_rchunks_exact_mut();
    }

    #[test]
    fn run_test_windows() {
        test_windows();
    }
}
