#!/usr/bin/env bash
# Frozen grid of PROTOCOL.md. Usage: run_grid.sh <k2m4r binary> <out.jsonl> <progress.log>
# The caller holds the CPU lock for the whole grid (flock) and sets ulimit -v 9000000.
set -uo pipefail
B=$1; OUT=$2; LOG=$3; BUDGET=2700   # wall seconds at 4 threads = 3 CPU-hours
: > "$OUT"; SECONDS=0
note() { echo "[$(date -u +%H:%M:%S) t=${SECONDS}s] $*" | tee -a "$LOG" >&2; }
run() { K2_PROGRESS=1 "$B" --threads 4 "$@" >> "$OUT" 2>> "$LOG"; }
last_secs() { tail -1 "$OUT" | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(r.get('secs',0) if not r.get('censored') else -1)"; }
rem() { echo $((BUDGET - SECONDS)); }
cap() { local c=$1; local r; r=$(rem); [ "$r" -lt "$c" ] && c=$r; echo "$c"; }
# 1. controls
for s in 1 2; do run --cell P --m 2 --n 16 --k 8 --targets 2 --dmin 2 --dmax 5 --seed $s; done
for s in 1 2; do run --cell P --m 2 --n 20 --k 10 --targets 2 --dmin 2 --dmax 5 --seed $s; done
run --cell N --m 3 --n 12 --k 4 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1
run --cell N --m 4 --n 8 --k 2 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1
note "controls done"
# 2. decisive-shape negative control (random system of the D1 shape, planted solution)
run --cell N-D1 --m 3 --n 21 --k 7 --targets 1 --dmin 4 --dmax 4 --pres random --maxcols 130000 --maxsecs $(cap 600) --seed 1
note "N-D1 done"
# 3. ladder L2 (m = 3, n = 19, k = 6) then decisive D1 (m = 3, n = 21, k = 7), target 0
run --cell L2 --m 3 --n 19 --k 6 --targets 1 --dmin 3 --dmax 4 --maxcols 130000 --maxsecs $(cap 600) --seed 1
note "L2 done"
run --cell D1 --m 3 --n 21 --k 7 --targets 1 --dmin 3 --dmax 4 --maxcols 130000 --maxsecs $(cap 1800) --seed 1
T1=$(last_secs); note "D1 target 0 done: secs=$T1"
# 4. ladder L1 (m = 4, n = 13, k = 3)
run --cell L1 --m 4 --n 13 --k 3 --targets 1 --dmin 3 --dmax 4 --maxcols 130000 --maxsecs $(cap 600) --seed 1
note "L1 done"
# 5. decisive cells D3 (m = 4, n = 16, k = 4; 396,607 columns, basis up to 19.7 GB > 9 GB cap: registered not run (memory))
#    and D2 (m = 3, n = 24, k = 8; 213,053 columns, basis up to 5.7 GB): D2 runs only if T1 * (213053/124314)^3 fits the remaining budget.
echo "{\"cell\":\"D3\",\"m\":4,\"n\":16,\"k\":4,\"seed\":1,\"target\":0,\"skipped\":\"gate: memory, 396607 columns need up to 19.7 GB of echelon basis, cap 9 GB (registered in PROTOCOL.md)\"}" >> "$OUT"
note "D3 not run (memory gate)"
if python3 - "$T1" "$(rem)" <<'PY'
import sys
t1, rem = float(sys.argv[1]), float(sys.argv[2])
sys.exit(0 if t1 > 0 and t1 * (213053 / 124314) ** 3 <= rem else 1)
PY
then
  run --cell D2 --m 3 --n 24 --k 8 --targets 1 --dmin 3 --dmax 4 --maxcols 220000 --maxsecs $(rem) --seed 1
  note "D2 done"
else
  echo "{\"cell\":\"D2\",\"m\":3,\"n\":24,\"k\":8,\"seed\":1,\"target\":0,\"skipped\":\"gate: projected time T1*5.03 = $(python3 -c "print(round(float('$T1')*(213053/124314)**3))")s > remaining $(rem)s (T1=$T1)\"}" >> "$OUT"
  note "D2 not run (time gate)"
fi
# 6. second targets with the remaining budget, in this order: D1, L1, L2 (seed 1, target 1), only if remaining >= 1.2 x first-target time
for cellspec in "D1 3 21 7" "L1 4 13 3" "L2 3 19 6"; do
  set -- $cellspec
  s0=$(python3 - "$OUT" "$1" <<'PY'
import sys, json
rs = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
t = [r.get("secs") for r in rs if r.get("cell") == sys.argv[2] and r.get("D") == 4 and "secs" in r and r.get("target") == 0]
print(int(t[0]) if t else 0)
PY
)
  r=$(rem)
  if [ "$s0" -gt 0 ] && [ "$r" -ge $((s0 * 12 / 10)) ]; then
    run --cell $1 --m $2 --n $3 --k $4 --targets 2 --only-target 1 --dmin 4 --dmax 4 --maxcols 130000 --maxsecs $r --seed 1
    note "$1 target 1 done"
  else
    note "$1 target 1 not run (remaining ${r}s < 1.2 x first-target time ${s0}s)"
  fi
done
note "grid finished"
