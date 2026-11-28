#!/usr/bin/env bash
# Additional L0 count diagnostics; original L2 specs stay unchanged.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
bin=${IC_FROZEN_ECBENCH:-/workspace/ic-frozen-20261025/ecbench-0a5137c2}
expected=efa25f209e57dd30fe191ddbd3a3c0d8465952cbe0a23b38a9dc7450f8051c9c
actual=$(sha256sum "$bin")
test "${actual%% *}" = "$expected"
export RAYON_NUM_THREADS=4 OMP_NUM_THREADS=1
for m in 14 15; do
  run="$study/pipeline-runs/quartic-m$m-local-counts-20261025-01"
  meta="$study/pipeline-receipts/quartic-m$m-local-counts-20261025-01-launch"
  spec="$study/specs/quartic-m$m-local-counts.json"
  test ! -e "$run"
  test ! -e "$meta"
  mkdir -p "$meta"
  git rev-parse HEAD > "$meta/workspace-head.txt"
  sha256sum "$bin" "$spec" "$study/SUPPORTED_LOCAL_QUARTIC_PROTOCOL.md" > "$meta/hashes.txt"
  printf '%s\n' "$bin run --wait --spec $spec --out $run --cpus none --allow-busy" > "$meta/run-command.txt"
  printf '%s\n' "$bin verify --dir $run --replay-all --exit-code --out $meta/replay.json" > "$meta/replay-command.txt"
  date -u +%FT%TZ > "$meta/started.txt"
  set +e
  (ulimit -v 12582912; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$meta/run-resources.txt" timeout -k 30 14400 "$bin" run --wait --spec "$spec" --out "$run" --cpus none --allow-busy) > "$meta/run.stdout" 2> "$meta/run.stderr"
  run_rc=$?
  printf '{"exit_code":%s,"outer_timeout_seconds":14400,"address_space_bytes":12884901888}\n' "$run_rc" > "$meta/run-status.json"
  (ulimit -v 12582912; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$meta/replay-resources.txt" timeout -k 30 14400 "$bin" verify --dir "$run" --replay-all --exit-code --out "$meta/replay.json") > "$meta/replay.stdout" 2> "$meta/replay.stderr"
  replay_rc=$?
  printf '{"exit_code":%s,"outer_timeout_seconds":14400,"address_space_bytes":12884901888}\n' "$replay_rc" > "$meta/replay-status.json"
  set -e
  date -u +%FT%TZ > "$meta/ended.txt"
  printf 'm=%s run_exit=%s replay_exit=%s; inspect complete native receipts\n' "$m" "$run_rc" "$replay_rc"
done
printf '%s\n' 'Both additional L0 rungs terminated. Visual reports, all failures, complete pricing/replay review and explicit publication/artifact gates remain required. Original L2 requirements stay open.' > "$study/pipeline-receipts/quartic-supported-local-20261025-01-queue/REVIEW_REQUIRED.txt"
