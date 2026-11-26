#!/usr/bin/env bash
# Extended ladder: larger caps on two targets, 900 s per run, 12 GiB address cap.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:?rr5-formulation-compare binary}
D=$R/evidence/ladder-02
mkdir -p $D
for scale in 1024 4096; do
  for target in planted4 query1; do
    for arm in graeffe sector4 sectors coupled deflated; do
      ( ulimit -v 12582912; python3 $R/tools/run.py $D/s$scale-$target-$arm $X --arm $arm --target $target --scale $scale --seconds 900 ) > /dev/null
      echo "s$scale $target $arm exit=$?"
    done
  done
done
echo LADDER-DONE
