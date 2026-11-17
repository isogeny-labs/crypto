#!/bin/bash
set -eu
repo=/home/ec2-user/crypto-n83-resume
study=$repo/research/koblitz_n83_factor_base_sweep_20261008
boundary=/home/ec2-user/scratch/n83-sat-boundary-20261019
export_round=/home/ec2-user/scratch/n83-selector-export-20261019
dst=$study/verification/native-selector-export-20261019
stage=$export_round/archive-stage
mkdir -p "$dst/records/v4" "$dst/records/v5" "$dst/transport/v4" "$dst/transport/v5" "$dst/validation"
for run in v4/pinned v4/free v5/free; do
    mkdir -p "$dst/records/$run"
    for file in config.json custody.json spec.json outer.json inspect-live.json inspect-final.json audit.json audit-failure.json stdout.log stderr.log; do
        src=$study/verification/native-phase-20261019-v2/exec/$run/$file
        if test -f "$src"; then cp -a "$src" "$dst/records/$run/$file"; fi
    done
done
cp -a "$boundary/cards" "$boundary/specs" "$dst/transport/v4/"
cp -a "$export_round/cards" "$export_round/specs" "$dst/transport/v5/"
cp -a "$boundary/pinned-relay.txt" "$boundary/free-relay.txt" "$boundary/pinned.assessment.json" "$boundary/pinned.transport-corrected.assessment.json" "$boundary/free.assessment.json" "$dst/transport/v4/"
cp -a "$export_round/free-relay.txt" "$export_round/free.assessment.json" "$dst/transport/v5/"
cp -a "$boundary/free-trace-audit.json" "$dst/records/v4/free/trace-only-audit.json"
cp -a "$boundary/host-cpu.json" "$boundary/host-kernel.txt" "$boundary/image-id.json" "$boundary/rustc-version.txt" "$boundary/binaries.sha256" "$boundary/transport-fix-binary.sha256" "$dst/"
cp -a "$boundary/validation" "$dst/validation/boundary"
cp -a "$export_round/validation" "$dst/validation/export"
cp -a "$export_round/forgery.stdout" "$export_round/forgery.stderr" "$export_round/forgery.exit" "$export_round/hash_file.rs" "$dst/"
test ! -e "$stage"
mkdir -p "$stage/runs/exec" "$stage/runs/binaries" "$stage/runs/build" "$stage/runs/transport"
cp -a "$study/verification/native-phase-20261019-v2/exec/v4" "$study/verification/native-phase-20261019-v2/exec/v5" "$stage/runs/exec/"
cp -a "$dst/transport/v4" "$dst/transport/v5" "$stage/runs/transport/"
cp -a "$dst/validation" "$stage/runs/"
cp -a "$export_round/forged-selector-phase" "$stage/runs/"
cp -a "$export_round/forgery.stdout" "$export_round/forgery.stderr" "$export_round/forgery.exit" "$export_round/hash_file.rs" "$stage/runs/"
cp -a "$boundary/runner-target/release/n83-sweep-runner" "$stage/runs/binaries/v4-runner"
cp -a "$boundary/transport-fix-target/release/n83-sweep-runner" "$stage/runs/binaries/v4-transport-checker"
cp -a "$export_round/runner-target/release/n83-sweep-runner" "$stage/runs/binaries/v5-runner"
for worker in koblitz_n83_fixed_witness_control koblitz_n83_selector_witness_control koblitz_n83_free_selector_export_control; do
    cp -a "$boundary/worker-target/release/examples/$worker" "$stage/runs/binaries/$worker"
done
cp -a "$boundary/transport-fix-src" "$stage/runs/build/"
cp -a "$boundary/rustc-version.txt" "$boundary/source-commit.txt" "$boundary/host-cpu.json" "$boundary/host-kernel.txt" "$boundary/image-id.json" "$stage/runs/build/"
cp -a "$repo/Cargo.toml" "$repo/Cargo.lock" "$stage/runs/build/"
cp -a "$study/verification/native-sat-boundary-20261019/PROTOCOL.md" "$stage/runs/build/boundary-protocol.md"
cp -a "$dst/PROTOCOL.md" "$stage/runs/build/export-protocol.md"
cp -a "$export_round/collect-evidence.sh" "$stage/runs/build/"
cp -a /home/ec2-user/scratch/n83-native-phase-20261019/inputs/witness "$stage/runs/inputs"
cp -a /home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/fixtures "$stage/runs/fixtures"
git -C "$repo" diff --binary 27d17bd17b9fe9c6b4ce04e0c9206706b2b5c804 cbe8e44da5ac0a792922d746368882a3b8f40170 -- src examples tools/n83-sweep-runner Cargo.toml Cargo.lock > "$stage/runs/build/source-delta-from-27d17bd1.patch"
git -C "$repo" ls-tree -r cbe8e44da5ac0a792922d746368882a3b8f40170 -- src examples tools/n83-sweep-runner Cargo.toml Cargo.lock > "$stage/runs/build/source-tree.txt"
cd "$stage/runs"
rg --files --hidden -g '!SHA256SUMS' | LC_ALL=C sort | while IFS= read -r file; do sha256sum "$file"; done > SHA256SUMS
sha256sum -c SHA256SUMS > "$export_round/raw-file-checks.log"
cd "$stage"
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > "$dst/runs.tar.xz"
cd "$dst"
sha256sum runs.tar.xz > runs.tar.xz.sha256
