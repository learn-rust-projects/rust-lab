#![feature(log_syntax)]

macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
}

use macro_example::recurrence;

fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1; ... ; a[n-1] + a[n-2]];
    // for e in fib.take(10) { println!("{}", e) }

    for e in fib.take(10) {
        println!("{}", e)
    }

    sing! {
        ^ < @ < . @ *
        '\x08' '{' '"' _ # ' '
        - @ '$' && / _ %
        ! ( '\t' @ | = >
        ; '\x08' '\'' + '$' ? '\x7f'
        , # '"' ~ | ) '\x07'
    }
}
