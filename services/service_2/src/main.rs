use sqlx::{Error, PgPool};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let user = query_user(1).await?;
    println!("{:?}", user);
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i16,
}

async fn connect_pg() -> Result<PgPool, Error> {
    let database_url = "postgres://postgres:postgres@localhost:5432/test_db_2";
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(
            std::thread::available_parallelism()
                .ok()
                .and_then(|v| v.get().try_into().ok())
                .unwrap_or(8),
        )
        .connect(database_url)
        .await
}

async fn query_user(id: i32) -> Result<Option<User>, Error> {
    let pool = connect_pg().await?;
    sqlx::query_as!(User, "SELECT * FROM users_2 WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
}
