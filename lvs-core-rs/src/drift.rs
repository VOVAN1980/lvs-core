use crate::{DriftParams, TcAmount};

pub fn apply_trust_success(tc: TcAmount, params: DriftParams) -> TcAmount {
    let mut v = tc + params.tc_gain_on_success;
    if v > params.tc_max {
        v = params.tc_max;
    }
    v
}

pub fn apply_trust_reject(tc: TcAmount, params: DriftParams) -> TcAmount {
    let mut v = tc - params.tc_loss_on_reject;
    if v < params.tc_min {
        v = params.tc_min;
    }
    v
}
