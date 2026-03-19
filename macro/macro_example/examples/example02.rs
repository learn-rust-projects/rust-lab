macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace_expr!(
                    ($tup_tys)
                    Default::default()
                ),
            )*
        )
    };
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

fn main() {
    assert_eq!(
        (Default::default(), Default::default(), Default::default()),
        (i64::default(), bool::default(), String::default())
    );
}
