#!/bin/sh
# Thin orchestration only: build specs with `crypto volcano ecbench-spec`, run each with ecbench, audit with 8 replays.
# usage: ./run_sessions.sh n19 n23 ...
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ECB=${ECBENCH:-$HERE/../../target/release/ecbench}
for n in "$@"; do
  for part in crater neighbors; do
    out="$HERE/sessions/$n-$part"
    [ -e "$out" ] && { echo "skip $out (exists)"; continue; }
    echo "== $n $part  $(date -u +%Y%m%dT%H%MZ)"
    "$ECB" run --wait --allow-busy --cpus none --spec "$HERE/specs/$n/$part.json" --out "$out" > "$HERE/sessions/$n-$part.run.log" 2>&1
    echo "   run exit $?"
    "$ECB" verify --dir "$out" --replay 8 > "$HERE/sessions/$n-$part.verify.log" 2>&1
    echo "   verify exit $?"
  done
done
