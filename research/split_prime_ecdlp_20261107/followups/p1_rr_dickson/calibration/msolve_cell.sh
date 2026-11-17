#!/usr/bin/env bash
# msolve_cell.sh IN.ms LOG [GBOUT]
# One msolve 0.6.5 job under the round-7 limits (research/ic_tree_gluing_20261103/run.sh,
# round7_rr/README.md): msolve -g 2 -t 1 -v 2, ulimit -v VLIMIT KiB (default 1572864 = 1.5 GiB),
# timeout TLIMIT s (default 1200), nice -n 10. Appends "exit=<rc> wall=<s>" like run.sh.
# Thin orchestration only; metrics come from `p1tool parse LOG`.
set -u
in=${1:?usage: msolve_cell.sh IN.ms LOG [GBOUT]}; log=${2:?}; gb=${3:-${log%.log}.gb}
start=$(date +%s.%N)
( ulimit -v "${VLIMIT:-1572864}"; nice -n 10 timeout "${TLIMIT:-1200}" msolve -g 2 -t 1 -v 2 -f "$in" -o "$gb" ) > "$log" 2>&1
rc=$?
end=$(date +%s.%N)
echo "exit=$rc wall=$(echo "$end - $start" | bc)" >> "$log"
[ -n "${KEEPGB:-}" ] || rm -f "$gb"
exit 0
