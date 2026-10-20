#!/usr/bin/env bash
set -uo pipefail

base=/workspace/n83_tiebreak
repository=/workspace/crypto
allocation=/tmp/n83_tiebreak_allocation_s0.json
trigger=/tmp/n83_tiebreak_trigger.json
mkdir -p "$base"
cd "$repository" || exit 70
export PATH=/root/.cargo/bin:$PATH
export PYTHONDONTWRITEBYTECODE=1
export CARGO_TARGET_DIR=/workspace/n83_target

test "$(git rev-parse HEAD)" = e7634a81c22b60ad37e5d6bdb817a3b84867ae86 || exit 71
python3 -m research.n83_highk_online_tiebreak_20261020.run --fixture-preflight \
  > "$base/fixture_preflight.log" 2>&1 || exit 72
cpus=$(python3 -c 'import json; p=json.load(open("/tmp/n83_tiebreak_allocation_s0.json")); ids=p["cpu_affinity_ids"]; assert len(ids)==4 and ids==sorted(set(ids)) and all(isinstance(i,int) and i>=0 for i in ids); print(",".join(map(str,ids)))') || exit 73
printf 'START %s\n' "$(date -u +%FT%TZ)" > "$base/run.status"

timeout --signal=TERM --kill-after=30s 14400 taskset -c "$cpus" \
  python3 -m research.n83_highk_online_tiebreak_20261020.runpod_runner \
  --binary /workspace/n83_target/release/examples/koblitz_orbit_dlp_fast \
  --rho-binary /workspace/n83_target/release/examples/n83_rho_optimization_eval \
  --trigger-receipt "$trigger" --provider-receipt "$allocation" \
  --output "$base/results" > "$base/screen_run.log" 2>&1
screen_code=$?
printf 'SCREEN_PRODUCER_EXIT %s %s\n' "$screen_code" "$(date -u +%FT%TZ)" > "$base/run.status"

verify_code=99
if [ -d "$base/results" ]; then
  python3 research/n83_highk_online_tiebreak_20261020/verify_screen.py \
    "$base/results" > "$base/screen_verify.log" 2>&1
  verify_code=$?
  tar -C "$base" -czf "$base/raw_results.tar.gz" results
  sha256sum "$base/raw_results.tar.gz" > "$base/raw_results.sha256"
fi
printf 'DONE screen=%s screen_verify=%s %s\n' \
  "$screen_code" "$verify_code" "$(date -u +%FT%TZ)" > "$base/run.status"
