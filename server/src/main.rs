use server::run;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL").unwrap_or("DATABASE_URL not set".to_string());
    run(&database_uri).await
}
