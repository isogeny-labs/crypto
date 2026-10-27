#!/bin/bash
# Diagnostic: how much of Table A's x reference is the 32-lane kernel?  Re-runs the rule-v3 single-target
# rows with the rho at more lanes (`ic price --single-target --rho-lanes L`, distinguished-point bits at
# their default for that lane count) on the run's own params file and rho seed.  IC and rho run in one
# process, in one unit, as in the table.  Shared host, not isolated: L0 diagnostic, never a table row.
# usage: lane_sweep.sh <ic-binary> <outdir>
set -u
IC="$1"; OUT="$2"; mkdir -p "$OUT"
base=research/ic_tool_program/rule/v3/runs
plan() { # dir targets lanes...
  d="$1"; shift; ts="$1"; shift
  for t in $ts; do for L in "$@"; do
    f="$base/$d/$t-R1.price.json"; seed=$(jq -r '.rho_policy.seed' "$f")
    curve=$(jq -r '.curve' "$f"); out="$OUT/$curve-$t-L$L.json"
    [ -f "$out" ] && continue
    RAYON_NUM_THREADS=1 "$IC" price --params "$base/$d/$t.params.json" --single-target --rho-seed "$seed" \
      --rho-lanes "$L" --repeats 1 --repeats-fast 1 --json --out "$out" > /dev/null 2> "$OUT/$curve-$t-L$L.err"
    echo "$curve $t L=$L exit=$?"
  done; done
}
plan k1n47 "T01 T02 T03 T04" 32 64 128 256 512 1024
plan k0n53 "T01 T02"        32 128 512 1024
plan k0n61 "T01"            32 1024
