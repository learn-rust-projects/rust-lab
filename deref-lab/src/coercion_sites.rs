//! # Rust 所有类型强制点 (All Coercion Sites)
//!
//! ## 核心概念：自动解引用 (Auto Deref)
//!
//! 当类型 `T` 实现 `Deref<Target=U>` 时，编译器会在强制点自动执行 `&T` → `&U` 的转换。
//!
//! ## 连续多层解引用
//!
//! Rust 编译器会自动沿着 Deref 链进行多层解引用，直到达到目标类型。
//!
//! ## 强制点分类
//!
//! ### A. Deref 强制点 (14 个)
//! ### B. Unsized 强制点 (6 个)
//! ### C. 指针强制点 (4 个)
//! ### D. 隐式强制点 (7 个)

use std::ops::{Deref, DerefMut};

// ============================================================================
// 多层嵌套测试类型 - 演示连续自动解引用
// ============================================================================

/// Level 1: 基础类型
#[derive(Debug)]
struct L1(String);
impl L1 {
    fn new(s: &str) -> Self {
        L1(s.to_string())
    }
}
impl Deref for L1 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Level 2: 包装 L1
#[derive(Debug)]
struct L2(L1);
impl L2 {
    fn new(s: &str) -> Self {
        L2(L1::new(s))
    }
}
impl Deref for L2 {
    type Target = L1;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Level 3: 包装 L2
#[derive(Debug)]
struct L3(L2);
impl L3 {
    fn new(s: &str) -> Self {
        L3(L2::new(s))
    }
}
impl Deref for L3 {
    type Target = L2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Level 4: 包装 L3 (4 层嵌套)
#[derive(Debug)]
struct L4(L3);
impl L4 {
    fn new(s: &str) -> Self {
        L4(L3::new(s))
    }
}
impl Deref for L4 {
    type Target = L3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============================================================================
// A. Deref 强制点测试
// ============================================================================

/// 强制点 1: let 绑定 - 自动解引用
fn test_let_coercion() {
    println!("\n--- 强制点 1: let 绑定 (自动解引用) ---");

    // L4 -> L3 -> L2 -> L1 -> String -> str (6 层解引用)
    let l4 = L4::new("single layer");
    let _: &str = &l4;

    // 多层引用: &L4 -> ... -> str
    let l3 = L3::new("L3");
    let _: &str = &l3;

    println!("  ✓ let _: &str = &L4");
}

/// 强制点 2: static 变量 - 内置类型的自动解引用
fn test_static_coercion() {
    println!("\n--- 强制点 2: static 变量 ---");

    // 内置类型可以
    static ARR: &[i32; 3] = &[1, 2, 3];

    // static Box
    static BOX: std::sync::LazyLock<Box<&str>> = std::sync::LazyLock::new(|| Box::new("lazy"));

    println!("  static &[i32; 3]: {:?}", ARR);
    println!("  static Box<&str>: {}", *BOX);

    println!("  ✓ static 强制转换");
}

/// 强制点 3: const 变量 - 内置类型的自动解引用
fn test_const_coercion() {
    println!("\n--- 强制点 3: const 变量 ---");

    const CONST_STR: &str = "const str";
    println!("  const &str: {}", CONST_STR);

    // const Vec 解引用为切片
    const CONST_VEC: &[i32] = &[1, 2, 3];
    println!("  const &[i32]: {:?}", CONST_VEC);

    println!("  ✓ const 强制转换");
}

/// 强制点 4: 函数参数 - 自动解引用 + 连续多层
fn test_fn_param_coercion() {
    println!("\n--- 强制点 4: 函数参数 (连续自动解引用) ---");

    fn accepts_str(_: &str) {}
    fn accepts_string(_: &String) {}

    // L4: 4 层解引用
    let l4 = L4::new("L4 to str");
    accepts_str(&l4);         // L4 -> L3 -> L2 -> L1 -> String -> str
    accepts_string(&l4);      // L4 -> L3 -> L2 -> L1 -> String

    // 多层引用: &&&&L4 -> &str
    accepts_str(&l4);         // 一层引用，自动解引用
    accepts_str(&&l4);        // 两层引用
    accepts_str(&&&l4);       // 三层引用

    // Box 多层
    let box_l4 = Box::new(L4::new("boxed L4"));
    accepts_str(&box_l4);     // Box<L4> -> L4 -> ... -> str

    let box_box_l4 = Box::new(Box::new(L4::new("boxed box L4")));
    accepts_str(&box_box_l4); // Box<Box<L4>> -> Box<L4> -> L4 -> ... -> str

    println!("  ✓ fn(&str) 参数: L4 -> L3 -> L2 -> L1 -> String -> str");
}

/// 强制点 5: 函数返回值 - 自动解引用
fn test_fn_return_coercion() {
    println!("\n--- 强制点 5: 函数返回值 ---");

    fn returns_l1() -> L1 {
        L1::new("returned")
    }

    // 返回 L1，它 Deref 到 String
    let l1 = returns_l1();
    let _: &String = &l1; // L1 -> String

    println!("  ✓ 返回值自动解引用");
}

/// 强制点 6: 结构体字段 - 自动解引用
struct Container<'a> {
    data: &'a str,
}

fn test_struct_field_coercion() {
    println!("\n--- 强制点 6: 结构体字段 (连续解引用) ---");

    // L4 自动解引用为 &str
    let l4 = L4::new("struct field");
    let _c = Container { data: &l4 }; // L4 -> ... -> str

    // 多层引用自动解引用
    let l3 = L3::new("&&l3");
    let _c2 = Container { data: &&l3 };

    println!("  ✓ Struct {{ data: &str }} 自动解引用");
}

/// 强制点 7: 枚举字段 - 自动解引用
enum MyOption<'a> {
    Some(&'a str),
    None,
}

fn test_enum_field_coercion() {
    println!("\n--- 强制点 7: 枚举字段 (连续解引用) ---");

    let l4 = L4::new("enum field");
    let _e = MyOption::Some(&l4); // L4 -> ... -> str

    // Box 嵌套
    let boxed = Box::new(L4::new("boxed"));
    let _e2 = MyOption::Some(&boxed); // Box<L4> -> L4 -> ... -> str

    println!("  ✓ Enum::Variant(&str) 自动解引用");
}

/// 强制点 8: 元组元素 - 自动解引用
fn test_tuple_coercion() {
    println!("\n--- 强制点 8: 元组元素 (连续解引用) ---");

    let l4 = L4::new("tuple element");

    // 元组中每个元素独立解引用
    let t: (&str, &str) = (&l4, &l4);
    println!("  元组解引用: ({}, {})", t.0, t.1);

    // 混合类型
    let mixed: (&str, &String) = (&l4, &l4);
    println!("  混合类型: ({}, {})", mixed.0, mixed.1);
}

/// 强制点 9: 闭包参数 - 自动解引用
fn test_closure_coercion() {
    println!("\n--- 强制点 9: 闭包参数 (连续自动解引用) ---");

    let l4 = L4::new("closure param");

    // 闭包参数也是强制点
    let f = |s: &str| println!("  闭包: {}", s);
    f(&l4);

    // 多层引用
    f(&&l4);
    f(&&&l4);

    println!("  ✓ |x: &str| 闭包参数自动解引用");
}

/// 强制点 10: if 表达式 - 自动解引用
fn test_if_coercion() {
    println!("\n--- 强制点 10: if 表达式 (自动解引用) ---");

    let l4 = L4::new("if expr");
    let l4_else = L4::new("else");

    // if 表达式的分支结果自动解引用
    let result: &str = if true { &l4 } else { &l4_else };
    println!("  ✓ if {{ &str }} 自动解引用");
    println!("  结果: {}", result);
}

/// 强制点 11: match arm - 自动解引用
fn test_match_coercion() {
    println!("\n--- 强制点 11: match arm (连续自动解引用) ---");

    let l4 = L4::new("match arm");

    // match arm 结果自动解引用
    let result: &str = match Some(&l4) {
        Some(s) => s, // 自动解引用
        None => "none",
    };

    println!("  ✓ match {{ Some(s) => s }} 自动解引用");
    println!("  结果: {}", result);
}

/// 强制点 12: break 表达式 - 自动解引用
fn test_break_coercion() {
    println!("\n--- 强制点 12: break 表达式 (自动解引用) ---");

    let l4 = L4::new("break expr");

    // loop break 自动解引用
    let result: &str = loop {
        break &l4;
    };

    println!("  ✓ loop {{ break &str }} 自动解引用");
    println!("  结果: {}", result);
}

/// 强制点 13: 赋值 RHS - 自动解引用
fn test_assignment_coercion() {
    println!("\n--- 强制点 13: 赋值 RHS (自动解引用) ---");

    let l4 = L4::new("assignment");

    // 赋值自动解引用
    let target: &str;
    target = &l4;

    println!("  ✓ target = &L4 自动解引用");
    println!("  target: {}", target);
}

/// 强制点 14: return 语句 - 自动解引用
fn returns_l4_ref<'a>(l4: &'a L4) -> &'a str {
    // 传递引用，返回解引用后的 &str
    l4 // 编译器自动解引用: &L4 -> &L3 -> &L2 -> &L1 -> &String -> &str
}

