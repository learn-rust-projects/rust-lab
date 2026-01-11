//! Option类型常用方法测试示例
#![feature(result_option_map_or_default)]
#![feature(option_zip)]
#![feature(option_reduce)]
use std::pin::{self, Pin};

/// 主测试函数
fn main() {
    println!("开始Option方法测试...\n");

    test_basic_methods();
    test_value_extraction();
    test_transformation_methods();
    test_filtering_methods();
    test_iterator_methods();
    test_comparison_methods();
    test_utility_methods();
    test_pattern_matching();
    test_trait_methods();

    if let Err(e) = test_question_operator() {
        eprintln!("?操作符测试失败: {}", e);
    }

    println!("\n🎉 所有Option方法测试完成！");
}
/// 测试Option的基本创建和判断方法
fn test_basic_methods() {
    println!("=== 基本创建和判断方法测试 ===");

    // 创建Some和None
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // is_some() 和 is_none()
    assert!(some_value.is_some());
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert!(none_value.is_none());

    // is_some_and() - 检查Some值是否满足条件
    assert!(some_value.is_some_and(|x| x > 40));
    assert!(some_value.is_none_or(|x| x >= 40));
    assert!(none_value.is_none_or(|x| x > 0));

    println!("✓ 基本方法测试通过");
}
/// 测试Option的值提取方法
fn test_value_extraction() {
    println!("\n=== 值提取方法测试 ===");
    let mut none = None;
    // get_or_insert() - 插入值（如果为None），否则返回当前值
    none.get_or_insert(50);
    none.get_or_insert_default();
    none.get_or_insert_with(|| 100);
    assert_eq!(none, Some(50));

    let some_value = Some("hello");
    let none_value: Option<&str> = None;

    // unwrap() - 提取Some值，None会panic
    assert_eq!(some_value.unwrap(), "hello");

    // unwrap_or() - 提取值或提供默认值
    assert_eq!(some_value.unwrap_or("default"), "hello");
    assert_eq!(none_value.unwrap_or("default"), "default");

    // unwrap_or_else() - 使用闭包提供默认值
    assert_eq!(some_value.unwrap_or_else(|| "default"), "hello");
    assert_eq!(none_value.unwrap_or_else(|| "default"), "default");

    // unwrap_or_default() - 使用类型的默认值
    let some_num = Some(100);
    let none_num: Option<i32> = None;
    assert_eq!(some_num.unwrap_or_default(), 100);
    assert_eq!(none_num.unwrap_or_default(), 0);

    // as_mut() - 获取可变引用（如果有）
    let mut some_num = Some(100);
    let none_num_as_ref = some_num.as_mut();
    println!("{:?}", none_num_as_ref);
    // as_ref() - 获取不可变引用（如果有）
    let some_num_as_ref = some_num.as_ref();
    println!("{:?}", some_num_as_ref);

    // as_pin_ref
    let mut some_num = Some(100);
    let pin = Pin::new(&some_num);
    let pin_as_ref = pin.as_pin_ref();
    println!("{:?}", pin_as_ref);

    // as_pin_mut
    let pin = Pin::new(&mut some_num);
    let pin_as_mut = pin.as_pin_mut();

    // as_slice
    let mut some_num = Some(100);
    let slice = some_num.as_slice();
    println!("{:?}", slice);
    // as_mut_slice
    let mut_slice = some_num.as_mut_slice();
    println!("{:?}", mut_slice);

    // expect
    let mut some_num = Some(100);
    let expect = some_num.expect("Some value is missing");
    println!("{:?}", expect);

    // unwrap unwrap_or unwrap_or_else unwrap_or_default
    let mut some_num = Some(100);
    let unwrap = some_num.unwrap();
    println!("{:?}", unwrap);

    let unwrap_or = some_num.unwrap_or(0);
    println!("{:?}", unwrap_or);

    let unwrap_or_else = some_num.unwrap_or_else(|| some_num.unwrap() + 1);
    println!("{:?}", unwrap_or_else);

    let unwrap_or_default = some_num.unwrap_or_default();
    println!("{:?}", unwrap_or_default);

    let unwrap_unchecked = unsafe { some_num.unwrap_unchecked() };
    println!("{:?}", unwrap_unchecked);

    // inspect() - 检查值而不转换
    let mut some_value = Some(5);
    let inspect_result = some_value.inspect(|x| println!("Value: {}", x));
    assert_eq!(inspect_result, Some(5));

    // as_deref() - 提取字符串切片（如果有）
    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));

    let x: Option<String> = None;
    assert_eq!(x.as_deref(), None);

    // as_deref_mut
    let mut x: Option<String> = Some("hey".to_owned());
    let as_deref_mut = x.as_deref_mut().map(|x| {
        x.make_ascii_lowercase();
        x
    });
    assert_eq!(as_deref_mut, Some("hey".to_owned().as_mut_str()));

    println!("✓ 值提取方法测试通过");
}

