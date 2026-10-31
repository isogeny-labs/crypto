#!/bin/bash
# The recompute check must pass on the committed board, fail on a 1% change of any compared figure, and
# fail when a board row is missing.  Needs only jq.  Run from the repository root.
set -u
cd "$(git rev-parse --show-toplevel)"
chk=scripts/ic_audit/check_board_recompute.sh
board=docs/ic/leaderboard.json
t=$(mktemp -d); trap 'rm -r "$t"' EXIT
bash $chk > /dev/null || { echo "FAIL: the committed board does not recompute"; exit 1; }
for field in s reference_s ratio_rho ratio_floor; do
  case $field in
    reference_s) path='.reference_s' ;;
    *) path=".best.$field" ;;
  esac
  for fam in A B C; do
    jq --arg fam "$fam" "(first(.board[]|select(.family==\$fam))|$path) |= . * 1.01" "$board" > "$t/b.json"
    if IC_BOARD="$t/b.json" bash $chk > /dev/null 2>&1; then echo "FAIL: a 1% change of $field in table $fam was not caught"; exit 1; fi
  done
done
jq 'del(.board[0])' "$board" > "$t/b.json"
if IC_BOARD="$t/b.json" bash $chk > /dev/null 2>&1; then echo "FAIL: a missing board row was not caught"; exit 1; fi
echo "recompute check tests: 12 tamper cases and the missing-row case are all caught"
# The reference-currency check must fail on a stale assessment and on an empty one.
cur=scripts/ic_audit/check_reference_currency.sh; cfg=docs/ic/validation/reference-currency.json
bash $cur > /dev/null || { echo "FAIL: the committed currency file is not current"; exit 1; }
jq '.tables.A.assessed_blobs["src/cryptanalysis/koblitz_strong_rho.rs"] = "0000000000000000000000000000000000000000"' $cfg > "$t/c.json"
if CURRENCY_CFG="$t/c.json" bash $cur > /dev/null 2>&1; then echo "FAIL: a stale rho assessment was not caught"; exit 1; fi
jq '.tables.A.assessment = ""' $cfg > "$t/c.json"
if CURRENCY_CFG="$t/c.json" bash $cur > /dev/null 2>&1; then echo "FAIL: an empty rho assessment was not caught"; exit 1; fi
echo "reference-currency check tests: stale and empty assessments are caught"
