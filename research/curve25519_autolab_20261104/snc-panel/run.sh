#!/usr/bin/env bash
# SNC-1/SNC-2 shared panel. Run under a benchrec record; see PROTOCOL.md.
set -e
D=research/curve25519_autolab_20261104/snc-panel
for k in 20 24 28 32; do
  ./target/release/ecbench run --wait --spec $D/spec-k$k.json --out $D/sessions/k$k --cpus auto --allow-busy
  ./target/release/ecbench verify --dir $D/sessions/k$k --replay 8 --out $D/verify-k$k.json
  ./target/release/ecbench band --dir $D/sessions/k$k --json $D/band-k$k.json --ranks > $D/band-k$k.txt
done
echo RUN-DONE
