use std::rc::Rc;
async fn bar() {}
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}

fn require_send(_: impl Send) {}
#[derive(Default)]
struct NotSend(Rc<()>);

fn main() {
    require_send(foo());
}
