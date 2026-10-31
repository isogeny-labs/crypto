#!/bin/sh
# Run every E4 session (one per spec in specs/), four at a time, L0 (no pinning,
# busy host allowed).  Each session gets its own lock file so they run side by
# side; wall time is not a metric.  `run_e4.sh one SPEC` runs one session.
cd "$(dirname "$0")"
if [ "$1" = one ]; then
  n=$(basename "$2" .json)
  s=$(date +%s)
  ../../../target/release/ecbench run --spec "$2" --out "runs/$n" --cpus none --allow-busy \
    --lock "$TMPDIR/e4-$n.lock" --quiet > "runs/$n.log" 2>&1
  echo "$n exit=$? wall_s=$(( $(date +%s) - s ))" >> runs/times.txt
  exit 0
fi
mkdir -p runs
ls specs/*.json | xargs -n 1 -P 4 sh ./run_e4.sh one
