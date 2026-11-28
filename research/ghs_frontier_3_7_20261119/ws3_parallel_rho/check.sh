#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
study="$repo/research/quartic_ghs_strong_rho_20261114"
out=$(mktemp -d /tmp/ghs-ws3-control.XXXXXX)
trap 'rm -rf "$out"' EXIT

: "${GHS_CPUSET_P1:?set one admitted/development CPU}"
: "${GHS_CPUSET_P2:?set two same-node CPUs}"
: "${GHS_CPUSET_P4:?set four same-node CPUs}"
: "${GHS_NUMA_NODE:?set the NUMA node for the CPUs}"

revision=${GHS_TEST_SOURCE_REVISION:-worktree-unbound}
GHS_SOURCE_REVISION="$revision" rustc --edition 2021 -C opt-level=3 \
  "$study/main.rs" -o "$out/ghs-strong-rho"
"$out/ghs-strong-rho" selftest >"$out/selftest.stdout" 2>"$out/selftest.stderr"
grep -qx 'SELFTEST OK' "$out/selftest.stdout"
grep -q 'standalone SHA-256 matches public vectors' "$out/selftest.stderr"

run_check() {
  local name=$1 workers=$2 cpus=$3 width=$4 active=$5 shards=$6 depth=$7 delay=$8 inject=$9
  "$out/ghs-strong-rho" check-parallel \
    --q 223 --instance-seed 1 --target-index 0 --target-policy ic-repeat \
    --arm "strong-b${width}" --batch-width "$width" --partitions 32 \
    --workers "$workers" --worker-start indexed-sha256-v1 \
    --dp-bits auto --dp-shards "$shards" --queue-depth "$depth" \
    --stop-check-steps 4 --stop-delay-checks "$delay" \
    --worker-cpus "$cpus" --numa-node "$GHS_NUMA_NODE" \
    --inject-tail "${active}/${width}" --inject-collision-step "$inject" \
    --output "$out/${name}.json" >/dev/null
  jq -e '
    .schema == "ghs-rho-parallel-check/v1" and
    .ok == true and
    .timing_observations == 0 and
    .performance_claims == 0 and
    .total_steps == (.useful_steps + .overshoot_steps) and
    .total_steps == ([.per_worker_total_steps[]] | add // 0) and
    .queue_submitted == .queue_processed and
    .rejected_records == 0 and
    .threaded == false and
    .scheduler == "deterministic-round-robin" and
    .worker_cpu_map_authoritative == false and
    .table_bytes_authoritative == false
  ' "$out/${name}.json" >/dev/null
  jq -c . "$out/${name}.json.dps.jsonl" >/dev/null
  test "$(sha256sum "$out/${name}.json.dps.jsonl" | awk '{print $1}')" = \
    "$(jq -r .dp_transcript_sha256 "$out/${name}.json")"
  test "$(wc -c <"$out/${name}.json.dps.jsonl")" = \
    "$(jq -r .dp_transcript_bytes "$out/${name}.json")"
  if (( active == 0 )); then
    jq -e '.total_steps == 0 and .k_found == null and .recovered_scalar_check == false' \
      "$out/${name}.json" >/dev/null
  else
    jq -e '.recovered_scalar_check == true and .k_found == .k' \
      "$out/${name}.json" >/dev/null
  fi
}

# D04: every active tail length. Physical vectors contain only active lanes,
# so inactive lanes cannot be read or charged by construction.
for width in 1 4 8 16 32; do
  for ((active=0; active<=width; active++)); do
    inject=16
    if (( active == 0 )); then inject=0; fi
    run_check "tail-w${width}-a${active}" 1 "$GHS_CPUSET_P1" \
      "$width" "$active" 1 2 0 "$inject"
  done
done

# D05: one sealed mathematical run under every requested shard count.
for shards in 1 2 4 8; do
  run_check "shards-${shards}" 2 "$GHS_CPUSET_P2" 4 4 "$shards" 2 0 32
done
sealed_digest=$(jq -r .dp_transcript_sha256 "$out/shards-1.json")
for shards in 2 4 8; do
  test "$(jq -r .dp_transcript_sha256 "$out/shards-${shards}.json")" = "$sealed_digest"
  test "$(jq -r .k_found "$out/shards-${shards}.json")" = \
    "$(jq -r .k_found "$out/shards-1.json")"
done

# D06: lossless producer blocking at shallow and deep queue bounds.
for depth in 1 2 64; do
  run_check "queue-${depth}" 2 "$GHS_CPUSET_P2" 4 4 4 "$depth" 0 32
done
jq -e '.queue_block_events > 0' "$out/queue-1.json" >/dev/null
queue_digest=$(jq -r .dp_transcript_sha256 "$out/queue-1.json")
for depth in 2 64; do
  test "$(jq -r .dp_transcript_sha256 "$out/queue-${depth}.json")" = "$queue_digest"
done

# D07: deterministic stop-delay accounting. The coordinator charges one
# aggregate check quantum per worker and retains every step as overshoot.
for delay in 0 1 3 8; do
  run_check "delay-${delay}" 2 "$GHS_CPUSET_P2" 4 4 4 2 "$delay" 32
  expected=$((delay * 4 * 2))
  jq -e --argjson expected "$expected" '.overshoot_steps == $expected' \
    "$out/delay-${delay}.json" >/dev/null
done
test "$(jq -r .job_id "$out/delay-0.json")" != "$(jq -r .job_id "$out/delay-3.json")"

# D09 subset: natural (non-injected) recovery and exact replay for P=1,2,4.
for spec in "1:$GHS_CPUSET_P1" "2:$GHS_CPUSET_P2" "4:$GHS_CPUSET_P4"; do
  workers=${spec%%:*}
  cpus=${spec#*:}
  run_check "natural-p${workers}-a" "$workers" "$cpus" 4 4 4 2 0 0
  run_check "natural-p${workers}-b" "$workers" "$cpus" 4 4 4 2 0 0
  cmp "$out/natural-p${workers}-a.json" "$out/natural-p${workers}-b.json"
  cmp "$out/natural-p${workers}-a.json.dps.jsonl" \
    "$out/natural-p${workers}-b.json.dps.jsonl"
  jq -e '.control_only == false and .recovered_scalar_check == true and .usable_collisions >= 1' \
    "$out/natural-p${workers}-a.json" >/dev/null
done

# D10 subset: unsupported timing, missing benchmark custody and invalid
# placement fail closed without a success record.
set +e
"$out/ghs-strong-rho" run-parallel >"$out/run-parallel.stdout" 2>"$out/run-parallel.stderr"
run_parallel_status=$?
env -u BENCHREC "$out/ghs-strong-rho" calibrate \
  --q 223 --instance-seed 1 --triples 1 --worker-cpus "$GHS_CPUSET_P1" \
  --numa-node "$GHS_NUMA_NODE" --output "$out/calibration.json" \
  >"$out/calibrate.stdout" 2>"$out/calibrate.stderr"
calibrate_status=$?
touch "$out/existing-calibration.json"
BENCHREC=development-preflight "$out/ghs-strong-rho" calibrate \
  --q 223 --instance-seed 1 --triples 1 --worker-cpus "$GHS_CPUSET_P1" \
  --numa-node "$GHS_NUMA_NODE" --output "$out/existing-calibration.json" \
  >"$out/calibrate-existing.stdout" 2>"$out/calibrate-existing.stderr"
calibrate_existing_status=$?
"$out/ghs-strong-rho" check-parallel \
  --q 223 --instance-seed 1 --target-index 0 --target-policy ic-repeat \
  --arm strong-b1 --batch-width 1 --partitions 32 --workers 1 \
  --worker-start indexed-sha256-v1 --dp-bits auto --dp-shards 1 \
  --queue-depth 1 --stop-check-steps 4 --stop-delay-checks 0 \
  --worker-cpus 999999 --numa-node "$GHS_NUMA_NODE" --inject-tail 1/1 \
  --inject-collision-step 16 --output "$out/invalid-affinity.json" \
  >"$out/invalid-affinity.stdout" 2>"$out/invalid-affinity.stderr"
affinity_status=$?
set -e
test "$run_parallel_status" -eq 3
test "$calibrate_status" -eq 1
test "$calibrate_existing_status" -eq 1
test "$affinity_status" -eq 1
test ! -e "$out/calibration.json"
test ! -e "$out/invalid-affinity.json"
grep -q 'RUN-PARALLEL UNAVAILABLE' "$out/run-parallel.stderr"
grep -q 'requires a BENCHREC reference' "$out/calibrate.stderr"
grep -q 'calibration output already exists' "$out/calibrate-existing.stderr"
grep -q 'not a subset of process affinity' "$out/invalid-affinity.stderr"

printf '%s\n' \
  'WS3 deterministic controls PASS' \
  'sha256_vectors=2' \
  'tail_cells=66' \
  'shard_cells=4' \
  'queue_cells=3' \
  'stop_delay_cells=4' \
  'natural_recovery_cells=6' \
  'exact_replay_pairs=3' \
  'fail_closed_cells=4' \
  'timing_observations=0 performance_claims=0'