/// 测试Option的转换方法
fn test_transformation_methods() {
    println!("\n=== 转换方法测试 ===");

    // insert() - 插入值（如果为None）
    let mut none = None;
    none.insert(3);
    assert_eq!(none, Some(3));

    // take() - 取出值并替换为None
    let mut opt = Some("hello");
    assert_eq!(opt.take(), Some("hello"));
    assert_eq!(opt, None);

    // take_if() - 取出值如果满足条件
    let mut opt = Some("hello");
    assert_eq!(opt.take_if(|x| x.starts_with("h")), Some("hello"));
    assert_eq!(opt, None);

    // replace() - 替换值并返回旧值
    let mut opt = Some("old");
    assert_eq!(opt.replace("new"), Some("old"));
    assert_eq!(opt, Some("new"));

    let some_value = Some(5);
    let none_value: Option<i32> = None;

    // map() - 对Some值进行转换
    assert_eq!(some_value.map(|x| x * 2), Some(10));
    assert_eq!(none_value.map(|x| x * 2), None);

    // map_or() - 转换或提供默认值
    assert_eq!(some_value.map_or(0, |x| x * 2), 10);
    assert_eq!(none_value.map_or(0, |x| x * 2), 0);

    // #![feature(result_option_map_or_default)]
    assert_eq!(none_value.map_or_default(|x| x + 0), 0);

    // map_or_else() - 使用闭包提供默认值
    assert_eq!(some_value.map_or_else(|| 0, |x| x * 2), 10);
    assert_eq!(none_value.map_or_else(|| 0, |x| x * 2), 0);

    // and_then() - 链式转换（flatMap）
    assert_eq!(some_value.and_then(|x| Some(x * 2)), Some(10));
    assert_eq!(none_value.and_then(|x| Some(x * 2)), None);

    // or() - 如果为None则使用另一个Option
    assert_eq!(some_value.or(Some(100)), Some(5));
    assert_eq!(none_value.or(Some(100)), Some(100));

    // or_else() - 使用闭包提供备选Option
    assert_eq!(some_value.or_else(|| Some(100)), Some(5));
    assert_eq!(none_value.or_else(|| Some(100)), Some(100));

    // ok_or() - 转换为Result
    let some_value = Some("success");
    let none_value: Option<&str> = None;

    assert_eq!(some_value.ok_or("error"), Ok("success"));
    assert_eq!(none_value.ok_or("error"), Err("error"));

    // ok_or_else() - 使用闭包提供错误
    assert_eq!(some_value.ok_or_else(|| "error"), Ok("success"));
    assert_eq!(none_value.ok_or_else(|| "error"), Err("error"));

    // x and y
    let x = Some(2);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);
    // x andthen
    let arr_2d = [["A0", "A1"], ["B0", "B1"]];
    let y = arr_2d.get(0).and_then(|i| i.get(0));
    assert_eq!(y, Some(&"A0"));

    // xor
    let x = Some(2);
    let y: Option<i32> = None;
    assert_eq!(x.xor(y), Some(2));
    assert_eq!(y.xor(x), Some(2));

    println!("✓ 转换方法测试通过");
}

/// 测试Option的过滤和条件方法
fn test_filtering_methods() {
    println!("\n=== 过滤和条件方法测试 ===");

    let some_value = Some(15);
    let none_value: Option<i32> = None;

    // filter() - 根据条件过滤
    assert_eq!(some_value.filter(|&x| x > 10), Some(15));
    assert_eq!(some_value.filter(|&x| x < 10), None);
    assert_eq!(none_value.filter(|&x| x > 10), None);

    println!("✓ 过滤和条件方法测试通过");
}

/// 测试Option的迭代器方法
fn test_iterator_methods() {
    println!("\n=== 迭代器方法测试 ===");

    let some_value = Some(42);
    let none_value: Option<i32> = None;

    // iter() - 创建迭代器
    let mut iter = some_value.iter();
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);

    let mut none_iter = none_value.iter();
    assert_eq!(none_iter.next(), None);

    // iter_mut() - 创建可变迭代器
    let mut mutable_opt = Some(String::from("hello"));
    for s in mutable_opt.iter_mut() {
        s.push_str(" world");
    }
    assert_eq!(mutable_opt, Some("hello world".to_string()));

    // into_iter() - 消费Option的迭代器
    let vec: Vec<i32> = some_value.into_iter().collect();
    assert_eq!(vec, vec![42]);

    println!("✓ 迭代器方法测试通过");
}

/// 测试Option的比较和排序方法
fn test_comparison_methods() {
    println!("\n=== 比较和排序方法测试 ===");

    let some_5 = Some(5);
    let some_10 = Some(10);
    let none: Option<i32> = None;

    // 比较操作
    assert!(some_5 < some_10);
    assert!(some_5 <= some_10);
    assert!(some_10 > some_5);
    assert!(some_10 >= some_5);
    assert!(none < some_5); // None < Some
    assert!(some_5 > none); // Some > None

    // cmp() - 三向比较
    use std::cmp::Ordering;
    assert_eq!(some_5.cmp(&some_10), Ordering::Less);
    assert_eq!(some_10.cmp(&some_5), Ordering::Greater);
    assert_eq!(some_5.cmp(&some_5), Ordering::Equal);

    println!("✓ 比较方法测试通过");
}

