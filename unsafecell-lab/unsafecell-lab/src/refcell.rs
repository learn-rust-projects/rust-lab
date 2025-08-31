use std::cell::RefCell;

// Change to English comments
#[test]
fn test_refcell_borrowing() {
    let x = RefCell::new(5);

    // Test mutable borrowing
    {
        let mut y = x.borrow_mut(); // Mutable borrow
        *y += 1;
    }

    // Test immutable borrowing
    {
        let z = x.borrow(); // Immutable borrow
        assert_eq!(*z, 6); // Assert that the value of x is 6
    }
}
