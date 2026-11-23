#!/usr/bin/env bash
# Exercise the actual workflow post-step and Rust's uncaptured panic output.
set -euo pipefail
fixture_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(git -C "$fixture_dir" rev-parse --show-toplevel)
workflow="$repo_root/.github/workflows/rust-all-targets.yml"
fixture_output=${1:?Pass a new empty output directory}
mkdir -p "$fixture_output"
fixture_output=$(cd -- "$fixture_output" && pwd)
test -z "$(ls -A "$fixture_output")"
awk '
  /^      - name:/ {
    selected = ($0 == "      - name: Retain checksums and post-run source state")
    in_run = 0
  }
  selected && /^        run: [|]/ { in_run = 1; next }
  selected && in_run && /^          / { sub(/^          /, ""); print; next }
  selected && in_run { exit }
' "$workflow" > "$fixture_output/post-step.sh"
test -s "$fixture_output/post-step.sh"
bash -n "$fixture_output/post-step.sh"
rg -q -F -- '--locked -- --nocapture' "$workflow"
SOURCE_REVISION=0000000000000000000000000000000000000000
export SOURCE_REVISION

run_post_step() {
  local name=$1 outcome=$2
  (
    cd "$fixture_output/$name"
    VALIDATION_DIR="$fixture_output/$name" LIBRARY_STEP_OUTCOME="$outcome" \
      bash "$fixture_output/post-step.sh"
  )
}

mkdir -p "$fixture_output"/{missing-parent,failed-parent,successful-parent,skipped-test,wrong-source}
run_post_step missing-parent failure
jq -e '.test_step_outcome == "failure" and
  .parent_exit_receipt_present == false and
  .cargo_parent_exit_code == null and .log_writer_exit_code == null' \
  "$fixture_output/missing-parent/completion.json" > /dev/null
printf '{"source_revision":"%s","cargo_parent_exit_code":101,"log_writer_exit_code":0}\n' \
  "$SOURCE_REVISION" > "$fixture_output/failed-parent/parent-exit.json"
run_post_step failed-parent failure
jq -e '.parent_exit_receipt_present == true and
  .cargo_parent_exit_code == 101 and .log_writer_exit_code == 0' \
  "$fixture_output/failed-parent/completion.json" > /dev/null
printf '{"source_revision":"%s","cargo_parent_exit_code":0,"log_writer_exit_code":0}\n' \
  "$SOURCE_REVISION" > "$fixture_output/successful-parent/parent-exit.json"
run_post_step successful-parent success
jq -e '.parent_exit_receipt_present == true and
  .cargo_parent_exit_code == 0 and .log_writer_exit_code == 0 and
  .parent_summary_requires_separate_verification == true' \
  "$fixture_output/successful-parent/completion.json" > /dev/null
run_post_step skipped-test skipped
jq -e '.test_step_outcome == "skipped" and
  .parent_exit_receipt_present == false and .cargo_parent_exit_code == null' \
  "$fixture_output/skipped-test/completion.json" > /dev/null
printf '{"source_revision":"1111111111111111111111111111111111111111","cargo_parent_exit_code":0,"log_writer_exit_code":0}\n' \
  > "$fixture_output/wrong-source/parent-exit.json"
if run_post_step wrong-source success > "$fixture_output/wrong-source/stdout.txt" \
    2> "$fixture_output/wrong-source/stderr.txt"; then
  echo 'wrong-source receipt was incorrectly accepted' >&2
  exit 1
fi
rg -q -F 'parent receipt source does not match' "$fixture_output/wrong-source/stderr.txt"
printf 'completion receipt cases: 5 passed\n'

rustc --edition=2021 --test "$fixture_dir/interrupted_harness.rs" \
  -o "$fixture_output/interrupted-harness"
set +e
timeout --signal=TERM --kill-after=1s 2s "$fixture_output/interrupted-harness" \
  --test-threads=2 --nocapture > "$fixture_output/interrupted-harness.log" 2>&1
fixture_timeout_status=$?
set -e
test "$fixture_timeout_status" -eq 124
rg -q -F 'diagnostic-fixture-panic' "$fixture_output/interrupted-harness.log"
if rg -q '^test result:' "$fixture_output/interrupted-harness.log"; then
  echo 'unexpected complete summary from the interrupted fixture' >&2
  exit 1
fi
printf 'interrupted Rust fixture: panic visible before parent summary; timeout wrapper exit=%s (not Cargo/library status)\n' \
  "$fixture_timeout_status"
rm "$fixture_output/interrupted-harness"
