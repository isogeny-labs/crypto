#!/usr/bin/env bash
# Exact local development cells; no benchmark or network submission.
set -euo pipefail
[[ $# == 2 && -x "$1" && ! -e "$2" ]] || { echo 'Usage: run_cocoa_cells.sh COCOA_LAUNCHER FRESH_DIR' >&2; exit 2; }
cocoa=$1
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$2"
result_dir=$(cd "$2" && pwd)
for cell in depth2-dense depth2-tower depth4-dense depth4-tower; do
  input="$study_dir/evidence/cocoa-inputs/$cell.cocoa5"
  cp "$input" "$result_dir/$cell.cocoa5"
  sha256sum "$input" > "$result_dir/$cell-input.sha256"
  set +e
  (ulimit -v 2097152; timeout --kill-after=2 45 "$cocoa" --no-preamble --no-prompt "$input") \
    > "$result_dir/$cell.stdout" 2> "$result_dir/$cell.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$result_dir/$cell.exit"
  printf '%s exit=%s\n' "$cell" "$status"
  if [[ $status == 0 ]] && grep -qx "END_COMPLETE_REGULARITY $cell" "$result_dir/$cell.stdout" && \
    ! grep -qi 'ERROR' "$result_dir/$cell.stdout" "$result_dir/$cell.stderr"; then
    printf 'producer-output-complete\n' > "$result_dir/$cell.admission"
  else
    printf 'not-admitted\n' > "$result_dir/$cell.admission"
    # Algorithm/field/syntax errors stop the qualification; timeouts stay missing.
    if grep -qi 'ERROR' "$result_dir/$cell.stdout" "$result_dir/$cell.stderr"; then
      printf 'Stopped after engine error; remaining cells unrun.\n'
      break
    fi
  fi
done
