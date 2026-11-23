#!/usr/bin/env bash
# One size of the sibling-points panel: run, audit, band. Usage: run_size.sh <k>
set -e
k=$1; D=research/curve25519_autolab_20261104/sibling-panel
./target/release/ecbench run --wait --spec $D/spec-k$k.json --out $D/sessions/k$k --cpus auto --allow-busy
./target/release/ecbench verify --dir $D/sessions/k$k --replay 8 --out $D/verify-k$k.json
./target/release/ecbench band --dir $D/sessions/k$k --json $D/band-k$k.json --ranks > $D/band-k$k.txt
echo "SIZE-DONE $k"
