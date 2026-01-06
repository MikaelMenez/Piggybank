use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
pub async fn all_of_kinds(
    State(pool): State<SqlitePool>,
    Path(tipo): Path<String>,
) -> Result<Json<Vec<Transactionjson>>, (StatusCode, String)> {
    let transactions =
        sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE tipo=?", tipo)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Erro ao buscar membros {}", e),
                )
            })?;

    Ok(Json(
        transactions
            .iter()
            .map(|x| Transactionjson {
                id: x.id,
                valor: x.valor as f64 / 100.0,
                tipo: x.tipo.clone(),
            })
            .collect(),
    ))
}
pub async fn create_table_transactions(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tipo TEXT NOT NULL,
            valor INTEGER NOT NULL
            )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}
use sqlx::{FromRow, SqlitePool};
pub async fn put_transactions(
    pool: &SqlitePool,
    transac: &mut Transactionjson,
) -> Result<(), sqlx::Error> {
    create_table_transactions(pool).await?;
    transac.valor *= 100.0;
    let valorint = transac.valor as i64;
    sqlx::query!(
        r#"
           UPDATE transactions
           SET tipo = ?, valor= ?
           WHERE id = ?
           "#,
        transac.tipo,
        valorint,
        transac.id
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn delete_transactions(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    create_table_transactions(pool).await?;
    sqlx::query!(
        r#"
        DELETE FROM transactions WHERE id = ?
           "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn del_transactions(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    if let Err(e) = create_table_transactions(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    match delete_transactions(&pool, path).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

pub async fn modify_transactions(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
    Json(mut membro): Json<Transactionjson>,
) -> impl IntoResponse {
    if let Err(e) = create_table_transactions(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    membro.id = Some(path);
    match put_transactions(&pool, &mut membro).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

pub async fn get_transactions(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Transactionjson>>, (StatusCode, String)> {
    let transactions = sqlx::query_as!(Transaction, "SELECT * FROM transactions")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao buscar membros {}", e),
            )
        })?;
    Ok(Json(
        transactions
            .iter()
            .map(|x| Transactionjson {
                id: x.id,
                valor: x.valor as f64 / 100.0,
                tipo: x.tipo.clone(),
            })
            .collect(),
    ))
}
pub async fn insert_transactions(
    pool: &SqlitePool,
    transac: &mut Transactionjson,
) -> Result<(), sqlx::Error> {
    create_table_transactions(pool).await?;
    transac.valor = transac.valor * 100.0;
    let valorint = transac.valor as i64;
    sqlx::query!(
        r#"
           INSERT INTO transactions
           (tipo, valor)
           VALUES (?, ?)
           "#,
        transac.tipo,
        valorint,
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    if let Err(e) = create_table_transactions(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }

    "hello world".into_response()
}
pub async fn add_transaction(
    State(pool): State<SqlitePool>,
    Json(membro): Json<Transactionjson>,
) -> impl IntoResponse {
    let mut membrojson = membro;
    if let Err(e) = create_table_transactions(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    match insert_transactions(&pool, &mut membrojson).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Transaction {
    id: Option<i64>,
    tipo: String,
    valor: i64,
}
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Transactionjson {
    id: Option<i64>,
    tipo: String,
    valor: f64,
}
