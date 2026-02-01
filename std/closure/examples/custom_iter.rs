use std::vec;

struct MyIter {
    data: Vec<i32>,
    pos: usize,
}

impl MyIter {
    fn new(data: Vec<i32>) -> Self {
        Self { data, pos: 0 }
    }

    // 实现了自定义的 fold 方法
    // F 泛型支持传入闭包或闭包引用 (&mut closure)
    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, i32) -> B,
    {
        for x in self {
            init = f(init, x);
        }
        init
    }

    // 实现了自定义的 for_each 方法
    fn for_each<F>(self, mut f: &mut F)
    where
        F: FnMut(i32),
    {
        // 辅助函数：将 FnMut(T) 适配为 fold 需要的 FnMut((), T)
        #[inline]
        fn call<T>(f: &mut impl FnMut(T)) -> impl FnMut((), T) {
            //  move |(), item| f(item) 相当于一个表达式，构建匿名结构体
            move |(), item| f(item)
        }

        self.fold((), call(&mut f));
    }
}

// Iterator 仅用于提供 next 方法
impl Iterator for MyIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.data.len() {
            let item = self.data[self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn main() {
    let iter = MyIter::new(vec![1, 2, 3, 4, 5]);

    let mut acc: i32 = 0;
    let mut closure = |x| {
        acc += x;
    };
    iter.for_each(&mut closure);
    println!("acc = {}", acc);
    // 传入闭包的引用 &mut closure
    // 这里 F 被推导为 &mut [closure type]

    let binding = [1, 2, 3, 4, 5];
    let iter = binding.iter();

    let mut closure = |x| {
        acc += x;
    };
    iter.for_each(&mut closure);
    println!("acc = {}", acc);

    // fn_once
    let x = Box::new(10);
    let mut fn_once_closure = || {
        println!("x = {}", x);
    };
    let fn_once_closure_ref = &mut fn_once_closure;

    let mut y = 0;
    let mut fn_mut_closure = || {
        y += 1;
    };
    println!("test start:| `F: FnMut` | `&mut F` | impl `FnMut` / `FnOnce`        |");
    // 注意这里传入的是 &mut FnMut => FnMut
    fn_mut_example(&mut fn_mut_closure);

    // 编译错误
    // cannot borrow `y` as immutable because it is also borrowed as mutable
    // immutable borrow occurs here
    // println!("call three = {}", y);

    // 注意这里传入的是 &mut FnMut => FnOnce
    fn_once_example(fn_once_closure_ref);
    println!("call four = {}", y);
    println!("test end:| `F: FnMut` | `&mut F` | impl `FnMut` / `FnOnce`        |");

    let x = 10;
    let fn_closure = || {
        println!("Hello from Fn closure {}", x);
    };
    let mut fn_ref = &fn_closure;

    println!("test start:| `F: Fn`    | `&F`     | impl `Fn` / `FnMut` / `FnOnce` |");
    fn_example(fn_ref);
    fn_mut_example(fn_ref);
    let mut fn_ref_2 = *fn_ref;
    fn_mut_example(&mut fn_ref_2);
    fn_once_example(fn_ref);
    fn_mut_example_ref(&mut (fn_ref));
    println!("test end:| `F: Fn`    | `&F`     | impl `Fn` / `FnMut` / `FnOnce` |");

    println!("test start:这里不能跨线程直接 move,因为 &dyn Fn() 不是 Send,但是可以本地调用");
    let closure = || println!("ref closure can not be moved to another thread");
    let ref_closure: &dyn Fn() = &closure;
    // 这里不能跨线程直接 move，因为 &dyn Fn() 不是 Send
    // std::thread::spawn(move || ref_closure()); // ❌ 编译错误
    ref_closure();
    println!("test end:这里不能跨线程直接 move,因为 &dyn Fn() 不是 Send,但是可以本地调用");
}
// fn_example
fn fn_example<F>(f: F)
where
    F: Fn(),
{
    println!("Hello from fn_example closure call one");
    f();
}

// fn_once_example
fn fn_once_example<F>(f: F)
where
    F: FnOnce(),
{
    println!("Hello from fn_once_example closure call one");
    f();
    // BOX -> drop
    // f();
}

// fn_mut_example
fn fn_mut_example<F>(mut f: F)
where
    F: FnMut(),
{
    println!("Hello from fn_mut_example closure call three");
    f();
    f();
    f();
}

fn fn_mut_example_ref<F>(f: &mut F)
where
    F: FnMut(),
{
    println!("Hello from fn_mut_example_ref closure call one");
    f();
}

// struct Wrapper<'a> {
//     value: &'a i32,
// }

// fn bad_ref(value: &i32) -> &Wrapper {
// Wrapper { value } 是一个未绑定到变量的临时值，表达式：Wrapper { value
// }结束后会被丢弃     &Wrapper { value }
// }
