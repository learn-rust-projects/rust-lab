use std::{sync::mpsc, thread};
const NUM_PRODUCERS: usize = 2;
use anyhow::Result;
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Msg {
    id: usize,
    data: u64,
}
impl Msg {
    fn new(id: usize, data: u64) -> Self {
        Self { id, data }
    }
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Msg>();
    // 创建producer线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(tx, i));
    }
    // 释放tx，确保所有producer线程都完成
    drop(tx);
    println!("Hello, world!");

    // 创建消费者
    let consumer_thread = thread::spawn(|| {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        43
    });
    let consumer_result = consumer_thread
        .join()
        .map_err(|e| anyhow::anyhow!("Consumer thread panicked: {:?}", e))?;
    println!(
        "Consumer thread finished with result: {:?}",
        consumer_result
    );
    Ok(())
}

fn producer(tx: mpsc::Sender<Msg>, id: usize) -> Result<()> {
    loop {
        let value = rand::random::<u64>();
        tx.send(Msg::new(id, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
        if rand::random::<u8>().is_multiple_of(2) {
            println!("Producer {} sent: {:?}", id, Msg::new(id, value));
            break;
        }
    }
    Ok(())
}
