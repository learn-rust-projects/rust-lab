use std::collections::BinaryHeap;

fn main() {
    test_push_pop();
    test_peek();
    test_peek_mut();
    test_common_methods();
    test_ord_type();
    test_iter();
    test_append();
    println!("All tests passed!");
}

fn test_push_pop() {
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(3);
    heap.push(8);
    heap.push(6);
    heap.push(9);
    heap.push(2);

    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);

    let mut heap2 = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
    assert_eq!(heap2.peek(), Some(&9));
    assert_eq!(heap2.pop(), Some(9));
}

fn test_peek() {
    let heap = BinaryHeap::from(vec![5, 1, 3]);
    assert_eq!(heap.peek(), Some(&5));

    let heap: BinaryHeap<i32> = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
}

fn test_peek_mut() {
    let mut heap = BinaryHeap::from(vec![5, 1, 3]);

    if let Some(mut max) = heap.peek_mut() {
        *max += 10;
    }

    assert_eq!(heap.peek(), Some(&15));

    let popped = heap.pop();
    assert_eq!(popped, Some(15));
    assert_eq!(heap.peek(), Some(&3));
}

fn test_common_methods() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    assert!(heap.is_empty());
    assert_eq!(heap.len(), 0);

    heap.push(1);
    heap.push(2);
    heap.push(3);
    assert!(!heap.is_empty());
    assert_eq!(heap.len(), 3);

    let capacity = heap.capacity();
    assert!(capacity >= 3);

    heap.clear();
    assert!(heap.is_empty());
}

fn test_ord_type() {
    #[derive(Debug, Eq, PartialEq)]
    struct Task {
        priority: u32,
        name: String,
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap: BinaryHeap<Task> = BinaryHeap::new();
    heap.push(Task {
        priority: 1,
        name: "low".to_string(),
    });
    heap.push(Task {
        priority: 3,
        name: "high".to_string(),
    });
    heap.push(Task {
        priority: 2,
        name: "medium".to_string(),
    });

    let task = heap.pop().unwrap();
    assert_eq!(task.priority, 3);
    assert_eq!(task.name, "high");
}

fn test_iter() {
    let heap = BinaryHeap::from(vec![3, 1, 5, 2, 4]);
    let items: Vec<&i32> = heap.iter().collect();
    assert_eq!(items.len(), 5);

    let mut sum = 0;
    for x in &heap {
        sum += *x;
    }
    assert_eq!(sum, 15);

    let heap = BinaryHeap::from(vec![3, 1, 5, 2, 4]);
    let max = heap.iter().max();
    assert_eq!(max, Some(&5));
}

fn test_append() {
    let mut heap1 = BinaryHeap::from(vec![5, 1, 3]);
    let mut heap2 = BinaryHeap::from(vec![6, 2, 4]);

    heap1.append(&mut heap2);

    assert!(heap2.is_empty());
    assert_eq!(heap1.len(), 6);

    assert_eq!(heap1.pop(), Some(6));
    assert_eq!(heap1.pop(), Some(5));
}
