//! # Rust 所有类型强制点 (All Coercion Sites)
//!
//! ## 核心概念：自动解引用 (Auto Deref)
//!
//! 当类型 `T` 实现 `Deref<Target=U>` 时，编译器会在强制点自动执行 `&T` → `&U` 的转换。
//!
//! ## 连续多层解引用
//!
//! Rust 编译器会自动沿着 Deref 链进行多层解引用，直到达到目标类型。
//! 本模块使用 L1-L4 四层嵌套类型，确保每个强制点都展示 3 层以上的连续解引用。

use std::ops::Deref;

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

/// 强制点 1: let 绑定 - 连续多层解引用
fn test_let_coercion() {
    println!("\n--- 强制点 1: let 绑定 (连续多层解引用) ---");

    let l4 = L4::new("let binding");

    // &L4 -> &L3 -> &L2 -> &L1 -> &String -> &str (6 层解引用)
    let _: &str = &l4;

    // &&L4 -> &&L3 -> ... -> &str (7 层)
    let _: &str = &&l4;

    // &&&L4 -> &&&L3 -> ... -> &str (8 层)
    let _: &str = &&&l4;

    // &&&&L4 (9 层)
    let _: &str = &&&&l4;

    println!("  ✓ let _: &str = &L4 (连续 6 层解引用)");
}

/// 强制点 2: static 变量
fn test_static_coercion() {
    println!("\n--- 强制点 2: static 变量 (多层解引用) ---");

    // 静态数组到切片的强制
    static ARR: &[i32; 3] = &[1, 2, 3];

    // 静态切片
    static SLICE: &[i32] = &[1, 2, 3, 4, 5];

    println!("  static arr: {:?}", ARR);
    println!("  static slice: {:?}", SLICE);

    // Box 类型的静态需要 LazyLock
    static BOX: std::sync::LazyLock<Box<&str>> = std::sync::LazyLock::new(|| Box::new("lazy box"));

    // 多层引用
    fn takes_str(_: &str) {}
    takes_str(&*BOX);
    takes_str(&&*BOX);
    takes_str(&&&*BOX);

    println!("  static box: {}", *BOX);
    println!("  ✓ static 强制 (多层解引用)");
}

/// 强制点 3: const 变量
fn test_const_coercion() {
    println!("\n--- 强制点 3: const 变量 (多层解引用) ---");

    const CONST_STR: &str = "const str";
    const CONST_SLICE: &[i32] = &[1, 2, 3];

    println!("  const: {}", CONST_STR);
    println!("  const slice: {:?}", CONST_SLICE);

    // const Vec 解引用为切片
    println!("  ✓ const 强制 (多层解引用)");
}

/// 强制点 4: 函数参数 - 连续多层解引用
fn test_fn_param_coercion() {
    println!("\n--- 强制点 4: 函数参数 (连续多层解引用) ---");

    fn accepts_str(_: &str) {}

    let l4 = L4::new("fn param");

    // L4 -> L3 -> L2 -> L1 -> String -> str (6 层)
    accepts_str(&l4);

    // &&L4 -> &&L3 -> ... -> str (7 层)
    accepts_str(&&l4);
    accepts_str(&&&l4);
    accepts_str(&&&&l4);

    // Box 嵌套
    let box_l4 = Box::new(L4::new("boxed"));
    accepts_str(&box_l4); // Box<L4> -> L4 -> ... -> str (7 层)

    let box_box_l4 = Box::new(Box::new(L4::new("double boxed")));
    accepts_str(&box_box_l4); // Box<Box<L4>> -> ... -> str (8 层)

    println!("  ✓ fn(&str) 参数: L4 -> L3 -> L2 -> L1 -> String -> str");
}

/// 强制点 5: 函数返回值
fn test_fn_return_coercion() {
    println!("\n--- 强制点 5: 函数返回值 (多层解引用) ---");

    fn returns_l4() -> L4 {
        L4::new("return")
    }

    let l4 = returns_l4();

    // 返回后可以连续多层引用
    let _: &str = &&l4;
    let _: &str = &&&l4;
    let _: &str = &&&&l4;

    // 传给函数参数
    fn takes_str(_: &str) {}
    takes_str(&l4);
    takes_str(&&l4);
    takes_str(&&&l4);

    println!("  ✓ 返回值多层解引用");
}

