use axum::Router;
use dotenv::dotenv;

use std::env;
use std::net::SocketAddr;

mod routes;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .unwrap();

    let app = Router::new()
        .merge(routes::keypair::routes());

    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("server is running on port {}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

































// use poem::{get, post, handler, listener::TcpListener, web::Path, Route, Server};

// #[handler]
// fn get_website(Path(website_id): Path<String>) -> String {
//     format!("website! {}", website_id)
// }

// #[handler]
// fn create_website(Path(website_id): Path<String>) -> String {
//     format!("website! {}", website_id)
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error>{
//     let app = Route::new()
//     .at("/status/:website_id", get(create_website))
//     .at("website", post(get_website));
// Server::new(TcpListener::bind("0.0.0.0:3000"))
// .run(app)
// .await
// }
