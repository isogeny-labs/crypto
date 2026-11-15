#!/bin/bash
# Smoke test of p3census: p < 2^18, <= 50 curves, <= 32 targets per curve, one core.
# Usage: BIN=/path/to/p3census ./run_smoke.sh   (writes the logs next to this script)
set -euo pipefail
cd "$(dirname "$0")"
B=${BIN:?set BIN to the p3census binary}
P=262111   # prime < 2^18 with p = 1 mod 3 and p mod 5 in {1,2,3}: psi3/psi5 root counts are not identically 0 (see README)
run() { nice -n 10 "$B" "$@"; }

{ echo "# selftest"; run selftest; } > selftest.txt 2>&1
run curves --p $P --count 50 --seed 1 --out curves.jsonl
run covariates --curves curves.jsonl --out cov.jsonl
{ time run yield --curves curves.jsonl --N 150 --out yield.jsonl; } 2> yield.log
{ time run rho --curves curves.jsonl --targets 32 --seed 7 --run A --out rhoA.jsonl --targets-out targetsA.csv; } 2> rhoA.log
{ time run rho --curves curves.jsonl --targets 32 --seed 7 --run B --out rhoB.jsonl; } 2> rhoB.log
run analyze --curves curves.jsonl --cov cov.jsonl --rho-a rhoA.jsonl --rho-b rhoB.jsonl \
    --yield yield.jsonl --seed 1 --out analysis.json 2> analysis.txt

# degeneracy check: for p = -1 mod 3 (resp. 5) prime-order curves have 0 roots of psi3 (resp. psi5)
: > psi_degeneracy.txt
for p in 262139 262111; do
  run curves --p $p --count 50 --seed 2 --out /tmp/p3_d_$p.jsonl
  run covariates --curves /tmp/p3_d_$p.jsonl --out /tmp/p3_dc_$p.jsonl
  python3 - "$p" >> psi_degeneracy.txt <<'PY'
import json, sys, collections
p = int(sys.argv[1]); rows = [json.loads(l) for l in open(f"/tmp/p3_dc_{p}.jsonl")]
print(f"p={p} (p mod 3 = {p%3}, p mod 5 = {p%5}): psi3_roots {dict(collections.Counter(r['psi3_roots'] for r in rows))} psi5_roots {dict(collections.Counter(r['psi5_roots'] for r in rows))}")
PY
done

# throughput / scaling: rho at four primes (20 curves x 32 targets each), coverage timing vs N
: > throughput.txt
for p in 16411 65521 131071 262139; do
  run curves --p $p --count 20 --seed 3 --out /tmp/p3_c_$p.jsonl
  run rho --curves /tmp/p3_c_$p.jsonl --targets 32 --seed 9 --run A --out /tmp/p3_r_$p.jsonl 2>&1 | sed "s/^/p=$p /" >> throughput.txt
  python3 - "$p" >> throughput.txt <<'PY'
import json, sys, statistics as st
p = sys.argv[1]
rows = [json.loads(l) for l in open(f"/tmp/p3_r_{p}.jsonl")]
sq = lambda r: r["n"] ** 0.5
print(f"p={p} S_total(mean ops/sqrt n)={st.mean(r['mean_over_sqrt_n'] for r in rows):.3f} walk/sqrt n={st.mean(r['mean_walk_ops']/sq(r) for r in rows):.3f} setup/sqrt n={st.mean(r['mean_setup_ops']/sq(r) for r in rows):.3f} escapes/target={st.mean(r['escapes_per_target'] for r in rows):.2f} unsolved={sum(r['unsolved'] for r in rows)} mismatches={sum(r['mismatch'] for r in rows)}")
PY
done
head -5 curves.jsonl > /tmp/p3_c5.jsonl
for N in 150 500 1500 3000; do
  run yield --curves /tmp/p3_c5.jsonl --N $N --out /tmp/p3_y5.jsonl 2>&1 | sed "s/^/coverage 5 curves /" >> throughput.txt
  python3 - "$N" >> throughput.txt <<'PY'
import json, sys, statistics as st
N = int(sys.argv[1]); r = [json.loads(l) for l in open("/tmp/p3_y5.jsonl")]
sec = st.mean(x["secs"] for x in r); pairs = N * (N - 1) / 2
print(f"N={N} lambda={r[0]['lambda']:.3f} secs/curve={sec:.4f} ns/pair={1e9*sec/pairs:.1f}")
PY
done
