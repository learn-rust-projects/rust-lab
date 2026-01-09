extern crate libc;

use libc::{c_int, c_void};

fn split_lines(buf: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < buf.len() {
        let ptr = unsafe {
            libc::memchr(
                buf.as_ptr().add(start) as *const c_void,
                b'\n' as c_int,
                buf.len() - start,
            )
        };

        match ptr.is_null() {
            true => {
                // 最后一行（没有 \n）
                lines.push(&buf[start..]);
                break;
            }
            false => unsafe {
                let end = (ptr as *const u8).offset_from(buf.as_ptr()) as usize;
                lines.push(&buf[start..end]);
                start = end + 1;
            },
        }
    }

    lines
}

fn main() {
    let data = b"line1\nline2\nline3";
    for line in split_lines(data) {
        println!("{:?}", line);
    }
}
