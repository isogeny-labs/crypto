#!/usr/bin/env bash
# Amendment 1: local L0 run of the frozen panel. Run under the bench record.
set -e
D=research/curve25519_autolab_20261104/siblings/stage1-eks
for k in 28 32; do
  ./target/release/ecbench run --wait --spec $D/spec-k$k-local.json --out $D/sessions/k$k --cpus auto --allow-busy
  ./target/release/ecbench verify --dir $D/sessions/k$k --replay 8 --out $D/verify-k$k.json
  ./target/release/ecbench band --dir $D/sessions/k$k --json $D/band-k$k.json --ranks > $D/band-k$k.txt
done
echo RUN-DONE
