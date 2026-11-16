#!/bin/bash
# Package a closed immutable snapshot; never copy the active writer database.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
expanded=${1:?usage: pack_checkpoint.sh EXPANDED NEW_SCRATCH_DIRECTORY}
scratch=${2:?}
study=research/prime61_disk_continue_20261018
name=prime61-drive-checkpoint-$expanded-20261018
mkdir -p "$scratch"
mkdir "$scratch/$name"
cp "$study/results/runs/checkpoint-$expanded.db.gz" "$scratch/$name/checkpoint.db.gz"
cp "$study/results/runs/verify-$expanded.stdout" "$scratch/$name/verification.jsonl"
cp "$study/results/runs/dump-$expanded.stdout" "$scratch/$name/records.jsonl"
cp "$study/spec.json" "$study/PROTOCOL.json" "$study/AMENDMENT.json" "$scratch/$name/"
cp "$study/results/pinned-build-inputs.sha256" "$scratch/$name/build-inputs.sha256"
printf '%s\n' b68341cd72b51acf9c5ce6c4f755351cc83053db > "$scratch/$name/source-revision.txt"
cat > "$scratch/$name/RESUME.txt" <<'EOF'
Verify SHA256SUMS before restoring; gunzip -k checkpoint.db.gz.
Build exact source b68341cd72b51acf9c5ce6c4f755351cc83053db on Linux with Rust 1.99.1:
  cargo build --release --no-default-features --example isogeny_disk_walk
Verify all maps/records and replay the first 16 curves:
  target/release/examples/isogeny_disk_walk verify spec.json checkpoint.db 16
Status and further cumulative targets use this same database:
  target/release/examples/isogeny_disk_walk status spec.json checkpoint.db
  target/release/examples/isogeny_disk_walk advance spec.json checkpoint.db NEXT_EXPANDED_LIMIT
  target/release/examples/isogeny_disk_walk measure spec.json checkpoint.db NEXT_MEASURED_LIMIT
Freeze the next resource envelope before extending the recorded experiment.
A selected-prime component is not a certified full horizontal class.
EOF
(
  cd "$scratch/$name"
  sha256sum checkpoint.db.gz verification.jsonl records.jsonl spec.json PROTOCOL.json AMENDMENT.json build-inputs.sha256 source-revision.txt RESUME.txt > SHA256SUMS
)
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$scratch" -cf - "$name" | gzip -n -6 > "$scratch/$name.tar.gz"
sha256sum "$scratch/$name.tar.gz"
stat -c '%s bytes' "$scratch/$name.tar.gz"