fn test_return_coercion() {
    println!("\n--- 强制点 14: return 语句 (自动解引用) ---");

    let l4 = L4::new("return");
    let result = returns_l4_ref(&l4);
    println!("  ✓ return &L4 自动解引用");
    println!("  结果: {}", result);
}

// ============================================================================
// B. Unsized 强制点测试
// ============================================================================

/// 强制点 15: 数组到切片 - 隐式 unsize
fn test_array_to_slice() {
    println!("\n--- 强制点 15: 数组到切片 (unsize) ---");

    // [T; N] -> &[T]
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr; // 自动 unsize

    // 函数参数中
    fn takes_slice(s: &[i32]) {
        println!("  切片: {:?}", s);
    }
    takes_slice(&arr);

    println!("  ✓ &[T; N] -> &[T] 自动 unsize");
    let _ = slice;
}

/// 强制点 16: str 到 [u8] - unsize
fn test_str_to_bytes() {
    println!("\n--- 强制点 16: str 到 [u8] (unsize) ---");

    let s: &str = "hello";
    let bytes: &[u8] = s.as_bytes();

    println!("  ✓ &str -> &[u8] unsize: {:?}", bytes);
}

/// 强制点 17: T 到 dyn Trait - unsize
fn test_trait_object() {
    println!("\n--- 强制点 17: T 到 dyn Trait (unsize) ---");

    trait Printable {
        fn print(&self);
    }

    struct Foo(String);
    impl Printable for Foo {
        fn print(&self) {
            println!("  Printable: {}", self.0);
        }
    }

    // 隐式转换为 dyn Trait
    let foo = Foo(String::from("printable"));
    let print: &dyn Printable = &foo;
    print.print();

    println!("  ✓ &T -> &dyn Trait 自动 unsize");
}

