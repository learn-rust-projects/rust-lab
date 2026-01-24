use enum_dispatch::enum_dispatch;

// 定义具体类型
pub struct RespNull;
pub struct RespSet;

// 给具体类型实现 trait
impl MyTrait for RespNull {
    async fn do_something(&self) {
        println!("RespNull called");
    }
}

impl MyTrait for RespSet {
    async fn do_something(&self) {
        println!("RespSet called");
    }
}

// 定义 trait
#[enum_dispatch]
trait MyTrait {
    async fn do_something(&self);
}

// 定义枚举，用元组变体包装类型
#[enum_dispatch(MyTrait)]
enum RespEnum {
    RespNull(RespNull),
    RespSet(RespSet),
}
#[tokio::main]
async fn main() {
    // 使用 From 构造枚举
    let r1: RespEnum = RespNull.into();
    let r2: RespEnum = RespSet.into();

    // 调用 trait 方法（宏生成的静态分发）
    r1.do_something().await; // 输出: RespNull called

    tokio::spawn(async move { r1.do_something().await });
    tokio::spawn(async move { r2.do_something().await });
    // // 正确匹配元组变体
    let r1: RespEnum = RespNull.into();
    match &r1 {
        RespEnum::RespNull(_) => println!("Matched RespNull"),
        RespEnum::RespSet(_) => println!("Matched RespSet"),
    }
}
