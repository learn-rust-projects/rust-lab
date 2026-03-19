/// 将表达式转换为表达式本身的宏，用于消除歧义
macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

/// 将 token stream 转换为表达式，然后调用 as_expr! 宏
macro_rules! foo {
    ($($tts:tt)*) => {
        as_expr!($($tts)*)
    };
}

fn main() {
    let x = foo!(1 + 2);
    println!("Result: {}", x);
}
