use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    param1: i32,
    param2: i32,
}

#[tokio::main]
async fn main() {
    let routes = warp::path("example")
        .and(warp::query::<Params>())
        .map(|params: Params| format!("{:#?}", params));

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