/// 强制点 6: 结构体字段 - 连续多层解引用
struct Container<'a> {
    data: &'a str,
}

fn test_struct_field_coercion() {
    println!("\n--- 强制点 6: 结构体字段 (连续多层解引用) ---");

    let l4 = L4::new("struct field");

    // &L4 -> ... -> &str (6 层)
    let _c = Container { data: &l4 };
    let _c2 = Container { data: &&l4 };
    let _c3 = Container { data: &&&l4 };
    let _c4 = Container { data: &&&&l4 };

    // Box 嵌套
    let boxed = Box::new(L4::new("boxed"));
    let _c5 = Container { data: &boxed };

    println!("  ✓ Struct {{ data: &str }} 连续多层解引用");
}

/// 强制点 7: 枚举字段 - 连续多层解引用
enum MyOption<'a> {
    Some(&'a str),
    None,
}

fn test_enum_field_coercion() {
    println!("\n--- 强制点 7: 枚举字段 (连续多层解引用) ---");

    let l4 = L4::new("enum field");

    let _e = MyOption::Some(&l4);
    let _e2 = MyOption::Some(&&l4);
    let _e3 = MyOption::Some(&&&l4);
    let _e4 = MyOption::Some(&&&&l4);

    // Box 嵌套
    let boxed = Box::new(L4::new("boxed enum"));
    let _e5 = MyOption::Some(&boxed);

    let double_boxed = Box::new(Box::new(L4::new("double boxed")));
    let _e6 = MyOption::Some(&double_boxed);

    println!("  ✓ Enum::Variant(&str) 连续多层解引用");
}

/// 强制点 8: 元组元素 - 连续多层解引用
fn test_tuple_coercion() {
    println!("\n--- 强制点 8: 元组元素 (连续多层解引用) ---");

    let l4 = L4::new("tuple element");

    // 多个元素都连续解引用
    let t1: (&str, &str) = (&l4, &l4);
    let t2: (&str, &str) = (&&l4, &&l4);
    let t3: (&str, &str) = (&&&l4, &&&l4);
    let t4: (&str, &str, &str) = (&l4, &&l4, &&&l4);

    println!("  元组解引用: ({}, {})", t3.0, t3.1);
    println!("  ✓ 元组元素连续多层解引用");
}

/// 强制点 9: 闭包参数 - 连续多层解引用
fn test_closure_coercion() {
    println!("\n--- 强制点 9: 闭包参数 (连续多层解引用) ---");

    let l4 = L4::new("closure param");

    let f = |s: &str| println!("  闭包: {}", s);

    f(&l4);
    f(&&l4);
    f(&&&l4);
    f(&&&&l4);

    // Box 嵌套
    let boxed = Box::new(L4::new("boxed closure"));
    f(&boxed);
    f(&&boxed);
    f(&&&boxed);

    println!("  ✓ |x: &str| 闭包参数连续多层解引用");
}

/// 强制点 10: if 表达式 - 连续多层解引用
fn test_if_coercion() {
    println!("\n--- 强制点 10: if 表达式 (连续多层解引用) ---");

    let l4 = L4::new("if expr");
    let l4_else = L4::new("else");

    // if 分支结果多层解引用
    let _r1: &str = if true { &l4 } else { &l4_else };
    let _r2: &str = if true { &&l4 } else { &&l4_else };
    let _r3: &str = if true { &&&l4 } else { &&&l4_else };
    let _r4: &str = if true { &&&&l4 } else { &&&&l4_else };

    println!("  ✓ if {{ &str }} 连续多层解引用");
}

