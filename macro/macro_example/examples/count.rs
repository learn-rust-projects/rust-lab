macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*};
}

macro_rules! count_tts2 {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts!($($tail)*)};
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts3 {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: u32 = Idents::__CountIdentsLast as u32;
            COUNT
        }
    };
}
// bit
macro_rules! count_tts4 {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}

macro_rules! abacus {
    ((- $($moves:tt)*) -> (+ $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((- $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (- $($count)*))
    };
    ((+ $($moves:tt)*) -> (- $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((+ $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (+ $($count)*))
    };

    // Check if the final result is zero.
    (() -> ()) => { true };
    (() -> ($($count:tt)+)) => { false };
}
fn main() {
    assert_eq!(count_tts!(0 1 2), 3);
    assert_eq!(count_tts2!(0 1 2), 3);
    assert_eq!(count_tts3!(0 1 2), 3);
    assert_eq!(count_tts4!(0 1 2), 3);
    const COUNT: u32 = count_idents!(A, B, C);
    assert_eq!(COUNT, 3);

    let equals_zero = abacus!((++-+-+++--++---++----+) -> ());
    assert!(equals_zero);
}
