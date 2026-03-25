use std::collections::VecDeque;

fn main() {
    test_push_pop();
    test_front_back();
    test_front_back_mut();
    test_len_is_empty();
    test_index_access();
    test_iterators();
    test_make_contiguous();
    test_conversions();
    println!("All tests passed!");
}

fn test_push_pop() {
    let mut deque = VecDeque::new();

    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(0);

    assert_eq!(deque.len(), 4);
    assert_eq!(deque.pop_back(), Some(3));
    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(deque.pop_back(), Some(2));
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_front(), None);

    let mut deque2 = VecDeque::with_capacity(2);
    deque2.push_front(1);
    deque2.push_back(2);
    assert_eq!(deque2.pop_front(), Some(1));
    assert_eq!(deque2.pop_back(), Some(2));
}

fn test_front_back() {
    let deque = VecDeque::from([1, 2, 3]);

    assert_eq!(deque.front(), Some(&1));
    assert_eq!(deque.back(), Some(&3));

    assert_eq!(deque.front(), Some(&1));
    assert_eq!(deque.back(), Some(&3));
}

fn test_front_back_mut() {
    let mut deque = VecDeque::from([1, 2, 3]);

    if let Some(x) = deque.front_mut() {
        *x = 10;
    }
    if let Some(x) = deque.back_mut() {
        *x = 30;
    }

    assert_eq!(deque.front(), Some(&10));
    assert_eq!(deque.back(), Some(&30));
}

fn test_len_is_empty() {
    let deque: VecDeque<i32> = VecDeque::new();
    assert!(deque.is_empty());
    assert_eq!(deque.len(), 0);

    let deque = VecDeque::from([1, 2, 3]);
    assert!(!deque.is_empty());
    assert_eq!(deque.len(), 3);
}

fn test_index_access() {
    let deque = VecDeque::from([10, 20, 30, 40]);
    assert_eq!(deque[0], 10);
    assert_eq!(deque[2], 30);
    assert_eq!(deque[3], 40);
}

fn test_iterators() {
    let deque = VecDeque::from([1, 2, 3]);

    let sum: i32 = deque.iter().sum();
    assert_eq!(sum, 6);

    let mut deque = VecDeque::from([1, 2, 3]);
    for x in deque.iter_mut() {
        *x *= 2;
    }
    assert_eq!(deque, VecDeque::from([2, 4, 6]));

    let deque = VecDeque::from([1, 2, 3]);
    let collected: Vec<i32> = deque.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

fn test_make_contiguous() {
    let mut deque = VecDeque::from([1, 2, 3, 4]);
    let slice = deque.make_contiguous();
    assert_eq!(slice, &[1, 2, 3, 4]);

    let slice2 = deque.make_contiguous();
    assert_eq!(slice2, &[1, 2, 3, 4]);

    let vec: Vec<i32> = Vec::from(deque.make_contiguous());
    assert_eq!(vec, vec![1, 2, 3, 4]);
}

fn test_conversions() {
    let deque = VecDeque::from([1, 2, 3, 4]);
    let vec: Vec<i32> = Vec::from(deque);
    assert_eq!(vec, vec![1, 2, 3, 4]);

    let vec = vec![1, 2, 3, 4];
    let deque = VecDeque::from(vec);
    assert_eq!(deque, VecDeque::from([1, 2, 3, 4]));
}