/// 强制点 18: impl Trait 返回值 - unsize
fn test_impl_trait() {
    println!("\n--- 强制点 18: impl Trait (unsize) ---");

    fn make_iterator() -> impl Iterator<Item = i32> {
        vec![1, 2, 3].into_iter()
    }

    let sum: i32 = make_iterator().sum();
    println!("  ✓ impl Trait 返回值: sum={}", sum);
}

/// 强制点 19: 闭包到函数指针 - unsize
fn test_closure_to_fn_ptr() {
    println!("\n--- 强制点 19: 闭包到函数指针 (unsize) ---");

    // 无捕获的闭包可以转为函数指针
    let fn_ptr: fn() = || println!("  fn pointer");
    fn_ptr();

    println!("  ✓ || -> fn() 自动 unsize");
}

/// 强制点 20: 结构体到 DST - unsize
fn test_struct_to_dst() {
    println!("\n--- 强制点 20: 结构体到 DST (unsize) ---");

    // Vec<T> 解引用为 [T] (DST)
    let v: Vec<i32> = vec![1, 2, 3];

    fn takes_slice(s: &[i32]) {
        println!("  &[i32]: {:?}", s);
    }
    takes_slice(&v);

    // Box<[T]> 是 DST
    let boxed: Box<[i32]> = v.into_boxed_slice();
    println!("  Box<[i32]>: {:?}", boxed);

    println!("  ✓ Vec<T> -> [T] DST unsize");
}

