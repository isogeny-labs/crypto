#!/bin/sh
# Thin orchestration only. usage: ./run_sessions.sh m0 m5
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ECB=${ECBENCH:-$HERE/../../target/release/ecbench}
for v in "$@"; do
  out="$HERE/sessions/$v"
  [ -e "$out" ] && { echo "skip $out (exists)"; continue; }
  echo "== $v  $(date -u +%Y%m%dT%H%MZ)"
  "$ECB" run --wait --allow-busy --cpus none --spec "$HERE/specs/$v/layers.json" --out "$out" > "$HERE/sessions/$v.run.log" 2>&1
  echo "   run exit $?"
  "$ECB" verify --dir "$out" --replay 8 > "$HERE/sessions/$v.verify.log" 2>&1
  echo "   verify exit $?"
done
