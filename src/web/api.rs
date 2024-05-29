use axum::{routing::get, Router};

#[tokio::main]
pub async fn start(){
    println!("Starting api...");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World! This is an API!" }));
    
    // run our app with hyper, listening globally on port 3000
    println!("Running on http://localhost:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}