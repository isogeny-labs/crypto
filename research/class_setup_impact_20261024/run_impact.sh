#!/bin/bash
# Impact of the pre-2026-10-18 rho.quartic / rho.cubic class set-up
# undercharge on every committed session that carries it.  For each
# session: ecbench compare (and table) on the session as recorded, and on a
# scratch copy whose totals carry the class set-up their phases already
# hold (examples/ecbench_class_setup_corrected_copy.rs).  Committed
# sessions are only read; copies live under $TMP and are deleted.
set -euo pipefail
cd "$(dirname "$0")/../.."
S=research/class_setup_impact_20261024
B=./target/release/ecbench
E=./target/release/examples/ecbench_class_setup_corrected_copy
TMP=${TMPDIR:-/tmp}/class-setup-impact
one() { # name dir pairs...
  local name=$1 d=$2; shift 2
  rm -rf "$TMP/fixed"; cp -r "$d" "$TMP/fixed"; rm -rf "$TMP/fixed/comparisons"; $E "$TMP/fixed" > /dev/null
  for v in recorded corrected; do
    local dd=$d; [ $v = corrected ] && dd=$TMP/fixed
    $B table --dir "$dd" > "$S/tables/$name.$v.md"
    for pair in "$@"; do
      $B compare --dir "$dd" --a "${pair%%/*}" --b "${pair##*/}" --json --allow-interrupted > "$S/compare/$name.${pair##*/}__${pair%%/*}.$v.json"
    done
  done
}
mkdir -p "$TMP"
one prime_rho_automorphism.quartic research/prime_rho_automorphism_20261015/sessions/quartic rho-neg/rho-quartic
one prime_rho_automorphism.cubic research/prime_rho_automorphism_20261015/sessions/cubic rho-neg/rho-cubic
for s in research/drastic_prime_conductor_gap_round2_20261012/sessions/d3-ell{2033609,1016839}-cubic; do
  one "conductor_gap_round2.$(basename $s)" "$s"
done
for s in research/drastic_prime_conductor_gap_round3_20261012/sessions/{d4-ell1016839-auto,d3-ell2033609-cubic,d3-ell1016839-cubic}; do
  one "conductor_gap_round3.$(basename $s)" "$s"
done
for t in research/volcano_ecdlp_census_20261017/sessions/V3-{D,N,ctl}-L0-c0 research/volcano_ecdlp_census_followup_20261023/sessions/V5-{D,N,ctl}-L0-c0; do
  rm -rf "$TMP/x"; mkdir -p "$TMP/x"; tar -xJf "$t.tar.xz" -C "$TMP/x"
  d=$(dirname "$(find "$TMP/x" -name session.json | head -1)")
  study=$(basename "$(dirname "$(dirname "$t")")" | sed 's/_2026.*//')
  one "$study.$(basename "$t")" "$d" rho-neg/rho-cubic rho-cubic/ic
  rm -rf "$TMP/x"
done
for s in research/isogeny-volcano-cost_20261017/sessions/{j0,j1728}/spec-crater-aut; do
  rm -rf "$TMP/x"; cp -r "$s" "$TMP/x"; gunzip "$TMP/x"/*.gz
  arms=$(jq -r '[.arms[].name] | join(" ")' "$TMP/x/spec.json")
  set -- $arms
  pairs=(); for b in "${@:2}"; do pairs+=("$1/$b"); done
  one "isogeny_volcano_cost.$(basename "$(dirname "$s")")" "$TMP/x" "${pairs[@]}"
  rm -rf "$TMP/x"
done
rm -rf "$TMP"
