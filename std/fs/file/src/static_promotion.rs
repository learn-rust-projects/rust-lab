#[cfg(test)]
mod tests {
    use std::io::Write; // 引入 Write trait，用于 flush

    // 为了在测试中能看到 println! 的输出，需要用 cargo test -- --nocapture 运行
    fn print_address_and_flush(label: &str, addr: *const ()) {
        // 使用 writeln! 和 io::stdout().flush() 确保输出立即显示
        let _ = writeln!(std::io::stdout(), "{}: {:p}", label, addr);
        let _ = std::io::stdout().flush();
    }
    // --- 第一次调用地址 ---: 0x0
    // 静态提升地址 (Promoted): 0x5c3fe23e02ec
    // 字符串字面量地址 (String): 0x5c3fe23e02cf
    // 栈上数据地址 (Stack 1): 0x7a3c529fe324

    // --- 第二次调用地址 ---: 0x0
    // 静态提升地址 (Promoted 2): 0x5c3fe23e02ec
    // 字符串字面量地址 (String 2): 0x5c3fe23e02cf
    // 栈上数据地址 (Stack 2): 0x7a3c529fe34c
    // tatic_promotion::tests::verify_static_promotion ... ok

    #[test]
    fn verify_static_promotion() {
        // --- 第一次调用：建立基准地址 ---

        // 场景 1: 静态提升 (&[i32] 字面量引用)
        let static_promoted_ref = &[1, 2, 3];

        // 场景 2: 字符串字面量 (天生 'static)
        let static_str = "Rust static promotion test";

        // 场景 3: 栈分配 (对比组)
        let stack_data = [4, 5, 6];
        let stack_ref = &stack_data;

        print_address_and_flush("\n--- 第一次调用地址 ---", 0 as *const ()); // 打印分隔线

        // 打印静态提升的地址
        let promoted_addr = static_promoted_ref.as_ptr() as *const ();
        print_address_and_flush("静态提升地址 (Promoted)", promoted_addr);

        // 打印字符串字面量地址
        let str_addr = static_str.as_ptr() as *const ();
        print_address_and_flush("字符串字面量地址 (String)", str_addr);

        // 打印栈上数据的地址
        let stack_addr = stack_ref.as_ptr() as *const ();
        print_address_and_flush("栈上数据地址 (Stack 1)", stack_addr);

        // --- 第二次调用：观察地址变化 ---

        // 再次创建栈上的数据（在新的作用域或逻辑上）
        let stack_data_2 = [7, 8, 9];
        let stack_ref_2 = &stack_data_2;

        // 再次引用静态提升和字符串字面量
        let static_promoted_ref_2 = &[1, 2, 3];
        let static_str_2 = "Rust static promotion test";

        print_address_and_flush("\n--- 第二次调用地址 ---", 0 as *const ()); // 打印分隔线

        // 打印静态提升的地址 (应该与第一次相同)
        let promoted_addr_2 = static_promoted_ref_2.as_ptr() as *const ();
        print_address_and_flush("静态提升地址 (Promoted 2)", promoted_addr_2);
        assert_eq!(
            promoted_addr, promoted_addr_2,
            "静态提升的地址应该保持不变！"
        );

        // 打印字符串字面量地址 (应该与第一次相同)
        let str_addr_2 = static_str_2.as_ptr() as *const ();
        print_address_and_flush("字符串字面量地址 (String 2)", str_addr_2);
        assert_eq!(str_addr, str_addr_2, "字符串字面量的地址应该保持不变！");

        // 打印新的栈上数据的地址 (应该与第一次不同)
        let stack_addr_2 = stack_ref_2.as_ptr() as *const ();
        print_address_and_flush("栈上数据地址 (Stack 2)", stack_addr_2);
        assert_ne!(stack_addr, stack_addr_2, "栈上数据的地址应该改变！");
    }
}
