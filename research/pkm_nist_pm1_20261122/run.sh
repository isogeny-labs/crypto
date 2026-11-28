#!/usr/bin/env bash
set -euo pipefail

bin=${1:?pass the built pkm_tower_pilot example binary}
out_dir=$(cd "$(dirname "$0")" && pwd)/results
mkdir -p "$out_dir"

run() {
  local label=$1 prime=$2 kind=$3 depth=$4 curve=$5
  local extra=()
  if [[ $curve == nist ]]; then extra=(--curve nist); fi
  "$bin" --p "$prime" --engine generic --builder wide \
    --kinds "$kind" --m 2 --controls tower --t-min "$depth" --max-t "$depth" \
    --planted 1 --random 1 --budget 15 --max-nnz 1000000 --verify \
    --ladder-t none --ladder-g none "${extra[@]}" \
    --out "$out_dir/$label.jsonl"
}

# Each prime's deep 2-primary side and its depth-1 opposite side.
run p192_plus P-192 dickson-torus 2 nist
run p192_minus P-192 kummer 1 nist
run p224_minus P-224 kummer 2 nist
run p224_plus P-224 dickson-torus 1 nist
run p256_plus P-256 dickson-torus 2 nist
run p256_minus P-256 kummer 1 nist
run p384_plus P-384 dickson-torus 2 nist
run p384_minus P-384 kummer 1 nist
run p521_plus P-521 dickson-torus 2 nist
run p521_minus P-521 kummer 1 nist

# Paired 59-bit controls with v2(p-1)=16 and v2(p+1)=18 respectively.
run s59_minus 288230376154267649 kummer 2 random
run s59_minus_opposite 288230376154267649 dickson-torus 1 random
run s59_plus 288230376151973887 dickson-torus 2 random
run s59_plus_opposite 288230376151973887 kummer 1 random
run s59_registry 288230445959117953 kummer 2 random
