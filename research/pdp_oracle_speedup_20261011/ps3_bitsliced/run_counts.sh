#!/usr/bin/env bash
# PS-3: instructions retired per refuted target under callgrind (protocols/PS-3.md "Unit").
# Ir(target) = Ir(passes = 2) - Ir(passes = 1) for the same target: the difference is exactly one extra
# `decide` call; the remainder is the arm's setup plus the harness's one warm-up call.
# valgrind 3.22 does not execute AVX-512; an arm that cannot run is recorded with its exit status.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
R=research/pdp_oracle_speedup_20261011
OUT=$R/ps3_bitsliced/${PS3_RAW:-raw}/counts_callgrind.jsonl
S=${PS3_SCRATCH:-/tmp/ps3_bin}/pdp_speedup
C=$R/baseline_v1/corpus
SCR=${PS3_CG_SCRATCH:-/tmp/ps3_cg}; mkdir -p "$SCR"
[ -f "$OUT" ] || : > "$OUT"
ir() {   # arm l passes -> prints "Ir pairs_per_target status"
  local arm=$1 l=$2 p=$3 f="$SCR/cg.$1.$2.$3.out" so
  so=$(python3 tools/isolated_bench.py busy --wait -- valgrind --tool=callgrind --callgrind-out-file="$f" \
        "$S" time --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --targets 1 --passes "$p" --min-secs 0 --label cg 2>"$SCR/err.$arm.$l.$p")
  local st=$?
  local irv=$(grep -a "Collected" "$SCR/err.$arm.$l.$p" | awk '{print $NF}')
  local ppt=$(printf '%s\n' "$so" | grep -a '^{' | tail -1 | jq -r '.pairs_per_target // 0')
  echo "${irv:-0} ${ppt:-0} $st"
}
for l in 9 10 11; do
  for arm in baseline group-scalar bs64 bs256 bs512; do
    grep -q "\"arm\":\"$arm\",\"l\":$l," "$OUT" && continue
    read -r i1 ppt s1 < <(ir "$arm" "$l" 1)
    read -r i2 _ s2 < <(ir "$arm" "$l" 2)
    if [ "$s1" != 0 ] || [ "$s2" != 0 ] || [ "$i1" = 0 ] || [ "$i2" = 0 ]; then
      echo "{\"arm\":\"$arm\",\"l\":$l,\"status\":\"did not run under valgrind\",\"exit_status\":[$s1,$s2]}" | tee -a "$OUT"; continue
    fi
    per=$((i2 - i1)); setup=$((i1 - 2 * per))
    echo "{\"arm\":\"$arm\",\"l\":$l,\"ir_passes1\":$i1,\"ir_passes2\":$i2,\"ir_per_target\":$per,\"ir_setup_and_warmup_remainder\":$setup,\"pairs_per_target\":$ppt,\"ir_per_pair\":$(echo "scale=3; $per / $ppt" | bc)}" | tee -a "$OUT"
  done
done
