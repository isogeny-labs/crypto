#!/usr/bin/env sh
# PROTOCOL.md section 6, steps 2-3: medium-tier fit-version-2 bounds of the six
# job sessions, one per arm and pricing weight (AGENTS.md section 5a), checked,
# the frontier page rebuilt, then the R(N) fits against the sealed toy bounds.
# Every figure is computed by `ecbench` or the frozen native example. Run from
# the repository root after the six sessions and their audits are committed at
#   research/prime_medium_tier_rho_20261107/sessions/J1 .. J6
#   research/prime_medium_tier_rho_20261107/sessions/J1.audit.json .. J6.audit.json
set -eu
E=${ECBENCH:-./target/release/ecbench}
D=research/prime_medium_tier_rho_20261107
S=$D/sessions
R=docs/bounds/records
dirs=""
for j in J1 J2 J3 J4 J5 J6; do dirs="$dirs --dir $S/$j --audit $S/$j.audit.json"; done
fit() { # ARM OUT LABEL
  for w in 0 0.1 1; do
    case $w in 0) sfx=; wa=;; *) sfx="-w$w"; wa="--weight $w";; esac
    # shellcheck disable=SC2086
    "$E" bound fit $dirs --arm "$1" --tier medium --label "$3$sfx" $wa --root . \
      --out "$R/$2$sfx.json" >/dev/null
    "$E" bound check --root . --record "$R/$2$sfx.json"
  done
}
fit rho-neg prime-medium-rho-neg "rho.negation on generic prime-order curves, log2 r 32-49 (medium tier, reference)"
fit rho-plain prime-medium-rho-plain "rho.plain on generic prime-order curves, log2 r 32-49 (medium tier)"
fit bsgs-neg prime-medium-bsgs-neg "bsgs.negation on generic prime-order curves, log2 r 32-49 (medium tier)"
fit rho-neg-v2 prime-medium-rho-neg-v2 "rho.negation_v2 on generic prime-order curves, log2 r 32-49 (medium tier)"
"$E" frontier build --bounds "$R" --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md

X=${RFIT:-./target/release/examples/prime_medium_tier_rho_rfit}
mkdir -p "$D/analysis"
"$X" --seed 20261107 --resamples 2000 --out "$D/analysis/rfit-rho-neg.json" \
  "$R/calibration-prime-rho-neg-v2.json" "$R/prime-medium-rho-neg.json"
"$X" --seed 20261107 --resamples 2000 --out "$D/analysis/rfit-rho-plain.json" \
  "$R/calibration-prime-rho-plain-v2.json" "$R/prime-medium-rho-plain.json"
"$X" --seed 20261107 --resamples 2000 --out "$D/analysis/rfit-bsgs-neg.json" \
  "$R/calibration-prime-bsgs-neg-v2.json" "$R/prime-medium-bsgs-neg.json"