// ============================================================================
// C. 指针强制点测试
// ============================================================================

/// 强制点 21-24: 指针强制
fn test_pointer_coercion() {
    println!("\n--- 强制点 21-24: 指针强制 ---");

    let ptr: *const i32 = std::ptr::null();
    let _void_ptr: *const () = ptr as *const ();
    println!("  ✓ *const T -> *const ()");

    let mut_ptr: *mut i32 = std::ptr::null_mut();
    let _void_mut_ptr: *mut () = mut_ptr as *mut ();
    println!("  ✓ *mut T -> *mut ()");

    // &T -> *const T
    let r: &i32 = &42;
    let _raw: *const i32 = r as *const i32;
    println!("  ✓ &T -> *const T");

    // &mut T -> *mut T
    let mut m: i32 = 42;
    let _raw_mut: *mut i32 = &mut m as *mut i32;
    println!("  ✓ &mut T -> *mut T");
}

// ============================================================================
// D. 隐式强制点测试
// ============================================================================

/// 强制点 25: 方法接收者 - 自动解引用查找方法
fn test_method_receiver() {
    println!("\n--- 强制点 25: 方法接收者 (连续自动解引用) ---");

    let l4 = L4::new("method receiver");

    // 方法解析会沿着 Deref 链查找
    let len = l4.len(); // L4 -> ... -> String -> len()
    let upper = l4.to_uppercase(); // L4 -> ... -> to_uppercase()

    // 多层引用
    let len2 = (&&l4).len();
    let len3 = (&&&l4).len();

    // Box
    let boxed = Box::new(L4::new("boxed"));
    let len4 = boxed.len();

    println!("  ✓ receiver.method() 自动解引用查找");
    println!("  l4.len()={}, upper={}", len, upper);
    println!("  (&&l4).len()={}, (&&&l4).len()={}", len2, len3);
    println!("  boxed.len()={}", len4);
}

/// 强制点 26: 索引 - 自动解引用
fn test_index_coercion() {
    println!("\n--- 强制点 26: 索引 (自动解引用) ---");

    let boxed_vec = Box::new(vec![1, 2, 3, 4, 5]);

    // Box<Vec<T>> -> Vec<T> -> &[T] -> Index
    println!("  boxed_vec[0] = {}", boxed_vec[0]);
    println!("  boxed_vec[2] = {}", boxed_vec[2]);

    // 多层 Box
    let double_boxed = Box::new(Box::new(vec![10, 20]));
    println!("  double[0] = {}", double_boxed[0]);

    println!("  ✓ expr[idx] 自动解引用");
}

/// 强制点 27: 解引用运算符 - 显式自动解引用
fn test_deref_operator() {
    println!("\n--- 强制点 27: 解引用运算符 (自动解引用) ---");

    let boxed = Box::new(L4::new("dereferenced"));

    // *boxed -> L4
    // **boxed -> L3
    // ***boxed -> L2
    // ****boxed -> L1
    // *****boxed -> String
    // ******boxed -> str
    let s: &str = &******boxed;

    // 多层引用
    let boxed_ref = &&boxed;
    let _ = *******boxed_ref;

    println!("  ✓ *expr 自动解引用");
}

