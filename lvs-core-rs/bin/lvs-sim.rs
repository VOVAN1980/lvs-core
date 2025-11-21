use rand::prelude::*;
use lvs_core::NetworkState;

fn main() {
    let num_nodes: usize = 100;
    let ticks: u64 = 50;

    let mut net = NetworkState::new(num_nodes, 100_000.0, 1.0);
    let mut rng = StdRng::from_entropy();

    println!("LVS Core Rust demo: {} nodes, {} ticks", num_nodes, ticks);

    for t in 1..=ticks {
        net.tick_once(&mut rng, 50);

        let (min_vu, max_vu, min_tc, max_tc) = net.stats();
        let avg_tc = net.nodes.iter().map(|n| n.tc).sum::<f64>() / net.nodes.len() as f64;

        println!(
            "Tick {}/{} :: self-VU[min={:.2}, max={:.2}], self-TC[avg={:.3}, min={:.3}, max={:.3}]",
            t, ticks, min_vu, max_vu, avg_tc, min_tc, max_tc
        );
    }

    println!("Simulation finished.");
}
