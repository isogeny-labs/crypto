#!/usr/bin/env bash
# Stage 2: local L0 run of the frozen prime-field panel. Run under the bench record.
set -e
D=research/curve25519_autolab_20261104/siblings/stage2-primefield
for t in a3 gen j0b7; do
  ./target/release/ecbench run --wait --spec $D/spec-$t.json --out $D/sessions/$t --cpus auto --allow-busy
  ./target/release/ecbench verify --dir $D/sessions/$t --replay 8 --out $D/verify-$t.json
  ./target/release/ecbench band --dir $D/sessions/$t --json $D/band-$t.json --ranks > $D/band-$t.txt
done
echo RUN-DONE
