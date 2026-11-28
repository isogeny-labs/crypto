#!/usr/bin/env bash
# Trusted-main publisher. Treat the unprivileged build bundle as data.
set -euo pipefail
check_only=false
if [[ $# == 3 && $1 == --check-only ]]; then
  check_only=true
  shift
elif [[ $# != 3 ]]; then
  echo 'usage: publish-pair.sh [--check-only] BUNDLE_DIR DRIVER_REVISION [NEW_OUTPUT.json]' >&2
  exit 2
fi
here=$(cd "$(dirname "$0")" && pwd)
bundle=$(realpath "$1")
revision=$2
output=${3:-}
: "${ISOLAB_IMAGE_REPOSITORY:?}" "${AWS_REGION:?}" "${GITHUB_RUN_ID:?}" "${GITHUB_RUN_ATTEMPT:?}"
[[ $revision =~ ^[a-f0-9]{40}$ && $GITHUB_RUN_ID =~ ^[0-9]+$ && $GITHUB_RUN_ATTEMPT =~ ^[0-9]+$ ]]
[[ $ISOLAB_IMAGE_REPOSITORY == 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks ]]
[[ $AWS_REGION == us-east-2 ]]
if [[ $check_only == false ]]; then
  [[ ! -e $output && ! -L $output ]] || { echo 'publisher output already exists' >&2; exit 2; }
fi
for name in baseline candidate panel optimized-options.json build-info.json SHA256SUMS; do
  [[ -f $bundle/$name && ! -L $bundle/$name ]] || { echo "missing or symlinked bundle file: $name" >&2; exit 2; }
done
overlay_hash=$(sha256sum "$here/native-metrics.patch" | cut -d' ' -f1)
jq -e --arg revision "$revision" --arg overlay_hash "$overlay_hash" '
  .schema=="point-index.build-pair/v2" and .driver_revision==$revision
  and .baseline_revision!=.candidate_revision
  and (.baseline_revision|test("^[a-f0-9]{40}$"))
  and (.candidate_revision|test("^[a-f0-9]{40}$"))
  and .target=="x86_64-unknown-linux-musl" and .rustflags=="-C target-cpu=x86-64"
  and .experiment_submitted==false
  and .calibrated_total_operations==null
  and .measurement_overlay_sha256==$overlay_hash
  and all(.baseline_sha256,.candidate_sha256,.panel_sha256,.optimized_options_sha256;
    type=="string" and test("^[a-f0-9]{64}$"))
' "$bundle/build-info.json" >/dev/null || { echo 'invalid build metadata' >&2; exit 2; }
jq -e '
  type=="object" and (keys==["batch_chunk","extra_scans","joint_targets","relation_graph","subgroup_provenance","walk"])
  and .batch_chunk==1 and .extra_scans==0 and .joint_targets==true
  and .relation_graph==false and .subgroup_provenance==false and .walk==null
' "$bundle/optimized-options.json" >/dev/null || { echo 'unexpected optimized policy' >&2; exit 2; }
for field in baseline candidate panel optimized-options.json; do
  key=$field
  [[ $field == optimized-options.json ]] && key=optimized_options
  digest=$(jq -er --arg key "$key" '.[$key+"_sha256"]' "$bundle/build-info.json")
  [[ $(sha256sum "$bundle/$field" | cut -d' ' -f1) == "$digest" ]] || { echo "bundle hash mismatch: $field" >&2; exit 2; }
done
expected_list=$(mktemp)
trap 'rm -f "$expected_list"' EXIT
for field in baseline candidate panel optimized-options.json; do
  key=$field
  [[ $field == optimized-options.json ]] && key=optimized_options
  digest=$(jq -er --arg key "$key" '.[$key+"_sha256"]' "$bundle/build-info.json")
  printf '%s  %s\n' "$digest" "$field" >> "$expected_list"
done
cmp -s "$expected_list" "$bundle/SHA256SUMS" || { echo 'bundle checksum list mismatch' >&2; exit 2; }
# A runnable image must carry the exact frozen public panel. The main-only
# workflow checks the full spec before this script; repeat the image-local
# closure checks here before any cloud identity is used.
panel_dir="$here/fresh-inputs-20261127/panel"
input_hashes="$here/fresh-inputs-20261127/inputs.sha256"
[[ -d $panel_dir && ! -L $panel_dir && -f $input_hashes && ! -L $input_hashes ]]
[[ -z $(find "$panel_dir" -type l -print -quit) ]] || { echo 'symlinked public input' >&2; exit 2; }
[[ $(wc -l < "$input_hashes") == 86 ]] || { echo 'incomplete public input list' >&2; exit 2; }
(cd "$panel_dir" && sha256sum -c "$input_hashes" >/dev/null) || { echo 'public input hash mismatch' >&2; exit 2; }
jq -e --slurpfile build "$bundle/build-info.json" '
  .schema=="hashed-ecdl-panel/v2" and .status=="prepared"
  and .baseline==$build[0].baseline_revision
  and .paired_baseline==$build[0].candidate_revision
  and .execution=="not_run" and .measurements==null
  and .development_inputs==60 and .holdout_inputs==24
  and .required_aa_rounds==5 and .required_ab_rounds==5
  and (.cases|length)==84
' "$panel_dir/manifest.json" >/dev/null || { echo 'public panel is not frozen to both arms' >&2; exit 2; }
[[ -f $here/run-pair.sh && ! -L $here/run-pair.sh ]] || { echo 'missing native runner' >&2; exit 2; }
if [[ $check_only == true ]]; then
  echo 'Pair bundle and public panel verified without AWS identity; no image or experiment submitted.'
  exit 0
fi
context=$(mktemp -d)
trap 'rm -rf "$context"; rm -f "$expected_list"' EXIT
for name in baseline candidate panel; do install -m755 "$bundle/$name" "$context/$name"; done
for name in optimized-options.json build-info.json SHA256SUMS; do install -m644 "$bundle/$name" "$context/$name"; done
install -m755 "$here/run-pair.sh" "$context/run-pair.sh"
install -m644 "$input_hashes" "$context/inputs.sha256"
cp -R -- "$panel_dir" "$context/inputs"
"$context/run-pair.sh" --check-only "$context" >/dev/null
registry=${ISOLAB_IMAGE_REPOSITORY%%/*}
repository=${ISOLAB_IMAGE_REPOSITORY#*/}
tag="gh-${GITHUB_RUN_ID}-${GITHUB_RUN_ATTEMPT}-${revision}-point-index"
aws ecr get-login-password --region "$AWS_REGION" | docker login --username AWS --password-stdin "$registry"
if ! aws ecr-public get-login-password --region us-east-1 \
  | docker login --username AWS --password-stdin public.ecr.aws >/dev/null 2>&1; then
  echo 'warning: public ECR authentication unavailable; pinned base pull may fail' >&2
fi
docker build --platform linux/amd64 --file "$here/Dockerfile.panel" \
  --build-arg "SOURCE_REVISION=$revision" --tag "$ISOLAB_IMAGE_REPOSITORY:$tag" "$context"
docker push "$ISOLAB_IMAGE_REPOSITORY:$tag"
digest=$(aws ecr describe-images --repository-name "$repository" --image-ids "imageTag=$tag" \
  --query 'imageDetails[0].imageDigest' --output text)
[[ $digest =~ ^sha256:[a-f0-9]{64}$ ]] || { echo 'missing immutable image digest' >&2; exit 1; }
jq -n --arg image "$ISOLAB_IMAGE_REPOSITORY@$digest" --arg revision "$revision" \
  --arg build "$(sha256sum "$bundle/build-info.json" | cut -d' ' -f1)" \
  --arg inputs "$(sha256sum "$input_hashes" | cut -d' ' -f1)" \
  --arg seeds "$(sha256sum "$panel_dir/seed-manifest.json" | cut -d' ' -f1)" \
  --arg runner "$(sha256sum "$here/run-pair.sh" | cut -d' ' -f1)" \
  --slurpfile source "$bundle/build-info.json" \
  '{schema:"point-index.image/v3",image_uri:$image,driver_revision:$revision,
    build_info_sha256:$build,baseline_revision:$source[0].baseline_revision,
    candidate_revision:$source[0].candidate_revision,
    baseline_sha256:$source[0].baseline_sha256,candidate_sha256:$source[0].candidate_sha256,
    panel_sha256:$source[0].panel_sha256,
    inputs_sha256:$inputs,seed_manifest_sha256:$seeds,runner_sha256:$runner,
    optimized_options_sha256:$source[0].optimized_options_sha256,
    measurement_overlay_sha256:$source[0].measurement_overlay_sha256,
    experiment_submitted:false}' > "$output"
echo 'Pair image published by immutable digest; no experiment submitted.'
