#!/usr/bin/env bash
# Release one heavy local program after the recorded quartic queue ends.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pid=${1:?quartic queue PID}
registered_start=${2:?Linux start ticks}
study=research/ic_algebraic_crossover_20261023
meta="$study/pipeline-receipts/completed-guess-hky-replay-20261025-queue"
test ! -e "$meta"
mkdir "$meta"
cp "$0" "$meta/queue.sh"
cp "$study/run-completed-guess-replay.sh" "$meta/runner.sh"
git rev-parse HEAD > "$meta/source-context.txt"
sha256sum "$meta/queue.sh" "$meta/runner.sh" /workspace/ic-frozen-20261025/ic_guess_completed_audit > "$meta/hashes.txt"
printf '{"watched_pid":%s,"linux_start_ticks":"%s","state":"waiting_for_supported_quartic_end_marker","cloud_commitment_usd":0}\n' "$pid" "$registered_start" > "$meta/queued.json"
date -u +%FT%TZ > "$meta/queued.txt"
while test -r "/proc/$pid/stat"; do
  current=$(awk '{print $22}' "/proc/$pid/stat")
  if test "$current" != "$registered_start"; then break; fi
  sleep 30
done
if ! test -e "$study/pipeline-receipts/quartic-supported-local-20261025-01-queue/REVIEW_REQUIRED.txt"; then
  printf '%s\n' 'Quartic process ended without both-rung review marker. No replay launch; inspect incomplete parent metadata.' > "$meta/INCOMPLETE.txt"
  exit 1
fi
date -u +%FT%TZ > "$meta/released.txt"
bash "$meta/runner.sh"
date -u +%FT%TZ > "$meta/ended.txt"
