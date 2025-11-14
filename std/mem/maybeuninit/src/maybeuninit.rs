#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    #[test]
    fn test_union_fields() {
        let mut x: MaybeUninit<u32> = MaybeUninit::uninit(); // 明确未初始化
        // 初始化
        unsafe {
            x.write(42);
            // x.as_mut_ptr().write(42);
        }
        // 现在“逻辑上”已初始化

        // 读取
        let value = unsafe { x.assume_init() }; // 安全读取
        print!("Value: {}", value);
    }
}
