import { DriftMergeResult, NodeSnapshot } from "./types";
import { LvsState } from "./state";

/**
 * Простая реализация Drift-консенсуса:
 * - локальный узел получает чужой snapshot
 * - мержит его через LvsState.applySnapshotMerge()
 * - возвращает delta по балансу и доверию
 */
export function applyDrift(
  state: LvsState,
  remoteSnapshot: NodeSnapshot
): DriftMergeResult {
  const before = state.toSnapshot();
  const res = state.applySnapshotMerge(remoteSnapshot);
  const after = state.toSnapshot();

  return {
    mergedSnapshot: after,
    trustDelta: res.trustDelta,
    vuDelta: res.vuDelta
  };
}
