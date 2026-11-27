#!/usr/bin/env bash
# Offline publisher contract; fake AWS and Docker, never run bundle binaries.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
mkdir "$scratch/bin" "$scratch/bundle"
mkdir -p "$scratch/study/fresh-inputs-20261127"
for name in publish-pair.sh native-metrics.patch Dockerfile.panel run-pair.sh; do
  cp "$here/$name" "$scratch/study/$name"
done
cp -R "$here/fresh-inputs-20261127/panel" "$scratch/study/fresh-inputs-20261127/panel"
publisher="$scratch/study/publish-pair.sh"
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
  --arg overlay_hash "$(sha256sum "$scratch/study/native-metrics.patch" | cut -d' ' -f1)" \
  '{schema:"point-index.build-pair/v2",baseline_revision:$baseline,candidate_revision:$candidate,
    driver_revision:$driver,target:"x86_64-unknown-linux-musl",rustflags:"-C target-cpu=x86-64",
    baseline_sha256:$baseline_hash,candidate_sha256:$candidate_hash,panel_sha256:$panel_hash,
    optimized_options_sha256:$options_hash,measurement_overlay_sha256:$overlay_hash,
    calibrated_total_operations:null,experiment_submitted:false}' > "$scratch/bundle/build-info.json"
jq --arg baseline "$base_revision" --arg candidate "$candidate_revision" \
  '.baseline=$baseline | .paired_baseline=$candidate' \
  "$scratch/study/fresh-inputs-20261127/panel/manifest.json" > "$scratch/frozen-manifest.json"
mv "$scratch/frozen-manifest.json" "$scratch/study/fresh-inputs-20261127/panel/manifest.json"
(cd "$scratch/study/fresh-inputs-20261127/panel" && \
  sha256sum manifest.json problems/*.json seed-manifest.json > ../inputs.sha256)
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
"$publisher" --check-only "$scratch/bundle" "$driver_revision" > "$scratch/check-only.out"
[[ ! -s $TEST_PUBLISH_CALLS && ! -e $TEST_EXECUTED ]]
mkdir -p "$scratch/payload"
for name in baseline candidate panel optimized-options.json build-info.json SHA256SUMS; do cp "$scratch/bundle/$name" "$scratch/payload/$name"; done
cp "$scratch/study/fresh-inputs-20261127/inputs.sha256" "$scratch/payload/inputs.sha256"
cp -R "$scratch/study/fresh-inputs-20261127/panel" "$scratch/payload/inputs"
"$scratch/study/run-pair.sh" --check-only "$scratch/payload" > "$scratch/runner-check.out"
"$publisher" "$scratch/bundle" "$driver_revision" "$scratch/image.json" > "$scratch/success.out"
[[ ! -e $TEST_EXECUTED ]]
jq -e --arg base "$base_revision" --arg candidate "$candidate_revision" --arg driver "$driver_revision" '
  .schema=="point-index.image/v3" and .baseline_revision==$base
  and .candidate_revision==$candidate and .driver_revision==$driver
  and (.measurement_overlay_sha256|test("^[a-f0-9]{64}$"))
  and (.inputs_sha256|test("^[a-f0-9]{64}$"))
  and (.runner_sha256|test("^[a-f0-9]{64}$"))
  and .image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:"+("0"*64)
  and .experiment_submitted==false
' "$scratch/image.json" >/dev/null
reject_before_cloud() {
  : > "$TEST_PUBLISH_CALLS"
  if "$publisher" "$scratch/bundle" "$driver_revision" "$scratch/rejected.json" > "$scratch/reject.out" 2> "$scratch/reject.err"; then
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
cp "$scratch/study/fresh-inputs-20261127/inputs.sha256" "$scratch/inputs-original.sha256"
printf 'tampered\n' >> "$scratch/study/fresh-inputs-20261127/panel/manifest.json"
reject_before_cloud
cp "$scratch/study/fresh-inputs-20261127/panel/manifest.json" "$scratch/payload/inputs/manifest.json"
if "$scratch/study/run-pair.sh" --check-only "$scratch/payload" > "$scratch/runner-reject.out" 2>&1; then
  echo 'runner accepted changed public input' >&2; exit 1
fi
cp "$scratch/payload/inputs/manifest.json" "$scratch/runner-tamper.json"
sed -i '$d' "$scratch/study/fresh-inputs-20261127/panel/manifest.json"
cp "$scratch/inputs-original.sha256" "$scratch/study/fresh-inputs-20261127/inputs.sha256"
cp "$scratch/study/fresh-inputs-20261127/panel/manifest.json" "$scratch/frozen-manifest.original.json"
jq '.baseline="pending-final-source-freeze"' "$scratch/frozen-manifest.original.json" \
  > "$scratch/study/fresh-inputs-20261127/panel/manifest.json"
(cd "$scratch/study/fresh-inputs-20261127/panel" && \
  sha256sum manifest.json problems/*.json seed-manifest.json > ../inputs.sha256)
reject_before_cloud
cp "$scratch/study/fresh-inputs-20261127/panel/manifest.json" "$scratch/payload/inputs/manifest.json"
cp "$scratch/study/fresh-inputs-20261127/inputs.sha256" "$scratch/payload/inputs.sha256"
if "$scratch/study/run-pair.sh" --check-only "$scratch/payload" > "$scratch/runner-reject.out" 2>&1; then
  echo 'runner accepted pending source marker' >&2; exit 1
fi
cp "$scratch/frozen-manifest.original.json" "$scratch/study/fresh-inputs-20261127/panel/manifest.json"
cp "$scratch/inputs-original.sha256" "$scratch/study/fresh-inputs-20261127/inputs.sha256"
jq '.batch_chunk=8' "$scratch/bundle/optimized-options.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/optimized-options.json"
options_hash=$(sha256sum "$scratch/bundle/optimized-options.json" | cut -d' ' -f1)
jq --arg hash "$options_hash" '.optimized_options_sha256=$hash' "$scratch/bundle/build-info.json" > "$scratch/changed.json"
mv "$scratch/changed.json" "$scratch/bundle/build-info.json"
(cd "$scratch/bundle" && sha256sum baseline candidate panel optimized-options.json > SHA256SUMS)
reject_before_cloud
printf 'Publisher and image runner accepted the frozen fixture and rejected changed or pending inputs before cloud access; no supplied executable ran.\n'
