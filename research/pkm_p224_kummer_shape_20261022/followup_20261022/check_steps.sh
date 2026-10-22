#!/bin/bash
# E3 of PROTOCOL.md 12.3: the step trace of a new run must equal an earlier trace line for line
# once the fields that are not counts are removed: each step's wall time breakdown, the resident
# memory, address space and live heap, and the non-zero count the fix adds.
#   check_steps.sh NEW.log OLD.log
set -eu
norm() {
  grep -E '^tower (step|stop in step)' "$1" \
    | sed -E 's/, [0-9.]+ ms \(rows [^)]*\)//; s/ \([0-9]+ non-zero\)//; s/, memory .*$//; s/\(oversize: [^)]*\)/(oversize)/'
}
if diff <(norm "$1") <(norm "$2") > /dev/null; then
  echo "TRACES EQUAL ($(norm "$1" | wc -l) lines)"
else
  echo "TRACES DIFFER"; diff <(norm "$1") <(norm "$2") | head -20; exit 1
fi
# The same check with the repository's own comparator, when PKM_TOWER_CHECK names the
# `pkm_tower_check` binary: the new log with its non-zero counts removed (the one field the
# fix adds), step for step.
if [ -n "${PKM_TOWER_CHECK:-}" ]; then
  tmp=$(mktemp)
  sed -E 's/ \([0-9]+ non-zero\)//' "$1" > "$tmp"
  "$PKM_TOWER_CHECK" compare-trace "$2" "$tmp" --steps 25 | tail -1
  rm -f "$tmp"
fi