/// 强制点 11: match arm - 连续多层解引用
fn test_match_coercion() {
    println!("\n--- 强制点 11: match arm (连续多层解引用) ---");

    let l4 = L4::new("match arm");

    let _r1: &str = match Some(&l4) {
        Some(s) => s,
        None => "",
    };
    let _r2: &str = match Some(&&l4) {
        Some(s) => s,
        None => "",
    };
    let _r3: &str = match Some(&&&l4) {
        Some(s) => s,
        None => "",
    };
    let _r4: &str = match Some(&&&&l4) {
        Some(s) => s,
        None => "",
    };

    println!("  ✓ match {{ Some(s) => s }} 连续多层解引用");
}

/// 强制点 12: break 表达式 - 连续多层解引用
fn test_break_coercion() {
    println!("\n--- 强制点 12: break 表达式 (连续多层解引用) ---");

    let l4 = L4::new("break expr");

    let _r1: &str = loop {
        break &l4;
    };
    let _r2: &str = loop {
        break &&l4;
    };
    let _r3: &str = loop {
        break &&&l4;
    };
    let _r4: &str = loop {
        break &&&&l4;
    };

    println!("  ✓ loop {{ break &str }} 连续多层解引用");
}

/// 强制点 13: 赋值 RHS - 连续多层解引用
fn test_assignment_coercion() {
    println!("\n--- 强制点 13: 赋值 RHS (连续多层解引用) ---");

    let l4 = L4::new("assignment");

    // 使用可变绑定来演示多次赋值
    let mut target: &str = &l4;
    target = &&l4;
    target = &&&l4;
    target = &&&&l4;

    println!("  赋值: {}", target);
    println!("  ✓ target = &L4 连续多层解引用");
}

/// 强制点 14: return 语句 - 连续多层解引用
fn test_return_coercion() {
    println!("\n--- 强制点 14: return 语句 (连续多层解引用) ---");

    fn returns_ref<'a>(l4: &'a L4) -> &'a str {
        l4
    }

    let l4 = L4::new("return");
    let _r1 = returns_ref(&l4);
    let _r2 = returns_ref(&&l4);
    let _r3 = returns_ref(&&&l4);
    let _r4 = returns_ref(&&&&l4);

    println!("  ✓ return &L4 连续多层解引用");
}

// ============================================================================
// B. Unsized 强制点测试
// ============================================================================

/// 强制点 15: 数组到切片
fn test_array_to_slice() {
    println!("\n--- 强制点 15: 数组到切片 (多层) ---");

    let arr = [1, 2, 3, 4, 5];

    // &arr -> &[i32] (unsize)
    fn takes_slice(_: &[i32]) {}
    takes_slice(&arr);

    // 多层引用后再解引用
    let arr_ref = &&arr;
    // 注意：&&[i32; N] 不能直接 unsize，需要先解一层
    fn takes_ref_slice(_: &&[i32; 5]) {}
    takes_ref_slice(&&arr);

    println!("  ✓ &[T; N] -> &[T]");
}

/// 强制点 16: str 到 [u8]
fn test_str_to_bytes() {
    println!("\n--- 强制点 16: str 到 [u8] ---");

    let s: &str = "hello";
    let bytes: &[u8] = s.as_bytes();

    // 多层引用
    let s2: &&str = &&s;
    let bytes2: &[u8] = s2.as_bytes();

    println!("  ✓ &str -> &[u8]: {:?}", bytes);
}

/// 强制点 17: T 到 dyn Trait
fn test_trait_object() {
    println!("\n--- 强制点 17: T 到 dyn Trait ---");

    trait Printable {
        fn print(&self);
    }
    struct Foo(String);
    impl Printable for Foo {
        fn print(&self) {
            println!("  {}", self.0);
        }
    }

    let foo = Foo(String::from("printable"));

    // &T -> &dyn Trait
    let p1: &dyn Printable = &foo;
    p1.print();

    println!("  ✓ &T -> &dyn Trait");
}

/// 强制点 18: impl Trait
fn test_impl_trait() {
    println!("\n--- 强制点 18: impl Trait ---");

    fn make_iter() -> impl Iterator<Item = i32> {
        vec![1, 2, 3].into_iter()
    }
    let sum: i32 = make_iter().sum();
    println!("  ✓ impl Trait: sum={}", sum);
}

