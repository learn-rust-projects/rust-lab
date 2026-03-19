/// 根据输入中是否包含 "call a:" 或 "call b:" 来决定调用哪个函数
///
/// 宏接受两个参数：
/// - `$ab`: 包含两个函数引用的元组 `(a: 函数A, b: 函数B)`
/// - token stream: 包含多个 token 的序列
///
/// 匹配规则：
/// 1. 如果遇到 `call a:`，调用第一个函数并传入后续 token 的字符串化形式
/// 2. 如果遇到 `call b:`，调用第二个函数并传入后续 token 的字符串化形式
/// 3. 否则跳过当前 token，递归处理剩余的 token
macro_rules! call_a_or_b_on_tail {
    // 匹配 "call a:" 模式，调用 `$a` 函数
    ((a: $a:ident, b: $b:ident), call a: $($tail:tt)*) => {
        $a(stringify!($($tail)*))
    };

    // 匹配 "call b:" 模式，调用 `$b` 函数
    ((a: $a:ident, b: $b:ident), call b: $($tail:tt)*) => {
        $b(stringify!($($tail)*))
    };

    // 跳过不匹配当前 token，递归处理剩余 token
    ($ab:tt, $_skip:tt $($tail:tt)*) => {
        call_a_or_b_on_tail!($ab, $($tail)*)
    };
}

fn compute_len(s: &str) -> Option<usize> {
    Some(s.len())
}

fn show_tail(s: &str) -> Option<usize> {
    println!("tail: {:?}", s);
    None
}

fn main() {
    assert_eq!(
        call_a_or_b_on_tail!(
            (a: compute_len, b: show_tail),
            the recursive part that skips over all these
            tokens doesn 't much care whether we will call a
            or call b: only the terminal rules care.
        ),
        None
    );
    assert_eq!(
        call_a_or_b_on_tail!(
            (a: compute_len, b: show_tail),
            and now, to justify the existence of two paths
            we will also call a: its input should somehow
            be self-referential, so let 's make it return
            some eighty-six!
        ),
        Some(87)
    );
}
