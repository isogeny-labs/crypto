#!/usr/bin/env bash
# E36 G2.154: existing solvers bit-identical under the E36 binary.  Runs
# macaulay, chain, chain-sparsest (schoolbook), chain and chain-sparsest-est
# (--karatsuba) at p = 101 seed 2, both arms, RAYON_NUM_THREADS=4 on two
# cores, and compares the canonical JSON (jq -S -c without *_ms, ms, wall
# fields) with E33's identity hashes and E33's stored B'' record.
#   usage: identity.sh <pinned quartic_pipeline_gls>
set -u
cd "$(dirname "$0")/../.."
out=experiments/40_pivot_rules/identity
mkdir -p "$out"
bin=$1
export RAYON_NUM_THREADS=4
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
run() { # name solver extra
  f=$out/$1.json
  [ -s "$f" ] || taskset -c 2,3 "$bin" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$2" $3 --json "$f" >"$out/$1.log" 2>&1
}
run macaulay macaulay ""
run chain chain ""
run chainsparsest chain-sparsest ""
run chain_karatsuba chain --karatsuba
run est_karatsuba chain-sparsest-est --karatsuba
e33=experiments/37_solver_b_pivot_count/identity_existing_solvers.json
{
  echo '{"what": "E36 G2.154: E36 binary vs E33 identity hashes (macaulay, chain, chainsparsest, chain_karatsuba) and E33 stored chain-sparsest-est --karatsuba record, p = 101 seed 2", "runs": ['
  first=1
  for c in macaulay chain chainsparsest chain_karatsuba; do
    want=$(jq -r --arg c "$c" '.runs[] | select(.config==$c) | .new' "$e33")
    got=$(canon "$out/$c.json")
    [ $first = 1 ] || echo ','; first=0
    printf '{"config": "%s", "reference": "%s", "e36": "%s", "identical": %s}' "$c" "$want" "$got" "$([ "$want" = "$got" ] && echo true || echo false)"
  done
  want=$(canon experiments/37_solver_b_pivot_count/e2e_chain-sparsest-est_karatsuba_p101s2.json)
  got=$(canon "$out/est_karatsuba.json")
  printf ',{"config": "chain-sparsest-est_karatsuba", "reference": "%s", "e36": "%s", "identical": %s}' "$want" "$got" "$([ "$want" = "$got" ] && echo true || echo false)"
  echo ']}'
} > experiments/40_pivot_rules/identity_existing_solvers.json
cat experiments/40_pivot_rules/identity_existing_solvers.json
