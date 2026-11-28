#!/usr/bin/env bash
# Recompute the complete producer accounting; HKY replay is a separate program.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
cargo build --release --example ic_guess_checkpoint --example ic_guess_completed_audit --example ic_cofactor_projection_audit
bin_root=$(realpath "${CARGO_TARGET_DIR:-target}/release/examples")
scratch=$(mktemp -d /tmp/ic-completed-guess-replay.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
archive=evidence/runs-balanced-guess-complete-all124-20261025.tar.xz
(cd "$study/evidence"; sha256sum -c "${archive#evidence/}.sha256")
mkdir -p "$scratch/$study/inputs"
tar -xJf "$study/$archive" -C "$scratch/$study"
cp "$study/inputs/guess-ell8.jsonl" "$scratch/$study/inputs/"
"$bin_root/ic_guess_checkpoint" "$scratch/$study" "$scratch/$study/runs/guess-cap200k-20261025-01" > "$scratch/receipt.json"
cmp "$scratch/receipt.json" "$study/pipeline-receipts/guess-completion-20261025-01/receipt.json"
cmp "$scratch/receipt.json" "$scratch/$study/pipeline-receipts/guess-completion-20261025-01/receipt.json"
"$bin_root/ic_guess_completed_audit" summary "$scratch/$study" > "$scratch/summary.json"
cmp "$scratch/summary.json" "$study/pipeline-receipts/guess-completion-20261025-01/summary.json"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
(ulimit -v 2097152; timeout -k 30 300 "$bin_root/ic_cofactor_projection_audit" "$study") > "$scratch/cofactor.json"
cmp "$scratch/cofactor.json" "$study/pipeline-receipts/cofactor-projection-20261025/receipt.json"
printf '%s\n' 'PASS: complete 124-attempt archive hash, raw producer accounting, registered-prefix minima/costs and finite cofactor-projection audit reproduce byte for byte. Independent HKY branch replay and independent word recount are not performed by this verifier.'
