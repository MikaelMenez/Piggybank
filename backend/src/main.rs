use axum::{
    Router,
    routing::{delete, get, post, put},
};
use piggy_bank;
#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite:app.db").await.unwrap();
    let app = Router::new()
        .route("/", get(piggy_bank::index))
        .route("/add_transaction", post(piggy_bank::add_transaction))
        .route("/transactions", get(piggy_bank::get_transactions))
        .route("/transactions/{tipo}", get(piggy_bank::all_of_kinds))
        .route(
            "/modify_transaction/{id}",
            put(piggy_bank::modify_transactions),
        )
        .route(
            "/delete_transaction/{id}",
            delete(piggy_bank::del_transactions),
        )
        .with_state(pool);
    let addr = "0.0.0.0:3000";
    let adress = addr.strip_prefix("0.0.0.0:").unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server is listening in address : http://127.0.0.1:{adress}/");
    axum::serve(listener, app).await.unwrap();
}
