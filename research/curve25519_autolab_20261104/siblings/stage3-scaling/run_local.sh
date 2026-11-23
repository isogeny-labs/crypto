#!/usr/bin/env bash
# Stage 3: local L0 run of the frozen scaling panel. Run under the bench record.
set -e
D=research/curve25519_autolab_20261104/siblings/stage3-scaling
for t in a3 gen j0b7; do for b in 34 37 40; do
  n=$t-$b
  [ -f $D/sessions/$n/session.json ] && grep -q '"status": *"complete"' $D/sessions/$n/session.json && continue
  rm -rf $D/sessions/$n
  ./target/release/ecbench run --wait --spec $D/spec-$n.json --out $D/sessions/$n --cpus auto --allow-busy
  ./target/release/ecbench verify --dir $D/sessions/$n --replay 8 --out $D/verify-$n.json
  ./target/release/ecbench band --dir $D/sessions/$n --json $D/band-$n.json --ranks > $D/band-$n.txt
done; done
echo RUN-DONE
