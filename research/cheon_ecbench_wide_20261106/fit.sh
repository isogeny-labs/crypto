#!/usr/bin/env sh
# PROTOCOL.md section 3: whole-session fit-version-2 bounds of the two wide
# sessions (no --tier, no --curve), one per session, arm and weight w in
# {0, 0.1, 1}, with the oracle labels of the earlier Cheon records. Then the
# frontier page. Every figure is computed by `ecbench`. Run from the
# repository root.
set -eu
E=${ECBENCH:-./target/release/ecbench}
R=docs/bounds/records
S=research/cheon_ecbench_wide_20261106/sessions
M='static_dh:n-1:d=shape(0.12943,0.015)'
P='static_dh:n+1:d=shape(0.21106,0.015)'
fit() { # SESSION ARM ORACLE OUT LABEL
  for w in 0 0.1 1; do
    case $w in 0) sfx=; wa=;; *) sfx="-w$w"; wa="--weight $w";; esac
    "$E" bound fit --dir "$S/$1-20261106" --arm "$2" --audit "$S/$1-20261106.audit.json" \
      --oracle "$3" --label "$5$sfx" $wa --root . --out "$R/$4$sfx.json" >/dev/null
  done
}
fit minus cheon-comb "$M" cheon-ecbench-wide-minus-comb "cheon.pminus1_comb on 14 n-1 shaped curves, 36 to 62 bits"
fit minus rho-neg "$M" cheon-ecbench-wide-minus-rho "rho.negation on 14 n-1 shaped curves, 36 to 62 bits (reference, queries nothing)"
fit plus cheon-comb "$P" cheon-ecbench-wide-plus-comb "cheon.pplus1_comb on 14 n+1 shaped curves, 36 to 62 bits"
fit plus rho-neg "$P" cheon-ecbench-wide-plus-rho "rho.negation on 14 n+1 shaped curves, 36 to 62 bits (reference, queries nothing)"
"$E" frontier build --bounds "$R" --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md
