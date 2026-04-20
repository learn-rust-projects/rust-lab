use std::{env, env::Args};

use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::LOCATION},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, prelude::FromRow};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: MySqlPool,
}

#[derive(Debug, FromRow)]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}
const BASE_URL: &str = "localhost:5432";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_level(true)
        .with_ansi(true)
        .pretty()
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(console).init();

    let listener = tokio::net::TcpListener::bind(BASE_URL).await.unwrap();
    info!("Starting server on {}", BASE_URL);
    let app_state = AppState::try_new(&env::var("DATABASE_URL")?).await?;
    let app = axum::Router::new()
        .route("/", axum::routing::post(shorten))
        .route("/{id}", axum::routing::get(redirect))
        .with_state(app_state);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
#[instrument]
async fn shorten(
    State(state): State<AppState>,
    Json(req): Json<ShortenReq>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = state
        .shorten(&req.url)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    let res = Json(ShortenRes {
        url: format!("http://{}/{}", BASE_URL, id),
    });
    Ok((StatusCode::CREATED, res))
}
#[instrument]
// url里面的参数可以被抽取出一个Path
async fn redirect(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = state.get_url(&id).await.map_err(|e| {
        info!("No URL found for ID {}", e);
        StatusCode::NOT_FOUND
    })?;
    info!("Found URL {} for ID {}", url, id);
    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    info!("Redirecting to {}", url);
    let res = (StatusCode::PERMANENT_REDIRECT, headers).into_response();
    info!("Response prepared for redirection to {}", url);
    Ok(res)
}

impl AppState {
    async fn try_new(url: &str) -> anyhow::Result<Self> {
        println!("Connecting to database at {}", url);
        let pool = MySqlPool::connect(url).await?;
        sqlx::query!(
            r#"
CREATE TABLE IF NOT EXISTS urls (
    id CHAR(6) PRIMARY KEY,
    url VARCHAR(512) NOT NULL UNIQUE
)
"#
        )
        .execute(&pool)
        .await?;
        info!("Database initialized");
        Ok(Self { db: pool })
    }
    async fn direct(&self, id: &str) -> anyhow::Result<String> {
        let record = sqlx::query!(
            r#"
SELECT * FROM urls WHERE id = ?
"#,
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(record.url)
    }

    async fn shorten(&self, url: &str) -> anyhow::Result<String> {
        let id = nanoid::nanoid!(6);
        sqlx::query(
            r#"
        INSERT INTO urls (id, url)
        VALUES (?, ?)
        ON DUPLICATE KEY UPDATE id = id
        "#,
        )
        .bind(&id)
        .bind(url)
        .execute(&self.db)
        .await?;

        let ret = sqlx::query_as::<_, UrlRecord>("SELECT id, url FROM urls WHERE url = ?")
            .bind(url)
            .fetch_one(&self.db)
            .await?;

        Ok(ret.id)
    }
    async fn get_url(&self, id: &str) -> anyhow::Result<String> {
        let ret: UrlRecord = sqlx::query_as("SELECT url FROM urls WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(ret.url)
    }
}
