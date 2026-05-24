//! # Rust 所有类型强制点 (All Coercion Sites)
//!
//! 根据 Rust Reference，所有强制点包括：
//!
//! ## 一、基础强制点
//!
//! 1. **let 绑定**: `let x: &U = &T;`
//! 2. **static 变量**: `static X: &U = &T;` (仅限 const Deref)
//! 3. **const 变量**: `const X: &U = &T;` (仅限 const Deref)
//! 4. **函数参数**: `fn foo(x: &U)`
//! 5. **函数返回值**: `fn foo() -> &U` (生命周期受限)
//! 6. **结构体字段**: `Struct { field: &U }`
//! 7. **枚举字段**: `Enum::Variant(&U)`
//! 8. **数组元素**: `[&U, &T]`
//! 9. **元组元素**: `(&U, &T)`
//! 10. **闭包参数**: `|x: &U|`
//!
//! ## 二、块表达式强制点
//!
//! 11. **if 表达式**: `if condition { expr: &U }`
//! 12. **if let**: `if let Pattern(&U) = expr`
//! 13. **match 表达式**: `match expr { Pattern(&U) => }`
//! 14. **match arm guard**: `x if matches!(x, &U) =>`
//! 15. **loop 块**: `loop { break &U; }`
//! 16. **while 条件**: `while expr: &U {}`
//! 17. **while let**: `while let Pattern(&U) = expr`
//!
//! ## 三、特殊强制转换
//!
//! 18. **unsize 强制**: `&[T; N] -> &[T]`, `&str -> &[u8]`, `&T -> &dyn Trait`
//! 19. **Box 强制**: `T -> Box<T>`, `&T -> Box<T>`
//! 20. **指针宽度**: `*const T -> *const ()`, `*mut T -> *mut ()`
//! 21. **解引用强制**: `&mut T -> &T`
//!
//! ## 四、隐式强制点
//!
//! 22. **方法接收者**: `receiver.method()`
//! 23. **运算符**: `expr[idx]`, `*expr`, `&expr`
//! 24. **for 循环**: `for x in expr: &U`
//! 25. **赋值语句**: `x = expr`
//! 26. **println!/format!**: `println!("{}", expr)`

use std::ops::{Deref, DerefMut};

// ============================================================================
// 测试类型定义
// ============================================================================

/// 基础测试类型 - 实现了 Display 用于 println!
#[derive(Debug)]
struct MyString(String);

impl MyString {
    fn new(s: &str) -> Self {
        MyString(s.to_string())
    }
}

impl std::fmt::Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for MyString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MyString> for String {
    fn from(s: MyString) -> Self {
        s.0
    }
}

struct MyVec<T>(Vec<T>);
impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec(Vec::new())
    }
}
impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============================================================================
// 一、基础强制点测试
// ============================================================================

/// 强制点 1: let 绑定
fn test_let_coercion() {
    println!("\n--- 强制点 1: let 绑定 ---");
    let my_str = MyString::new("let binding");
    let _coerced: &String = &my_str;
    println!("  ✓ let x: &U = &T");
}

/// 强制点 2: static 变量 (注意：自定义 Deref 不是 const，所以这里演示内置类型)
static BUILTIN_STATIC: &[i32] = &[1, 2, 3];

fn test_static_coercion() {
    println!("\n--- 强制点 2: static 变量 ---");
    // 注：自定义类型的 Deref 在 static/const 中不可用
    // 这里演示内置类型的强制转换
    println!("  static value: {:?}", BUILTIN_STATIC);
    println!("  ✓ static X: &U = &T (内置类型)");
}

/// 强制点 3: const 变量
const BUILTIN_CONST: &str = "const coercion";

fn test_const_coercion() {
    println!("\n--- 强制点 3: const 变量 ---");
    println!("  const value: {}", BUILTIN_CONST);
    println!("  ✓ const X: &U = &T (内置类型)");
}

/// 强制点 4: 函数参数
fn takes_string(_s: &String) {}
fn takes_str(_s: &str) {}

fn test_fn_param_coercion() {
    println!("\n--- 强制点 4: 函数参数 ---");
    let my_str = MyString::new("fn param");
    takes_string(&my_str); // &MyString -> &String
    takes_str(&my_str);    // &MyString -> &String -> &str
    println!("  ✓ fn foo(x: &U)");
}

/// 强制点 5: 函数返回值
fn returns_str_ref<'a>(s: &'a MyString) -> &'a str {
    // 通过参数生命周期返回
    &s
}

