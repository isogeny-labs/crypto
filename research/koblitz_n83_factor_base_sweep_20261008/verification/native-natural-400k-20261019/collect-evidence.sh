#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-400k-20261019
prior=/home/ec2-user/scratch/n83-natural-100k-20261019
old=/home/ec2-user/scratch/n83-xor-opt-20261019
root=/home/ec2-user/scratch/n83-docker-sweep-20261018
study=research/koblitz_n83_factor_base_sweep_20261008
proof="$study/verification/native-natural-400k-20261019"
bundle="$work/publication/raw-bundle"
mkdir -p "$bundle"/{runs,plans,bin,inputs,source,validation,reports,protocols}
cp "$work/collect-evidence.sh" "$work/finalize-report.sh" "$work/publish-raw.sh" "$work/verify-fresh-native.sh" "$bundle/"
cp -a "$proof/exec/." "$bundle/runs/"
cp -a "$work/cards" "$work/specs" "$work/specs-v11" "$bundle/plans/"
cp "$work"/manifest*.json "$work"/summary*.json "$work"/host-*.txt "$work/rustc-version.txt" "$work/retained-worker-sources.diff" "$work/worker.sha256" "$work"/runner-*.log "$work"/docker-readiness.* "$bundle/validation/"
cp "$work"/k4096-v10-later-absence.* "$bundle/validation/"
cp "$work/bin/runner-v10" "$work/bin/runner-v11" "$bundle/bin/"
cp "$old/candidate-target/release/examples/koblitz_n83_factor_base_export" "$bundle/bin/indexed-natural"
cp "$root/bin/docker" "$bundle/bin/docker-wrapper"
cp -a "$root/inputs/fixtures" "$root/inputs/k2048" "$root/inputs/k4096" "$bundle/inputs/"
cp "$old/full-lib-tests.restarted.log" "$old/indexed-differential.restarted.log" "$old/example-tests.log" "$bundle/validation/"
cp "$prior/post-merge-full-lib.log" "$prior/post-merge-loopback-tests.log" "$prior/post-merge-exporter-tests.log" "$prior/post-merge-model-checker-tests.log" "$bundle/validation/"
cp "$proof/PROTOCOL.md" "$proof/INSPECTION-V11.md" "$proof/INSPECTION-OBSERVATIONS.md" "$proof/SUMMARY.json" "$proof/RESULT.md" "$bundle/protocols/"
cp -a "$study/reports/run-20261019T2015Z/." "$bundle/reports/"
for revision in 660f0f4b10 d216680177 cad91042bf; do
 git archive --format=tar "$revision" Cargo.toml Cargo.lock src examples/koblitz_n83_factor_base_export.rs examples/koblitz_n83_free_selector_export_control.rs examples/koblitz_n83_fixed_witness_control.rs examples/koblitz_n83_selector_witness_control.rs tools/n83-sweep-runner "$study/primary_adapter.rs" "$study/size-frontier-v2.json" > "$bundle/source/frozen-$revision.tar"
done
git rev-parse HEAD > "$bundle/source/collection-head.txt"
git log -8 --format=fuller > "$bundle/source/git-log.txt"
cp "$study/verification/native-natural-200k-20261019/PUBLICATION.md" "$bundle/UPSTREAM.md"
cat > "$bundle/README.md" <<'TEXT'
Complete new raw native N83 K2048/K4096 400000-conflict continuation.
Every actual attempt/output/failure/censor and exact original cards, specs,
relays, receipts and assessments are preserved. Fresh v10 cards use full
core14/30 with unchanged strict gates and retained worker/query/inputs.
Original absolute paths remain frozen. Full relevant worker/guard sources,
root/standalone Cargo manifests/locks, exact ELFs, public inputs, host metadata,
inherited/new guard validation, reports and plots are retained. Prior tests
are inherited, with socket-denial/rerun qualifications in logs and report.
UPSTREAM.md links prior immutable raw publications, including earlier failures.
No credentials, environment dumps, host filesystem or Cargo caches.
Docker image identified by digest and actual inspect records, not full image.
Fresh-record replay uses original host input/worker paths; restore them for
exact historical replay or use a separately audited relocation.
Verify from raw-bundle/: sha256sum -c SHA256SUMS.
Deterministic pack from publication/: tar --sort=name --mtime='2026-10-19 00:00:00Z'
--owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
TEXT
(cd "$bundle"; find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS.tmp; mv ../SHA256SUMS.tmp SHA256SUMS)
cd "$work/publication"
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz raw-bundle/SHA256SUMS > archive-and-manifest.sha256
stat -c '%s' runs.tar.xz > archive-bytes.txt
