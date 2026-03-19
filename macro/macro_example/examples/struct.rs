macro_rules! struct_item_matcher {
    // Unit-Struct
    (
        $( #[$meta:meta] )*
    //  ^~~~attributes~~~~^
        $vis:vis struct $name:ident;
    ) => {
        $( #[$meta] )*
        $vis struct $name;
    };

    // Tuple-Struct
    (
        $( #[$meta:meta] )*
    //  ^~~~attributes~~~~^
        $vis:vis struct $name:ident (
            $(
                $( #[$field_meta:meta] )*
    //          ^~~~field attributes~~~~^
                $field_vis:vis $field_ty:ty
    //          ^~~~~~a single field~~~~~~^
            ),*
        $(,)? );
    ) => {
        $( #[$meta] )*
        $vis struct $name (
            $(
                $( #[$field_meta] )*
                $field_vis $field_ty
            ),*
        );
    };

    // Named-Struct
    (
        $( #[$meta:meta] )*
    //  ^~~~attributes~~~~^
        $vis:vis struct $name:ident {
            $(
                $( #[$field_meta:meta] )*
    //          ^~~~field attributes~~~!^
                $field_vis:vis $field_name:ident : $field_ty:ty
    //          ^~~~~~~~~~~~~~~~~a single field~~~~~~~~~~~~~~~^
            ),*
        $(,)? }
    ) => {
        $( #[$meta] )*
        $vis struct $name {
            $(
                $( #[$field_meta] )*
                $field_vis $field_name : $field_ty
            ),*
        }
    }
}

struct_item_matcher!(
    #[allow(dead_code)]
    #[derive(Copy, Clone)]
    pub(crate) struct Foo {
        pub bar: i32,
        baz: &'static str,
        qux: f32,
    }
);
struct_item_matcher!(
    #[derive(Copy, Clone)]
    pub(crate) struct Bar;
);
struct_item_matcher!(
    #[derive(Clone)]
    pub(crate) struct Baz(i32, pub f32, String);
);
fn main() {
    let _: Foo = Foo {
        bar: 42,
        baz: "macros can be nice",
        qux: 3.1423,
    };
    let _: Bar = Bar;
    let _: Baz = Baz(2, 0.1234, String::new());
}
