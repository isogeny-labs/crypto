#!/bin/sh
# Focused tests link the actual ecbench library; no benchmark/timing session.
set -eu
binary=$1
out=${2:?fresh output directory required}
test ! -e "$out"
mkdir -p "$out"
"$binary" prime_mont4 --list > "$out/tests.list" 2> "$out/list.stderr"
test ! -s "$out/list.stderr"
test "$(rg -c '^prime_mont4::.*: test$' "$out/tests.list")" -eq 5
timeout 300 "$binary" prime_mont4 --nocapture --test-threads=1 > "$out/tests.stdout" 2> "$out/tests.stderr"
test ! -s "$out/tests.stderr"
rg -q '^test result: ok\. 5 passed; 0 failed;' "$out/tests.stdout"
printf '{"exit":0,"tests_passed":5,"scope":"focused public integration correctness tests","full_library_test_suite_passed":false,"benchmark_session_executed":false,"runtime_measurements":null}\n' > "$out/exit.json"
