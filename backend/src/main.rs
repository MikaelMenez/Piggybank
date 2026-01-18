use axum::{
    Router,
    routing::{delete, get, post, put},
};
use http::HeaderValue;
use piggy_bank;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("app.db")
            .create_if_missing(true),
    )
    .await
    .unwrap();
    piggy_bank::create_table_transactions(&pool).await.unwrap();
    let app = Router::new()
        .route("/", get(piggy_bank::index))
        .route("/add_transaction", post(piggy_bank::add_transaction))
        .route("/transactions", get(piggy_bank::get_transactions))
        .route(
            "/transactions/by_tipo/{tipo}",
            get(piggy_bank::all_of_kinds),
        )
        .route(
            "/transactions/by_date/{mes}/{ano}",
            get(piggy_bank::filter_by_date),
        )
        .route(
            "/modify_transaction/{id}",
            put(piggy_bank::modify_transactions),
        )
        .route(
            "/delete_transaction/{id}",
            delete(piggy_bank::del_transactions),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _| {
                    let origin_str = origin.to_str().unwrap_or("");
                    origin_str.starts_with("http://127.0.0.1")
                        || origin_str.starts_with("http://localhost")
                }))
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .with_state(pool);
    let addr = "0.0.0.0:46000";
    let adress = addr.strip_prefix("0.0.0.0:").unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server is listening in address : http://127.0.0.1:{adress}/");
    axum::serve(listener, app).await.unwrap();
}
