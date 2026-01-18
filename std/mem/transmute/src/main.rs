//! 标准库mem模块常用方法测试程序
//!
//! 这个程序展示了标准库中mem模块的各种方法，包括：
//! - transmute: 在类型之间转换（重点）
//! - size_of / align_of: 获取类型大小和对齐要求
//! - replace / swap / take: 值操作方法
//! - forget: 忘记值，不运行析构函数
//! - zeroed / uninitialized: 内存初始化方法（unsafe）
//! - drop: 显式释放值

use std::mem;

fn main() {
    println!("=== 标准库mem模块常用方法测试程序 ===");

    // 运行所有测试示例
    test_size_and_align();
    test_value_operations();
    test_transmute_basic();
    test_transmute_advanced();
    test_transmute_string();
    test_unsafe_initialization();
    test_forget_and_drop();

    println!("\n🎉 所有mem方法测试完成！");
}

/// 测试size_of和align_of方法
fn test_size_and_align() {
    println!("\n--- 测试size_of和align_of方法 ---");

    // size_of - 获取类型的内存大小
    println!("i8的大小: {}", mem::size_of::<i8>());
    println!("i32的大小: {}", mem::size_of::<i32>());
    println!("i64的大小: {}", mem::size_of::<i64>());
    println!("usize的大小: {}", mem::size_of::<usize>());

    // size_of_val - 获取值的内存大小
    let x: i32 = 42;
    println!("x (i32)的值大小: {}", mem::size_of_val(&x));

    // align_of - 获取类型的对齐要求
    println!("\n类型对齐要求:");
    println!("i8的对齐: {}", mem::align_of::<i8>());
    println!("i32的对齐: {}", mem::align_of::<i32>());
    println!("f64的对齐: {}", mem::align_of::<f64>());
    println!("Vec<i32>的对齐: {}", mem::align_of::<Vec<i32>>());

    // align_of_val - 获取值的对齐要求
    println!("\nx (i32)的值对齐: {}", mem::align_of_val(&x));
}

/// 测试replace、swap和take方法
fn test_value_operations() {
    println!("\n--- 测试replace、swap和take方法 ---");

    // replace - 替换值并返回旧值
    let mut vec = vec![1, 2, 3];
    let old_vec = mem::replace(&mut vec, vec![4, 5, 6]);
    println!("replace: 旧值: {:?}, 新值: {:?}", old_vec, vec);

    // swap - 交换两个值
    let mut x = 10;
    let mut y = 20;
    println!("swap前: x = {}, y = {}", x, y);
    mem::swap(&mut x, &mut y);
    println!("swap后: x = {}, y = {}", x, y);

    // take - 取出值并替换为默认值
    let mut s = String::from("hello");
    println!("take前: s = \"{}\"", s);
    let taken = mem::take(&mut s);
    println!("take后: taken = \"{}\", s = \"{}\"", taken, s);
    assert!(s.is_empty());
}

/// 测试transmute基本用法
fn test_transmute_basic() {
    println!("\n--- 测试transmute基本用法 ---");

    // transmute将值转换为不同的类型
    // 注意：只有当源类型和目标类型大小相同时才能使用transmute

    // 将i32转换为u32
    let x: i32 = -1;
    println!("i32值: {}, 十六进制: {:X}", x, x);

    unsafe {
        let y: u32 = mem::transmute(x);
        println!("transmute为u32: {}, 十六进制: {:X}", y, y);
    }

    // 将f32转换为i32
    let f: f32 = std::f32::consts::PI;
    println!("\nf32值: {}", f);

    unsafe {
        let i: i32 = mem::transmute(f);
        println!("transmute为i32: {}, 十六进制: {:X}", i, i);
    }

    // 验证大小必须相同
    println!("\n类型大小验证:");
    println!(
        "i32大小: {}, u32大小: {}",
        mem::size_of::<i32>(),
        mem::size_of::<u32>()
    );
    println!(
        "f32大小: {}, i32大小: {}",
        mem::size_of::<f32>(),
        mem::size_of::<i32>()
    );
    assert_eq!(mem::size_of::<i32>(), mem::size_of::<u32>());
    assert_eq!(mem::size_of::<f32>(), mem::size_of::<i32>());
}

/// 测试transmute高级用法
fn test_transmute_advanced() {
    println!("\n--- 测试transmute高级用法 ---");

    // 将引用转换为原始指针
    let x: i32 = 42;
    let x_ref: &i32 = &x;

    unsafe {
        // 引用转原始指针
        let raw_ptr: *const i32 = mem::transmute(x_ref);
        println!("引用: {:p}, transmute为原始指针: {:p}", x_ref, raw_ptr);
        // 验证原始指针的值
        assert_eq!(*raw_ptr, x);
    }

    // 将原始指针转换为引用（危险操作）
    unsafe {
        let raw_ptr = &x as *const i32 as *mut u8;
        println!("\n原始指针: {:p}", raw_ptr);

        // 转换为u8引用
        let u8_ref: &u8 = mem::transmute(raw_ptr);
        println!("转换为u8引用: {}, 十六进制: {:X}", *u8_ref, *u8_ref);

        // 这演示了如何访问内存中的单个字节
    }

    // transmute数组和切片
    let arr: [i8; 4] = [1, 2, 3, 4];
    println!("\n数组: {:?}", arr);

    unsafe {
        // 将数组转换为i32
        let i: i32 = mem::transmute(arr);
        println!("transmute为i32: {}, 十六进制: {:X}", i, i);

        // 将i32转换回数组
        let arr2: [i8; 4] = mem::transmute(i);
        assert_eq!(arr, arr2);
        println!("转换回数组: {:?}", arr2);
    }
}

