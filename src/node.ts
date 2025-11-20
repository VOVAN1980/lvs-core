import { LvsState } from "./state";
import { NodeId, NodeSnapshot } from "./types";
import { applyDrift } from "./drift";

export class LvsNode {
  readonly id: NodeId;
  private state: LvsState;
  private readonly peers: NodeId[];

  // буфер входящих снапшотов (будто пришли по сети)
  private inbox: NodeSnapshot[] = [];

  constructor(id: NodeId, peers: NodeId[]) {
    this.id = id;
    this.peers = peers.filter(p => p !== id);
    this.state = new LvsState(id, peers);
  }

  getSnapshot(): NodeSnapshot {
    return this.state.toSnapshot();
  }

  receiveSnapshot(snapshot: NodeSnapshot) {
    this.inbox.push(snapshot);
  }

  getTick(): number {
    return this.state.getTick();
  }

  getEntropy(): number {
    return this.state.getEntropy();
  }

  getAccounts() {
    return this.state.getAccounts();
  }

  /** Инициирует локальный перевод VU к другому узлу */
  sendValue(to: NodeId, amount: number): boolean {
    const tx = this.state.applyLocalTransfer(to, amount);
    return !!tx;
  }

  /** Один шаг времени узла */
  tick(): { mergedFrom: number; vuDelta: number; trustDelta: number } {
    this.state.incrementTick();

    let mergedFrom = 0;
    let totalVuDelta = 0;
    let totalTrustDelta = 0;

    while (this.inbox.length > 0) {
      const snap = this.inbox.shift()!;
      const res = applyDrift(this.state, snap);
      mergedFrom++;
      totalVuDelta += res.vuDelta;
      totalTrustDelta += res.trustDelta;
    }

    return {
      mergedFrom,
      vuDelta: totalVuDelta,
      trustDelta: totalTrustDelta
    };
  }

  pickRandomPeer(): NodeId | null {
    if (this.peers.length === 0) return null;
    const idx = Math.floor(Math.random() * this.peers.length);
    return this.peers[idx];
  }
}
