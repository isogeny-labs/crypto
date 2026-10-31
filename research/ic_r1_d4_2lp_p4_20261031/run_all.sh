#!/bin/bash
# Runs the per-p slices of SPEC-r1.json in ascending p; logs per session.
cd "$(dirname "$0")"
E=${ECBENCH:?}
for p in 101 181 269 397 521 769 1033; do
  [ -e sessions/p$p ] && continue
  /usr/bin/time -p $E run --wait --spec specs/SPEC-r1-p$p.json --out sessions/p$p > sessions/p$p.run.log 2>&1
  echo "p=$p exit=$?" >> sessions/progress.log
done
