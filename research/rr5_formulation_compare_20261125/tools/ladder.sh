#!/usr/bin/env bash
# Resource ladder on identical public targets. Each run is a fresh process under a
# 12 GiB address-space cap; resource caps scale together (pairs, symbolic monomials,
# matrix entries = frozen #4974 values times SCALE); deadline 300 s per run.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:?rr5-formulation-compare binary}
D=$R/evidence/${PANEL:-ladder-01}
mkdir -p $D
for scale in ${SCALES:-1 4 16 64 256}; do
  for target in planted4 planted2 query1 query2 h0; do
    arms="coupled sectors sector4 graeffe deflated"
    [ "$target" = planted2 ] && arms="coupled sectors sector2"
    for arm in $arms; do
      ( ulimit -v 12582912; python3 $R/tools/run.py $D/s$scale-$target-$arm $X --arm $arm --target $target --scale $scale --seconds 300 ) > /dev/null
      echo "s$scale $target $arm exit=$?"
    done
  done
done
echo LADDER-DONE
