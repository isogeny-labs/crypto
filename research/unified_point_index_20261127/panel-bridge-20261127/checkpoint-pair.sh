#!/usr/bin/env bash
# Trusted-main source/data checkpoint for the paired native panel. No solve.
set -euo pipefail
check_only=false
if [[ ${1:-} == --check-only ]]; then check_only=true; shift; fi
[[ $# == 8 ]] || { echo 'usage: checkpoint-pair.sh [--check-only] CONTROL_DIR DRIVER_DIR DRIVER_SHA BASE_DIR BASE_SHA CANDIDATE_DIR CANDIDATE_SHA NEW_OUTPUT' >&2; exit 2; }
control=$(realpath -- "$1")
driver=$(realpath -- "$2")
driver_sha=$3
baseline=$(realpath -- "$4")
baseline_sha=$5
candidate=$(realpath -- "$6")
candidate_sha=$7
output=$(realpath -m -- "$8")
study=research/unified_point_index_20261127/panel-bridge-20261127
for revision in "$driver_sha" "$baseline_sha" "$candidate_sha"; do
  [[ $revision =~ ^[a-f0-9]{40}$ ]] || { echo 'invalid source SHA' >&2; exit 2; }
done
[[ $driver_sha != "$baseline_sha" && $driver_sha != "$candidate_sha" && $baseline_sha != "$candidate_sha" ]] || { echo 'source revisions are not distinct' >&2; exit 2; }
for pair in "$control:$driver_sha" "$driver:$driver_sha" "$baseline:$baseline_sha" "$candidate:$candidate_sha"; do
  dir=${pair%:*}; revision=${pair#*:}
  [[ $(git -C "$dir" rev-parse HEAD) == "$revision" ]] || { echo "checkout SHA mismatch: $dir" >&2; exit 2; }
  git -C "$dir" diff --quiet HEAD || { echo "dirty checkout: $dir" >&2; exit 2; }
  [[ -z $(git -C "$dir" ls-files --others --exclude-standard) ]] || { echo "untracked checkout: $dir" >&2; exit 2; }
done
[[ -x $control/$study/check-frozen-inputs.sh && -x $control/scripts/checkpoint.sh ]] || { echo 'missing trusted main checkpoint tooling' >&2; exit 2; }
"$control/$study/check-frozen-inputs.sh" "$driver" "$driver_sha"
jq -e --arg baseline "$baseline_sha" --arg candidate "$candidate_sha" \
  '.baseline_revision==$baseline and .candidate_revision==$candidate' \
  "$driver/$study/publish-inputs.json" >/dev/null || { echo 'arm revisions disagree with the frozen manifest' >&2; exit 2; }
[[ -f $baseline/crates/hashed-ecdl-tensor/Cargo.toml && -f $candidate/crates/hashed-ecdl-tensor/Cargo.toml ]] || { echo 'incomplete native arm sources' >&2; exit 2; }
[[ $output != "$control"* && $output != "$driver"* && $output != "$baseline"* && $output != "$candidate"* ]] || { echo 'receipt output inside a source checkout' >&2; exit 2; }
[[ ! -e $output && ! -L $output ]] || { echo 'checkpoint receipt already exists' >&2; exit 2; }
for temporary in point-index-spec.json point-index-protocol.json point-index-driver-source.tar.gz \
  point-index-baseline-source.tar.gz point-index-candidate-source.tar.gz .resume; do
  [[ ! -e $driver/$temporary && ! -L $driver/$temporary ]] || { echo "checkpoint staging path exists: $temporary" >&2; exit 2; }
done
if [[ $check_only == true ]]; then
  echo 'Frozen three-source checkpoint closure admitted without AWS; no upload or experiment submitted.'
  exit 0
fi
[[ ${GITHUB_RUN_ID:-} =~ ^[0-9]+$ && ${GITHUB_RUN_ATTEMPT:-} =~ ^[0-9]+$ ]] || { echo 'missing trusted run identity' >&2; exit 2; }
command -v aws >/dev/null
mkdir -- "$output"
aws sts get-caller-identity --output json | jq -e '.Account=="104186811093"' > "$output/caller-account-check.json"
spec_path=$(jq -er .spec_path "$driver/$study/publish-inputs.json")
protocol_path=$(jq -er .protocol_path "$driver/$study/publish-inputs.json")
cp -- "$driver/$spec_path" "$driver/point-index-spec.json"
cp -- "$driver/$protocol_path" "$driver/point-index-protocol.json"
git -C "$driver" archive --format=tar.gz "$driver_sha" -- Cargo.toml Cargo.lock \
  crates/hashed-ecdl-tensor docs/experiment-seeds.md "$study" \
  .github/workflows/point-index-panel-build.yml \
  .github/workflows/point-index-panel-publish.yml \
  .github/workflows/point-index-panel-checkpoint.yml \
  > "$driver/point-index-driver-source.tar.gz"
git -C "$baseline" archive --format=tar.gz "$baseline_sha" -- crates/hashed-ecdl-tensor \
  > "$driver/point-index-baseline-source.tar.gz"
git -C "$candidate" archive --format=tar.gz "$candidate_sha" -- crates/hashed-ecdl-tensor \
  > "$driver/point-index-candidate-source.tar.gz"
for name in point-index-driver-source.tar.gz point-index-baseline-source.tar.gz point-index-candidate-source.tar.gz; do
  [[ $(stat -c %s "$driver/$name") -le 134217728 ]] || { echo 'source closure exceeds 128MiB' >&2; exit 1; }
done
(cd "$driver" && sha256sum point-index-spec.json point-index-protocol.json \
  point-index-driver-source.tar.gz point-index-baseline-source.tar.gz \
  point-index-candidate-source.tar.gz) > "$output/payload.sha256"
mkdir "$driver/.resume"
cat > "$driver/.resume/RESUME.md" <<EOF
# Resume: point-index native pair source checkpoint

## Goal
Measure the complete cold point-index index-calculus pipeline against its exact native predecessor on matched public inputs.

## Lead / hypothesis
A validated shared point index may reduce duplicate map work. No runtime gain is assumed.

## Spec
Driver $driver_sha; baseline $baseline_sha; candidate $candidate_sha. Frozen spec $spec_path and protocol $protocol_path are included with the three source archives and public panel in this checkpoint.

## State
Checkpoint-only GitHub run $GITHUB_RUN_ID attempt $GITHUB_RUN_ATTEMPT. No image publication or scientific computation was started here.

## Next steps
Verify the upload and full S3 payload readback; publish the same tested pair by immutable image digest, then admit native L2/A-A and exact replay before any paired timing.

## Runs in flight
Only the source checkpoint upload/readback; no EKS measurement was submitted.

## Stop and success conditions
No experiment without checkpoint readback, final image identity, native L2 admission and the original 1800-second deadline. Preserve failures and source identities; no automatic measurement retries.
EOF
export CHECKPOINT_MAX_FILE_MB=128 CHECKPOINT_MAX_TOTAL_MB=300
export CHECKPOINT_PREFIX="checkpoints/isolab-point-index/$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT"
(cd "$driver" && "$control/scripts/checkpoint.sh" push --force > "$output/upload.txt")
(cd "$driver" && "$control/scripts/checkpoint.sh" gate > "$output/gate.txt")
read -r _ _ uri < "$driver/.resume/.last-push"
[[ $uri == s3://setec-astronomy-artifacts/checkpoints/isolab-point-index/*/ ]] || { echo 'unexpected checkpoint URI' >&2; exit 1; }
aws s3 cp --only-show-errors "${uri}checkpoint.tar.gz" "$output/checkpoint.tar.gz"
mkdir "$output/readback"
tar -xzf "$output/checkpoint.tar.gz" -C "$output/readback" -- ./meta.json ./RESUME.md ./untracked.tar.gz
jq -e --arg sha "$driver_sha" '.head==$sha' "$output/readback/meta.json" > "$output/source-check.txt"
cmp "$driver/.resume/RESUME.md" "$output/readback/RESUME.md"
mkdir "$output/readback/payload"
tar -xzf "$output/readback/untracked.tar.gz" -C "$output/readback/payload" -- \
  point-index-spec.json point-index-protocol.json point-index-driver-source.tar.gz \
  point-index-baseline-source.tar.gz point-index-candidate-source.tar.gz
(cd "$output/readback/payload" && sha256sum -c "$output/payload.sha256") > "$output/payload-readback.txt"
archive_hash=$(sha256sum "$output/checkpoint.tar.gz" | cut -d' ' -f1)
jq -n --arg driver "$driver_sha" --arg baseline "$baseline_sha" --arg candidate "$candidate_sha" \
  --arg uri "$uri" --arg archive "$archive_hash" \
  --arg spec "$(sha256sum "$driver/point-index-spec.json" | cut -d' ' -f1)" \
  --arg protocol "$(sha256sum "$driver/point-index-protocol.json" | cut -d' ' -f1)" \
  --arg inputs "$(jq -er .inputs_sha256 "$driver/point-index-spec.json")" \
  --arg seeds "$(jq -er .seed_manifest_sha256 "$driver/point-index-spec.json")" \
  '{schema:"point-index.source-checkpoint/v1",driver_revision:$driver,
   baseline_revision:$baseline,candidate_revision:$candidate,checkpoint_uri:$uri,
   archive_sha256:$archive,spec_sha256:$spec,protocol_sha256:$protocol,
   inputs_sha256:$inputs,seed_manifest_sha256:$seeds,
   upload_readback_verified:true,experiment_submitted:false}' > "$output/checkpoint.json"
echo 'Native pair source checkpoint uploaded and independently read back; no experiment submitted.'
