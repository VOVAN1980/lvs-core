use serde::{Deserialize, Serialize};

pub type NodeId = u32;
pub type Tick = u64;

// VU и TC держим в f64 – как в TS (number)
pub type VuAmount = f64;
pub type TcAmount = f64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VaultGuard {
    pub min_vu: VuAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub id: NodeId,
    pub vu_self: VuAmount,
    pub vu_vault: VuAmount,
    pub tc: TcAmount,
    pub is_validator: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub from: NodeId,
    pub to: NodeId,
    pub amount_vu: VuAmount,
}

/// Параметры модели дрейфа доверия (под них мы потом 1:1 перенесём формулы из drift.ts)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DriftParams {
    pub tc_gain_on_success: TcAmount,
    pub tc_loss_on_reject: TcAmount,
    pub tc_min: TcAmount,
    pub tc_max: TcAmount,
}

impl Default for DriftParams {
    fn default() -> Self {
        Self {
            tc_gain_on_success: 0.002,
            tc_loss_on_reject: 0.006,
            tc_min: 0.0,
            tc_max: 1.0,
        }
    }
}
