#!/usr/bin/env bash
# L7 orchestration (thin): native `ecbench band` re-reads of committed sessions,
# then the Rust table builder. No measurement is made; every input is committed.
set -euo pipefail
REPO=/home/user/crypto
LANE=$REPO/research/ecc2k130_open7_20261114/L7-e2e-units
SP=/tmp/claude-0/-home-user/e01afaa4-f4f8-52ad-ba5a-6c307eb9b47e/scratchpad
ECB=${ECB:-$SP/cargo-target/release/ecbench}
TOOL=$SP/L7/target/release/l7tab
mkdir -p "$LANE/raw"
cd "$REPO"

# 1. tool
(cd "$LANE/tool" && CARGO_TARGET_DIR=$SP/L7/target cargo build --release -j1 -q)

# 2. band re-reads (current reading, and the pre-2026-11-08 reading for the control)
band() { # id dir
  local id=$1 d=$2 s; s=$(basename "$d")
  timeout 300 prlimit --as=2000000000 "$ECB" band --dir "$d" --json "$LANE/raw/band-$id-$s.json" > "$LANE/raw/band-$id-$s.md"
  timeout 300 prlimit --as=2000000000 "$ECB" band --dir "$d" --no-price-table-work --json "$LANE/raw/band-$id-$s.pre1108.json" > "$LANE/raw/band-$id-$s.pre1108.md"
}
if [ -x "$ECB" ]; then
  sha256sum "$ECB" > "$LANE/raw/ecbench.sha256"
  for s in K13 K17 K19 K23 K37 K41 K43 K47; do band FC research/ic_frontier_campaign_20261015/phase1/sessions/$s; done
  for s in a0-n13 a1-n17 a0-n19 a0-n23 a1-n29 a0-n31 a0-n37 a0-n41; do
    band FO research/ecbench_frobenius_orbit_vs_rho_20261014/sessions/$s
    band LZ research/ecbench_lazy_orbit_members_20261018/sessions/$s
  done
else
  echo "ecbench not built: band re-reads skipped, frozen readings used" | tee "$LANE/raw/ecbench.MISSING"
fi

# 3. table, fits, figures (twice: determinism check)
"$TOOL" "$REPO" "$LANE"
cp "$LANE/results.json" "$SP/L7/results.run1.json"
"$TOOL" "$REPO" "$LANE"
cmp "$SP/L7/results.run1.json" "$LANE/results.json" && echo "determinism: results.json byte-identical over two runs" | tee "$LANE/raw/determinism.txt"

# 4. hashes
(cd "$LANE/raw" && sha256sum $(ls | grep -v SHA256SUMS) > SHA256SUMS)
