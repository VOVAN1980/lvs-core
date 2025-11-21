use crate::{apply_trust_reject, apply_trust_success, DriftParams, NodeState, Tx, VuAmount};

impl NodeState {
    pub fn can_send(&self, amount: VuAmount, vault_min: VuAmount) -> bool {
        // защищённый минимум – нельзя уйти ниже
        self.vu_self - amount >= vault_min
    }

    pub fn apply_tx_out(&mut self, tx: &Tx, params: DriftParams, vault_min: VuAmount) -> bool {
        if !self.can_send(tx.amount_vu, vault_min) {
            // отказ – не хватает свободного VU
            self.tc = apply_trust_reject(self.tc, params);
            return false;
        }
        self.vu_self -= tx.amount_vu;
        self.tc = apply_trust_success(self.tc, params);
        true
    }

    pub fn apply_tx_in(&mut self, tx: &Tx, params: DriftParams) {
        self.vu_self += tx.amount_vu;
        self.tc = apply_trust_success(self.tc, params);
    }
}
