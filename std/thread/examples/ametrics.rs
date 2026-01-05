use anyhow::Result;
use rand::Rng;
use thread::AmapMetrics;
const N_WORKERS: usize = 4;
const M_WORKERS: usize = 4;
fn main() -> Result<()> {
    let metric = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "call.thread.worker.2",
        "call.thread.worker.3",
        "call.process_requse.page.1",
        "call.process_requset.page.2",
        "call.process_requset.page.3",
        "call.process_requset.page.4",
    ]);
    println!("{metric}");
    for idx in 0..N_WORKERS {
        task_worker(idx, metric.clone())?;
    }
    for _ in 0..M_WORKERS {
        requset_woker(metric.clone())?;
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
        println!("{metric}");
    }
}

fn task_worker(idx: usize, metric: AmapMetrics) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(std::time::Duration::from_millis(
                rng.random_range(100..5000),
            ));
            metric.inc(format!("call.thread.worker.{idx}"))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn requset_woker(metrics: AmapMetrics) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("call.process_requset.page.{page}"))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
