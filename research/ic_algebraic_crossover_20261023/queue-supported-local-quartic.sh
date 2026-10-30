#!/usr/bin/env bash
# Freeze the runner before waiting; no cloud provisioning or publication.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
pid=${1:?guessing queue PID}
expected_start=${2:?Linux process start ticks}
test "$pid" = 8353
test "$(awk '{print $22}' "/proc/$pid/stat")" = "$expected_start"
meta="$study/pipeline-receipts/quartic-supported-local-20261025-01-queue"
test ! -e "$meta"
mkdir -p "$meta"
cp "$0" "$meta/queue.sh"
cp "$study/run-supported-local-quartic.sh" "$meta/runner.sh"
git rev-parse HEAD > "$meta/source-commit.txt"
printf '{"watched_pid":%s,"linux_start_ticks":"%s","cloud_commitment_usd":0,"state":"waiting_for_guessing_end_marker"}\n' "$pid" "$expected_start" > "$meta/queued.json"
sha256sum "$meta/queue.sh" "$meta/runner.sh" "$study/SUPPORTED_LOCAL_QUARTIC_PROTOCOL.md" "$study/specs/quartic-m14-local-counts.json" "$study/specs/quartic-m15-local-counts.json" /workspace/ic-frozen-20261025/ecbench-0a5137c2 > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/queued.txt"
while kill -0 "$pid" 2>/dev/null; do
  current_start=$(awk '{print $22}' "/proc/$pid/stat" 2>/dev/null || true)
  if test "$current_start" != "$expected_start"; then
    printf '%s\n' 'Watched process identity changed; no launch.' > "$meta/INCOMPLETE.txt"
    exit 3
  fi
  sleep 30
done
if test ! -e "$study/pipeline-receipts/guess-cap200k-queued-20261025/ended.txt"; then
  printf '%s\n' 'Guessing process ended without its registered end marker; no launch.' > "$meta/INCOMPLETE.txt"
  exit 4
fi
date -u +%FT%TZ > "$meta/released.txt"
IC_FROZEN_ECBENCH=/workspace/ic-frozen-20261025/ecbench-0a5137c2 bash "$meta/runner.sh"
date -u +%FT%TZ > "$meta/ended.txt"
