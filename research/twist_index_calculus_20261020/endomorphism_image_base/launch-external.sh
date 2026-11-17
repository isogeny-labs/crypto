#!/usr/bin/env bash
# A launch failure has no running instance; reconcile that job before retrying.
set -euo pipefail
image_bundle=${1:?frozen external source bundle in base64}
image_receipts=${2:?new receipt directory}
mkdir "$image_receipts"
image_b64=$(cat "$image_bundle")
image_args=()
image_offset=0
for image_part in A B C D E F G H I J K L M N O P Q R S T; do
  image_args+=(--env "IMAGE_BUNDLE_$image_part=${image_b64:image_offset:100000}")
  image_offset=$((image_offset+100000))
done
set +e
bash taskq/deploy/ephemeral/ephemeral run --detach --spot --no-source --toolchain rust --type c7i.xlarge --ttl 60m --name twist-image-msolve --env RAYON_NUM_THREADS=1 "${image_args[@]}" -- "$(cat research/twist_index_calculus_20261020/endomorphism_image_base/run-external-cloud.sh)" > "$image_receipts/launch.stdout" 2> "$image_receipts/launch.stderr"
image_status=$?
printf '%s\n' "$image_status" > "$image_receipts/launch.exit"
cat "$image_receipts/launch.stdout"
exit "$image_status"
