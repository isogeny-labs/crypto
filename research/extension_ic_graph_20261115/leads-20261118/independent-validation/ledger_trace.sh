#!/usr/bin/env bash
# Trace the pcc ledger registration of the lane's claims (read-only git).
# Usage: ledger_trace.sh REPO_ROOT REV > results/ledger.txt
set -uo pipefail
cd "${1:?REPO_ROOT}" || exit 2
REV=${2:?REV}
echo "validated rev $REV"
for id in feaf31 6432aa e9f39d 7defbb 6224b9 853115 0ecee3; do
  c=PCC-20261117-$id
  in_rev=$(git show "$REV:pcc/ledger.jsonl" | grep -c "\"$c\"")
  added=$(git log --full-history --format=%h --reverse -S "\"$c\"" "$REV" -- pcc/ledger.jsonl | head -1)
  echo "$c: entries at ${REV:0:10} = $in_rev; first ledger commit adding it = ${added:-none}"
done
# The two merges that resolved pcc/ledger.jsonl to main's side
for m in bdd757487a 96e50a56d3; do
  echo "merge $(git log -1 --format='%h %cI %s' $m)"
  for id in e9f39d 7defbb 6224b9 853115; do
    a=$(git show $m^1:pcc/ledger.jsonl | grep -c "PCC-20261117-$id"); b=$(git show $m:pcc/ledger.jsonl | grep -c "PCC-20261117-$id")
    [[ $a != 0 || $b != 0 ]] && echo "  PCC-20261117-$id: first parent $a -> merge $b"
  done
  echo "  ledger lines: first parent $(git show $m^1:pcc/ledger.jsonl | wc -l), second parent $(git show $m^2:pcc/ledger.jsonl | wc -l), merge $(git show $m:pcc/ledger.jsonl | wc -l)"
done
# Repository-wide: claim files without a ledger entry at REV
files=$(git ls-tree --name-only "$REV" pcc/claims/ | sed 's#.*/##; s#\.json$##' | sort -u)
registered=$(git show "$REV:pcc/ledger.jsonl" | grep -o '"claim_id": *"PCC-[0-9A-Za-z-]*"' | sed 's/.*"\(PCC-[^"]*\)"/\1/' | sort -u)
total=$(echo "$files" | wc -l)
orphan=$(comm -23 <(echo "$files") <(echo "$registered") | wc -l)
echo "claim files at ${REV:0:10}: $total; without a ledger entry: $orphan; ledger entries: $(git show "$REV:pcc/ledger.jsonl" | wc -l)"
