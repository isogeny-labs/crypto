#!/usr/bin/env bash
# Put the twelve medium-tier bound constants that fit.sh sealed into the open
# benchmark record, one `benchrec metric` per arm and pricing weight. The value,
# interval and level are copied from the sealed bound record by jq; nothing is
# computed here.   record_bounds.sh BENCH_RECORD     (run from the repository root)
set -euo pipefail
[[ $# == 1 ]] || { echo 'usage: record_bounds.sh BENCH_RECORD' >&2; exit 2; }
rec=$1
R=docs/bounds/records
for arm in rho-neg rho-plain bsgs-neg rho-neg-v2; do
  for w in 0 0.1 1; do
    case $w in 0) sfx='';; *) sfx="-w$w";; esac
    f=$R/prime-medium-$arm$sfx.json
    [[ -f $f ]] || { echo "FAIL: $f is missing" >&2; exit 60; }
    value=$(jq -er '.constant.s.value' "$f")
    lo=$(jq -er '.constant.s.ci95[0]' "$f")
    hi=$(jq -er '.constant.s.ci95[1]' "$f")
    level=$(jq -er '.level' "$f")
    python3 tools/benchrec.py metric "$rec" --name "prime-medium/$arm/w$w/s" --value "$value" --unit S \
      --spread-low "$lo" --spread-high "$hi" \
      --note "constant.s of $f (level $level, weight $w GAE per unpriced native operation; counted work only, no wall time)"
  done
done
printf 'recorded 12 metrics in %s\n' "$rec"
