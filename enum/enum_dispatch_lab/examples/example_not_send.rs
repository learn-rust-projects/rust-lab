use std::rc::Rc;

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
        let x = Rc::new(1);
        bar().await;
        println!("{}", x);
        println!("RespSet called");
    }
}
// // 将标记的内容加上，就不是send 了
async fn bar() {
    let x = Rc::new(1);
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
    let r1: RespEnum = RespNull.into();
    r2.do_something().await;
    RespNull.do_something().await; //可行
    // spawn_local 明确承诺：任务永远不会被移动到其他线程
    tokio::task::spawn_local(async move {
        r1.do_something().await;
    });
    // tokio::spawn(async move { r1.do_something().await });
}