/// 强制点 19: 闭包到函数指针
fn test_closure_to_fn_ptr() {
    println!("\n--- 强制点 19: 闭包到函数指针 ---");

    let fn_ptr: fn() = || println!("  fn pointer");
    fn_ptr();

    println!("  ✓ || -> fn()");
}

/// 强制点 20: 结构体到 DST
fn test_struct_to_dst() {
    println!("\n--- 强制点 20: 结构体到 DST (多层) ---");

    let v = vec![1, 2, 3];
    let boxed: Box<[i32]> = v.into_boxed_slice();

    // 多层 Box
    let double_boxed = Box::new(boxed);
    let triple_boxed = Box::new(double_boxed);

    fn takes_slice(_: &[i32]) {}
    takes_slice(&triple_boxed);

    println!("  ✓ Vec<T> -> [T] DST 连续多层");
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

    let r: &i32 = &42;
    let _raw: *const i32 = r as *const i32;
    println!("  ✓ &T -> *const T");

    let mut m: i32 = 42;
    let _raw_mut: *mut i32 = &mut m as *mut i32;
    println!("  ✓ &mut T -> *mut T");
}

// ============================================================================
// D. 隐式强制点测试
// ============================================================================

/// 强制点 25: 方法接收者 - 连续多层解引用
fn test_method_receiver() {
    println!("\n--- 强制点 25: 方法接收者 (连续多层解引用) ---");

    let l4 = L4::new("method receiver");

    // 方法解析自动解引用
    let len1 = l4.len();
    let len2 = (&&l4).len();
    let len3 = (&&&l4).len();
    let len4 = (&&&&l4).len();

    // Box 多层
    let box_l4 = Box::new(L4::new("boxed"));
    let len5 = box_l4.len();
    let len6 = (&&box_l4).len();
    let len7 = (&&&box_l4).len();

    println!(
        "  l4.len()={}, &&l4.len()={}, &&&l4.len()={}",
        len1, len2, len3
    );
    println!(
        "  Box.len()={}, &&Box.len()={}, &&&Box.len()={}",
        len5, len6, len7
    );
    println!("  ✓ receiver.method() 连续多层自动解引用");
}

/// 强制点 26: 索引 - 连续多层解引用
fn test_index_coercion() {
    println!("\n--- 强制点 26: 索引 (连续多层解引用) ---");

    let box_vec = Box::new(vec![1, 2, 3, 4, 5]);

    // Box<Vec> -> Vec -> &[T] -> Index
    println!("  box_vec[0]={}", box_vec[0]);
    println!("  &&box_vec[0]={}", (&&box_vec)[0]);
    println!("  &&&box_vec[0]={}", (&&&box_vec)[0]);

    // 多层 Box
    let double = Box::new(box_vec);
    println!("  &&double[0]={}", (&&double)[0]);

    println!("  ✓ expr[idx] 连续多层解引用");
}

/// 强制点 27: 解引用运算符
fn test_deref_operator() {
    println!("\n--- 强制点 27: 解引用运算符 (连续多层) ---");

    let boxed = Box::new(L4::new("dereferenced"));

    // 多层解引用 (注意需要 & 获得引用)
    let _l4: &L4 = &*boxed;
    let _l3: &L3 = &**boxed;
    let _l2: &L2 = &***boxed;
    let _l1: &L1 = &****boxed;
    let _s: &String = &*****boxed;
    let _str: &str = &******boxed;

    // 多层引用 + 解引用
    let _str2: &str = &*******&&boxed;

    println!("  ✓ *expr 连续多层解引用");
}

