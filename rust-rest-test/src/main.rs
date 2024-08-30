use axum::{
    extract::{self, Path},
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use redis::{Client, Commands, RedisResult};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> RedisResult<()> {
    let redis_client = redis::Client::open("redis://localhost:6380")?;

    let client = Box::leak(Box::new(redis_client));
    let repository = Box::leak(Box::new(Repository { client }));
    let service = Box::leak(Box::new(Service { repository }));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let router = Router::new()
        .route("/page", post(|body| service.post_message(body)))
        .route("/:id", get(|key| service.get_message(key)))
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    println!("listen");
    axum::serve(listener, router).await.unwrap();
    println!("stop");

    Ok(())
}

struct Service {
    repository: &'static Repository,
}

impl Service {
    pub async fn post_message(&self, page_request: Json<Page>) -> extract::Json<LinkResponse> {
        let link = Service::get_key().await;
        self.repository
            .save_page(&link, &page_request.data)
            .await
            .unwrap();

        return Json::from(LinkResponse { link });
    }

    pub async fn get_message(&self, Path(key): Path<String>) -> extract::Json<PageResponse> {
        let page = self.repository.get_page(key).await;
        return Json::from(PageResponse { page });
    }

    async fn get_key() -> String {
        return rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
    }
}

struct Repository {
    client: &'static Client,
}

impl Repository {
    async fn save_page(&self, key: &String, value: &String) -> RedisResult<()> {
        let mut con = self.client.get_connection()?;
        con.set(key, value)?;
        Ok(())
    }

    async fn get_page(&self, key: String) -> String {
        let mut con = self.client.get_connection().unwrap();
        con.get(key).unwrap_or("".to_string())
    }
}

#[derive(Deserialize)]
struct Page {
    data: String,
    user: String,
}

#[derive(Serialize)]
struct LinkResponse {
    link: String,
}

#[derive(Serialize)]
struct PageResponse {
    page: String,
}
