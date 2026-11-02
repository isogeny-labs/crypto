#!/usr/bin/env bash
# E37 G2.164 (with the §9.44 amendment): existing configurations byte
# identical under the E37 binary.  At p = 101 seed 2, salt 0, both arms,
# RAYON_NUM_THREADS=4: macaulay, chain, chain-sparsest (schoolbook) and
# chain --karatsuba against E33's identity hashes; chain-sparsest-est
# --karatsuba against E33's stored record; chain-fifo, chain-static,
# chain-fifo-static --karatsuba against E36's stored records; chain and
# chain-fifo --karatsuba against E36's pinned binary; the default
# quartic_pipeline_v4 run against a binary built from the pre-E37 tree
# 024c0ca417.  Canonical JSON: jq -S -c without *_ms, ms, wall fields.
#   usage: identity.sh <E37 bin dir> <E36 pinned gls binary> <pre-E37 v4 binary> <cpus>
set -u
cd "$(dirname "$0")/../.."
out=experiments/44_pivot_rules_confirm/identity
mkdir -p "$out"
nb=$1; old_gls=$2; old_v4=$3; cpus=$4
export RAYON_NUM_THREADS=4
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
run() { # file binary solver extra...
  f=$out/$1.json; b=$2; s=$3; shift 3
  [ -s "$f" ] || taskset -c "$cpus" "$b" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$s" "$@" --json "$f" >"$out/$1.log" 2>&1
}
run macaulay "$nb/quartic_pipeline_gls" macaulay
run chain "$nb/quartic_pipeline_gls" chain
run chainsparsest "$nb/quartic_pipeline_gls" chain-sparsest
run chain_karatsuba "$nb/quartic_pipeline_gls" chain --karatsuba
run est_karatsuba "$nb/quartic_pipeline_gls" chain-sparsest-est --karatsuba
for r in chain-fifo chain-static chain-fifo-static; do run "${r}_karatsuba" "$nb/quartic_pipeline_gls" "$r" --karatsuba; done
run e36bin_chain_karatsuba "$old_gls" chain --karatsuba
run e36bin_chain-fifo_karatsuba "$old_gls" chain-fifo --karatsuba
v4() { # file binary
  f=$out/$1.json
  [ -s "$f" ] || taskset -c "$cpus" "$2" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --json "$f" >"$out/$1.log" 2>&1
}
v4 v4_default "$nb/quartic_pipeline_v4"
v4 v4_default_pre_e37 "$old_v4"
e33=experiments/37_solver_b_pivot_count/identity_existing_solvers.json
e36=experiments/40_pivot_rules
row() { printf '{"config": "%s", "reference": "%s", "reference_hash": "%s", "e37": "%s", "identical": %s}' \
  "$1" "$2" "$3" "$4" "$([ "$3" = "$4" ] && echo true || echo false)"; }
{
  echo '{"what": "E37 G2.164: E37 binary vs E33 identity hashes, E33/E36 stored records, E36 pinned binary and a pre-E37 v4 binary; p = 101 seed 2, salt 0", "runs": ['
  for c in macaulay chain chainsparsest chain_karatsuba; do
    row "$c" "E33 identity hash" "$(jq -r --arg c "$c" '.runs[] | select(.config==$c) | .new' "$e33")" "$(canon "$out/$c.json")"; echo ','
  done
  row est_karatsuba "E33 stored record" "$(canon experiments/37_solver_b_pivot_count/e2e_chain-sparsest-est_karatsuba_p101s2.json)" "$(canon "$out/est_karatsuba.json")"; echo ','
  for r in chain-fifo chain-static chain-fifo-static; do
    row "${r}_karatsuba" "E36 stored record" "$(canon "$e36/e2e_${r}_karatsuba_p101s2.json")" "$(canon "$out/${r}_karatsuba.json")"; echo ','
  done
  row chain_karatsuba "E36 pinned binary" "$(canon "$out/e36bin_chain_karatsuba.json")" "$(canon "$out/chain_karatsuba.json")"; echo ','
  row chain-fifo_karatsuba "E36 pinned binary" "$(canon "$out/e36bin_chain-fifo_karatsuba.json")" "$(canon "$out/chain-fifo_karatsuba.json")"; echo ','
  row v4_default "pre-E37 binary (024c0ca417)" "$(canon "$out/v4_default_pre_e37.json")" "$(canon "$out/v4_default.json")"
  echo ']}'
} > experiments/44_pivot_rules_confirm/identity_existing_solvers.json
# E24's stored record vs the E37 default v4 record, field by field (reported, not judged).
jq -n --slurpfile a experiments/29_quartic_pipeline_v4/p101s2.json --slurpfile b "$out/v4_default.json" '
  ($a[0][0]) as $x | ($b[0][0]) as $y |
  {what: "E24 stored p101s2 record vs E37 default v4 record, top-level fields (timings excluded)",
   only_in_e37: [($y|keys[]) as $k | select($x|has($k)|not) | $k],
   differing: [($x|keys[]) as $k | select(($k|test("_ms$|^ms$|wall"))|not) | select($x[$k] != $y[$k]) | $k],
   equal: [($x|keys[]) as $k | select(($k|test("_ms$|^ms$|wall"))|not) | select($x[$k] == $y[$k]) | $k]}' \
  > experiments/44_pivot_rules_confirm/identity_e24_fields.json
cat experiments/44_pivot_rules_confirm/identity_existing_solvers.json experiments/44_pivot_rules_confirm/identity_e24_fields.json