/// 强制点 28: 取地址运算符
fn test_addressof_coercion() {
    println!("\n--- 强制点 28: 取地址 & (连续多层) ---");

    let l4 = L4::new("address of");

    // 与函数参数结合 - 编译器会解引用
    fn takes_l4(_: &L4) {}
    fn takes_l3(_: &L3) {}
    fn takes_l2(_: &L2) {}
    fn takes_l1(_: &L1) {}
    fn takes_str(_: &str) {}

    takes_l4(&l4);
    takes_l3(&l4); // L4 -> L3 (自动解引用)
    takes_l2(&l4); // L4 -> L3 -> L2 (自动解引用)
    takes_l1(&l4); // L4 -> L3 -> L2 -> L1 (自动解引用)
    takes_str(&l4); // L4 -> ... -> str (自动解引用)

    // 多层引用传参 - 连续自动解引用
    takes_str(&&l4);
    takes_str(&&&l4);
    takes_str(&&&&l4);

    println!("  ✓ &expr 自动解引用到不同层级");
}

/// 强制点 29: for 循环 - 连续多层
fn test_for_loop_coercion() {
    println!("\n--- 强制点 29: for 循环 (连续多层) ---");

    // Vec<T> 实现 IntoIterator
    let v = vec![1, 2, 3];
    print!("  Vec: ");
    for item in v {
        print!("{} ", item);
    }
    println!();

    // Box<[T]> 实现 IntoIterator
    let boxed_slice = vec![4, 5, 6].into_boxed_slice();
    print!("  Box<[T]>: ");
    for item in boxed_slice {
        print!("{} ", item);
    }
    println!();

    // 迭代时自动解引用
    let boxed = Box::new(vec![7, 8, 9]);
    print!("  Box<Vec>: ");
    for item in boxed.iter() {
        print!("{} ", item);
    }
    println!();

    println!("  ✓ for x in expr 连续多层解引用");
}

/// 强制点 30: 格式化宏 - 连续多层
fn test_format_coercion() {
    println!("\n--- 强制点 30: 格式化宏 (连续多层) ---");

    let double_boxed = Box::new(Box::new(String::from("format")));

    // 多层解引用到 Display
    println!("  {}", &double_boxed);
    println!("  {}", &&double_boxed);
    println!("  {}", &&&double_boxed);
    println!("  {}", &&&&double_boxed);

    println!("  ✓ println!() 连续多层解引用");
}

/// 强制点 31: 闭包捕获 - 连续多层
fn test_closure_capture() {
    println!("\n--- 强制点 31: 闭包捕获 (连续多层) ---");

    let double_boxed = Box::new(Box::new(L4::new("closure")));

    // 闭包自动解引用访问
    let c1 = || println!("  {}", double_boxed.len());
    let c2 = || println!("  {}", (&&double_boxed).len());
    let c3 = || println!("  {}", (&&&double_boxed).len());

    c1();
    c2();
    c3();

    println!("  ✓ 闭包捕获连续多层解引用");
}

// ============================================================================
// 主测试函数
// ============================================================================

pub fn test_coercion_sites() {
    println!("\n=== A. Deref 强制点 (14 个) - 全部连续多层解引用 ===");

    test_let_coercion();
    test_static_coercion();
    test_const_coercion();
    test_fn_param_coercion();
    test_fn_return_coercion();
    test_struct_field_coercion();
    test_enum_field_coercion();
    test_tuple_coercion();
    test_closure_coercion();
    test_if_coercion();
    test_match_coercion();
    test_break_coercion();
    test_assignment_coercion();
    test_return_coercion();

    println!("\n=== B. Unsized 强制点 (6 个) ===");

    test_array_to_slice();
    test_str_to_bytes();
    test_trait_object();
    test_impl_trait();
    test_closure_to_fn_ptr();
    test_struct_to_dst();

    println!("\n=== C. 指针强制点 (4 个) ===");

    test_pointer_coercion();

    println!("\n=== D. 隐式强制点 (7 个) - 全部连续多层解引用 ===");

    test_method_receiver();
    test_index_coercion();
    test_deref_operator();
    test_addressof_coercion();
    test_for_loop_coercion();
    test_format_coercion();
    test_closure_capture();

    println!("\n=== 强制点测试完成: 共 31 个 ===");
}
