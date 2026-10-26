#!/usr/bin/env bash
set -euo pipefail
if [[ ${IC_HKY_BATCH_WATCHDOG:-} != 1 ]]; then
  export IC_HKY_BATCH_WATCHDOG=1
  exec timeout -k 60 250000 bash "$0" "$@"
fi
cd "$(git rev-parse --show-toplevel)"
ic_study=research/ic_algebraic_crossover_20261023
ic_run="$ic_study/pipeline-receipts/completed-guess-hky-restart-20261026"
ic_bin=/workspace/ic-frozen-20261026/ic_guess_completed_audit-restart
ic_meta="$ic_study/hky-restart-records.py"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 HKY_CHUNK=1024
export CHECKPOINT_TOKEN_FILE=/workspace/.ic-private-checkpoint/upload-token
test -f "$CHECKPOINT_TOKEN_FILE"
python3 "$ic_meta" guard
if [[ ${1:-} == --resume ]]; then test -d "$ic_run"; else test ! -e "$ic_run"; mkdir -p "$ic_run"; fi
exec 9>"$ic_run/.batch.lock"
flock -n 9 || { echo 'This batch already has a live owner.' >&2; exit 1; }
if [[ ${1:-} == --resume ]]; then python3 "$ic_meta" restore; fi
date -u +%FT%TZ > "$ic_run/runner-started-$(date -u +%Y%m%dT%H%M%SZ).txt"
python3 "$ic_meta" host
sha256sum "$ic_bin" "$ic_study/protocol-hky-restart-20261026.json" "$ic_study/inputs/guess-ell8.jsonl" > "$ic_run/pins.txt"
ic_manifest() {
  (cd "$ic_run"
    rg --files -uu -g '!SHA256SUMS*' -g '!.batch.lock' -g '!checkpoint-transport.txt' | LC_ALL=C sort | xargs -d '\n' sha256sum > SHA256SUMS.tmp
    sync -f SHA256SUMS.tmp
    mv SHA256SUMS.tmp SHA256SUMS
    sync -f .)
}
ic_checkpoint() {
  ic_manifest
  bash scripts/checkpoint.sh gate >> "$ic_run/checkpoint-transport.txt" 2>&1
}
ic_refresh() {
  "$ic_bin" replay-summary "$ic_study" "$ic_run" > "$ic_run/native-summary.json.tmp"
  mv "$ic_run/native-summary.json.tmp" "$ic_run/native-summary.json"
  python3 "$ic_meta" snapshot
}
ic_checkpoint
ic_ordinal=0
for ic_draw in 0 3 5 6; do
  for ic_k in 0 1 2 3 4; do
    for ((ic_branch=0;ic_branch<(1<<ic_k);ic_branch++)); do
      if ((ic_ordinal%2==0)); then ic_arms=(hky hky-aa); else ic_arms=(hky-aa hky); fi
      for ic_arm in "${ic_arms[@]}"; do
        ic_folder="$ic_run/$ic_arm/d$ic_draw-k$ic_k-b$ic_branch"
        if test -f "$ic_folder/ended.txt"; then continue; fi
        python3 "$ic_meta" guard
        mkdir -p "$ic_folder"
        cp "$ic_run/current-host.txt" "$ic_folder/host-record.txt"
        printf '%s\n' "$ic_bin case $ic_study $ic_draw $ic_k $ic_branch" > "$ic_folder/command.txt"
        date -u +%FT%TZ > "$ic_folder/started.txt"
        ic_checkpoint
        set +e
        python3 tools/isolated_bench.py busy -- bash -c '
          ulimit -v 8388608
          exec /workspace/.tools/usr/bin/time -f "wall_seconds=%e\nmax_rss_kib=%M\nexit=%x" -o "$1" timeout -k 30 900 "$2" case "$3" "$4" "$5" "$6"
        ' bash "$ic_folder/resources.txt" "$ic_bin" "$ic_study" "$ic_draw" "$ic_k" "$ic_branch" > "$ic_folder/output.json" 2> "$ic_folder/stderr.txt"
        ic_rc=$?
        set -e
        printf '{"exit_code":%s,"address_space_bytes":8589934592,"timeout_seconds":900,"wall_valid":false}\n' "$ic_rc" > "$ic_folder/status.json"
        date -u +%FT%TZ > "$ic_folder/ended.txt"
        ic_refresh
        ic_checkpoint
        printf 'arm=%s draw=%s k=%s branch=%s exit=%s\n' "$ic_arm" "$ic_draw" "$ic_k" "$ic_branch" "$ic_rc"
      done
      ic_ordinal=$((ic_ordinal+1))
    done
  done
done
ic_refresh
python3 "$ic_meta" final
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export "$ic_study/exports/hky-restart-20261026/export.json" --protocol "$ic_study/protocol-hky-restart-20261026.json" --verify-files > "$ic_run/export-validation.txt" 2>&1
date -u +%FT%TZ > "$ic_run/ended.txt"
printf '%s\n' 'All248registeredprocesses terminated or retain interruption/caps. Inspect every raw result, native summary, A/A pair, original censored cost and export. Archive complete raw tree, independently review and inspect visual report/PDF. No automatic promotion.' > "$ic_run/REVIEW_REQUIRED.txt"
ic_checkpoint
