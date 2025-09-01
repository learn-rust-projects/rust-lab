use std::cell::UnsafeCell;
#[warn(unused)]
#[allow(dead_code)]
struct MyStruct {
    data: UnsafeCell<i32>,
}
#[allow(dead_code)]
impl MyStruct {
    fn set_data(&self, value: i32) {
        unsafe {
            // by dereferencing the raw pointer
            *self.data.get() = value;
        }
    }

    fn get_data(&self) -> i32 {
        unsafe { *self.data.get() }
    }
}

#[test]
fn test_my_struct() {
    let s = MyStruct {
        data: UnsafeCell::new(5),
    };

    // set data to 10
    s.set_data(10);

    // Test that get_data() returns 10
    assert_eq!(s.get_data(), 10); // assert that get_data() returns 10
}