/// 测试Option的实用工具方法
fn test_utility_methods() {
    println!("\n=== 实用工具方法测试 ===");

    // transpose() - 转换Option<Result>为Result<Option>
    let opt_result: Option<Result<i32, &str>> = Some(Ok(42));
    let result_opt: Result<Option<i32>, &str> = opt_result.transpose();
    assert_eq!(result_opt, Ok(Some(42)));

    let opt_err: Option<Result<i32, &str>> = Some(Err("error"));
    let err_opt: Result<Option<i32>, &str> = opt_err.transpose();
    assert_eq!(err_opt, Err("error"));

    // flatten() - 展平嵌套Option
    let nested: Option<Option<i32>> = Some(Some(42));
    assert_eq!(nested.flatten(), Some(42));

    let double_none: Option<Option<i32>> = Some(None);
    assert_eq!(double_none.flatten(), None);

    // zip
    let opt_a = Some(1);
    let opt_b = Some(2);
    let zipped = opt_a.zip(opt_b);
    assert_eq!(zipped, Some((1, 2)));

    // zip_with() - 合并两个Option，使用提供的函数
    // #![feature(option_zip)]
    let zipped_with = opt_a.zip_with(opt_b, |a, b| a + b);
    assert_eq!(zipped_with, Some(3));

    // #![feature(option_reduce)]
    let opt_c = Some(3);
    let reduced = opt_a.reduce(opt_c, |x, y: i32| y + x);
    assert_eq!(reduced, Some(4));

    // unzip
    let (a, b) = zipped.unzip();
    assert_eq!(a, Some(1));
    assert_eq!(b, Some(2));

    let items = vec![2_u16, 1, 0];
    let res: Option<Vec<u16>> = items.iter().map(|x| x.checked_sub(1)).collect();
    assert_eq!(res, None); // 因为 0.checked_sub(1) 返回 None
    let res: Vec<u16> = items
        .iter()
        .map(|x| x.checked_sub(1).unwrap_or(0))
        .collect();
    assert_eq!(res, vec![1, 0, 0]); // 因为 0.checked_sub(1) 返回 None
    println!("✓ 实用工具方法测试通过");
}

/// 测试Option在模式匹配中的使用
fn test_pattern_matching() {
    println!("\n=== 模式匹配测试 ===");

    let some_value = Some(42);
    let none_value: Option<i32> = None;

    // if let 模式匹配
    if let Some(x) = some_value {
        assert_eq!(x, 42);
    } else {
        panic!("应该匹配Some");
    }

    if let None = none_value {
        // 正确匹配None
    } else {
        panic!("应该匹配None");
    }

    // match 表达式
    match some_value {
        Some(x) => assert_eq!(x, 42),
        None => panic!("不应该匹配None"),
    }

    match none_value {
        Some(_) => panic!("不应该匹配Some"),
        None => { /* 正确匹配None */ }
    }

    println!("✓ 模式匹配测试通过");
}

/// 测试Option与?操作符的使用
fn test_question_operator() -> Result<(), &'static str> {
    println!("\n=== ?操作符测试 ===");

    fn get_value() -> Option<i32> {
        Some(42)
    }

    fn get_none() -> Option<i32> {
        None
    }

    // 在Option中使用?
    let value = get_value().ok_or("获取值失败")?;
    assert_eq!(value, 42);

    // 在Result中使用Option的?
    let result: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = result.ok();
    assert_eq!(opt, Some(42));

    println!("✓ ?操作符测试通过");
    Ok(())
}

fn test_trait_methods() {
    // copied() - 复制Option<&T> 或 Option<&mut T> 中的值
    let x = 12;
    let opt_x = Some(&x);
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));
    // cloned() - 克隆Option<&T>中的值
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    println!("✓ 特征方法测试通过");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_option_methods() {
        test_basic_methods();
        test_value_extraction();
        test_transformation_methods();
        test_filtering_methods();
        test_iterator_methods();
        test_comparison_methods();
        test_utility_methods();
        test_pattern_matching();
        assert!(test_question_operator().is_ok());
    }

    #[test]
    fn test_option_creation() {
        let some = Some(42);
        let none: Option<i32> = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_option_unwrap_methods() {
        let some = Some("hello");
        let none: Option<&str> = None;

        assert_eq!(some.unwrap_or("default"), "hello");
        assert_eq!(none.unwrap_or("default"), "default");
        assert_eq!(none.unwrap_or_default(), "");
    }

    #[test]
    fn test_option_transformations() {
        let some = Some(5);
        let none: Option<i32> = None;

        assert_eq!(some.map(|x| x * 2), Some(10));
        assert_eq!(none.map(|x| x * 2), None);
        assert_eq!(some.and_then(|x| Some(x * 2)), Some(10));
    }
}
