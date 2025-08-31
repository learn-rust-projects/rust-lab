#[cfg(test)]
mod tests {
    use volatile::VolatileRef;

    // 测试 VolatileRef 借用操作
    #[test]
    fn test_volatile_ref_borrow_mut() {
        let mut data = 42u32;
        let mut volatile_ref = VolatileRef::from_mut_ref(&mut data);

        // 可变借用并修改数据
        let mut borrowed_mut = volatile_ref.borrow_mut();

        // 下面这行代码如果取消注释会导致编译错误，因为 data 已经被借用为可变引用
        // data = 18;
        //   ^^^^^^^^^ `data` is assigned to here but it was already borrowed

        borrowed_mut.as_mut_ptr().write(100); // 通过 VolatilePtr 写入数据

        // 确认修改后的数据
        assert_eq!(data, 100);
    }

    // 测试 VolatileRef 借用操作（只读）
    #[test]
    fn test_volatile_ref_borrow() {
        let mut data = 42u32;
        let volatile_ref = VolatileRef::from_mut_ref(&mut data);

        // 通过只读借用读取数据
        let borrowed = volatile_ref.borrow();

        let value = borrowed.as_ptr().read(); // 通过 VolatilePtr 读取数据
        assert_eq!(value, 42); // 确保读取到的值是原始数据
    }

    // 测试可变借用后是否能进行读写
    #[test]
    fn test_volatile_ref_borrow_mut_read_write() {
        let mut data = 42u32;
        let mut volatile_ref = VolatileRef::from_mut_ref(&mut data);

        // 获取可变借用并修改数据
        let mut borrowed_mut = volatile_ref.borrow_mut();

        borrowed_mut.as_mut_ptr().write(200); // 写入新值

        // 通过只读借用读取数据
        let borrowed = volatile_ref.borrow();

        let value = borrowed.as_ptr().read(); // 读取新值
        assert_eq!(value, 200); // 确保读取到修改后的数据
    }
}
