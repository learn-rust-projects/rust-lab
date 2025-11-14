#[cfg(test)]
mod tests {
    use std::mem::ManuallyDrop;

    union U2 {
        a: u32,
        b: ManuallyDrop<String>,
    }
    // 定义 union
    union U {
        a: u32,
        b: f32,
    }

    #[test]
    fn test_union_fields() {
        let mut u = U { a: 42 };

        unsafe {
            // 写入 a，读取 a
            assert_eq!(u.a, 42);

            // 读取 b，内存解释为 f32（未定义行为，主要用于观察 bit 重解）
            let b_val = u.b;
            println!("u.b (reinterpret a as f32) = {}", b_val);

            // 写入 b
            u.b = 3.1445;
            assert!((u.b - 3.1445).abs() < f32::EPSILON);

            // 读取 a，内存解释为 u32
            let a_val = u.a;
            println!("u.a (reinterpret b as u32) = {}", a_val);
        }
    }
}
