#!/usr/bin/env sh
# PROTOCOL.md section 3: whole-session fit-version-2 bounds of the medium-tier
# sessions (no --tier, no --curve), one per session, arm and weight, with the
# original records' oracle labels. Then the frontier page. Every figure is
# computed by `ecbench`. Run from the repository root.
set -eu
E=${ECBENCH:-./target/release/ecbench}
R=docs/bounds/records
S=research/cheon_ecbench_medium_20261106/sessions
M='static_dh:n-1:d=shape(0.12943,0.015)'
P='static_dh:n+1:d=shape(0.21106,0.015)'
fit() { # SESSION ARM ORACLE OUT LABEL
  for w in 0 0.1 1; do
    case $w in 0) sfx=; wa=;; *) sfx="-w$w"; wa="--weight $w";; esac
    "$E" bound fit --dir "$S/$1-20261106" --arm "$2" --audit "$S/$1-20261106.audit.json" \
      --oracle "$3" --label "$5$sfx" $wa --root . --out "$R/$4$sfx.json" >/dev/null
  done
}
fit minus cheon-comb "$M" cheon-ecbench-medium-minus-comb "cheon.pminus1_comb on the medium-tier n-1 shaped curves"
fit minus rho-neg "$M" cheon-ecbench-medium-minus-rho "rho.negation on the medium-tier n-1 shaped curves (reference, queries nothing)"
fit minus-naive cheon-naive "$M" cheon-ecbench-medium-minus-naive "cheon.pminus1_naive on the medium-tier n-1 shaped curves"
fit minus-naive rho-neg "$M" cheon-ecbench-medium-minus-naive-rho "rho.negation beside cheon.pminus1_naive (medium tier, 36 to 52 bits)"
fit plus cheon-comb "$P" cheon-ecbench-medium-plus-comb "cheon.pplus1_comb on the medium-tier n+1 shaped curves"
fit plus rho-neg "$P" cheon-ecbench-medium-plus-rho "rho.negation on the medium-tier n+1 shaped curves (reference, queries nothing)"
"$E" frontier build --bounds "$R" --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md