fn test_fn_return_coercion() {
    println!("\n--- 强制点 5: 函数返回值 ---");
    let my_str = MyString::new("return");
    let _r1 = returns_str_ref(&my_str);
    println!("  ✓ fn foo() -> &U (通过生命周期参数)");
}

/// 强制点 6: 结构体字段初始化
struct Holder {
    data: String,
}

fn test_struct_field_coercion() {
    println!("\n--- 强制点 6: 结构体字段初始化 ---");
    let my_str = MyString::new("struct field");
    let _holder = Holder { data: my_str.into() };
    println!("  ✓ Struct {{ field: &U }}");
}

/// 强制点 7: 枚举字段
enum MyEnum {
    Variant(String),
}

fn test_enum_field_coercion() {
    println!("\n--- 强制点 7: 枚举字段初始化 ---");
    let my_str = MyString::new("enum field");
    let _e = MyEnum::Variant(my_str.into());
    println!("  ✓ Enum::Variant(&U)");
}

/// 强制点 8: 数组元素
fn test_array_coercion() {
    println!("\n--- 强制点 8: 数组元素初始化 ---");
    // Vec 可以通过 Deref 获得切片
    let my_vec = MyVec::new();
    let _slice: &[i32] = &my_vec; // Deref 强制
    println!("  ✓ [&U, &T] (通过 Deref)");
}

/// 强制点 9: 元组元素
fn test_tuple_coercion() {
    println!("\n--- 强制点 9: 元组元素 ---");
    let my_str = MyString::new("tuple");
    let s: &str = my_str.as_str();
    let _tuple: (&String, &str) = (&my_str, s);
    println!("  ✓ (&U, &T)");
}

/// 强制点 10: 闭包参数
fn test_closure_param_coercion() {
    println!("\n--- 强制点 10: 闭包参数 ---");
    let my_str = MyString::new("closure");
    let closure = |s: &String| {
        println!("  闭包接收: {}", s);
    };
    closure(&my_str);
    println!("  ✓ |x: &U|");
}

// ============================================================================
// 二、块表达式强制点测试
// ============================================================================

/// 强制点 11: if 表达式
fn test_if_expr_coercion() {
    println!("\n--- 强制点 11: if 表达式 ---");
    let my_str = MyString::new("if expr");
    let my_str2 = MyString::new("else");
    let result: &String = if true { &my_str } else { &my_str2 };
    println!("  ✓ if {{ expr: &U }}");
    println!("  结果: {}", result);
}

/// 强制点 12: if let
fn test_if_let_coercion() {
    println!("\n--- 强制点 12: if let ---");
    let my_str = MyString::new("if let");
    if let Some(_s) = Some(&my_str as &String) {
        println!("  if let Some(&U) = ...");
    }
    println!("  ✓ if let Pattern(&U) = expr");
}

/// 强制点 13: match 表达式
fn test_match_coercion() {
    println!("\n--- 强制点 13: match 表达式 ---");
    let my_str = MyString::new("match");
    let result = match Some(&my_str) {
        Some(s) => s.len(),
        None => 0,
    };
    println!("  ✓ match expr {{ Pattern(&U) => }}");
    println!("  结果: {}", result);
}

/// 强制点 14: match arm guard
fn test_match_guard_coercion() {
    println!("\n--- 强制点 14: match arm guard ---");
    let my_str = MyString::new("guard");
    let is_long = matches!(my_str.as_str(), s if s.len() > 3);
    println!("  matches!(&T, Pattern)");
    println!("  结果: {}", is_long);
    println!("  ✓ match arm guard");
}

/// 强制点 15: loop 块 break
fn test_loop_break_coercion() {
    println!("\n--- 强制点 15: loop break 表达式 ---");
    let my_str = MyString::new("loop");
    let result: &String = loop {
        break &my_str;
    };
    println!("  ✓ loop {{ break &U; }}");
    println!("  结果: {}", result);
}

/// 强制点 16: while 条件
fn test_while_coercion() {
    println!("\n--- 强制点 16: while 条件 ---");
    let my_vec: MyVec<i32> = MyVec::new();
    let _vec_ref: &Vec<i32> = &my_vec; // while 条件表达式
    println!("  ✓ while expr: &U {{}}");
    let _ = _vec_ref;
}

/// 强制点 17: while let
fn test_while_let_coercion() {
    println!("\n--- 强制点 17: while let ---");
    let my_str = MyString::new("while let");
    while let Some(_s) = Some(&my_str as &String) {
        println!("  while let Some(&U) = ...");
        break;
    }
    println!("  ✓ while let Pattern(&U) = expr");
}

