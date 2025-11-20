import { AccountState, NodeId, Transfer, NodeSnapshot } from "./types";

export class LvsState {
  readonly selfId: NodeId;
  private accounts: Map<NodeId, AccountState>;
  private entropy: number;
  private currentTick = 0;
  private transfers: Transfer[] = [];

  constructor(selfId: NodeId, initialPeers: NodeId[]) {
    this.selfId = selfId;
    this.accounts = new Map();
    this.entropy = Math.random();

    const all = new Set<NodeId>([selfId, ...initialPeers]);
    for (const id of all) {
      this.accounts.set(id, {
        id,
        vu: id === selfId ? 1000 : 0,   // стартовый VU только у себя
        tc: id === selfId ? 1 : 0.5     // базовое доверие
      });
    }
  }

  getTick(): number {
    return this.currentTick;
  }

  getEntropy(): number {
    return this.entropy;
  }

  getAccounts(): AccountState[] {
    return Array.from(this.accounts.values());
  }

  getAccount(id: NodeId): AccountState | undefined {
    return this.accounts.get(id);
  }

  getTransfers(): Transfer[] {
    return this.transfers;
  }

  incrementTick(): void {
    this.currentTick++;
    // простая псевдо-энтропия
    this.entropy = (this.entropy * 9301 + 49297) % 233280 / 233280;
  }

  applyLocalTransfer(to: NodeId, amount: number): Transfer | null {
    const fromId = this.selfId;
    const from = this.accounts.get(fromId);
    const toAcc = this.accounts.get(to);

    if (!from || !toAcc) return null;
    if (amount <= 0) return null;
    if (from.vu - amount < 0) return null;

    from.vu -= amount;
    toAcc.vu += amount;

    // примитивная модель доверия
    from.tc = Math.max(0, from.tc - amount * 0.0001);
    toAcc.tc = Math.min(toAcc.tc + amount * 0.0001, 10);

    const transfer: Transfer = {
      id: `tx_${fromId}_${to}_${Date.now()}_${Math.floor(Math.random() * 1e6)}`,
      from: fromId,
      to,
      amount,
      createdAt: Date.now()
    };
    this.transfers.push(transfer);
    return transfer;
  }

  applySnapshotMerge(snapshot: NodeSnapshot): { vuDelta: number; trustDelta: number } {
    let vuDelta = 0;
    let trustDelta = 0;

    for (const incoming of snapshot.accounts) {
      const local = this.accounts.get(incoming.id);
      if (!local) {
        this.accounts.set(incoming.id, { ...incoming });
        vuDelta += incoming.vu;
        trustDelta += incoming.tc;
        continue;
      }

      const oldVu = local.vu;
      const oldTc = local.tc;

      const weightLocal = local.tc + 0.001;
      const weightIncoming = incoming.tc + 0.001;
      const wSum = weightLocal + weightIncoming;

      const newVu = (local.vu * weightLocal + incoming.vu * weightIncoming) / wSum;
      const newTc = (local.tc * 0.8 + incoming.tc * 1.2) / 2;

      local.vu = newVu;
      local.tc = newTc;

      vuDelta += newVu - oldVu;
      trustDelta += newTc - oldTc;
    }

    return { vuDelta, trustDelta };
  }

  toSnapshot(): NodeSnapshot {
    return {
      nodeId: this.selfId,
      tick: this.currentTick,
      accounts: this.getAccounts(),
      entropy: this.entropy
    };
  }
}
