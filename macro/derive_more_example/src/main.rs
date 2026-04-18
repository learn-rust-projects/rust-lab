use derive_more::{Add, AsMut, AsRef, Display, From, FromStr, Into, IntoIterator};
// from实现从单个字段转换,into实现从包装字段转换为单个字段
#[derive(PartialEq, From, Add, Into, FromStr)]
struct MyInt(i32);

#[derive(AsRef, AsMut)]
struct MyWrapper(String);
// From实现从元组转换过来，Into实现从结构体转换成元组
#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display("int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}

#[derive(IntoIterator)]
struct MyVec(Vec<i32>);

// You can specify the field you want to derive `IntoIterator` for
#[derive(IntoIterator)]
struct Numbers {
    #[into_iterator(owned, ref, ref_mut)]
    numbers: Vec<i32>,
    useless: bool,
}
fn main() {
    assert!(11 == (MyInt(5) + 6.into() + "10".parse::<MyInt>().unwrap()).into());
    assert!((5, 6) == Point2D { x: 5, y: 6 }.into());
    assert!(MyEnum::Int(15) == (MyEnum::Int(8) + 7.into()).unwrap());
    assert!(MyEnum::Int(15).to_string() == "int: 15");
    assert!(MyEnum::Uint(42).to_string() == "42");
    assert!(MyEnum::Nothing.to_string() == "nothing");
    assert!(MyWrapper("hello".to_string()).as_ref() == "hello");
    assert!(MyWrapper("hello".to_string()).as_mut() == "hello");

    assert_eq!(Some(5), MyVec(vec![5, 8]).into_iter().next());

    let mut nums = Numbers {
        numbers: vec![100, 200],
        useless: false,
    };
    assert_eq!(Some(&100), (&nums).into_iter().next());
    assert_eq!(Some(&mut 100), (&mut nums).into_iter().next());
    assert_eq!(Some(100), nums.into_iter().next());
}
