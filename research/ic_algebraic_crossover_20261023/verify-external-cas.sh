#!/usr/bin/env bash
# Rebuild the equations and all external Boolean zero sets; no CAS licence needed.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study="$root/research/ic_algebraic_crossover_20261023"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
(
  cd "$study/evidence"
  sha256sum -c runs-external-cas-20261025-01.tar.xz.sha256
)
tar -xJf "$study/evidence/runs-external-cas-20261025-01.tar.xz" -C "$scratch"
cd "$root"
cargo build --release --example ic_cas_correctness
target/release/examples/ic_cas_correctness verify "$scratch/runs/external-cas-20261025-01" > "$scratch/replay.json"
cmp "$scratch/replay.json" "$study/pipeline-receipts/external-cas-strengthened-20261025/replay.json"
printf '%s\n' 'PASS: archived external inputs, rebuilt equations and all 44 zero sets reproduce exactly.'
