#!/usr/bin/env bash
# Resume one authoritatively interrupted case from its immutable S3 checkpoint.
set -euo pipefail
image_bundle=${1:?frozen v1 base64 source}
image_receipts=${2:?new receipt directory}
image_p=${3:?prime}
image_seed=${4:?seed}
image_previous=${5:?previous job ID}
[[ "$image_p" =~ ^[0-9]+$ && "$image_seed" =~ ^[012]$ && "$image_previous" =~ ^[A-Za-z0-9._-]+$ ]]
mkdir "$image_receipts"
image_b64=$(cat "$image_bundle")
image_args=()
image_offset=0
for image_part in A B C D E F G H I J K L M N O P Q R S T; do
  image_args+=(--env "IMAGE_BUNDLE_$image_part=${image_b64:image_offset:100000}")
  image_offset=$((image_offset+100000))
done
set +e
bash taskq/deploy/ephemeral/ephemeral run --detach --spot --no-source --toolchain rust --type c7i.2xlarge --ttl 150m --name "twist-resume-p$image_p-s$image_seed" --env "COLD_PRIMES=$image_p" --env "COLD_SEEDS=$image_seed" --env "COLD_PREVIOUS_PREFIX=s3://ephemeral-runs-104186811093-us-east-1/jobs/$image_previous/out" "${image_args[@]}" -- "$(cat research/twist_index_calculus_20261020/endomorphism_image_base/run-cold.sh)" > "$image_receipts/launch.stdout" 2> "$image_receipts/launch.stderr"
image_status=$?
printf '%s\n' "$image_status" > "$image_receipts/launch.exit"
cat "$image_receipts/launch.stdout"
exit "$image_status"
