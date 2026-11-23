#!/bin/sh
# Paired public map-data planning only. Never calls ecbench run.
set -eu
binary=$1
checker=$2
out=${3:?fresh output directory required}
study=research/prime_conductor_gap80_20261031/wide-harness-20261118
test ! -e "$out"
mkdir -p "$out"
"$binary" plan --json --spec "$study/inputs/wide-paired-plan.json" > "$out/plan.json" 2> "$out/plan.stderr"
test ! -s "$out/plan.stderr"
jq -e '(.workloads|length)==192 and .workloads[0].curve.field_bits==202 and .workloads[96].curve.field_bits==96 and (.workloads|all(.target_law=="uniform_scalar_sha256_shared_v1"))' "$out/plan.json" > "$out/shape.check"
jq -r '.[] as $p|["source","destination","coordinate_control"][] as $end|$p.targets[]|[.scalar,.[$end][0],.[$end][1]]|@tsv' "$study/paired-map-packets.json" > "$out/producer.tsv"
jq -r '.workloads[]|[.planted,.target[0],.target[1]]|@tsv' "$out/plan.json" > "$out/consumer.tsv"
"$checker" "$out/producer.tsv" "$out/consumer.tsv" > "$out/rows.json" 2> "$out/rows.stderr"
test ! -s "$out/rows.stderr"
awk 'NR==1 {$3=$3 "0"} {print}' "$out/producer.tsv" > "$out/tampered.tsv"
set +e
"$checker" "$out/tampered.tsv" "$out/consumer.tsv" > "$out/tampered.stdout" 2> "$out/tampered.stderr"
tampered=$?
"$binary" plan --json --spec "$study/inputs/wide-paired-unsupported-ic.json" > "$out/unsupported-ic.stdout" 2> "$out/unsupported-ic.stderr"
ic=$?
set -e
test "$tampered" -eq 2
test ! -s "$out/tampered.stdout"
test "$ic" -ne 0
test ! -s "$out/unsupported-ic.stdout"
rg -q 'no prime_mont4 adapter' "$out/unsupported-ic.stderr"
printf '{"tampered_row_exit":%s,"unsupported_ic_exit":%s,"benchmark_session_executed":false,"high_fidelity_admitted":false,"runtime_measurements":null}\n' "$tampered" "$ic" > "$out/exit.json"
