#!/bin/sh
# PROTOCOL.md analysis, thin orchestration only (every figure is native `ecbench`):
#   0. ic-ksum4 at 20-40 bits must reproduce #2808's per-run counts (replicate.mjs)
#   1. replay-all audit of every test session (counts must reproduce exactly)
#   2. the AGENTS.md §5a band with pairwise verdicts
#   3. exponent fits per arm, tier and band weight (`bound fit --weight`)
set -u
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
S=test/sessions
A=analysis
SIZES="20 24 28 32 36 40 44 48"
mkdir -p test/audits "$A/bounds"
node replicate.mjs "$S" ../prime_ksum_table_20261028/test/sessions > "$A/replication.txt" \
  || { echo "ic-ksum4 replication FAILED: study stops (PROTOCOL.md)"; exit 1; }
for b in $SIZES; do
  [ -f "test/audits/P$b.json" ] || "$E" verify --dir "$S/P$b" --replay-all --out "test/audits/P$b.json" > /dev/null 2>&1 \
    || echo "audit FAILED: P$b (receipt kept)"
done
dirs=""; audits=""
for b in $SIZES; do dirs="$dirs $S/P$b"; audits="$audits --audit test/audits/P$b.json"; done
# shellcheck disable=SC2086
"$E" band --dir $dirs --weights 0,0.1,1 --ranks --json "$A/band-prime.json" > "$A/band-prime.md"

RULE="per-size parameters chosen on the train panel by research/prime_ksum_k5_20261028/PROTOCOL.md (params.json; ic-ksum4 below 44 bits: research/prime_ksum_table_20261028/params.json)"
fit() { # ARM TIER WEIGHT
  case "$1" in
    ic-*) set -- "$@" --param-rule "$RULE" ;;
  esac
  arm=$1 tier=$2 w=$3; shift 3
  # shellcheck disable=SC2086
  "$E" bound fit --dir $dirs $audits --arm "$arm" --tier "$tier" --weight "$w" --root ../.. "$@" \
    --label "prime-ksum-k5 $arm $tier w=$w" --out "$A/bounds/P-$arm-$tier-w$w.json" \
    > /dev/null 2>> "$A/bounds/fit.log" || echo "fit refused: $arm $tier w=$w" >> "$A/bounds/fit.log"
}
: > "$A/bounds/fit.log"
for w in 0 0.1 1; do
  for arm in rho-neg bsgs-neg ic-ksum4 ic-ksum5; do
    for t in toy medium; do fit "$arm" "$t" "$w"; done
  done
done
