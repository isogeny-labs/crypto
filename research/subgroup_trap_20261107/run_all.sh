#!/usr/bin/env bash
# Full run with the final binary: audit, exact mass, C1 reconciliation, score, figure.
set -euo pipefail
cd "$(dirname "$0")"
R=${1:?run dir}
C=../c1_twisted_line_ladder_20261027/reports/run-20261027T1502Z
B=./target/release/subgroup_trap
mkdir -p "$R"
(cd ../c1_twisted_line_ladder_20261027 && sha256sum -c --quiet FREEZE.sha256)
sha256sum "$C"/yield_k*.jsonl > "$R/inputs.sha256"
git rev-parse HEAD > "$R/commit.txt"
{ hostname; nproc; rustc --version; } > "$R/host.txt"
$B audit ../lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt \
  ../oakley_lift_dense_20261027/reports/run-20261027T1445Z/rows.json > "$R/audit.jsonl"
: > "$R/mass.jsonl"
for k in 5 7; do $B mass $k >> "$R/mass.jsonl"; done
for k in 5 7 9 11; do s=$SECONDS; $B c1 "$C" $k > "$R/c1_k$k.jsonl"; echo "c1 k=$k $((SECONDS-s))s" >> "$R/run.log"; done
$B score "$R"
