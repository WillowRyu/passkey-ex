mod database;
mod routes;
mod utils;

use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database_connection = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database_connection).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