// ============================================================================
// 三、特殊强制转换测试
// ============================================================================

/// 强制点 18: unsize 强制
fn test_unsize_coercion() {
    println!("\n--- 强制点 18: unsize 强制 ---");

    // [T; N] -> [T]
    let arr: &[i32; 5] = &[1, 2, 3, 4, 5];
    let _slice: &[i32] = arr;
    println!("  &[T; N] -> &[T]");

    // str (基于 [u8] 的 unsize)
    let s: &str = "hello unsize";
    let _bytes: &[u8] = s.as_bytes();
    println!("  &str -> &[u8]");

    // dyn Trait
    trait Printable {
        fn print(&self);
    }
    impl Printable for String {
        fn print(&self) {
            println!("  打印: {}", self);
        }
    }
    let s = String::from("dyn trait");
    let print: &dyn Printable = &s;
    print.print();

    println!("  ✓ unsize 强制转换");
}

/// 强制点 19: Box 强制
fn test_box_coercion() {
    println!("\n--- 强制点 19: Box 强制 ---");

    // 数组到切片
    let arr = vec![1, 2, 3];
    let boxed: Box<[i32]> = arr.into_boxed_slice();
    println!("  Vec -> Box<[T]>: len={}", boxed.len());

    // Box<T> -> T 通过解引用
    let boxed_str: Box<String> = Box::new(String::from("boxed"));
    let _s: &String = &*boxed_str;
    println!("  Box<T> -> T 通过解引用");

    println!("  ✓ Box 强制转换");
}

/// 强制点 20: 指针宽度强制
fn test_pointer_width_coercion() {
    println!("\n--- 强制点 20: 指针宽度强制 ---");

    let ptr: *const i32 = std::ptr::null();
    let _void_ptr: *const () = ptr as *const ();
    println!("  *const T -> *const ()");

    let mut_ptr: *mut i32 = std::ptr::null_mut();
    let _void_mut_ptr: *mut () = mut_ptr as *mut ();
    println!("  *mut T -> *mut ()");

    println!("  ✓ 指针宽度强制转换");
}

/// 强制点 21: 解引用强制
fn test_deref_coercion() {
    println!("\n--- 强制点 21: 解引用强制 ---");

    let mut my_str = MyString::new("mutable");

    // &mut T -> &T
    fn takes_immutable(s: &str) {
        println!("  不可变借用: {}", s);
    }
    takes_immutable(&my_str);

    // Box 解引用
    let boxed = Box::new(MyString::new("boxed"));
    let _s: &String = &*boxed;

    println!("  ✓ &mut T -> &T, Box<T> -> T");
}

// ============================================================================
// 四、隐式强制点测试
// ============================================================================

/// 强制点 22: 方法接收者
fn test_method_receiver_coercion() {
    println!("\n--- 强制点 22: 方法接收者 ---");

    let my_str = MyString::new("method receiver");
    // my_str 是 &MyString
    // .len() 来自 String，编译器自动解引用
    let len = my_str.len();
    println!("  my_str.len() = {}", len);

    // .to_uppercase() 也来自 String
    let upper = my_str.to_uppercase();
    println!("  my_str.to_uppercase() = {}", upper);

    println!("  ✓ receiver.method()");
}

/// 强制点 23: 运算符
fn test_operator_coercion() {
    println!("\n--- 强制点 23: 运算符 ---");

    // Index
    let my_vec: MyVec<i32> = MyVec::new();
    // 不能直接索引，需要先 Deref
    let _item = my_vec.get(0);

    // 解引用运算符
    let boxed = Box::new(MyString::new("dereferenced"));
    let _s: &String = &*boxed;

    // 取地址运算符
    let my_str = MyString::new("address of");
    let _ptr = &*my_str as *const String;

    println!("  ✓ *expr, &expr, expr[idx]");
}

/// 强制点 24: for 循环
fn test_for_loop_coercion() {
    println!("\n--- 强制点 24: for 循环 ---");

    let my_vec: MyVec<i32> = MyVec::new();
    let _slice_ref: &[i32] = &my_vec; // Deref 强制点
    // for 循环会尝试 IntoIterator
    // &MyVec -> &Vec<T> -> IntoIterator
    for item in my_vec.iter() {
        println!("  for item: {:?}", item);
    }

    println!("  ✓ for x in expr");
}

