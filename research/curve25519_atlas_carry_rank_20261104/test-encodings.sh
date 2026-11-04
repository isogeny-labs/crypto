#!/usr/bin/env bash
# Thin development-test orchestration; arithmetic, encodings and replay are native.
set -euo pipefail
study_binary=${1:?native study binary}
z3_binary=${2:?native z3 binary}
fixture_dir=${3:?fresh fixture directory}
scope=${4:-full}
test "$scope" = full || test "$scope" = pinned-only
"$study_binary" --encoding-fixtures "$fixture_dir"
failed=0
printf 'case\texpected\tactual\tsolver_exit\treplay\n' > "$fixture_dir/summary.tsv"
while IFS=$'\t' read -r stem expected arm x y inf; do
  if test "$scope" = pinned-only && [[ "$stem" != *-pinned ]]; then continue; fi
  solver_exit=0
  "$z3_binary" -T:30 "$fixture_dir/$stem.smt2" > "$fixture_dir/$stem.out" 2>&1 || solver_exit=$?
  actual=empty
  read -r actual < "$fixture_dir/$stem.out" || true
  replay=not_applicable
  if test "$actual" != "$expected"; then failed=1; fi
  if test "$actual" = sat; then
    replay=pass
    "$study_binary" --replay-fixture "$arm" "$x" "$y" "$inf" "$fixture_dir/$stem.out" development-only || { replay=fail; failed=1; }
    if test "$solver_exit" != 0; then failed=1; fi
  fi
  # A nonzero get-model-after-UNSAT exit is recorded, not hidden; the exact
  # native expected status remains the functional oracle for this fixture.
  printf '%s\t%s\t%s\t%s\t%s\n' "$stem" "$expected" "$actual" "$solver_exit" "$replay" >> "$fixture_dir/summary.tsv"
done < "$fixture_dir/cases.tsv"
echo "encoding fixtures finished in scope=$scope; failed=$failed; see summary.tsv"
exit "$failed"
