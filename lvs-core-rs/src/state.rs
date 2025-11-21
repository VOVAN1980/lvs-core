use rand::prelude::*;

use crate::{apply_trust_reject, DriftParams, NodeId, NodeState, Tick, Tx, VuAmount};

#[derive(Debug, Clone)]
pub struct NetworkState {
    pub tick: Tick,
    pub nodes: Vec<NodeState>,
    pub drift: DriftParams,
    pub vault_min: VuAmount,
}

impl NetworkState {
    pub fn new(num_nodes: usize, total_vu: VuAmount, vault_min: VuAmount) -> Self {
        let per_node = total_vu / num_nodes as VuAmount;
        let mut nodes = Vec::with_capacity(num_nodes);

        for i in 0..num_nodes {
            nodes.push(NodeState {
                id: i as NodeId,
                vu_self: per_node,
                vu_vault: vault_min,
                tc: 0.5,
                is_validator: false,
            });
        }

        Self {
            tick: 0,
            nodes,
            drift: DriftParams::default(),
            vault_min,
        }
    }

    pub fn random_pair<R: Rng>(&self, rng: &mut R) -> (NodeId, NodeId) {
        let n = self.nodes.len() as NodeId;
        let a = rng.gen_range(0..n);
        let mut b = rng.gen_range(0..n);
        if a == b {
            b = (b + 1) % n;
        }
        (a, b)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> &mut NodeState {
        let idx = self
            .nodes
            .iter()
            .position(|n| n.id == id)
            .expect("node exists");
        &mut self.nodes[idx]
    }

    pub fn apply_tx(&mut self, tx: &Tx) {
        let params = self.drift;
        let vault_min = self.vault_min;

        let (from_ok, to_ok);

        // сначала исходящий
        {
            let from = self.get_node_mut(tx.from);
            from_ok = from.apply_tx_out(tx, params, vault_min);
        }

        if from_ok {
            // если всё норм – зачисляем
            let to = self.get_node_mut(tx.to);
            to.apply_tx_in(tx, params);
            to_ok = true;
        } else {
            // транзакция отвергнута – на стороне получателя тоже падение TC
            let to = self.get_node_mut(tx.to);
            to.tc = apply_trust_reject(to.tc, params);
            to_ok = false;
        }

        let _ = to_ok;
    }

    pub fn tick_once<R: Rng>(&mut self, rng: &mut R, tx_per_tick: usize) {
        for _ in 0..tx_per_tick {
            let (a, b) = self.random_pair(rng);
            let amount = 10.0; // потом сделаем параметром

            let tx = Tx {
                from: a,
                to: b,
                amount_vu: amount,
            };

            self.apply_tx(&tx);
        }

        self.tick += 1;
    }

    pub fn stats(&self) -> (VuAmount, VuAmount, VuAmount, VuAmount) {
        let mut min_vu = f64::MAX;
        let mut max_vu = f64::MIN;
        let mut min_tc = f64::MAX;
        let mut max_tc = f64::MIN;

        for n in &self.nodes {
            if n.vu_self < min_vu {
                min_vu = n.vu_self;
            }
            if n.vu_self > max_vu {
                max_vu = n.vu_self;
            }
            if n.tc < min_tc {
                min_tc = n.tc;
            }
            if n.tc > max_tc {
                max_tc = n.tc;
            }
        }

        (min_vu, max_vu, min_tc, max_tc)
    }
}
