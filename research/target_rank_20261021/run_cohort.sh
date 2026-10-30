#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."
base=research/target_rank_20261021

python3 "$base/make_specs.py"
cargo build --release --offline --locked --bin ecbench
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench run \
    --spec "$base/specs/order-genus-$bits.json" \
    --out "$base/sessions/order-genus-$bits" \
    --cpus none --allow-busy --settle 0 \
    --lock "/tmp/crypto-target-rank-$bits.lock" --quiet
  mkdir -p "$base/replay"
  target/release/ecbench verify \
    --dir "$base/sessions/order-genus-$bits" --replay-all \
    > "$base/replay/order-genus-$bits.json"
done
python3 "$base/analyze.py" > "$base/summary.json"
