use macro_example::recurrence;

fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1; ... ; a[n-1] + a[n-2]];
    // for e in fib.take(10) { println!("{}", e) }

    for e in fib.take(10) {
        println!("{}", e)
    }
}
