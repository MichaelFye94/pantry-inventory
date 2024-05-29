mod web;

use tokio::runtime::Runtime;
use web::api;

fn main() {
    println!("Hello, world!");

    api::start();
}
