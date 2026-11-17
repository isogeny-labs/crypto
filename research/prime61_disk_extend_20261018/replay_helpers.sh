#!/bin/bash
# Replay native diagnostics, audits and figures into new paths; preserve evidence.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pinned_target=${1:?usage: replay_helpers.sh PINNED_TARGET_DIR EXTRACTED_RUN_DIR NEW_SCRATCH_DIR}
runs=${2:?}
scratch=${3:?}
study=research/prime61_disk_extend_20261018
prior=research/prime61_disk_continue_20261018
mkdir "$scratch"
deps=$pinned_target/release/deps
json_rlib=$(rg --files "$deps" -g 'libserde_json-*.rlib' | head -n 1)
crypto_rlib=$(rg --files "$deps" -g 'libcrypto_lib-*.rlib' | head -n 1)
for source in genus_span issue_identity; do
    rustc --edition=2021 -O -L "dependency=$deps" --extern "crypto_lib=$crypto_rlib" --extern "serde_json=$json_rlib" "$study/$source.rs" -o "$scratch/$source"
done
rustc --edition=2021 --test -O -L "dependency=$deps" --extern "crypto_lib=$crypto_rlib" --extern "serde_json=$json_rlib" "$study/genus_span.rs" -o "$scratch/genus-tests"
"$scratch/genus-tests" > "$scratch/genus-tests.log"
"$scratch/genus_span" > "$scratch/genus-span.json"
cmp "$scratch/genus-span.json" "$study/results/genus-span.json"
"$scratch/issue_identity" "$scratch/genus-span.json" "$study/proof/GenusSpan.lean" "$scratch/discriminants.identity.json" > "$scratch/identity-check.json"
cmp "$scratch/discriminants.identity.json" "$study/results/discriminants.identity.json"
rustc --edition=2021 -O -L "dependency=$deps" --extern "serde_json=$json_rlib" "$prior/audit_prefix.rs" -o "$scratch/audit-prefix"
"$scratch/audit-prefix" "$prior/results/runs/dump-256.stdout" "$runs/dump-1024.stdout" > "$scratch/immutable-prefix.json"
rustc --edition=2021 -O -L "dependency=$deps" --extern "serde_json=$json_rlib" "$study/visual.rs" -o "$scratch/visual"
"$scratch/visual" "$runs" "$scratch/reports" "$scratch/genus-span.json"
rustc --edition=2021 -O -L "dependency=$deps" --extern "serde_json=$json_rlib" "$study/component_visual.rs" -o "$scratch/component-visual"
"$scratch/component-visual" "$study/results/class-relations.jsonl" "$scratch/reports/component.svg"
cp "$study/reports/report.typ" "$scratch/reports/report.typ"
# All Typst inputs are in reports/. Render with an installed Typst binding.
bash tools/lean/proof-gate.sh --out "$scratch/proof-gate" "$study/proof" > "$scratch/proof-gate.log" 2>&1
