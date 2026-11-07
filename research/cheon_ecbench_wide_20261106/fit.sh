#!/usr/bin/env bash
# PROTOCOL.md section 3 with amendment 2: fit-version-2 bounds over all 14
# per-size chunk sessions of a family together (no --tier, no --curve), one
# per family, arm and weight w in {0, 0.1, 1}, with the oracle labels of the
# earlier Cheon records. Then the frontier page. Every figure is computed by
# `ecbench`. Run from the repository root with the sessions' records present.
set -euo pipefail
E=${ECBENCH:-./target/release/ecbench}
R=docs/bounds/records
S=research/cheon_ecbench_wide_20261106/sessions
M='static_dh:n-1:d=shape(0.12943,0.015)'
P='static_dh:n+1:d=shape(0.21106,0.015)'
BITS="36 38 40 42 44 46 48 50 52 54 56 58 60 62"
fit() { # FAMILY ARM ORACLE OUT LABEL
  local src=()
  for b in $BITS; do src+=(--dir "$S/$1-b$b" --audit "$S/$1-b$b.audit.json"); done
  for w in 0 0.1 1; do
    case $w in 0) sfx=; wa=();; *) sfx="-w$w"; wa=(--weight "$w");; esac
    "$E" bound fit "${src[@]}" --arm "$2" --oracle "$3" --label "$5$sfx" "${wa[@]}" \
      --root . --out "$R/$4$sfx.json" >/dev/null
  done
}
fit minus cheon-comb "$M" cheon-ecbench-wide-minus-comb "cheon.pminus1_comb on 14 n-1 shaped curves, 36 to 62 bits"
fit minus rho-neg "$M" cheon-ecbench-wide-minus-rho "rho.negation on 14 n-1 shaped curves, 36 to 62 bits (reference, queries nothing)"
fit plus cheon-comb "$P" cheon-ecbench-wide-plus-comb "cheon.pplus1_comb on 14 n+1 shaped curves, 36 to 62 bits"
fit plus rho-neg "$P" cheon-ecbench-wide-plus-rho "rho.negation on 14 n+1 shaped curves, 36 to 62 bits (reference, queries nothing)"
"$E" frontier build --bounds "$R" --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md
