// Базовые типы LVS-узла

export type NodeId = string;

export interface AccountState {
  id: NodeId;
  vu: number;      // Value Units
  tc: number;      // Trust Credits
}

export interface Transfer {
  id: string;
  from: NodeId;
  to: NodeId;
  amount: number;
  createdAt: number;
}

export interface NodeSnapshot {
  nodeId: NodeId;
  tick: number;
  accounts: AccountState[];
  entropy: number;
}

export interface DriftMergeResult {
  mergedSnapshot: NodeSnapshot;
  trustDelta: number;
  vuDelta: number;
}
