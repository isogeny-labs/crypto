#!/usr/bin/env bash
set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/integration-publication"
proof="$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/native-allocation-profile-20261021"
mkdir "$pub" "$pub/integration"
raw="$pub/integration"
cp -a "$work/integrated-validation" "$raw/"
cp "$work/validate-integrated.sh" "$work/seal-integration.sh" "$work/publish-integration.sh" "$work/integration-source-freeze.txt" "$raw/"
cp "$repo/research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261021T1617Z/CLARIFICATION.md" "$proof/INTEGRATION.md" "$repo/benchmarks/20261021-n83-storage-profile/README.md" "$raw/"
git -C "$repo" bundle create "$raw/SOURCE-INTEGRATED.bundle" HEAD ^05ec85b5287ebf137f1b1e16c8e91705d48cf77f
git -C "$repo" bundle verify "$raw/SOURCE-INTEGRATED.bundle" > "$raw/source-verify.txt" 2>&1
git -C "$repo" diff --exit-code 7e8ae96c4af60ac63311b4c3c29efcb83321861e HEAD -- src/cryptanalysis/sat.rs tools/n83-sweep-runner > "$raw/unchanged-native-source.txt"
git -C "$repo" diff --exit-code 05ec85b5287ebf137f1b1e16c8e91705d48cf77f HEAD -- benchmarks/README.md > "$raw/preserved-benchmark-table.txt"
printf '%s\n' 'Post-profile concurrent-main integration verification, no new SAT query/worker/cap. Retains full release checks at combined source d5aac8e9, exact source bundle with remote05ec85b5 prerequisite, documentary admission clarification. Original raw03754a1f and sidecarfdf07b85 remain immutable and freshly verified separately. No fake scalar/rho validation certificate. This supplement cannot include its own upload receipts; final external proof supplies them. Normalized sortedGNUtar,UTC2026-10-21,xz6T1.' > "$raw/README-ARCHIVE.txt"
(cd "$raw"; find . -type f ! -path ./SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS; mv ../SHA256SUMS SHA256SUMS)
(cd "$pub"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - integration | xz -6 -T1 > integration-verification.tar.xz)
sha256sum "$pub/integration-verification.tar.xz" "$raw/SHA256SUMS" > "$pub/hashes.txt"
stat -c %s "$pub/integration-verification.tar.xz" > "$pub/bytes.txt"
