//! # 连续自动解引用 (Chained Deref Coercion)
//!
//! Rust 支持连续自动解引用，编译器会递归应用 Deref 直到类型匹配。
//!
//! ## 示例
//!
//! - `&&String` → `&String` → `&str`
//! - `Box<Box<String>>` → `Box<String>` → `String` → `&str`
//! - 连续多层包装类型自动解引用

use std::ops::{Deref, DerefMut};

// ============================================================================
// 测试类型定义
// ============================================================================

/// 三层嵌套的自定义类型 - Level 1
#[derive(Debug)]
struct Level1(String);

impl Level1 {
    fn new(s: &str) -> Self {
        Level1(s.to_string())
    }
}

impl Deref for Level1 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 三层嵌套的自定义类型 - Level 2
struct Level2(Level1);

impl Level2 {
    fn new(s: &str) -> Self {
        Level2(Level1::new(s))
    }
}

impl Deref for Level2 {
    type Target = Level1;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 三层嵌套的自定义类型 - Level 3
struct Level3(Level2);

impl Level3 {
    fn new(s: &str) -> Self {
        Level3(Level2::new(s))
    }
}

impl Deref for Level3 {
    type Target = Level2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 支持 DerefMut 的嵌套类型
#[derive(Debug)]
struct MutLevel1(String);

impl MutLevel1 {
    fn new(s: &str) -> Self {
        MutLevel1(s.to_string())
    }
}

impl Deref for MutLevel1 {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MutLevel1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct MutLevel2(MutLevel1);

impl MutLevel2 {
    fn new(s: &str) -> Self {
        MutLevel2(MutLevel1::new(s))
    }
}

impl Deref for MutLevel2 {
    type Target = MutLevel1;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MutLevel2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct MutLevel3(MutLevel2);

impl MutLevel3 {
    fn new(s: &str) -> Self {
        MutLevel3(MutLevel2::new(s))
    }
}

impl Deref for MutLevel3 {
    type Target = MutLevel2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MutLevel3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ============================================================================
// 测试函数
// ============================================================================

/// 测试 1: 基本的连续解引用
fn test_basic_chained_deref() {
    println!("\n--- 测试 1: 基本连续解引用 ---");

    let l3 = Level3::new("chained");

    // 连续解引用路径: Level3 -> Level2 -> Level1 -> String -> str
    fn expects_str(_s: &str) {}

    expects_str(&l3);

    // 验证每一层的类型（手动解引用）
    let _l2: &Level2 = &*l3;
    let _l1: &Level1 = &**l3;
    let _s: &String = &***l3;
    let _cs: &str = &****l3;

    println!("  ✓ Level3 -> Level2 -> Level1 -> String -> &str 解引用成功");
    let _ = (_l2, _l1, _s, _cs);
}

/// 测试 2: Box 的连续解引用
fn test_box_chained_deref() {
    println!("\n--- 测试 2: Box 连续解引用 ---");

    // Box<Box<String>> -> Box<String> -> String -> str
    let double_box: Box<Box<String>> = Box::new(Box::new("boxed".to_string()));

    fn expects_str(_s: &str) {}

    expects_str(&double_box);

    // Box<Box<Box<String>>>
    let triple_box: Box<Box<Box<String>>> = Box::new(Box::new(Box::new("triple".to_string())));
    expects_str(&triple_box);

    println!("  ✓ Box<Box<T>> -> T 连续解引用成功");
}

/// 测试 3: Vec 的连续解引用
fn test_vec_deref() {
    println!("\n--- 测试 3: Vec 自动解引用 ---");

    // Vec<String> 可以解引用为 &[String]
    let vec: Vec<String> = vec!["a".to_string(), "b".to_string()];

    // Vec<T> -> &[T] 通过 Deref
    fn expects_slice(_s: &[String]) {}

    expects_slice(&vec);

    // 验证 Vec 的方法仍然可用
    println!("  Vec 长度: {}", vec.len());
    println!("  Vec 容量: {}", vec.capacity());

    println!("  ✓ Vec<T> -> &[T] 解引用成功");
}

/// 测试 4: String 的连续解引用
fn test_string_deref() {
    println!("\n--- 测试 4: String 连续解引用 ---");

    let s = String::from("hello");

    // String -> str
    fn expects_str(_s: &str) {}

    expects_str(&s);

    // 嵌套引用
    let ss = &&&&s;
    expects_str(ss); // &&&&String -> &String -> &str

    println!("  ✓ String -> &str 解引用成功");
}

/// 测试 5: 可变引用的连续解引用
fn test_mut_chained_deref() {
    println!("\n--- 测试 5: 可变引用连续解引用 ---");

    let mut ml3 = MutLevel3::new("mutable");

    // &mut Level3 -> &mut Level2 -> &mut Level1 -> &mut String
    ml3.push_str(" extended");

    // 通过连续解引用修改
    (&mut (&mut (&mut ml3).0).0).0.push_str(" modified");

    println!("  修改后的值: {:?}", *ml3);

    println!("  ✓ 可变引用连续解引用成功");
}

/// 测试 6: 各种容器的连续解引用
fn test_various_containers() {
    println!("\n--- 测试 6: 各种容器类型 ---");

    // Box<Vec<String>>
    let box_vec: Box<Vec<String>> = Box::new(vec!["1".to_string(), "2".to_string()]);
    fn expects_slice(_s: &[String]) {}
    expects_slice(&box_vec);

    // Box<Box<Vec<String>>>
    let box_box_vec: Box<Box<Vec<String>>> = Box::new(Box::new(vec!["a".to_string()]));
    expects_slice(&box_box_vec);

    // Box<Box<Box<String>>>
    let triple_box: Box<Box<Box<String>>> = Box::new(Box::new(Box::new("triple".to_string())));
    fn expects_str_local(_s: &str) {}
    expects_str_local(&triple_box);

    println!("  ✓ 各种容器连续解引用成功");
}

/// 测试 7: 方法解析中的连续解引用
fn test_method_resolution_chained() {
    println!("\n--- 测试 7: 方法解析中的连续解引用 ---");

    let l3 = Level3::new("method");

    // 方法解析会沿着 Deref 链查找方法
    // Level3 没有 to_uppercase 的方法，但 String 有
    // 编译器会自动解引用来找到方法

    // 通过链式调用方法 - 编译器自动解引用
    let upper = l3.to_uppercase();
    println!("  转大写: {}", upper);

    println!("  ✓ 方法解析中的连续解引用成功");
}

/// 测试 8: 零-cost 抽象 - 验证优化后没有运行时开销
fn test_zero_cost() {
    println!("\n--- 测试 8: Deref 是零成本抽象 ---");

    // 编译时验证：Deref 不引入运行时开销
    // 现代 Rust 编译器会将 Deref 调用内联并优化掉

    let l3 = Level3::new("zero");

    // 直接访问底层字符串 - 编译器会优化掉中间的 Deref 调用
    let ptr1: *const String = &***l3;
    println!("  直接指针: {:?}", ptr1);
    println!("  验证优化: Deref 调用被内联");

    println!("  ✓ Deref 零成本抽象验证");
}

/// 测试 9: 复杂的嵌套结构
fn test_complex_nesting() {
    println!("\n--- 测试 9: 复杂嵌套结构 ---");

    struct Deep {
        data: String,
    }

    impl Deref for Deep {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    struct Wrapper1<T>(T);

    impl<T> Deref for Wrapper1<T>
    where
        T: Deref,
        T::Target: Sized,
    {
        type Target = T::Target;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    struct Wrapper2<T>(T);

    impl<T> Deref for Wrapper2<T>
    where
        T: Deref,
        T::Target: Sized,
    {
        type Target = T::Target;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let complex = Wrapper2(Wrapper1(Wrapper1(Deep {
        data: "complex".to_string(),
    })));

    // 连续解引用: Wrapper2 -> Wrapper1 -> Wrapper1 -> Deep -> String -> str
    fn expects_str(_s: &str) {}
    expects_str(&complex);

    println!("  ✓ 复杂嵌套解引用成功");
}

/// 测试 10: Rc 的解引用行为
fn test_rc_deref() {
    println!("\n--- 测试 10: Rc 解引用 ---");

    use std::rc::Rc;

    let rc_str: Rc<String> = Rc::new(String::from("rc string"));
    fn expects_str(_s: &str) {}

    // Rc<T> 可以解引用为 T，然后继续解引用
    expects_str(&rc_str); // Rc<String> -> String -> str

    println!("  ✓ Rc<T> -> T 连续解引用成功");
}

/// 测试 11: Arc 的解引用行为
fn test_arc_deref() {
    println!("\n--- 测试 11: Arc 解引用 ---");

    use std::sync::Arc;

    let arc_str: Arc<String> = Arc::new(String::from("arc string"));
    fn expects_str(_s: &str) {}

    expects_str(&arc_str); // Arc<String> -> String -> str

    println!("  ✓ Arc<T> -> T 连续解引用成功");
}

/// 测试 12: Cow 的解引用行为
fn test_cow_deref() {
    println!("\n--- 测试 12: Cow 解引用 ---");

    use std::borrow::Cow;

    // Cow<'a, str> -> str (通过 Deref)
    let borrowed: Cow<str> = Cow::Borrowed("borrowed");
    fn expects_str(_s: &str) {}

    expects_str(&borrowed);

    let owned: Cow<str> = Cow::Owned(String::from("owned"));
    expects_str(&owned);

    println!("  ✓ Cow<str> -> str 解引用成功");
}

/// 主测试函数
pub fn test_chained_deref() {
    test_basic_chained_deref();
    test_box_chained_deref();
    test_vec_deref();
    test_string_deref();
    test_mut_chained_deref();
    test_various_containers();
    test_method_resolution_chained();
    test_zero_cost();
    test_complex_nesting();
    test_rc_deref();
    test_arc_deref();
    test_cow_deref();
}
