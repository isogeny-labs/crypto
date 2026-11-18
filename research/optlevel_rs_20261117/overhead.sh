#!/usr/bin/env bash
# The tools' own overhead, Python tool against Rust tool, interleaved.
#  (1) start-up: wall time of `validate` on one committed record, N runs each;
#  (2) what a measured kernel sees: for a command that reports no kernel_seconds (`/bin/true --ops 1`) the tool's own
#      wall_s per run is the whole fork/exec/wait path; for a 50 ms kernel that reports kernel_seconds, startup_share
#      = 1 - kernel_seconds / wall_s is what the tool's wall clock adds around the timed region;
#  (3) the whole `measure --repeat 5` of that 50 ms kernel, wall and CPU.
# usage: overhead.sh PYTHON_TOOL RUST_BIN C_KERNEL_BINARY RECORD OUT.tsv [N]
set -euo pipefail
PY=$1; RS=$2; K=$3; REC=$4; OUT=$5; N=${6:-20}; T=$(mktemp -d)
now() { date +%s%N; }
echo -e "what\ttool\trun\tvalue\tunit\tloadavg1" > "$OUT"
row() { echo -e "$1\t$2\t$3\t$4\t$5\t$(cut -d' ' -f1 /proc/loadavg)" >> "$OUT"; }
for i in $(seq 1 "$N"); do
  a=$(now); python3 "$PY" validate "$REC" > /dev/null; b=$(now); "$RS" validate "$REC" > /dev/null; c=$(now)
  row validate_wall py $i $(( (b - a) / 1000 )) us; row validate_wall rs $i $(( (c - b) / 1000 )) us
done
for i in 1 2 3; do
  for t in py rs; do
    if [ $t = py ]; then tool=(python3 "$PY"); else tool=("$RS"); fi
    "${tool[@]}" measure --timing-only --repeat 10 --ops 1 --op-class xor_u64 --out $T/true.$t.json -- /bin/true 2> /dev/null
    row true_wall_per_run_median $t $i "$(jq '.points["1"].wall_s.median * 1e6' $T/true.$t.json)" us
    row true_wall_per_run_min $t $i "$(jq '.points["1"].wall_s.min * 1e6' $T/true.$t.json)" us
    a=$(now); s=$(awk '{print $14 + $15 + $16 + $17}' /proc/$$/stat)
    "${tool[@]}" measure --timing-only --repeat 5 --out $T/k.$t.json -- "$K" modmuladd_p31 --seconds 0.05 2> /dev/null
    b=$(now); e=$(awk '{print $14 + $15 + $16 + $17}' /proc/$$/stat)
    row kernel50ms_startup_share_median $t $i "$(jq '.startup_share' $T/k.$t.json)" fraction
    row kernel50ms_wall_minus_kernel_median $t $i "$(jq '[.runs[] | select(.copies == 1) | (.wall_s - .kernel_seconds) * 1e6] | sort | .[length / 2 | floor]' $T/k.$t.json)" us
    row kernel50ms_rate_median $t $i "$(jq '.points["1"].ops_per_s.median' $T/k.$t.json)" ops_per_s
    row measure_repeat5_total_wall $t $i $(( (b - a) / 1000000 )) ms
    row measure_repeat5_kernel_seconds_sum $t $i "$(jq '[.runs[].kernel_seconds] | add * 1000' $T/k.$t.json)" ms
    row measure_repeat5_tool_and_children_cpu $t $i $(( (e - s) * 10 )) ms
  done
done
rm -rf $T
