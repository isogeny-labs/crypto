#!/bin/sh
# PROTOCOL.md analysis, thin orchestration only (every figure is native `ecbench`):
#   1. replay-all audit of every test session (counts must reproduce exactly)
#   2. the AGENTS.md §5a band with pairwise verdicts
#   3. exponent fits per arm, tier and band weight (`bound fit --weight`)
set -u
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
S=test/sessions
A=analysis
mkdir -p test/audits "$A/bounds"
for b in 20 24 28 32 36 40; do
  [ -f "test/audits/P$b.json" ] || "$E" verify --dir "$S/P$b" --replay-all --out "test/audits/P$b.json" > /dev/null 2>&1 \
    || echo "audit FAILED: P$b (receipt kept)"
done
"$E" band --dir "$S"/P20 "$S"/P24 "$S"/P28 "$S"/P32 "$S"/P36 "$S"/P40 --ranks --json "$A/band-prime.json" > "$A/band-prime.md"

RULE="per-size parameters chosen on the train panel by research/prime_ksum_table_20261028/PROTOCOL.md (params.json; ic-pair: research/ic_frontier_campaign_20261015/params.json)"
fit() { # ARM TIER WEIGHT
  dirs=""; audits=""
  for b in 20 24 28 32 36 40; do dirs="$dirs $S/P$b"; audits="$audits --audit test/audits/P$b.json"; done
  case "$1" in
    ic-*)
      # shellcheck disable=SC2086
      "$E" bound fit --dir $dirs $audits --arm "$1" --tier "$2" --weight "$3" --root ../.. \
        --param-rule "$RULE" --label "prime-ksum $1 $2 w=$3" --out "$A/bounds/P-$1-$2-w$3.json" ;;
    *)
      # shellcheck disable=SC2086
      "$E" bound fit --dir $dirs $audits --arm "$1" --tier "$2" --weight "$3" --root ../.. \
        --label "prime-ksum $1 $2 w=$3" --out "$A/bounds/P-$1-$2-w$3.json" ;;
  esac > /dev/null 2>> "$A/bounds/fit.log" || echo "fit refused: $1 $2 w=$3" >> "$A/bounds/fit.log"
}
: > "$A/bounds/fit.log"
for w in 0 0.1 1; do
  for arm in rho-neg bsgs-neg kangaroo ic-pair ic-ksum3 ic-ksum4; do
    for t in toy medium; do fit "$arm" "$t" "$w"; done
  done
done
