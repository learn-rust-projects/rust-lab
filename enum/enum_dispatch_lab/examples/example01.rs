use enum_dispatch::enum_dispatch;

// 定义 trait
#[enum_dispatch]
trait MyTrait {
    fn do_something(&self);
}

// 定义具体类型
pub struct RespNull;
pub struct RespSet;

// 给具体类型实现 trait
impl MyTrait for RespNull {
    fn do_something(&self) {
        println!("RespNull called");
    }
}

impl MyTrait for RespSet {
    fn do_something(&self) {
        println!("RespSet called");
    }
}

// 定义枚举，用元组变体包装类型
#[enum_dispatch(MyTrait)]
enum RespEnum {
    RespNull(RespNull),
    RespSet(RespSet),
}

fn main() {
    // 使用 From 构造枚举
    let r1: RespEnum = RespNull.into();
    let r2: RespEnum = RespSet.into();

    // 调用 trait 方法（宏生成的静态分发）
    r1.do_something(); // 输出: RespNull called
    r2.do_something(); // 输出: RespSet called

    // 正确匹配元组变体
    match &r1 {
        RespEnum::RespNull(_) => println!("Matched RespNull"),
        RespEnum::RespSet(_) => println!("Matched RespSet"),
    }
}
