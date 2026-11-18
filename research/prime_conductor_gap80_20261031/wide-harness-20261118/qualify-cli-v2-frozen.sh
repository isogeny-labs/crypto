#!/bin/sh
# Actual ecbench CLI plan/schema checks; never invokes run or verify.
set -eu
binary=$1
checker=$2
out=${3:?fresh output directory required}
study=research/prime_conductor_gap80_20261031/wide-harness-20261118
test ! -e "$out"
mkdir -p "$out"
"$binary" plan --json --spec "$study/inputs/wide-source-plan-v2.json" > "$out/plan.json" 2> "$out/plan.stderr"
test ! -s "$out/plan.stderr"
jq -e '(.workloads|length)==64 and .workloads[0].curve.field_bits==202 and .workloads[32].curve.field_bits==96 and (.workloads|all(.target_law=="uniform_scalar_sha256_shared_v1"))' "$out/plan.json" > "$out/plan-shape.check"
jq -r '.[]|.targets[]|[.scalar,.point[0],.point[1]]|@tsv' "$study/source-vectors.json" > "$out/producer.tsv"
jq -r '.workloads[]|[.planted,.target[0],.target[1]]|@tsv' "$out/plan.json" > "$out/consumer.tsv"
"$checker" "$out/producer.tsv" "$out/consumer.tsv" > "$out/rows.json" 2> "$out/rows.stderr"
test ! -s "$out/rows.stderr"
set +e
"$binary" plan --json --spec "$study/inputs/wide-unsupported-ic-plan-v2.json" > "$out/unsupported-ic.stdout" 2> "$out/unsupported-ic.stderr"
code=$?
set -e
test "$code" -ne 0
test ! -s "$out/unsupported-ic.stdout"
rg -q 'no prime_mont4 adapter' "$out/unsupported-ic.stderr"
printf '{"unsupported_ic_exit":%s,"scope":"schema/planner equality only","benchmark_session_executed":false,"runtime_measurements":null}\n' "$code" > "$out/exit.json"
