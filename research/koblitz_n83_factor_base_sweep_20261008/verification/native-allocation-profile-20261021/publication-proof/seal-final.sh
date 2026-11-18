#!/usr/bin/env bash
set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/publication"
proof="$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/native-allocation-profile-20261021"
mkdir "$pub" "$pub/runs"
raw="$pub/runs"
cp -a "$work/bin" "$work/build-attempt1" "$work/build-v2" "$work/guard-attempt2" "$work/final-validation" "$work/analysis" "$work/live-capture" "$work/pdf-inspection" "$raw/"
mkdir "$raw/plans" "$raw/readiness" "$raw/inputs" "$raw/reference" "$raw/proof" "$raw/checkpoint-proofs"
cp -a "$work/cards" "$work/specs" "$raw/plans/"
find "$work" -maxdepth 1 -type f -exec cp -t "$raw/readiness" {} +
cp -a "$proof/exec" "$raw/"
find "$proof" -mindepth 1 -maxdepth 1 ! -name exec -exec cp -a -t "$raw/proof" {} +
for id in fixtures k2048 k4096; do cp -a "/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/$id" "$raw/inputs/"; done
for id in k2048-v20 k4096-v20; do cp -a "$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-1m-20261020/exec/$id" "$raw/reference/"; done
cp /home/ec2-user/scratch/n83-xor-opt-20261019/candidate-target/release/examples/koblitz_n83_factor_base_export "$raw/bin/retained-worker"
for phase in checkpoint p22-launch-v2 completed-k2048 queued-checkpoint; do
 mkdir "$raw/checkpoint-proofs/$phase"
 find "$work/$phase" -maxdepth 1 -type f ! -name '*.tar.xz' -exec cp -t "$raw/checkpoint-proofs/$phase" {} +
 cp -a "$work/$phase/store" "$raw/checkpoint-proofs/$phase/"
done
cp -a "$repo/research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261021T1617Z" "$raw/report"
cp -a "$repo/benchmarks/20261021-n83-storage-profile" "$raw/benchmark-copies"
git -C "$repo" bundle create "$raw/SOURCE.bundle" HEAD ^37679b3586992e52e44cd1e994f589aa2a9b204b
git -C "$repo" bundle verify "$raw/SOURCE.bundle" > "$raw/source-bundle-verify.txt" 2>&1
git -C "$repo" rev-parse HEAD HEAD^{tree} > "$raw/source-publication-freeze.txt"
cat > "$raw/README.md" <<'TEXT'
Native N83 allocation-ownership diagnostic raw evidence,2026-10-21UTC.
K2048-p22 andK4096-p22 complete450000conflicts/models0/relations0/rank0. Both
native audits/memory/storage-prefix checks PASS:451matched conflict snapshots
includingzero,47storage snapshots,29compliant8GiB/swap0samples each, verified
startup/resource/relay/receipt/cleanup. Exact worker source674d2075,queued guard
7e8ae96c; SOURCE.bundle retains unpublished freezes and final report/proof source
with published37679b3586992e52e44cd1e994f589aa2a9b204b prerequisite.

Every available original retained: K2048-p21 shell redirection failure before
card-run (no receipt/duration); K4096-p21 pre-worker lock refusal, actual receipt;
compile/link failures and queued build/readiness records; unused early ELFs;
actual cards/specs/prompts/assessments/relays/raw streams/profiles, public inputs,
worker/guards/reference1Mtrees/full release checks and inspected visual report.
No credentials/environment dumps/private targets. Exact full source for the
initial failed development compilation was not snapshotted; its log is retained.
No measurement depends on it. Original1M/800K failures and full old raw archives
remain separately verified prerequisites, not retrospectively admitted.

Capacity,logical bytes,allocator totals and residency are distinct. No candidate
reclamation or higher cap, timing gain, UNSAT, selected base or recovery result.
Historical absolute binds require restored original paths; extracted inputs/ELFs
are compared separately. Negative controls are constructed read-only-copy tamper
checks, never actual search attempts. Upload/fresh verification receipts live
outside this immutable archive and are included in a later verification sidecar.
Manifest all payloads. Canonical root runs, sorted GNUtar,2026-10-21UTC,uid/gid0,
normalized public modes,xz-6-T1. See readiness/seal-final.sh/publish-final.sh.
TEXT
(cd "$raw"; find . -type f ! -path ./SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > ../SHA256SUMS; mv ../SHA256SUMS SHA256SUMS)
(cd "$pub"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > runs.tar.xz)
sha256sum "$pub/runs.tar.xz" "$raw/SHA256SUMS" > "$pub/hashes.txt"
stat -c %s "$pub/runs.tar.xz" > "$pub/bytes.txt"
chown -R ec2-user:ec2-user "$pub"
