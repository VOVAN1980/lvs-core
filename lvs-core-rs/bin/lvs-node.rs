use std::time::Duration;
use rand::prelude::*;
use lvs_core::{NetworkState};
use serde_json::json;

// сетевой модуль
use tokio::runtime::Runtime;
use lvs_core::net::{connect_node, send};


fn main() {
    // --- запускаем асинхронный WebSocket-клиент ---
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        if let Err(e) = connect_node("ws://127.0.0.1:9001/ws").await {
            eprintln!("WS error: {}", e);
        }
    });

    // --- ID ноды ---
    let node_id = std::env::var("LVS_NODE_ID").unwrap_or_else(|_| "node-1".to_string());
    println!("Starting LVS node: {}", node_id);

    // --- Локальная мини-сеть (12 агентов) ---
    let mut net = NetworkState::new(12, 10_000.0, 1.0);
    let mut rng = StdRng::from_entropy();

    // --- Основной цикл тиков ---
    for t in 1..=20 {
        net.tick_once(&mut rng, 20);

        let (min_vu, max_vu, min_tc, max_tc) = net.stats();
        let avg_tc =
            net.nodes.iter().map(|n| n.tc).sum::<f64>() / net.nodes.len() as f64;

        // Локальный вывод
        println!(
            "[{}] Tick {} :: VU[{:.2} .. {:.2}], TC[avg={:.3}, min={:.3}, max={:.3}]",
            node_id, t, min_vu, max_vu, avg_tc, min_tc, max_tc
        );

        // --- JSON-пакет на сервер ---
        let pkt = json!({
            "node": node_id,
            "tick": t,
            "vu_min": min_vu,
            "vu_max": max_vu,
            "tc_avg": avg_tc
        })
        .to_string();

        // отправка async
        rt.spawn(async move {
            let _ = send(pkt).await;
        });

        std::thread::sleep(Duration::from_millis(250));
    }

    println!("Node {} shutdown.", node_id);
}
