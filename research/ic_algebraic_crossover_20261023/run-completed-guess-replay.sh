#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
meta="$study/pipeline-receipts/completed-guess-hky-replay-20261025"
bin=/workspace/ic-frozen-20261025/ic_guess_completed_audit
test ! -e "$meta"
mkdir -p "$meta"
git rev-parse HEAD > "$meta/source-context.txt"
sha256sum "$bin" "$study/COMPLETED_GUESS_REPLAY_PROTOCOL.md" "$study/pipeline-receipts/guess-completion-20261025-01/receipt.json" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 HKY_CHUNK=1024
for draw in 0 3 5 6; do
  for k in 0 1 2 3 4; do
    for ((branch=0;branch<(1<<k);branch++)); do
      folder="$meta/d$draw-k$k-b$branch"
      mkdir "$folder"
      printf '%s\n' "$bin case $study $draw $k $branch" > "$folder/command.txt"
      date -u +%FT%TZ > "$folder/started.txt"
      set +e
      (ulimit -v 8388608; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$folder/resources.txt" timeout -k 30 900 "$bin" case "$study" "$draw" "$k" "$branch") > "$folder/output.json" 2> "$folder/stderr.txt"
      rc=$?
      set -e
      printf '{"exit_code":%s,"address_space_bytes":8589934592,"timeout_seconds":900}\n' "$rc" > "$folder/status.json"
      date -u +%FT%TZ > "$folder/ended.txt"
      printf 'draw=%s k=%s branch=%s replay_exit=%s\n' "$draw" "$k" "$branch" "$rc"
    done
  done
done
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'All 124 independent HKY attempts terminated. Inspect every cap/mismatch and preserve producer unknowns/costs. Native aggregate, raw archive, inspected visual report and publication gates remain required. No automatic claim promotion.' > "$meta/REVIEW_REQUIRED.txt"
