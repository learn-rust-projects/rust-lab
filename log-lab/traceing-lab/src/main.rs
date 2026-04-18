use std::{fmt::write, io::Write, time::Duration};

use axum::{Router, extract::Request, routing::get};
use tokio::{
    join,
    time::{Instant, sleep},
};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_level(true)
        .with_ansi(true)
        .with_span_events(FmtSpan::FULL)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let file = tracing_appender::rolling::daily("/tmp/logs", "append-test.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        // .with(opentelemetry)
        .init();

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument(fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
async fn index_handler(req: Request) -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status_code = 200, "index handler completed");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    let sl: tokio::time::Sleep = sleep(Duration::from_millis(11));
    // spawn multiple tasks

    let t1 = task1();
    let t2 = task2();
    let t3 = task3();
    join!(sl, t1, t2, t3);
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");
    "Hello, World!"
}

#[instrument]
async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

#[instrument]
async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

#[instrument]
async fn task3() {
    sleep(Duration::from_millis(30)).await;
}
