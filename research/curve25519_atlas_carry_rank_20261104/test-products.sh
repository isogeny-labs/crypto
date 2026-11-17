#!/usr/bin/env bash
set -euo pipefail
study_binary=${1:?native study binary}
z3_binary=${2:?native z3 binary}
fixture_dir=${3:?fresh fixture directory}
codec=${4:-nia}
test "$codec" = nia || test "$codec" = bv
if test "$codec" = bv; then "$study_binary" --product-fixtures "$fixture_dir" bv
else "$study_binary" --product-fixtures "$fixture_dir"; fi
while IFS=$'\t' read -r stem expected; do
  "$z3_binary" -T:10 "$fixture_dir/$stem.smt2" > "$fixture_dir/$stem.out"
  read -r actual < "$fixture_dir/$stem.out"
  test "$actual" = "$expected"
done < "$fixture_dir/cases.tsv"
echo '196 product-encoding fixtures matched expected outcomes (producer observation)'
