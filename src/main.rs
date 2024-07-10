// export DATABASE_URL="postgresql://admin@localhost:26257/seq_test?sslmode=require"
// export DATABASE_URL=postgresql://admin@localhost:26257/seq_test
mod schema;
mod ds;
mod server;
mod pool;
use axum;
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = pool::Pool::new(database_url, 100).unwrap();
    let server = server::mk_server(db_pool).await?;
    axum::Server::bind(&"127.0.0.1:7777".parse().unwrap())
        .serve(server.into_make_service())
        .await
        .unwrap();
    Ok(())
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let mut connection = diesel::PgConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));


    // let _ = create_payment_flow(&mut connection, "test".to_string());
    // let _ = fetch_payments_flow(&mut connection,"test".to_string());
}