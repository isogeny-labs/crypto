#!/bin/bash
# Immutable closed snapshot plus exact source/spec and finite proof evidence.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
batch=${1:?usage: pack_checkpoint.sh BATCH NEW_SCRATCH_DIRECTORY}
scratch=${2:?}
study=research/prime61_disk_extend_20261018
name=prime61-extended-checkpoint-$batch-20261018
mkdir -p "$scratch"
mkdir "$scratch/$name"
cp "$study/results/runs/checkpoint-$batch.db.gz" "$scratch/$name/checkpoint.db.gz"
cp "$study/results/runs/verify-$batch.stdout" "$scratch/$name/verification.jsonl"
cp "$study/results/runs/dump-$batch.stdout" "$scratch/$name/records.jsonl"
cp "$study/spec.json" "$study/PROTOCOL.json" "$study/CLASS-COVERAGE.md" "$study/proof-map.json" "$scratch/$name/"
cp "$study/results/genus-span.json" "$study/results/discriminants.identity.json" "$study/results/identity-check.json" "$scratch/$name/"
cp "$study/proof/GenusSpan.lean" "$study/proof/lean-toolchain" "$study/results/proof-gate.log" "$scratch/$name/"
cp "$study/RELATIONS_PROTOCOL.json" "$study/COMPONENT-CERTIFICATE.md" "$study/component-proof-map.json" "$study/class_relations.gp" "$study/verify_relations.gp" "$scratch/$name/"
cp "$study/proof/ComponentCoordinates.lean" "$study/proof/lakefile.lean" "$scratch/$name/"
cp "$study/results/class-relations.jsonl" "$study/results/class-relations-witness-check.json" "$study/results/component-proof-gate.log" "$study/results/pari-environment.json" "$scratch/$name/"
cp "$study/results/pinned-build-inputs.sha256" "$scratch/$name/build-inputs.sha256"
printf '%s\n' 'Native worker: b68341cd72b51acf9c5ce6c4f755351cc83053db' 'Protocol: 90d82f8; native genus diagnostic: 7d9caf6; finite Lean proof source: 452559d' 'Relation protocol: 81d504f; producer: b36b0dc; witness checker and coordinate proof: 6c33852' > "$scratch/$name/source-revision.txt"
cat > "$scratch/$name/RESUME.txt" <<'EOF'
Check SHA256SUMS before restoring; gunzip -k checkpoint.db.gz.
Build exact worker revision b68341cd72b51acf9c5ce6c4f755351cc83053db, Rust 1.99.1, Linux/system libsqlite3.so.0:
  cargo build --release --no-default-features --example isogeny_disk_walk
  target/release/examples/isogeny_disk_walk verify spec.json checkpoint.db 16
  target/release/examples/isogeny_disk_walk status spec.json checkpoint.db
Further advance/measure limits are cumulative and must use a new frozen resource envelope.
The selected component is not exhausted. Full class number/generation remain uncertified.
The genus diagnostic has partial Lean formalization; see CLASS-COVERAGE.md and proof-map.json.
The additive explicit relation certificate fixes selected subgroup size 545296208;
see COMPONENT-CERTIFICATE.md and component-proof-map.json for unformalized bridges.
This does not certify that the selected component is the whole horizontal class.
The full proof project, raw commands and reports are in PR 2144's prime61_disk_extend_20261018 study.
EOF
(
    cd "$scratch/$name"
    sha256sum checkpoint.db.gz verification.jsonl records.jsonl spec.json PROTOCOL.json CLASS-COVERAGE.md proof-map.json genus-span.json discriminants.identity.json identity-check.json GenusSpan.lean lean-toolchain proof-gate.log build-inputs.sha256 source-revision.txt RESUME.txt RELATIONS_PROTOCOL.json COMPONENT-CERTIFICATE.md component-proof-map.json class_relations.gp verify_relations.gp ComponentCoordinates.lean lakefile.lean class-relations.jsonl class-relations-witness-check.json component-proof-gate.log pari-environment.json > SHA256SUMS
)
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$scratch" -cf - "$name" | gzip -n -6 > "$scratch/$name.tar.gz"
sha256sum "$scratch/$name.tar.gz"
stat -c '%s bytes' "$scratch/$name.tar.gz"
