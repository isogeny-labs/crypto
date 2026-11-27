#!/usr/bin/env bash
# Offline publisher contract; fake AWS and Docker, never run bundle binaries.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
mkdir "$scratch/bin" "$scratch/bundle"
export TEST_PUBLISH_CALLS="$scratch/calls" TEST_EXECUTED="$scratch/executed"
export AWS_REGION=us-east-2 GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=1
export ISOLAB_IMAGE_REPOSITORY=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
base_revision=$(printf 'a%.0s' {1..40})
candidate_revision=$(printf 'b%.0s' {1..40})
driver_revision=$(printf 'c%.0s' {1..40})
for name in baseline candidate panel; do
  cat > "$scratch/bundle/$name" <<'EOF'
#!/bin/sh
touch "$TEST_EXECUTED"
exit 23
EOF
  chmod +x "$scratch/bundle/$name"
done
cp "$here/optimized-options.json" "$scratch/bundle/optimized-options.json"
(cd "$scratch/bundle" && sha256sum baseline candidate panel optimized-options.json > SHA256SUMS)
jq -n --arg baseline "$base_revision" --arg candidate "$candidate_revision" --arg driver "$driver_revision" \
  --arg baseline_hash "$(sha256sum "$scratch/bundle/baseline" | cut -d' ' -f1)" \
  --arg candidate_hash "$(sha256sum "$scratch/bundle/candidate" | cut -d' ' -f1)" \
  --arg panel_hash "$(sha256sum "$scratch/bundle/panel" | cut -d' ' -f1)" \
  --arg options_hash "$(sha256sum "$scratch/bundle/optimized-options.json" | cut -d' ' -f1)" \
  --arg overlay_hash "$(sha256sum "$here/native-metrics.patch" | cut -d' ' -f1)" \
  '{schema:"point-index.build-pair/v2",baseline_revision:$baseline,candidate_revision:$candidate,
    driver_revision:$driver,target:"x86_64-unknown-linux-musl",rustflags:"-C target-cpu=x86-64",
    baseline_sha256:$baseline_hash,candidate_sha256:$candidate_hash,panel_sha256:$panel_hash,
    optimized_options_sha256:$options_hash,measurement_overlay_sha256:$overlay_hash,
    calibrated_total_operations:null,experiment_submitted:false}' > "$scratch/bundle/build-info.json"
cat > "$scratch/bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'aws %s\n' "$*" >> "$TEST_PUBLISH_CALLS"
case "$*" in
  'ecr get-login-password --region us-east-2') echo test-token ;;
  'ecr-public get-login-password --region us-east-1') echo public-token ;;
  'ecr describe-images --repository-name isolab/tasks --image-ids imageTag=gh-123-1-'*) printf 'sha256:%064d\n' 0 ;;
  *) exit 2 ;;
esac
EOF
cat > "$scratch/bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'docker %s\n' "$*" >> "$TEST_PUBLISH_CALLS"
if [[ $1 == login ]]; then cat >/dev/null; fi
EOF
chmod +x "$scratch/bin/aws" "$scratch/bin/docker"
export PATH="$scratch/bin:$PATH"
"$here/publish-pair.sh" "$scratch/bundle" "$driver_revision" "$scratch/image.json" > "$scratch/success.out"
[[ ! -e $TEST_EXECUTED ]]
jq -e --arg base "$base_revision" --arg candidate "$candidate_revision" --arg driver "$driver_revision" '
  .schema=="point-index.image/v2" and .baseline_revision==$base
  and .candidate_revision==$candidate and .driver_revision==$driver
  and (.measurement_overlay_sha256|test("^[a-f0-9]{64}$"))
  and .image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:"+("0"*64)
  and .experiment_submitted==false
' "$scratch/image.json" >/dev/null
reject_before_cloud() {
  : > "$TEST_PUBLISH_CALLS"
  if "$here/publish-pair.sh" "$scratch/bundle" "$driver_revision" "$scratch/rejected.json" > "$scratch/reject.out" 2> "$scratch/reject.err"; then
    echo 'publisher accepted an invalid bundle' >&2
    exit 1
  fi
  [[ ! -s $TEST_PUBLISH_CALLS && ! -e $TEST_EXECUTED && ! -e $scratch/rejected.json ]]
}
cp "$scratch/bundle/candidate" "$scratch/candidate-original"
printf tampered >> "$scratch/bundle/candidate"
reject_before_cloud
cp "$scratch/candidate-original" "$scratch/bundle/candidate"
cp "$scratch/bundle/build-info.json" "$scratch/build-info-original.json"
jq '.driver_revision="0000000000000000000000000000000000000000"' "$scratch/bundle/build-info.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/build-info.json"
reject_before_cloud
cp "$scratch/build-info-original.json" "$scratch/bundle/build-info.json"
jq '.measurement_overlay_sha256="0000000000000000000000000000000000000000000000000000000000000000"' "$scratch/bundle/build-info.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/build-info.json"
reject_before_cloud
cp "$scratch/build-info-original.json" "$scratch/bundle/build-info.json"
cp "$scratch/bundle/SHA256SUMS" "$scratch/checksum-original"
printf 'invalid extra entry\n' >> "$scratch/bundle/SHA256SUMS"
reject_before_cloud
cp "$scratch/checksum-original" "$scratch/bundle/SHA256SUMS"
jq '.batch_chunk=8' "$scratch/bundle/optimized-options.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/optimized-options.json"
options_hash=$(sha256sum "$scratch/bundle/optimized-options.json" | cut -d' ' -f1)
jq --arg hash "$options_hash" '.optimized_options_sha256=$hash' "$scratch/bundle/build-info.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/build-info.json"
(cd "$scratch/bundle" && sha256sum baseline candidate panel optimized-options.json > SHA256SUMS)
reject_before_cloud
printf 'Publisher accepted the exact bundle and rejected hash, source, checksum-list and policy changes before cloud access; no supplied executable ran.\n'
