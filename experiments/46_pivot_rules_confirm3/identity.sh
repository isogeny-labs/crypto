#!/usr/bin/env bash
# E39 G2.191 (binary clauses): existing configurations byte identical under
# the E39 binaries (canonical JSON: jq -S -c without *_ms, ms, wall fields).
#  (a) E37's eleven identity configurations, p = 101 seed 2, salt 0, against
#      E37's records in experiments/44_pivot_rules_confirm/identity/ (E37
#      binary; the two e36bin_* records are E36's pinned binary);
#  (b) E37's stored end-to-end records at p = 101 seed 2, salt 0xE37E0001;
#  (c) E37's e37_checks (unchanged source, built at the E39 commit) against
#      E37's checks.json;
#  (d) E37's e37_walk_e24 (unchanged source, built at the E39 commit) against
#      E37's walk_e24.json (walk (iii) on E24's curves, the E24 rho);
#  (e) E38's e38_checks (unchanged source, built at the E39 commit) against
#      E38's pilot.json (P1) and pilot_p2.json (P2, --fallback cfmid).
# E38's identity.sh with clause (e) added and the output directory moved.
#   usage: identity.sh <E39 bin dir> <cpus>
set -u
cd "$(dirname "$0")/../.."
e37=experiments/44_pivot_rules_confirm
e38=experiments/45_pivot_rules_confirm2
out=experiments/46_pivot_rules_confirm3/identity
mkdir -p "$out"
nb=$1; cpus=$2
export RAYON_NUM_THREADS=4
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
run() { # name binary solver extra...
  name=$1; f=$out/$1.json; b=$2; s=$3; shift 3
  [ -s "$f" ] || taskset -c "$cpus" "$b" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$s" "$@" --json "$f" >"$out/$name.log" 2>&1
}
g=$nb/quartic_pipeline_gls; v=$nb/quartic_pipeline_v4
run macaulay "$g" macaulay
run chain "$g" chain
run chainsparsest "$g" chain-sparsest
run chain_karatsuba "$g" chain --karatsuba
run est_karatsuba "$g" chain-sparsest-est --karatsuba
for r in chain-fifo chain-static chain-fifo-static; do run "${r}_karatsuba" "$g" "$r" --karatsuba; done
[ -s "$out/v4_default.json" ] || taskset -c "$cpus" "$v" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
  --max-residuals 5000000 --json "$out/v4_default.json" >"$out/v4_default.log" 2>&1
for s in chain-counted chain-fifo chain-static chain-fifo-static chain-sparsest-est; do
  run "e37salt_gls_$s" "$g" "$s" --karatsuba --stream-salt E37E0001
done
run e37salt_e24_chain-fifo "$v" chain-fifo --karatsuba --stream-salt E37E0001
[ -s "$out/e37_checks.json" ] || RAYON_NUM_THREADS=1 taskset -c "$cpus" "$nb/e37_checks" \
  --json "$out/e37_checks.json" >/dev/null 2>"$out/e37_checks.log"
[ -s "$out/e37_walk_e24.json" ] || taskset -c "$cpus" "$nb/e37_walk_e24" \
  --json "$out/e37_walk_e24.json" >"$out/e37_walk_e24.log" 2>&1
[ -s "$out/e38_pilot.json" ] || RAYON_NUM_THREADS=1 taskset -c "$cpus" "$nb/e38_checks" \
  --salt E38D0001 --calls 8 --json "$out/e38_pilot.json" >/dev/null 2>"$out/e38_pilot.log"
[ -s "$out/e38_pilot_p2.json" ] || RAYON_NUM_THREADS=1 taskset -c "$cpus" "$nb/e38_checks" \
  --salt E38D0101 --calls 8 --fallback cfmid --json "$out/e38_pilot_p2.json" >/dev/null 2>"$out/e38_pilot_p2.log"
row() { printf '{"config": "%s", "reference": "%s", "reference_hash": "%s", "e39": "%s", "identical": %s}' \
  "$1" "$2" "$3" "$4" "$([ "$3" = "$4" ] && echo true || echo false)"; }
{
  echo '{"what": "E39 G2.191: E39 binaries vs E37 identity records, E37 stored end-to-end records (salt 0xE37E0001), E37 checks.json and walk_e24.json, E38 pilot.json and pilot_p2.json; p = 101 seed 2", "runs": ['
  for c in macaulay chain chainsparsest chain_karatsuba est_karatsuba chain-fifo_karatsuba chain-static_karatsuba chain-fifo-static_karatsuba v4_default; do
    row "$c" "E37 identity record" "$(canon "$e37/identity/$c.json")" "$(canon "$out/$c.json")"; echo ','
  done
  row chain_karatsuba "E36 pinned binary (E37 identity)" "$(canon "$e37/identity/e36bin_chain_karatsuba.json")" "$(canon "$out/chain_karatsuba.json")"; echo ','
  row chain-fifo_karatsuba "E36 pinned binary (E37 identity)" "$(canon "$e37/identity/e36bin_chain-fifo_karatsuba.json")" "$(canon "$out/chain-fifo_karatsuba.json")"; echo ','
  for s in chain-counted chain-fifo chain-static chain-fifo-static chain-sparsest-est; do
    row "gls_${s}_p101s2_salt_E37E0001" "E37 stored e2e record" "$(canon "$e37/e2e/gls_${s}_p101s2.json")" "$(canon "$out/e37salt_gls_$s.json")"; echo ','
  done
  row "e24_chain-fifo_p101s2_salt_E37E0001" "E37 stored e2e record" "$(canon "$e37/e2e/e24_chain-fifo_p101s2.json")" "$(canon "$out/e37salt_e24_chain-fifo.json")"; echo ','
  row e37_checks "E37 checks.json" "$(canon "$e37/checks.json")" "$(canon "$out/e37_checks.json")"; echo ','
  row e37_walk_e24 "E37 walk_e24.json" "$(canon "$e37/walk_e24.json")" "$(canon "$out/e37_walk_e24.json")"; echo ','
  row e38_pilot "E38 pilot.json (P1)" "$(canon "$e38/pilot.json")" "$(canon "$out/e38_pilot.json")"; echo ','
  row e38_pilot_p2 "E38 pilot_p2.json (P2)" "$(canon "$e38/pilot_p2.json")" "$(canon "$out/e38_pilot_p2.json")"
  echo ']}'
} > experiments/46_pivot_rules_confirm3/identity_existing.json
cat experiments/46_pivot_rules_confirm3/identity_existing.json
