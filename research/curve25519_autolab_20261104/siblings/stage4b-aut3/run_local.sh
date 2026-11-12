#!/usr/bin/env bash
# Stage 4b: local L0 run of the frozen panel, then the native analysis (ecbench only).
set -e
D=research/curve25519_autolab_20261104/siblings/stage4b-aut3
for b in 34 37 40; do
  n=j0b7-$b
  [ -f $D/sessions/$n/session.json ] && grep -q '"status": *"complete"' $D/sessions/$n/session.json && continue
  rm -rf $D/sessions/$n
  ./target/release/ecbench run --wait --spec $D/spec-$n.json --out $D/sessions/$n --cpus auto --allow-busy
  ./target/release/ecbench verify --dir $D/sessions/$n --replay 8 --out $D/verify-$n.json
  ./target/release/ecbench band --dir $D/sessions/$n --json $D/band-$n.json --ranks > $D/band-$n.txt
done
echo RUN-DONE
