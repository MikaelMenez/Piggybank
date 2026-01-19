use axum::{
    Router,
    routing::{delete, get, post, put},
};
use http::HeaderValue;
use piggy_bank;
use sqlx::SqlitePool;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteConnectOptions;

use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
#[tokio::main]
async fn main() {
    let db_path = "app.db";

    // Cria DB se não existir
    if !sqlx::Sqlite::database_exists(db_path)
        .await
        .unwrap_or(false)
    {
        sqlx::Sqlite::create_database(db_path).await.unwrap();
        println!("🆕 Banco criado: {}", db_path);
    }
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("app.db")
            .create_if_missing(true),
    )
    .await
    .unwrap();
    sqlx::migrate!("./migrations") // Pasta migrations/
        .run(&pool)
        .await
        .unwrap();
    piggy_bank::create_table_transactions(&pool).await.unwrap();
    let fallbacks = ServeDir::new("./dist").fallback(ServeFile::new("./dist/index.html"));
    let app = Router::new()
        .fallback_service(fallbacks)
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
    let _child1 = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("Server is listening in address : http://127.0.0.1:{adress}/");

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap()
    });
    use std::process::Command;

    println!("Opening browser 🌐");

    // Tenta webbrowser primeiro
    if let Err(_) = webbrowser::open(format!("http://127.0.0.1:46000/").as_str()) {
        // Fallback xdg-open (KDE)
        Command::new("xdg-open")
            .arg("http://127.0.0.1:46000/")
            .spawn()
            .unwrap();
    }

    tokio::signal::ctrl_c().await.unwrap();
}
