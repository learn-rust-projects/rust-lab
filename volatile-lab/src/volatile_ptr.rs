#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volatile_ptr_write_and_read() {
        // Simulate a memory area (e.g., memory starting at address 0x1000)
        let mut memory_area: [u8; 4] = [0; 4];

        // Convert raw pointer to NonNull
        let ptr: NonNull<u8> =
            NonNull::new(&mut memory_area[0] as *mut u8).expect("Pointer should not be null");

        // Create VolatilePtr using NonNull
        let volatile_ptr: VolatilePtr<u8> = unsafe { VolatilePtr::new(ptr) };

        // Use unsafe block for memory operations
        volatile_ptr.write(42); // Write 42 to the starting address of the memory

        // Confirm that the memory content has changed
        assert_eq!(memory_area[0], 42);

        // Read data
        let value = volatile_ptr.read(); // Read data from memory

        // Confirm that the value read is correct
        assert_eq!(value, 42);
    }
}
