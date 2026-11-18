#!/bin/sh
# Focused correctness only; links the real root library and unchanged solvers.
set -eu
binary=$1
out=${2:?fresh output directory required}
test ! -e "$out"
mkdir -p "$out"
"$binary" --list > "$out/tests.list" 2> "$out/list.stderr"
test ! -s "$out/list.stderr"
test "$(rg -c ': test$' "$out/tests.list")" -eq 11
timeout 300 "$binary" --nocapture --test-threads=1 > "$out/tests.stdout" 2> "$out/tests.stderr"
test ! -s "$out/tests.stderr"
rg -q '^test result: ok\. 11 passed; 0 failed;' "$out/tests.stdout"
printf '{"exit":0,"tests_passed":11,"scope":"actual-library scheduling and legacy wide-field integration controls","full_library_test_suite_passed":false,"benchmark_session_executed":false,"runtime_measurements":null}\n' > "$out/exit.json"
