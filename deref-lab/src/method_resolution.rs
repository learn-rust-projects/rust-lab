//! # 方法解析中的 Deref
//!
//! Rust 的方法解析会沿着 Deref 链查找方法，这是"Deref trait 的主要用途"：
//! 使得自定义类型能够"借用"其他类型的方法。
//!
//! ## 查找过程
//!
//! 当调用 `receiver.method()` 时，编译器会：
//! 1. 在 `T` 上查找方法
//! 2. 在 `T` 的 Deref 目标类型上查找方法
//! 3. 递归直到找到或报错
//!
//! ## 自动解引用的优先级
//!
//! 1. 首先尝试 `*self`（Deref）
//! 2. 然后尝试 `&self`
//! 3. 然后尝试 `&mut self`
//! 4. 最后尝试 `self`（值）

use std::ops::{Deref, DerefMut};

// ============================================================================
// 测试类型定义
// ============================================================================

/// 只有 Deref 的包装类型
struct DerefOnly<T>(T);

impl<T> Deref for DerefOnly<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 同时有 Deref 和 DerefMut 的包装类型
struct DerefMutOnly<T>(T);

impl<T> Deref for DerefMutOnly<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DerefMutOnly<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 带有自己方法的类型
struct CustomType {
    data: String,
}

impl CustomType {
    fn new(s: &str) -> Self {
        CustomType {
            data: s.to_string(),
        }
    }

    fn custom_method(&self) -> &str {
        "custom"
    }

    fn mut_method(&mut self) {
        self.data.push_str("_mutated");
    }
}

impl Deref for CustomType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for CustomType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// ============================================================================
// 测试函数
// ============================================================================

/// 测试 1: 基本方法解析
fn test_basic_method_resolution() {
    println!("\n--- 测试 1: 基本方法解析 ---");

    let wrapped = DerefOnly(String::from("hello"));

    // String 有 len() 方法
    // wrapped 是 DerefOnly<String>
    // 编译器会自动解引用来调用 String::len()
    let len = wrapped.len();
    println!("  wrapped.len() = {}", len);

    // 同样的逻辑适用于其他 String 方法
    let upper = wrapped.to_uppercase();
    println!("  wrapped.to_uppercase() = {}", upper);

    println!("  ✓ 自动方法解析成功");
}

/// 测试 2: 可变方法的解析
fn test_mut_method_resolution() {
    println!("\n--- 测试 2: 可变方法解析 ---");

    let mut wrapped = DerefMutOnly(String::from("before"));

    // String 有 push_str 方法
    wrapped.push_str(" after");
    println!("  推送后: {}", *wrapped);

    // Vec 的方法
    let mut vec_wrapped = DerefMutOnly(vec![1, 2, 3]);
    vec_wrapped.push(4);
    println!("  Vec 推送后: {:?}", *vec_wrapped);

    println!("  ✓ 可变方法解析成功");
}

/// 测试 3: 优先使用自己实现的方法
fn test_method_precedence() {
    println!("\n--- 测试 3: 方法优先级 ---");

    let custom = CustomType::new("test");

    // CustomType 有自己的 custom_method
    let result = custom.custom_method();
    println!("  custom_method() = {}", result);

    // String 也有 contains 方法
    // 编译器会通过 Deref 调用 String 的方法
    let has_data = custom.contains("tes");
    println!("  contains(\"tes\") = {}", has_data);

    println!("  ✓ 方法优先级正确");
}

