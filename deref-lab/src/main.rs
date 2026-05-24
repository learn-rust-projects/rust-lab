//! # Deref Lab
//!
//! Rust 类型强制点（Deref Coercion）测试案例集合
//!
//! ## 什么是 Deref 强制转换？
//!
//! Deref 强制转换是 Rust 编译器自动执行的隐式转换，当类型实现了 `Deref` trait 时，
//! 可以将类型自动转换为其目标类型。
//!
//! ## 主要类型强制点
//!
//! 1. **&T → &U**: 当 `T: Deref<Target=U>` 时
//! 2. **&mut T → &mut U**: 当 `T: DerefMut<Target=U>` 时
//! 3. **&mut T → &U**: 可变引用到不可变引用的转换
//! 4. **函数参数**: 传递参数时自动应用 Deref
//! 5. **方法接收者**: `expr.method()` 中的自动解引用
//!
//! ## 连续自动解引用
//!
//! Rust 支持连续自动解引用（chained deref coercion），即多次应用 Deref 直到达到目标类型。
//! 例如：`&&String` 可以自动解引用为 `&str`

mod chained_deref;
mod coercion_sites;
mod method_resolution;

pub use chained_deref::*;
pub use coercion_sites::*;
pub use method_resolution::*;

fn main() {
    println!("=== Rust Deref 强制转换实验室 ===\n");

    println!("【1】类型强制点测试:");
    println!("---------------------------");
    test_coercion_sites();

    println!("\n【2】连续自动解引用测试:");
    println!("---------------------------");
    test_chained_deref();

    println!("\n【3】方法解析中的 Deref:");
    println!("---------------------------");
    test_method_resolution();

    println!("\n=== 所有测试完成 ===");
}
