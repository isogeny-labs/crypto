#!/usr/bin/env bash
# Amendment 5: writes specs/SPEC-<job>.json and jobs_ext.txt (the queue, in run
# order) for the extension. Does not touch jobs.txt or the existing specs.
# Thin orchestration only: every number is ecbench's.
set -euo pipefail
cd "$(dirname "$0")"
ORACLE='gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1,charge_unsolved=1'
MSEED=202610150
curve() { printf '{"kind":"fp4_torsion","p":%d,"seed":%d,"max_cofactor":16}' "$1" "$2"; }
ic_arm() { printf '{"name":"%s","role":"candidate","method":{"id":"ic.pipeline","params":{"factor_base":"fghr-y-line:zero=1","oracle":"%s%s","linalg":"wiedemann","max_trials":"400000"}}}' "$1" "$ORACLE" "$2"; }
rho_arm='{"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}}'
spec() { # job p seed targets target_seed arms label
  [ -e "specs/SPEC-$1.json" ] && { echo "refusing to overwrite specs/SPEC-$1.json" >&2; exit 1; }
  cat > "specs/SPEC-$1.json" <<J
{"schema":"ecbench.spec/v1","label":"$7","description":"research/gaudry_quartic_d4_exponent_20261015/PROTOCOL.md amendment 5, job $1","workloads":{"curves":[$(curve "$2" "$3")],"targets_per_curve":$4,"target_seed":$5},"arms":[$6],"measurement":{"rounds":1,"warmup":0,"order":"alternate","seed":$MSEED,"isolation_required":"L0","timeout_seconds":172800}}
J
  echo "$1" >> jobs_ext.txt
}
start_seed() { awk -F'\t' -v p="$1" -v c="$2" '$1==p && $2==c {print $3}' curves.tsv; }
tseed1() { echo $((2026101500000 + $1 * 10 + $2)); }              # the original paired target seeds
tseedx() { echo $((2026102000000 + $1 * 100000 + $2 * 10 + $3)); } # chain, p, t
: > jobs_ext.txt
# 1. rho-only runs on the new curves (cheap, first): 16 targets each.
for c in 2 3; do for p in 29 37 53 61 101 149 197 293 401 509; do
  spec "xrho-c$c-p$p" "$p" "$(start_seed $p $c)" 16 $((2026102990000 + c * 10000 + p)) "$rho_arm" "rho on E(F_$p^4), chain $c curve, 16 targets"
done; done
# 2. Long jobs first, longest first (estimated from the first 72 jobs).
lp2() { spec "lp2-p$1-t$2" "$1" 1 1 "$(tseed1 "$1" "$2")" "$rho_arm,$(ic_arm ic-d4-lp2 ',large_primes=2')" "k=4 IC double large prime and rho on E(F_$1^4), target $2"; }
lp1() { spec "lp1-p$1-t$2" "$1" 1 1 "$(tseed1 "$1" "$2")" "$rho_arm,$(ic_arm ic-d4-lp1 ',large_primes=1')" "k=4 IC single large prime and rho on E(F_$1^4), target $2"; }
main1() { spec "main-p$1-t$2" "$1" 1 1 "$(tseed1 "$1" "$2")" "$rho_arm,$(ic_arm ic-d4-w '')" "k=4 IC (D4 oracle, Wiedemann) and rho on E(F_$1^4), target $2"; }
xmain() { spec "xmain-c$1-p$2-t$3" "$2" "$(start_seed $2 $1)" 1 "$(tseedx "$1" "$2" "$3")" "$rho_arm,$(ic_arm ic-d4-w '')" "k=4 IC and rho on a chain $1 curve of E(F_$2^4), target $3"; }
lp2 701 0; lp2 701 1; lp2 509 0; lp2 509 1; lp1 293 0; lp1 293 1; lp2 401 0; lp2 401 1
main1 1009 2; main1 701 2
for p in 509 401; do for c in 2 3; do for t in 0 1; do xmain $c $p $t; done; done; main1 $p 2; done
for c in 2 3; do for t in 0 1; do xmain $c 293 $t; done; done
lp1 197 0; lp1 197 1
for p in 197 149 101 61 53 37 29; do for c in 2 3; do for t in 0 1; do xmain $c $p $t; done; done; done
