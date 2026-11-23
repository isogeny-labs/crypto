#!/bin/sh
# Bounded public correctness tests only; no benchmark session or timing report.
set -eu
binary=$1
out=${2:?fresh output directory required}
test ! -e "$out"
mkdir -p "$out"
timeout 300 "$binary" prime_mont4 --nocapture --test-threads=1 > "$out/tests.stdout" 2> "$out/tests.stderr"
test ! -s "$out/tests.stderr"
printf '{"exit":0,"scope":"bounded public correctness tests","benchmark_session_executed":false,"runtime_measurements":null}\n' > "$out/exit.json"
