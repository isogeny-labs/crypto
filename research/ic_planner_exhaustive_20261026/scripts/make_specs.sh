#!/usr/bin/env bash
# Specs of PROTOCOL.md, every IC arm built from `ic plan --ecbench-arm` (ic-planner/3,
# w = 0) with only the stated parameter changed. Usage: scripts/make_specs.sh
set -euo pipefail
cd "$(dirname "$0")/.."
IC=../../target/release/ic
TS=20261105; MS=20261106
spec() { # file label curve-json targets arms-json
  jq -n --arg label "$2" --argjson curve "$3" --argjson t "$4" --argjson arms "$5" --argjson ts $TS --argjson ms $MS '{
    schema: "ecbench.spec/v1", label: $label,
    description: "research/ic_planner_exhaustive_20261026/PROTOCOL.md",
    workloads: {curves: [$curve], targets_per_curve: $t, target_seed: $ts},
    arms: $arms,
    measurement: {rounds: 3, warmup: 1, order: "alternate_measured", seed: $ms, isolation_required: "L2", timeout_seconds: 1800}}' > "specs/$1.json"
}
plan_arm() { $IC plan "$@" --ecbench-arm plan --json | jq -c '.ecbench_arm'; }
with_fb() { jq -c --arg n "$2" --arg fb "$3" --arg t "${4:-}" '.name = $n | .role = "baseline" | .method.params.factor_base = $fb
  | if $t != "" then .method.params.targets = $t else . end' <<<"$1"; }
size_of() { jq -r '.method.params.factor_base' <<<"$1" | sed 's/.*size=//'; }
dim_of() { jq -r '.method.params.factor_base' <<<"$1" | sed 's/.*dimension=//'; }
cols_of() { jq -r '.method.params.factor_base' <<<"$1" | sed 's/.*columns=\([0-9]*\).*/\1/'; }
r() { python3 -c "print(max(1, round($1)))"; }

# R: residual ladder at new sizes (16 targets) and W-prime held out (plan x 1.41).
prime() { # tag curve
  local p arms n0; p=$(plan_arm --curve "$2"); n0=$(size_of "$p")
  arms=$(jq -cn --argjson p "$p" '[{name:"rho",role:"reference",method:{id:"rho.negation"}}, $p]')
  arms=$(jq -c --argjson a "$(with_fb "$p" m141 "prime-abscissa:size=$(r "1.41*$n0")")" '. + [$a]' <<<"$arms")
  for b in 2 4 8 16 32; do
    arms=$(jq -c --argjson a "$(with_fb "$p" res$b "prime-abscissa:size=$b" residual)" '. + [$a]' <<<"$arms")
  done
  spec "R-$1" "exhaustive, prime curve $1: plan, 1.41 N0, residual collector B = 2-32" "$2" 16 "$arms"
}
prime p16 '{"kind":"prime_explicit","p":53411,"a":4589,"b":186,"group_order":53117,"r":53117,"gx":43982,"gy":22687}'
prime p18 '{"kind":"prime_explicit","p":212099,"a":188885,"b":173081,"group_order":212437,"r":212437,"gx":174945,"gy":10250}'
prime p36 '{"kind":"prime_explicit","p":68719476731,"a":68719476728,"b":33,"group_order":68719022539,"r":68719022539,"gx":2,"gy":51741851039}'
prime p37 '{"kind":"prime_explicit","p":81640512137,"a":6206588639,"b":31351229805,"group_order":81640511861,"r":81640511861,"gx":3,"gy":17330771300}'

# W-binary held out: d0 - 1 and d0 + 1 against the plan.
binary() { # tag n seed
  local c="{\"kind\":\"binary_random\",\"n\":$2,\"seed\":$3,\"max_cofactor\":8}" p d0 arms
  p=$(plan_arm --curve "$c"); d0=$(dim_of "$p")
  arms=$(jq -cn --argjson p "$p" --argjson lo "$(with_fb "$p" dm1 "binary-subspace:dimension=$((d0-1))")" \
    --argjson hi "$(with_fb "$p" dp1 "binary-subspace:dimension=$((d0+1))")" \
    '[{name:"rho",role:"reference",method:{id:"rho.negation"}}, $p, $lo, $hi]')
  spec "B-$1" "exhaustive, binary curve $1 (held out): d0 - 1, d0, d0 + 1" "$c" 8 "$arms"
}
binary hb19 19 59297; binary hb23 23 59297; binary hb25 25 59297; binary hb27 27 827675

# K: Koblitz routing at all six §23 sizes, with the weighted plan (0.71 c0) and the descent.
koblitz() { # tag a n
  local c="{\"kind\":\"koblitz\",\"a\":$2,\"n\":$3}" p c0 arms
  p=$($IC plan --koblitz "$2:$3" --ecbench-arm pipeline --json | jq -c '.ecbench_arm'); c0=$(cols_of "$p")
  arms=$(jq -cn --argjson p "$p" --argjson w "$(with_fb "$p" pipeline71 "compact-orbit-scan:columns=$(r "0.71*$c0"),raw_x_cap=100000000")" \
    '[{name:"rho",role:"reference",method:{id:"rho.signed_frobenius_strong"}}, ($p | .role = "candidate"), $w,
      {name:"descent",role:"baseline",method:{id:"ic.koblitz_descent"}}]')
  spec "K-$1" "exhaustive, Koblitz curve $1: pipeline, pipeline at 0.71 c0, §23 descent, strong rho" "$c" 8 "$arms"
}
koblitz k47 1 47; koblitz k57 0 57; koblitz k41 0 41; koblitz k53 0 53; koblitz k59 1 59; koblitz k61 0 61
