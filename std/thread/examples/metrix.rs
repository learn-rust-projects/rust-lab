use anyhow::Result;
use rand::Rng;
use thread::Metric;
const N_WORKERS: usize = 4;
const M_WORKERS: usize = 4;
fn main() -> Result<()> {
    let metric = Metric::new();
    println!("{}", metric);
    for idx in 0..N_WORKERS {
        task_worker(idx, metric.clone())?;
    }
    for _ in 0..M_WORKERS {
        requset_woker(metric.clone())?;
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
        println!("{}", metric);
    }
}

fn task_worker(idx: usize, metric: Metric) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(std::time::Duration::from_millis(
                rng.random_range(100..5000),
            ));
            metric.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn requset_woker(metrics: Metric) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("call.process_requse.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
