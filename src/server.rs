use std::net::SocketAddr;

use crate::routes::create_routes;

pub async fn run() {
    // build our application with a single route
    let app = create_routes();

    // run it with hyper on localhost:3000
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
