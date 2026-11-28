#!/usr/bin/env bash
# End-to-end timing of graph-control arms: thin orchestration only.
# Usage: bench_e2e.sh ROUNDS OUT.jsonl ARM=BINARY:ARGS [ARM=BINARY:ARGS ...]
# Each arm runs once per round, in the listed order, as a separate process.
# Per run it records external process wall (date ns), user and system CPU
# (bash `time` keyword), the normalised output hash, the producer's
# in-process clocks and stage ledger, and the recovered scalar.
set -euo pipefail
rounds=$1; out=$2; shift 2
norm='{rows:.relation_rows, q:[.queries[]|del(.source_ms,.graph_and_replay_ms)], s:.recovered_scalar, rep:.recovered_scalar_replay, lp:.lp_rank, pr:.projected_rows, fb:.factor_base_target_rank, probes:.probe_additions, cls:.classes_by_base_terms}'
tmp=$(mktemp -d)
TIMEFORMAT='%R %U %S'
: > "$out"
for round in $(seq 1 "$rounds"); do
  for spec in "$@"; do
    arm=${spec%%=*}; rest=${spec#*=}; bin=${rest%%:*}; args=${rest#*:}
    t0=$(date +%s%N)
    # shellcheck disable=SC2086
    { time "$bin" $args > "$tmp/out.json" 2> /dev/null ; } 2> "$tmp/time"
    t1=$(date +%s%N)
    read -r _real user sys < "$tmp/time"
    hash=$(jq -S "$norm" "$tmp/out.json" | sha256sum | cut -c1-64)
    jq -c --arg arm "$arm" --argjson round "$round" --arg bin "$(sha256sum "$bin" | cut -c1-16)" \
      --argjson wall_ms "$(( (t1 - t0) / 1000000 ))" --argjson user_s "$user" --argjson sys_s "$sys" --arg hash "$hash" \
      '{round:$round, arm:$arm, bin:$bin, process_wall_ms:$wall_ms, cpu_user_s:$user_s, cpu_sys_s:$sys_s,
        output_hash:$hash, recovered_scalar, queries:(.queries|length), cold_wall_ms, table_ms,
        collection_ms:([.queries[].source_ms]|add), stage_us:(.stage_us // null), vm_hwm_kb:(.vm_hwm_kb // null), levers:(.levers // "baseline")}' \
      "$tmp/out.json" >> "$out"
  done
done
rm -r "$tmp"
