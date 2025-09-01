#[allow(unused_imports)]
use std::{
    fs::File,
    io::{self, Write},
};

// 11.1.1 Using the `assert!` macro to check results

#[test]
fn test_assert() {
    let x = 5;
    assert!(x > 3);
    assert!(x > 3, "x should be greater than 3, but got {x}");
}
// 11.1.2 Using the `assert_eq!` and `assert_ne!` macros to compare values

// 比较的值必须实现`PartialEq`和`Debug`特征

#[test]
fn test_assert_eq() {
    let x = 5;
    let y = 5;
    assert_eq!(x, y, "x and y should be equal, but got {x} and {y}");
}

#[test]
fn test_assert_ne() {
    let x = 4;
    let y = 5;
    assert_ne!(x, y, "x and y should not be equal, but got {x} and {y}");
}

// 11.1.3 Adding custom failure messages
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}

// 11.1.4 Using `should_panic` to check for panics
#[derive(Debug)]
#[allow(dead_code)]
pub struct Guess {
    value: i32,
}
#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}
// 如果你直接在生产模块里写 #[test] fn
// ...，这个测试函数仍然会出现在编译单元中（不过只有在 cargo test 时才运行）。
// 为了避免这个问题，你可以在测试模块里写 #[cfg(test)] mod tests { ... }，
// 这样测试模块就只会在 cargo test 时编译，而不会在 cargo build 时编译。
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_with_message() {
        Guess::new(200);
    }
}

// 11.1.5 Using `Result<T, E>` in tests

#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        // Error: "two plus two does not equal four"
        // 输出比较简洁
        Err(String::from("two plus two does not equal four"))
    }
}
#[allow(dead_code)]
// Define missing helper functions
fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 11.1.6 Running tests in parallel and controlling test execution order

// By default, tests run in parallel to speed up execution.
// You can control this behavior with the `--test-threads` flag.
// cargo test -- --test-threads=1

// By default, test output is captured and displayed only for failed tests.
// You can use the `--show-output` flag to show output for all tests.
// can show paninc output
// cargo test -- --show-output

// You can also run specific tests by name using the `--test` flag.
// cargo test it_works

// 实时显示输出
// cargo test -- --nocapture

// 此命令运行名称中带有add的所有测试，并过滤掉名为one_hundred的测试。另请注意，
// 测试所在的模块成为测试名称的一部分，
// 因此我们可以通过过滤模块名称来运行模块中的所有测试。
// cargo test tests
// running 2 tests
// test basic_01::tests::greater_than_100 - should panic ... ok
// test basic_01::tests::greater_than_100_with_message - should panic ... ok

// 11.1.7 Ignoring tests
// #[ignore]
// #[test]
// fn test_ignored() {
//     // 测试代码
// }

// 【只想运行被忽略的测试】expensive_test函数被列为ignored
// 。如果我们只想运行被忽略的测试，我们可以使用cargo test -- --ignored ：
// cargo test -- --ignored
// running 1 test
// test basic_01::test_ignored ... ignored

// 11.1.8 Using `cargo test` command-line options
// 11.1.9 Capturing standard output and error during tests
// 11.1.10 Writing integration tests
// 单元测试和集成测试。单元测试规模较小，更专注，一次单独测试一个模块，
// 并且可以测试私有接口。集成测试完全独立于库之外，
// 并以与任何其他外部代码相同的方式使用您的代码，仅使用公共接口，
// 并且可能每次测试执行多个模块。
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}
fn internal_adder(a: usize, b: usize) -> usize {
    a + b
}
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     ├── common
//     │   └── mod.rs
//     └── integration_test.rs
// 不要将`common`模块视为集成测试文件,
// common模块是为了在多个测试文件中共享代码而设计的, 不是为了测试代码而设计的,
// 所以不要将`common`模块视为集成测试文件.
// 我们可以调用 `common::setup()`函数来设置测试环境.

// 11.1.11 Using test frameworks and libraries
// 11.1.12 Benchmarking code
// 11.1.13 Testing asynchronous code
// 11.1.14 Property-based testing
// 11.1.15 Mocking and stubbing in tests
// 11.1.16 Testing for memory safety and concurrency issues
// 11.1.17 Continuous integration and automated testing
// 11.1.18 Best practices for writing effective tests
// 11.1.19 Common pitfalls and how to avoid them in testing
