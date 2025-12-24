#[cfg(test)]
mod tests {
    use std::mem::ManuallyDrop;

    #[test]
    fn test_union_fields() {
        let mut s = ManuallyDrop::new(String::from("hello"));

        // s 作用域结束时不会自动 drop
        println!("{:?}", s);

        // 手动释放
        unsafe {
            ManuallyDrop::drop(&mut s);
        }
    }
}
