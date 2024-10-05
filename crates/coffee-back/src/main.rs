use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::routing::delete;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use dotenv::dotenv;
use rand::random;
use shared::OrderInfo;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("INIT: starting...");
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().ok();
    println!("INIT: dotenv...");

    let db = Db::default();
    println!("INIT: db...");
    // build our application with a route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(Any),
        )
        .route("/up", get(up))
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/order", post(order))
        .route("/complete", delete(complete))
        .route("/fetch", get(fetch))
        .layer(Extension(db));
    println!("INIT: app...");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(env::var("BACK_URI").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("INIT: byebye...");
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello :3 this is a coffee server powered by axum"
}

async fn up() -> StatusCode {
    StatusCode::OK
}

async fn order(Extension(db): Extension<Db>, Json(payload): Json<OrderInfo>) -> impl IntoResponse {
    let rand = random::<u8>();

    let mut payload = payload;
    payload.date = Option::from(SystemTime::now());
    payload.id = Some(rand);
    db.0.write().await.insert(rand, payload);
    (StatusCode::OK, Json(rand))
}

async fn fetch(Extension(db): Extension<Db>) -> impl IntoResponse {
    let mut list: Vec<OrderInfo> = db.0.read().await.iter().map(|a| a.1.clone()).collect();
    list.sort_by_key(|a| a.date);
    (StatusCode::OK, Json(list))
}

async fn complete(Extension(db): Extension<Db>, Json(id): Json<u8>) -> impl IntoResponse {
    let result = db.0.read().await.get(&id).cloned();
    match result {
        None => StatusCode::NOT_FOUND,
        Some(_) => {
            db.0.write().await.remove(&id);
            StatusCode::NO_CONTENT
        }
    }
}

// order id | order info
#[derive(Clone, Default)]
pub struct Db(Arc<RwLock<HashMap<u8, OrderInfo>>>);