/// 测试 4: Deref 到 Deref 的方法解析
fn test_multi_level_deref_method() {
    println!("\n--- 测试 4: 多层 Deref 方法解析 ---");

    struct Inner(String);

    impl Inner {
        fn new(s: &str) -> Self {
            Inner(s.to_string())
        }
    }

    impl Deref for Inner {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    struct Middle(Inner);

    impl Middle {
        fn new(s: &str) -> Self {
            Middle(Inner::new(s))
        }
    }

    impl Deref for Middle {
        type Target = Inner;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    struct Outer(Middle);

    impl Outer {
        fn new(s: &str) -> Self {
            Outer(Middle::new(s))
        }
    }

    impl Deref for Outer {
        type Target = Middle;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let outer = Outer::new("multi-level");

    // Outer -> Middle -> Inner -> String -> str
    // 方法解析会在整个链上查找
    let len = outer.len();
    println!("  outer.len() = {}", len);

    let upper = outer.to_uppercase();
    println!("  outer.to_uppercase() = {}", upper);

    let split: Vec<&str> = outer.split('-').collect();
    println!("  outer.split('-') = {:?}", split);

    println!("  ✓ 多层 Deref 方法解析成功");
}

/// 测试 5: 迭代器方法解析
fn test_iterator_method_resolution() {
    println!("\n--- 测试 5: 迭代器方法解析 ---");

    let wrapped_vec = DerefOnly(vec![1, 2, 3, 4, 5]);

    // Vec 实现了 IntoIterator，所以可以直接在 wrapped_vec 上迭代
    let sum: i32 = wrapped_vec.iter().sum();
    println!("  sum = {}", sum);

    // 链式方法调用
    let doubled: Vec<i32> = wrapped_vec.iter().map(|x| x * 2).collect();
    println!("  doubled = {:?}", doubled);

    // 过滤
    let evens: Vec<&i32> = wrapped_vec.iter().filter(|x| *x % 2 == 0).collect();
    println!("  evens = {:?}", evens);

    println!("  ✓ 迭代器方法解析成功");
}

/// 测试 6: 运算符方法解析
fn test_operator_method_resolution() {
    println!("\n--- 测试 6: 运算符方法解析 ---");

    // Index trait
    let wrapped = DerefOnly(vec![10, 20, 30, 40, 50]);

    println!("  wrapped[0] = {}", wrapped[0]);
    println!("  wrapped[2] = {}", wrapped[2]);

    // Range index
    println!("  wrapped[1..3] = {:?}", &wrapped[1..3]);

    println!("  ✓ 运算符方法解析成功");
}

/// 测试 7: 位置 vs 可变借用的方法解析
fn test_borrow_kind_resolution() {
    println!("\n--- 测试 7: 借用类型方法解析 ---");

    // &T 的方法
    let wrapped = DerefOnly(String::from("immutable"));
    let len = (&wrapped).len();
    println!("  (&wrapped).len() = {}", len);

    // &mut T 的方法
    let mut mutable_wrapped = DerefMutOnly(String::from("mutable"));
    mutable_wrapped.push_str("_extended");
    println!("  扩展后: {}", *mutable_wrapped);

    println!("  ✓ 借用类型方法解析成功");
}

/// 测试 8: 自动解引用在方法调用中的行为
fn test_autoderef_behavior() {
    println!("\n--- 测试 8: 自动解引用行为 ---");

    let wrapped = DerefOnly(String::from("test"));

    // 直接调用 - 编译器自动解引用
    let result1 = wrapped.len();
    println!("  wrapped.len() = {}", result1);

    // 显式解引用一层
    let result2 = (&*wrapped).len();
    println!("  (&*wrapped).len() = {}", result2);

    // 多层引用 - 自动解引用
    let double_ref = &&wrapped;
    let result3 = double_ref.len(); // 自动解引用两层
    println!("  (&&wrapped).len() = {}", result3);

    println!("  ✓ 自动解引用行为验证成功");
}

/// 测试 9: 闭包和函数指针中的 Deref
fn test_fn_traits_deref() {
    println!("\n--- 测试 9: 函数 trait 中的 Deref ---");

    let wrapped = DerefOnly(String::from("callback"));

    let closure = || {
        // 在闭包内部，wrapped 仍然可以通过 Deref 使用
        println!("  闭包中: {}", wrapped.len());
    };

    closure();

    // 函数指针
    fn takes_string(s: &str) -> usize {
        s.len()
    }

    let result = takes_string(&wrapped);
    println!("  函数指针调用结果: {}", result);

    println!("  ✓ 函数 trait 中的 Deref 成功");
}

/// 测试 10: 方法链式调用中的 Deref
fn test_chained_method_calls() {
    println!("\n--- 测试 10: 方法链式调用 ---");

    let wrapped = DerefOnly(String::from("hello world"));

    // 链式调用 - 每一层都可能触发 Deref
    let result: Vec<char> = wrapped
        .to_uppercase() // String -> String
        .trim() // &str -> &str
        .chars() // &str -> Chars
        .collect();

    println!("  chars: {:?}", result);

    println!("  ✓ 方法链式调用成功");
}

/// 测试 11: 结构体方法解析
fn test_struct_method_resolution() {
    println!("\n--- 测试 11: 结构体方法解析 ---");

    struct MyStruct {
        data: Vec<String>,
    }

    impl Deref for MyStruct {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    let s = MyStruct {
        data: vec!["a".to_string(), "b".to_string()],
    };

    // 通过 Deref 调用 Vec 的方法
    let len = s.len();
    println!("  s.len() = {}", len);

    let is_empty = s.is_empty();
    println!("  s.is_empty() = {}", is_empty);

    println!("  ✓ 结构体方法解析成功");
}

/// 测试 12: 元组结构体方法解析
fn test_tuple_struct_method_resolution() {
    println!("\n--- 测试 12: 元组结构体方法解析 ---");

    struct MyTupleStruct(String);

    impl Deref for MyTupleStruct {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let t = MyTupleStruct(String::from("tuple"));

    // 调用 String 的方法
    let upper = t.to_uppercase();
    println!("  t.to_uppercase() = {}", upper);

    let contains = t.contains("uple");
    println!("  t.contains(\"uple\") = {}", contains);

    println!("  ✓ 元组结构体方法解析成功");
}

/// 主测试函数
pub fn test_method_resolution() {
    test_basic_method_resolution();
    test_mut_method_resolution();
    test_method_precedence();
    test_multi_level_deref_method();
    test_iterator_method_resolution();
    test_operator_method_resolution();
    test_borrow_kind_resolution();
    test_autoderef_behavior();
    test_fn_traits_deref();
    test_chained_method_calls();
    test_struct_method_resolution();
    test_tuple_struct_method_resolution();
}
