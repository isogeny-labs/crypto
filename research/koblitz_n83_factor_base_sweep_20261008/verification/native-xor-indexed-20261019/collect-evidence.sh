#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
opt=/home/ec2-user/scratch/n83-xor-opt-20261019
bundle="$opt/publication/raw-bundle"
mkdir -p "$bundle"/{runs,plans,bin,source,validation,external,inputs,protocols,reports}
cp "$opt/collect-evidence.sh" "$bundle/"
study=research/koblitz_n83_factor_base_sweep_20261008
cp -a "$study/verification/native-phase-20261019-v2/exec/xor-indexed" "$bundle/runs/"
cp -a "$study/verification/native-phase-20261019-v2/exec/xor-profile" "$bundle/runs/"
cp -a "$study/verification/native-phase-20261019-v2/exec/xor-early" "$bundle/runs/"
for dir in cards cards-root cards-root-v2 cards-v3 cards-v4 cards-v5 specs specs-root specs-root-v2 specs-v3 specs-v4 specs-v5 profile early; do
 cp -a "$opt/$dir" "$bundle/plans/"
done
cp "$opt"/manifest*.json "$opt"/summary*.json "$opt/ROLE-RELAY-NOTE.md" "$bundle/plans/"
cp -a "$opt/external/." "$bundle/external/"
cp -a "$opt/bin/." "$bundle/bin/"
cp "$opt/analysis-v5-target/release/n83-sweep-runner" "$bundle/bin/runner-analysis-v5"
cp "$opt/baseline-target/release/examples/koblitz_n83_free_selector_export_control" "$bundle/bin/baseline-free-selector"
cp "$opt/candidate-target/release/examples/koblitz_n83_free_selector_export_control" "$bundle/bin/indexed-free-selector"
cp "$opt/candidate-target/release/examples/koblitz_n83_factor_base_export" "$bundle/bin/indexed-natural"
cp /home/ec2-user/scratch/n83-selector-export-20261019/runner-target/release/n83-sweep-runner "$bundle/bin/runner-original-profile"
cp /home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench "$bundle/bin/ecbench-store"
cp /home/ec2-user/scratch/n83-docker-sweep-20261018/bin/docker "$bundle/bin/docker-wrapper"
cp -a /home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/{fixtures,k2048,k4096} "$bundle/inputs/"
cp "$opt"/*.log "$opt"/host-*.txt "$opt/rustc-version.txt" "$bundle/validation/"
cp -a "$opt/validation/." "$bundle/validation/"
cp -a "$study/verification/native-xor-indexed-20261019/." "$bundle/protocols/"
cp -a "$study/reports/run-20261019T1720Z/." "$bundle/reports/"
for revision in d2020ca503 4122fddca6 6634ce0955 dc7f38c496 5778423303 40d554ab57 2ccbc3da6b; do
 git show "$revision:$study/verification/native-xor-indexed-20261019/PROTOCOL.md" > "$bundle/protocols/original-$revision.md" 2>/dev/null || rm -f "$bundle/protocols/original-$revision.md"
done
# Historical profile protocols have their own original tracked paths.
for revision in d2020ca503 4122fddca6; do
 git show "$revision:$study/verification/native-xor-profile-20261019/PROTOCOL.md" > "$bundle/protocols/profile-$revision.md"
done
git ls-tree -r --name-only d2020ca503 "$study/verification" > "$bundle/source/profile-source-tree.txt"
for revision in d2020ca503 4122fddca6 40d554ab57 HEAD; do
 label="$revision"
 git archive --format=tar "$revision" Cargo.toml Cargo.lock src examples/koblitz_n83_free_selector_export_control.rs examples/koblitz_n83_factor_base_export.rs examples/koblitz_n83_selector_witness_control.rs examples/koblitz_n83_fixed_witness_control.rs "$study/primary_adapter.rs" "$study/size-frontier-v2.json" tools/n83-sweep-runner > "$bundle/source/$label.tar"
done
git log -25 --format=fuller > "$bundle/source/git-log.txt"
git rev-parse HEAD > "$bundle/source/collection-head.txt"
git diff HEAD > "$bundle/source/uncommitted-tracked.diff"
cp "$study/verification/native-phase-publication-20261019/RESULT.md" "$bundle/UPSTREAM-49MB.md"
cat > "$bundle/README.md" <<'TEXT'
Complete raw native XOR profiles/comparisons and natural continuation.

All source/code and public synthetic inputs are preserved; no credentials
or runtime environment dumps are included. Original paths in frozen specs,
receipts and custody remain literal. Restore those paths for exact original
native replay; the fresh-download audit on the collection host uses unchanged
original input/worker paths and independently checks extracted raw outputs.

runs/ includes stopped, refused, unknown, completed and censored observations.
plans/ includes superseded/unattempted plans; presence of a plan is not a run.
Control word counts and ratios come from the native runner source snapshot.
Older source protocols are preserved by revision; original filenames may differ.
The daemon image digest and constraints are in every inspect/config file;
this archive does not contain the host, Docker image filesystem or Cargo cache.
Rebuild sources with the recorded Rust toolchain/Cargo.lock. Upstream CMS
release asset, license, authors, version and release metadata are in external/.
External source redistribution follows the retained MIT license.

Verify payloads from raw-bundle/: sha256sum -c SHA256SUMS.
Deterministic pack from publication/: tar --sort=name --mtime='2026-10-19 00:00:00Z'
--owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
TEXT
(
 cd "$bundle"
 find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS.tmp
 mv ../SHA256SUMS.tmp SHA256SUMS
)
cd "$opt/publication"
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz raw-bundle/SHA256SUMS > archive-and-manifest.sha256
stat -c '%s' runs.tar.xz > archive-bytes.txt
