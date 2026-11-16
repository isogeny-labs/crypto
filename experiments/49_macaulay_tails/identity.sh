#!/usr/bin/env bash
# E42 G2.230, existing configurations under the E42 binary: E40's seven
# (macaulay, chain, chain-sparsest, chain --karatsuba at GLS p = 101 seed 2,
# salt 0, against E33's identity hashes; chain-counted --karatsuba GLS
# p = 101 s2 and E24 p = 101 s1, chain-fifo --karatsuba GLS p = 101 s2 with
# --stream-salt E37E0001 against E37's stored records) and V
# (chain-profile --karatsuba, E37E0001, GLS p = 101 s2 and E24 p = 101 s1)
# against E40's stored V records.  Canonical JSON: jq -S -c without *_ms,
# ms, wall fields.
#   usage: identity.sh <E42 bin dir> <cpus>
set -u
cd "$(dirname "$0")/../.."
out=experiments/49_macaulay_tails/identity
mkdir -p "$out"
nb=$1; cpus=$2
export RAYON_NUM_THREADS=4
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
run() { # name binary p seed salt solver extra...
  name=$1; f=$out/$1.json; b=$2; p=$3; s=$4; salt=$5; solver=$6; shift 6
  [ -s "$f" ] || taskset -c "$cpus" "$b" --sizes "$p" --first-seed "$s" --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$solver" --stream-salt "$salt" "$@" --json "$f" >"$out/$name.log" 2>&1
}
run macaulay "$nb/quartic_pipeline_gls" 101 2 0 macaulay
run chain "$nb/quartic_pipeline_gls" 101 2 0 chain
run chainsparsest "$nb/quartic_pipeline_gls" 101 2 0 chain-sparsest
run chain_karatsuba "$nb/quartic_pipeline_gls" 101 2 0 chain --karatsuba
run gls_counted_e37 "$nb/quartic_pipeline_gls" 101 2 E37E0001 chain-counted --karatsuba
run gls_fifo_e37 "$nb/quartic_pipeline_gls" 101 2 E37E0001 chain-fifo --karatsuba
run e24_counted_e37 "$nb/quartic_pipeline_v4" 101 1 E37E0001 chain-counted --karatsuba
run gls_profile_e40 "$nb/quartic_pipeline_gls" 101 2 E37E0001 chain-profile --karatsuba
run e24_profile_e40 "$nb/quartic_pipeline_v4" 101 1 E37E0001 chain-profile --karatsuba
e33=experiments/37_solver_b_pivot_count/identity_existing_solvers.json
e37=experiments/44_pivot_rules_confirm/e2e
e40=experiments/47_macaulay_entries/e2e
row() { printf '{"config": "%s", "reference": "%s", "reference_hash": "%s", "e42": "%s", "identical": %s}' \
  "$1" "$2" "$3" "$4" "$([ "$3" = "$4" ] && echo true || echo false)"; }
{
  echo '{"what": "E42 G2.230: existing configurations under the E42 binary vs E33 identity hashes, E37 and E40 stored records", "runs": ['
  for c in macaulay chain chainsparsest chain_karatsuba; do
    row "$c" "E33 identity hash" "$(jq -r --arg c "$c" '.runs[] | select(.config==$c) | .new' "$e33")" "$(canon "$out/$c.json")"; echo ','
  done
  row "gls chain-counted --karatsuba E37E0001 p101s2" "E37 stored record" "$(canon $e37/gls_chain-counted_p101s2.json)" "$(canon "$out/gls_counted_e37.json")"; echo ','
  row "gls chain-fifo --karatsuba E37E0001 p101s2" "E37 stored record" "$(canon $e37/gls_chain-fifo_p101s2.json)" "$(canon "$out/gls_fifo_e37.json")"; echo ','
  row "e24 chain-counted --karatsuba E37E0001 p101s1" "E37 stored record" "$(canon $e37/e24_chain-counted_p101s1.json)" "$(canon "$out/e24_counted_e37.json")"; echo ','
  row "gls chain-profile --karatsuba E37E0001 p101s2" "E40 stored record" "$(canon $e40/gls_chain-profile_p101s2.json)" "$(canon "$out/gls_profile_e40.json")"; echo ','
  row "e24 chain-profile --karatsuba E37E0001 p101s1" "E40 stored record" "$(canon $e40/e24_chain-profile_p101s1.json)" "$(canon "$out/e24_profile_e40.json")"
  echo ']}'
} > experiments/49_macaulay_tails/identity_existing_solvers.json
cat experiments/49_macaulay_tails/identity_existing_solvers.json
