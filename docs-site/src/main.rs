use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DbErr};


const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

async fn run () -> Result<(), DbErr> {
    let mut opt = ConnectOptions::new(DATABASE_URL);
opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Info)
    .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

let db = Database::connect(opt).await?;

|db: DatabaseConnection| {
    assert!(db.ping().await.is_ok());
    db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
}

Ok(())

}


fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