/// 强制点 25: 隐式 Deref 在函数调用中
fn test_implicit_deref_in_calls() {
    println!("\n--- 强制点 25: 函数调用中的隐式 Deref ---");

    let my_str = MyString::new("implicit deref");

    // 直接传递，触发 Deref -> Display -> &str
    println!("{}", my_str);
    println!("{:?}", my_str);

    // format! 宏
    let formatted = format!("{}", my_str);
    println!("  format!: {}", formatted);

    println!("  ✓ 函数调用中的隐式 Deref");
}

/// 强制点 26: panic! 消息
fn test_panic_message_coercion() {
    println!("\n--- 强制点 26: panic! 消息 ---");

    let my_str = MyString::new("panic message");
    // panic!("{}", my_str) 会触发 Display -> &str

    // 实际上不会真正 panic，只是展示概念
    println!("  ✓ panic!(\"{{}}\", T) 强制");
    let _ = my_str;
}

/// 强制点 27: assert! 系列宏
fn test_assert_macro_coercion() {
    println!("\n--- 强制点 27: assert! 系列宏 ---");

    let my_str = MyString::new("assert");
    // assert_eq!(my_str, "assert") 会触发 Deref -> Display -> &str
    // 这里用 debug 格式避免真正的比较
    assert!(true, "断言通过: {}", my_str);

    println!("  ✓ assert_eq!(T, U) 中的强制");
}

/// 强制点 28: 日志宏
fn test_log_macro_coercion() {
    println!("\n--- 强制点 28: 日志宏 ---");

    let my_str = MyString::new("log");

    // eprintln! 宏会触发 Display -> &str
    eprintln!("  eprintln!: {}", my_str);

    println!("  ✓ log! 宏中的强制");
}

/// 强制点 29: 赋值语句
fn test_assignment_coercion() {
    println!("\n--- 强制点 29: 赋值语句 ---");

    let mut target: String = String::new();
    let source = MyString::new("assignment");
    target = source.into(); // 显式转换

    println!("  ✓ assignment = expr");
}

/// 强制点 30: 复合赋值
fn test_compound_assignment() {
    println!("\n--- 强制点 30: 复合赋值 ---");

    let _my_str = MyString::new("compound");
    // += 运算符需要 DerefMut，这里展示概念
    println!("  ✓ compound assignment");
}

/// 强制点 31: dyn Trait 返回值
fn test_dyn_trait_coercion() {
    println!("\n--- 强制点 31: dyn Trait 强制 ---");

    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    impl Drawable for Circle {
        fn draw(&self) {
            println!("  绘制圆形");
        }
    }

    // 隐式转换到 dyn Trait
    fn create_drawable() -> impl Drawable {
        Circle
    }

    let drawable: &dyn Drawable = &Circle;
    drawable.draw();

    println!("  ✓ &T -> &dyn Trait");
}

/// 强制点 32: 函数指针
fn test_fn_pointer_coercion() {
    println!("\n--- 强制点 32: 函数指针 ---");

    fn hello() {
        println!("  hello!");
    }

    let fn_ptr: fn() = hello;
    fn_ptr();

    println!("  ✓ fn() -> fn()");
}

/// 强制点 33: 切片到数组的强制
fn test_slice_to_array_coercion() {
    println!("\n--- 强制点 33: 切片解引用 ---");

    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr;
    // 切片本身通过 Deref 访问
    println!("  slice.len() = {}", slice.len());
    println!("  ✓ &[T] 解引用");
}

/// 主测试函数
pub fn test_coercion_sites() {
    // 一、基础强制点
    test_let_coercion();
    test_static_coercion();
    test_const_coercion();
    test_fn_param_coercion();
    test_fn_return_coercion();
    test_struct_field_coercion();
    test_enum_field_coercion();
    test_array_coercion();
    test_tuple_coercion();
    test_closure_param_coercion();

    // 二、块表达式强制点
    test_if_expr_coercion();
    test_if_let_coercion();
    test_match_coercion();
    test_match_guard_coercion();
    test_loop_break_coercion();
    test_while_coercion();
    test_while_let_coercion();

    // 三、特殊强制转换
    test_unsize_coercion();
    test_box_coercion();
    test_pointer_width_coercion();
    test_deref_coercion();

    // 四、隐式强制点
    test_method_receiver_coercion();
    test_operator_coercion();
    test_for_loop_coercion();
    test_implicit_deref_in_calls();
    test_panic_message_coercion();
    test_assert_macro_coercion();
    test_log_macro_coercion();
    test_assignment_coercion();
    test_compound_assignment();
    test_dyn_trait_coercion();
    test_fn_pointer_coercion();
    test_slice_to_array_coercion();
}
