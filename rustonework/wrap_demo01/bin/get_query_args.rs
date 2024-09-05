use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    let query_map =
        warp::query::<HashMap<String, String>>().map(|pairs: HashMap<String, String>| {
            // 处理查询参数
            println!("Query pairs: {:#?}", pairs);
            warp::reply::json(&pairs)
        });

    warp::serve(query_map).run(([127, 0, 0, 1], 8000)).await;
}
