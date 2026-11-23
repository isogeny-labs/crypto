#!/bin/bash
# Equality gate: the reference engine (frozen binary, --engine hky) against
# the fast engine (--engine hky2) on the same cells from the same seeds.
# usage: run_gate.sh OUTDIR [extra hky2 flags…]     (exit 0 only if every set is EQUAL
#                                                    and both negative controls fail)
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
REF=${REF:-$HERE/bin/pilot_baseline}
CAND=${CAND:-$HERE/bin/pilot_candidate}
EQ=${EQ:-$HERE/bin/hky_engine_equality}
OUT=$1; shift
mkdir -p "$OUT"
P1=2013265921; P0=786433
common=(--ladder-t none --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
fail=0
run_set() { # name, flags…
  local name=$1; shift
  if [ ! -s "$OUT/$name.ref.jsonl" ]; then
    "$REF" "${common[@]}" "$@" --engine hky --out "$OUT/$name.ref.jsonl" 2> "$OUT/$name.ref.log" || { echo "$name: reference run failed"; fail=1; }
  fi
  "$CAND" "${common[@]}" "$@" --engine hky2 "${EXTRA[@]}" --out "$OUT/$name.cand.jsonl" 2> "$OUT/$name.cand.log" || { echo "$name: candidate run failed"; fail=1; }
  printf '%-22s ' "$name"
  "$EQ" compare "$OUT/$name.ref.jsonl" "$OUT/$name.cand.jsonl" | tail -1
  [ "${PIPESTATUS[0]}" -eq 0 ] || fail=1
}
EXTRA=("$@")
# m = 2 Kummer tower, both draws: the study's cell
run_set m2-tower-t1-8      --p $P1 --planted 0 --random 2 --hky-cmax 7 --kinds kummer --m 2 --controls tower --t-min 1 --max-t 8
# planted targets: consistent systems, 1 in no closure, every closure a proper subspace
run_set m2-tower-planted   --p $P1 --planted 2 --random 0 --hky-cmax 5 --kinds kummer --m 2 --controls tower --t-min 2 --max-t 7
# null control (random coefficients on S3's support: not symmetric, the split must be refused)
run_set m2-null-t2-7       --p $P1 --planted 1 --random 2 --hky-cmax 6 --kinds kummer --m 2 --controls null --t-min 2 --max-t 7
# naive control: no tower, free variables only
run_set m2-naive-t2-4      --p $P1 --planted 1 --random 1 --hky-cmax 6 --kinds kummer --m 2 --controls naive --t-min 2 --max-t 4
# m = 3 and m = 4: free chain unknowns, three and four towers
run_set m3-tower-t2-3      --p $P1 --planted 1 --random 2 --hky-cmax 5 --kinds kummer --m 3 --controls tower --t-min 2 --max-t 3
run_set m4-tower-t2        --p $P1 --planted 0 --random 1 --hky-cmax 4 --kinds kummer --m 4 --controls tower --t-min 2 --max-t 2
# general tower rules (isogeny towers, y² = a·y·y' + b·y' + c·y + d) at the small prime
run_set p0-kummer-isogeny  --p $P0 --planted 1 --random 2 --hky-cmax 6 --kinds kummer,isogeny --m 2 --controls tower --t-min 1 --max-t 6
# censoring by max_cols is the same decision in both engines
common=(--ladder-t none --budget 1800 --hky-max-cols 1500 --hky-max-entries 300000000)
run_set censor-max-cols    --p $P1 --planted 0 --random 1 --hky-cmax 7 --kinds kummer --m 2 --controls tower --t-min 5 --max-t 7
common=(--ladder-t none --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
# negative controls: a faulty candidate must NOT pass (planted set: closures without 1)
for fault in 1 2; do
  "$CAND" "${common[@]}" --p $P1 --planted 2 --random 0 --hky-cmax 5 --kinds kummer --m 2 --controls tower --t-min 2 --max-t 7 \
     --engine hky2 "${EXTRA[@]}" --hky2-fault $fault --out "$OUT/neg-fault$fault.cand.jsonl" 2> "$OUT/neg-fault$fault.cand.log"
  printf '%-22s ' "neg-fault$fault"
  "$EQ" compare --expect-mismatch "$OUT/m2-tower-planted.ref.jsonl" "$OUT/neg-fault$fault.cand.jsonl" | tail -1
  [ "${PIPESTATUS[0]}" -eq 0 ] || fail=1
done
if [ $fail -eq 0 ]; then echo "GATE PASS"; else echo "GATE FAIL"; fi
exit $fail
