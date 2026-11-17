#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-100k-20261019
old=/home/ec2-user/scratch/n83-xor-opt-20261019
root=/home/ec2-user/scratch/n83-docker-sweep-20261018
study=research/koblitz_n83_factor_base_sweep_20261008
proof="$study/verification/native-natural-100k-20261019"
bundle="$work/publication/raw-bundle"
mkdir -p "$bundle"/{runs,plans,bin,inputs,source,validation,reports,protocols}
cp "$work/collect-evidence.sh" "$bundle/"
cp -a "$proof/exec/." "$bundle/runs/"
cp -a "$work/cards" "$work/specs" "$work/specs-v6c" "$bundle/plans/"
cp "$work"/manifest*.json "$work"/host-*.txt "$work/rustc-version.txt" "$work/inherited-worker-hashes.sha256" "$bundle/validation/"
cp "$old/bin/runner-preflight-v4" "$bundle/bin/"
cp "$old/analysis-v5-target/release/n83-sweep-runner" "$bundle/bin/runner-analysis-v5"
cp "$old/candidate-target/release/examples/koblitz_n83_factor_base_export" "$bundle/bin/indexed-natural"
cp "$root/bin/docker" "$bundle/bin/docker-wrapper"
cp -a "$root/inputs/fixtures" "$root/inputs/k2048" "$root/inputs/k4096" "$bundle/inputs/"
cp "$old/full-lib-tests.restarted.log" "$old/indexed-differential.restarted.log" "$old/example-tests.log" "$old/runner-tests-analysis-v5.log" "$bundle/validation/"
cp "$proof/PROTOCOL.md" "$proof/LAUNCH-SETUP-V6C.md" "$proof/SUMMARY.json" "$proof/RESULT.md" "$bundle/protocols/"
cp -a "$study/reports/run-20261019T1835Z/." "$bundle/reports/"
git archive --format=tar 660f0f4b10 Cargo.toml Cargo.lock src examples/koblitz_n83_factor_base_export.rs examples/koblitz_n83_free_selector_export_control.rs examples/koblitz_n83_fixed_witness_control.rs examples/koblitz_n83_selector_witness_control.rs tools/n83-sweep-runner "$study/primary_adapter.rs" "$study/size-frontier-v2.json" "$proof" > "$bundle/source/frozen-660f0f4b10.tar"
git rev-parse HEAD > "$bundle/source/collection-head.txt"
git log -8 --format=fuller > "$bundle/source/git-log.txt"
cp "$study/verification/native-xor-indexed-20261019/PUBLICATION.md" "$bundle/UPSTREAM.md"
cat > "$bundle/README.md" <<'TEXT'
Complete new raw native N83 K2048/K4096 100000-conflict continuation.

runs/ contains every actual output and attempt, including failures/censors.
plans/ includes original failed launch and separately assessed corrective
cards; no absent receipt or raw relay is fabricated. A pre-launch shell
failure is not a timed solve. All specs retain original absolute paths.
Sources, exact worker/guard/analysis ELF, public inputs, host/toolchain and
inherited validation are preserved. Validation logs are inherited from the
unchanged worker/source, not falsely reported as new test reruns.
UPSTREAM.md identifies the prior content-addressed complete XOR study and
1000/10000-conflict raw trials used in historical report panels.

No credentials, runtime environment dumps, host filesystem or Cargo caches.
The Docker image is identified by immutable digest and actual inspect data.
Fresh-record replay on the collection host uses unchanged original host
input/worker paths; restore those paths for exact historical replay.
Verify from raw-bundle/: sha256sum -c SHA256SUMS.
Deterministic pack from publication/: tar --sort=name --mtime='2026-10-19 00:00:00Z'
--owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
TEXT
(cd "$bundle"; find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS.tmp; mv ../SHA256SUMS.tmp SHA256SUMS)
cd "$work/publication"
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz raw-bundle/SHA256SUMS > archive-and-manifest.sha256
stat -c '%s' runs.tar.xz > archive-bytes.txt
