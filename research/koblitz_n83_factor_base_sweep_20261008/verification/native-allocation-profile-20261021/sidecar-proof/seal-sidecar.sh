#!/usr/bin/env bash
set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/publication"
proof="$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/native-allocation-profile-20261021"
mkdir "$pub/verification" "$pub/verification/proof"
raw="$pub/verification"
find "$proof" -mindepth 1 -maxdepth 1 ! -name exec -exec cp -a -t "$raw/proof" {} +
cp "$work/seal-sidecar.sh" "$work/publish-sidecar.sh" "$raw/"
git -C "$repo" bundle create "$raw/SOURCE-PUBLICATION.bundle" HEAD ^37679b3586992e52e44cd1e994f589aa2a9b204b
git -C "$repo" bundle verify "$raw/SOURCE-PUBLICATION.bundle" > "$raw/source-bundle-verify.txt" 2>&1
git -C "$repo" rev-parse HEAD HEAD^{tree} > "$raw/source.txt"
printf '%s\n' 'Verification sidecar for raw SHA03754a1f073698cc4f786f03a2d3acd6eda253712370f266d11553b85982ea98. Actual raw PUT/HEAD/freshGET/payload/native positive+negative replay/repack receipts; final tests, exact experimental source histories. SOURCE-PUBLICATION.bundle requires published37679b35; includes source/report and raw publication proof commits. Main raw has full ELFs/inputs/traces/failures/report. This sidecar cannot include its own upload receipt; those external receipts are in final PUBLICATION.md. Sorted normalized GNUtar,UTC2026-10-21,xz6T1.' > "$raw/README.txt"
(cd "$raw"; find . -type f ! -path ./SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SIDECAR-SHA256SUMS; mv ../SIDECAR-SHA256SUMS SHA256SUMS)
(cd "$pub"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - verification | xz -6 -T1 > verification.tar.xz)
sha256sum "$pub/verification.tar.xz" "$raw/SHA256SUMS" > "$pub/sidecar-hashes.txt"
stat -c %s "$pub/verification.tar.xz" > "$pub/sidecar-bytes.txt"
