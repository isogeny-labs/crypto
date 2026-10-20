#!/usr/bin/env bash
set -uo pipefail

base=/workspace/n83_distribution
repository=/workspace/crypto
allocation=/tmp/n83_distribution_allocation_s0.json
mkdir -p "$base"
cd "$repository" || exit 70
export PATH=/root/.cargo/bin:$PATH
export PYTHONDONTWRITEBYTECODE=1
export CARGO_TARGET_DIR=/workspace/n83_target

test "$(git rev-parse HEAD)" = e7634a81c22b60ad37e5d6bdb817a3b84867ae86 || exit 71
python3 -m research.n83_highk_online_distribution_20261020.check_dispatch > "$base/dispatch.log" 2>&1 || exit 72
cpus=$(python3 -c 'import json; p=json.load(open("/tmp/n83_distribution_allocation_s0.json")); ids=p["cpu_affinity_ids"]; assert len(ids)==4 and ids==sorted(set(ids)) and all(isinstance(i,int) and i>=0 for i in ids); print(",".join(map(str,ids)))') || exit 73
printf 'START %s\n' "$(date -u +%FT%TZ)" > "$base/run.status"

timeout --signal=TERM --kill-after=30s 24000 taskset -c "$cpus" \
  python3 -m research.n83_highk_online_distribution_20261020.runpod_runner \
  --stratum 0 --phase screen \
  --binary /workspace/n83_target/release/examples/koblitz_orbit_dlp_fast \
  --rho-binary /workspace/n83_target/release/examples/n83_rho_optimization_eval \
  --provider-receipt "$allocation" --output "$base/results" \
  > "$base/screen_run.log" 2>&1
screen_code=$?
printf 'SCREEN_PRODUCER_EXIT %s %s\n' "$screen_code" "$(date -u +%FT%TZ)" > "$base/run.status"

screen_verify_code=99
holdout_code=98
holdout_verify_code=98
if [ -d "$base/results" ]; then
  python3 research/n83_highk_online_distribution_20261020/verify_screen.py \
    "$base/results" > "$base/screen_verify.log" 2>&1
  screen_verify_code=$?
  if [ "$screen_code" -eq 0 ] && [ "$screen_verify_code" -eq 0 ]; then
    candidate=$(python3 -c 'import json,sys; p=json.load(open(sys.argv[1])); print(p["selected_candidate_id"] or "")' "$base/results/screen_selection.json")
    if [ -n "$candidate" ]; then
      timeout --signal=TERM --kill-after=30s 24000 taskset -c "$cpus" \
        python3 -m research.n83_highk_online_distribution_20261020.runpod_runner \
        --stratum 0 --phase holdout --candidate-id "$candidate" \
        --provider-receipt "$allocation" --output "$base/results" \
        > "$base/holdout_run.log" 2>&1
      holdout_code=$?
      python3 research/n83_highk_online_distribution_20261020/verify_holdout.py \
        "$base/results" > "$base/holdout_verify.log" 2>&1
      holdout_verify_code=$?
    else
      printf '{"schema":"n83.online-distribution-not-dispatched/v1","reason":"SCREEN_BOUNDED_UNKNOWN"}\n' \
        > "$base/results/holdout_not_dispatched.json"
    fi
  else
    printf '{"schema":"n83.online-distribution-not-dispatched/v1","reason":"SCREEN_NOT_ADMITTED"}\n' \
      > "$base/results/holdout_not_dispatched.json"
  fi
  tar -C "$base" -czf "$base/raw_results.tar.gz" results
  sha256sum "$base/raw_results.tar.gz" > "$base/raw_results.sha256"
fi
printf 'DONE screen=%s screen_verify=%s holdout=%s holdout_verify=%s %s\n' \
  "$screen_code" "$screen_verify_code" "$holdout_code" "$holdout_verify_code" \
  "$(date -u +%FT%TZ)" > "$base/run.status"
