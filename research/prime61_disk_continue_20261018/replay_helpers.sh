#!/bin/bash
# Rebuild independent native audits/visuals into a fresh scratch directory.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pinned_target=${1:?usage: replay_helpers.sh PINNED_TARGET_DIR EXTRACTED_RUN_DIR NEW_SCRATCH_DIR}
runs=${2:?}
scratch=${3:?}
mkdir "$scratch"
study=research/prime61_disk_continue_20261018
dependency="$pinned_target/release/deps"
serde_json_rlib=$(rg --files "$dependency" -g 'libserde_json-*.rlib' | head -n 1)
for helper in audit_prefix visual; do
    rustc --edition=2021 -O -L "dependency=$dependency" --extern "serde_json=$serde_json_rlib" "$study/$helper.rs" -o "$scratch/$helper"
done
"$scratch/audit_prefix" research/prime61_disk_walk_20261017/results/pilot/dump-32.stdout "$runs/dump-256.stdout" > "$scratch/immutable-prefix.json"
"$scratch/visual" "$runs" "$scratch/reports"
cp "$study/reports/report.typ" "$scratch/reports/report.typ"
# Render this copied Typst document using your installed Typst renderer.
# No cryptographic/research computation is delegated to the renderer.
