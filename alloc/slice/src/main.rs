#![feature(allocator_api)]
#![feature(slice_concat_trait)]
#![allow(deprecated)]

use std::{alloc::System, cmp::Ordering};

fn main() {
    println!("=== alloc/slice [T] Methods Examples ===");

    sorting_methods();
    conversion_methods();
    replication_methods();
    concatenation_methods();
    ascii_case_methods();

    println!("\nAll examples completed successfully.");
}

fn sorting_methods() {
    println!("\n--- Sorting Methods ---");

    // sort
    // Sorts the slice in ascending order, preserving initial order of equal
    // elements. This sort is stable (i.e., does not reorder equal elements) and
    // O(n * log(n)) worst-case.
    let mut v = [4, -5, 1, -3, 2];
    println!("Original: {:?}", v);
    v.sort();
    println!("Sorted: {:?}", v);
    assert_eq!(v, [-5, -3, 1, 2, 4]);

    // sort_by
    // Sorts the slice in ascending order with a comparison function, preserving
    // initial order of equal elements.
    let mut v = [4, -5, 1, -3, 2];
    v.sort_by(|a, b| b.cmp(a));
    println!("Sort by cmp: {:?}", v);
    assert_eq!(v, [-5, -3, 1, 2, 4]);

    // reverse sorting
    v.sort_by(|a, b| b.cmp(a));
    println!("Reverse sort: {:?}", v);
    assert_eq!(v, [4, 2, 1, -3, -5]);

    // sort_by_key
    // Sorts the slice in ascending order with a key extraction function, preserving
    // initial order of equal elements.
    let mut v = [4i32, -5, 1, -3, 2];
    v.sort_by_key(|k| k.abs());
    println!("Sort by key (abs): {:?}", v);
    assert_eq!(v, [1, 2, -3, 4, -5]);

    // sort_by_cached_key
    // Sorts the slice in ascending order with a key extraction function, preserving
    // initial order of equal elements. Useful when the key function is
    // expensive.
    let mut v = [4i32, -5, 1, -3, 2, 10];
    // Strings are sorted by lexicographical order.
    v.sort_by_cached_key(|k| k.to_string());
    println!("Sort by cached key (to_string): {:?}", v);
    // "-3", "-5", "1", "10", "2", "4" lexicographically
    assert_eq!(v, [-3, -5, 1, 10, 2, 4]);
}

fn conversion_methods() {
    println!("\n--- Conversion Methods ---");

    // to_vec
    // Copies self into a new Vec.
    let s = [10, 40, 30];
    let x = s.to_vec();
    println!("Slice: {:?}, Vec: {:?}", s, x);
    // Here, `s` and `x` can be modified independently.

    // to_vec_in
    // Copies self into a new Vec with an allocator.
    let s = [10, 40, 30];
    let x = s.to_vec_in(System);
    println!("Vec with System allocator: {:?}", x);

    // into_vec (for Box<[T]>)
    // Converts self into a vector without clones or allocation.
    let s: Box<[i32]> = Box::new([10, 40, 30]);
    let x = s.into_vec();
    println!("Box<[i32]> into_vec: {:?}", x);
    assert_eq!(x, vec![10, 40, 30]);
}

fn replication_methods() {
    println!("\n--- Replication Methods ---");

    // repeat
    // Creates a vector by copying a slice n times.
    let v = [1, 2];
    let repeated = v.repeat(3);
    println!("Repeat [1, 2] 3 times: {:?}", repeated);
    assert_eq!(repeated, vec![1, 2, 1, 2, 1, 2]);
}

fn concatenation_methods() {
    println!("\n--- Concatenation Methods ---");

    // concat
    // Flattens a slice of T into a single value Self::Output.
    let slices = ["hello", "world"];
    let concatenated = slices.concat();
    println!("Concat ['hello', 'world']: {:?}", concatenated);
    assert_eq!(concatenated, "helloworld");

    let arrays = [[1, 2], [3, 4]];
    let concatenated_arrays = arrays.concat();
    println!("Concat [[1, 2], [3, 4]]: {:?}", concatenated_arrays);
    assert_eq!(concatenated_arrays, [1, 2, 3, 4]);

    // join
    // Flattens a slice of T into a single value Self::Output, placing a given
    // separator between each.
    let joined = ["hello", "world"].join(" ");
    println!("Join ['hello', 'world'] with space: {:?}", joined);
    assert_eq!(joined, "hello world");

    let joined_arrays = [[1, 2], [3, 4]].join(&0);
    println!("Join [[1, 2], [3, 4]] with 0: {:?}", joined_arrays);
    assert_eq!(joined_arrays, [1, 2, 0, 3, 4]);

    let joined_arrays_slice = [[1, 2], [3, 4]].join(&[0, 0][..]);
    println!(
        "Join [[1, 2], [3, 4]] with [0, 0]: {:?}",
        joined_arrays_slice
    );
    assert_eq!(joined_arrays_slice, [1, 2, 0, 0, 3, 4]);

    // connect (deprecated)
    // Same as join.
    let connected = ["hello", "world"].connect(" ");
    println!("Connect ['hello', 'world'] with space: {:?}", connected);
    assert_eq!(connected, "hello world");
}

fn ascii_case_methods() {
    println!("\n--- ASCII Case Methods (for [u8]) ---");

    // to_ascii_uppercase
    // Returns a vector containing a copy of this slice where each byte is mapped to
    // its ASCII upper case equivalent.
    let bytes = b"hello world";
    let upper = bytes.to_ascii_uppercase();
    println!("Original: {:?}, Upper: {:?}", bytes, upper);
    assert_eq!(upper, b"HELLO WORLD");

    // to_ascii_lowercase
    // Returns a vector containing a copy of this slice where each byte is mapped to
    // its ASCII lower case equivalent.
    let bytes = b"HELLO WORLD";
    let lower = bytes.to_ascii_lowercase();
    println!("Original: {:?}, Lower: {:?}", bytes, lower);
    assert_eq!(lower, b"hello world");
}
