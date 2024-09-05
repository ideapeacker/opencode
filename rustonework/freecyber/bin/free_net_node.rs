use freecyber::get_nodes_from_free_proxy_world;
use freecyber::proxycli;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::prelude::*;
use std::io::LineWriter;

// curl -x socks5://199.102.104.70:4145 -k https://ipinfo.io
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match get_nodes_from_free_proxy_world().await {
        Ok(nodes) => {
            // 根据 last_check 排序
            // 根据 speed 排序
            let mut heap = BinaryHeap::new();
            for node in nodes {
                heap.push(Reverse(node));
            }
            // 获取并移除最小元素（根据 `speed` 字段排序后的最小值）
            let file = std::fs::File::create("abc.txt").unwrap();
            let mut writer = LineWriter::new(file);

            while let Some(item) = heap.pop() {
                let node = item.0;
                let _ = writer.write(format!("{}:{}\n", node.ip(), node.port()).as_bytes());
                //let s = proxycli::forward("127.0.0.1", 1080, node.ip(), node.port()).await;
            }
            // FreeProxyWorld 得到的节点国家不准确, 需重新获取!!!
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}