/// 强制点 28: 取地址运算符 - 自动解引用后取地址
fn test_addressof_coercion() {
    println!("\n--- 强制点 28: 取地址 & (自动解引用) ---");

    let l4 = L4::new("address of");

    fn takes_l4(_: &L4) {}
    fn takes_l3(_: &L3) {}
    fn takes_l2(_: &L2) {}
    fn takes_l1(_: &L1) {}

    takes_l4(&l4);
    takes_l3(&l4); // 自动解引用: L4 -> L3
    takes_l2(&l4); // 自动解引用: L4 -> L3 -> L2
    takes_l1(&l4); // 自动解引用: L4 -> L3 -> L2 -> L1

    println!("  ✓ &expr 自动解引用到不同层级");
}

/// 强制点 29: for 循环 - IntoIterator + 自动解引用
fn test_for_loop_coercion() {
    println!("\n--- 强制点 29: for 循环 (自动解引用) ---");

    // Vec<T> 实现 IntoIterator
    let vec = vec![1, 2, 3];
    print!("  for item in Vec<T>: ");
    for item in vec {
        print!("{} ", item);
    }
    println!();

    // Box<[T]> 实现 IntoIterator
    let boxed_slice: Box<[i32]> = vec![4, 5, 6].into_boxed_slice();
    print!("  for item in Box<[T]>: ");
    for item in boxed_slice {
        print!("{} ", item);
    }
    println!();

    println!("  ✓ for x in expr 自动解引用 + IntoIterator");
}

/// 强制点 30: 格式化宏 - Display trait 自动解引用
fn test_format_coercion() {
    println!("\n--- 强制点 30: 格式化宏 (自动解引用) ---");

    // 使用实现了 Display 的类型
    let s = String::from("format");

    println!("  println!: {}", s);
    println!("  format!: {}", format!("{}", s));
    eprintln!("  eprintln!: {}", s);

    // 多层引用
    let boxed = Box::new(String::from("boxed"));
    println!("  Box<String>: {}", boxed);

    println!("  ✓ println!() 自动解引用");
}

/// 强制点 31: 闭包捕获 - 自动解引用捕获
fn test_closure_capture() {
    println!("\n--- 强制点 31: 闭包捕获 (自动解引用) ---");

    let l4 = L4::new("closure capture");

    // 闭包可以捕获解引用后的值
    let closure = || {
        // 这里使用 l4，闭包会通过 Deref 访问
        println!("  闭包中使用: {}", l4.len());
    };
    closure();

    println!("  ✓ 闭包捕获时自动解引用");
}

// ============================================================================
// 主测试函数
// ============================================================================

pub fn test_coercion_sites() {
    println!("\n=== A. Deref 强制点 (14 个) ===");

    test_let_coercion();         // 1
    test_static_coercion();      // 2
    test_const_coercion();       // 3
    test_fn_param_coercion();    // 4
    test_fn_return_coercion();   // 5
    test_struct_field_coercion();// 6
    test_enum_field_coercion();  // 7
    test_tuple_coercion();       // 8
    test_closure_coercion();     // 9
    test_if_coercion();          // 10
    test_match_coercion();       // 11
    test_break_coercion();       // 12
    test_assignment_coercion();  // 13
    test_return_coercion();      // 14

    println!("\n=== B. Unsized 强制点 (6 个) ===");

    test_array_to_slice();       // 15
    test_str_to_bytes();         // 16
    test_trait_object();        // 17
    test_impl_trait();           // 18
    test_closure_to_fn_ptr();    // 19
    test_struct_to_dst();        // 20

    println!("\n=== C. 指针强制点 (4 个) ===");

    test_pointer_coercion();     // 21-24

    println!("\n=== D. 隐式强制点 (7 个) ===");

    test_method_receiver();      // 25
    test_index_coercion();       // 26
    test_deref_operator();      // 27
    test_addressof_coercion();  // 28
    test_for_loop_coercion();    // 29
    test_format_coercion();      // 30
    test_closure_capture();      // 31

    println!("\n=== 强制点测试完成: 共 31 个 ===");
}
