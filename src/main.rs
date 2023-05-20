use server::run;

mod handlers;
mod middlewares;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    run().await;
}