/// 测试transmute字符串操作
fn test_transmute_string() {
    println!("\n--- 测试transmute字符串操作 ---");

    // 将字符串转换为Vec<u8>
    let s = String::from("hello");
    println!("原始字符串: {}", s);
    println!("字符串大小: {}", mem::size_of::<String>());
    println!("Vec<u8>大小: {}", mem::size_of::<Vec<u8>>());

    // String和Vec<u8>具有相同的内存布局，可以安全地transmute
    assert_eq!(mem::size_of::<String>(), mem::size_of::<Vec<u8>>());

    unsafe {
        // 将String转换为Vec<u8>
        let vec: Vec<u8> = mem::transmute(s);
        println!("transmute为Vec<u8>: {:?}", vec);

        // 转换回String
        let s2: String = mem::transmute(vec);
        println!("转换回String: {}", s2);

        assert_eq!(s2, "hello");
    }
}

/// 测试unsafe初始化方法
fn test_unsafe_initialization() {
    println!("\n--- 测试unsafe初始化方法 ---");

    // 注意：zeroed和uninitialized是不安全的，需要特别小心使用

    // zeroed - 创建一个全零的实例
    unsafe {
        let zeroed_i32: i32 = mem::zeroed();
        println!("zeroed i32: {}, 十六进制: {:X}", zeroed_i32, zeroed_i32);

        let zeroed_arr: [i8; 4] = mem::zeroed();
        println!("zeroed数组: {:?}", zeroed_arr);
    }

    // 注意：使用mem::uninitialized已被弃用，现在推荐使用MaybeUninit
    // 这里为了演示目的仍然包含它
    unsafe {
        // 创建一个未初始化的i32
        let mut uninit_i32: i32 = mem::zeroed(); // 使用zeroed代替uninitialized
        println!("\nuninit i32 (使用zeroed): {}", uninit_i32);

        // 初始化后使用
        uninit_i32 = 123;
        println!("初始化后: {}", uninit_i32);
    }
}

/// 测试forget和drop方法
fn test_forget_and_drop() {
    println!("\n--- 测试forget和drop方法 ---");

    // drop - 显式释放值
    let mut vec = vec![1, 2, 3];
    println!("创建Vec: {:?}", vec);
    drop(vec);
    // println!("vec: {:?}", vec); // 编译错误，vec已被drop

    // forget - 忘记值，不运行析构函数
    let s = String::from("hello, forget");
    println!("创建String: \"{}\"", s);

    unsafe {
        mem::forget(s);
        // println!("s: {}", s); // 编译错误，s已被forget
        println!("String已被forget，不会运行析构函数");
    }

    // 使用forget实现内存泄漏
    let v = vec![Box::new(1), Box::new(2), Box::new(3)];
    println!("\n创建Vec<Box>: {:?}", v);

    unsafe {
        mem::forget(v);
        println!("Vec<Box>已被forget，内部的Box不会被释放");
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_size_align_consistency() {
        assert_eq!(mem::size_of::<i32>(), mem::size_of::<u32>());
        assert_eq!(mem::size_of::<f32>(), mem::size_of::<i32>());
        assert_eq!(mem::size_of::<String>(), mem::size_of::<Vec<u8>>());
    }

    #[test]
    fn test_transmute_basic_safety() {
        let x: i32 = -1;

        unsafe {
            let y: u32 = mem::transmute(x);
            // 验证内存表示
            assert_eq!(y, 0xFFFFFFFF);
        }
    }

    #[test]
    fn test_value_operations() {
        let mut x = 10;
        let mut y = 20;

        mem::swap(&mut x, &mut y);
        assert_eq!(x, 20);
        assert_eq!(y, 10);

        let old = mem::replace(&mut x, 30);
        assert_eq!(old, 20);
        assert_eq!(x, 30);

        let taken = mem::take(&mut y);
        assert_eq!(taken, 10);
        assert_eq!(y, 0); // i32的默认值
    }

    #[test]
    fn test_string_vec_transmute() {
        let s = String::from("test");

        unsafe {
            let vec: Vec<u8> = mem::transmute(s);
            assert_eq!(vec, b"test");

            let s2: String = mem::transmute(vec);
            assert_eq!(s2, "test");
        }
    }
}
