import { LvsNode } from "./node";
import { NodeId } from "./types";

const NODE_COUNT = 100;
const TICKS = 50;

function createNodeIds(count: number): NodeId[] {
  const ids: NodeId[] = [];
  for (let i = 0; i < count; i++) {
    ids.push(`node_${i + 1}`);
  }
  return ids;
}

function runSimulation() {
  const nodeIds = createNodeIds(NODE_COUNT);

  const nodes = new Map<NodeId, LvsNode>();
  for (const id of nodeIds) {
    const peers = nodeIds.filter(x => x !== id);
    nodes.set(id, new LvsNode(id, peers));
  }

  console.log(`LVS Core TS demo: ${NODE_COUNT} nodes, ${TICKS} ticks\n`);

  for (let tick = 0; tick < TICKS; tick++) {
    // 1. случайные переводы между узлами
    for (const node of nodes.values()) {
      if (Math.random() < 0.4) {
        const peerId = node.pickRandomPeer();
        if (peerId) {
          const amount = 5 + Math.floor(Math.random() * 20);
          node.sendValue(peerId, amount);
        }
      }
    }

    // 2. обмен снапшотами (грубо имитируем сеть)
    const snapshots = new Map<NodeId, ReturnType<LvsNode["getSnapshot"]>>();
    for (const [id, node] of nodes.entries()) {
      snapshots.set(id, node.getSnapshot());
    }

    for (const [id, node] of nodes.entries()) {
      // каждый узел получает 2 случайных снапшота
      const others = Array.from(snapshots.entries()).filter(([oid]) => oid !== id);
      for (let i = 0; i < 2 && others.length > 0; i++) {
        const idx = Math.floor(Math.random() * others.length);
        const [otherId, snap] = others.splice(idx, 1)[0];
        node.receiveSnapshot(snap);
      }
    }

    // 3. тик у каждого узла
    let totalVu = 0;
    let minVu = Number.POSITIVE_INFINITY;
    let maxVu = Number.NEGATIVE_INFINITY;

    let avgTrust = 0;
    let minTrust = Number.POSITIVE_INFINITY;
    let maxTrust = Number.NEGATIVE_INFINITY;

    for (const node of nodes.values()) {
      const res = node.tick();
      const accounts = node.getAccounts();
      const selfState = accounts.find((a: any) => a.id === node.id)!;

      totalVu += selfState.vu;
      minVu = Math.min(minVu, selfState.vu);
      maxVu = Math.max(maxVu, selfState.vu);

      avgTrust += selfState.tc;
      minTrust = Math.min(minTrust, selfState.tc);
      maxTrust = Math.max(maxTrust, selfState.tc);
    }

    avgTrust /= NODE_COUNT;

    console.log(
      `Tick ${tick + 1}/${TICKS} :: ` +
        `self-VU[min=${minVu.toFixed(2)}, max=${maxVu.toFixed(2)}], ` +
        `self-TC[avg=${avgTrust.toFixed(3)}, min=${minTrust.toFixed(3)}, max=${maxTrust.toFixed(3)}]`
    );
  }

  console.log("\nSimulation finished.");
}

runSimulation();
