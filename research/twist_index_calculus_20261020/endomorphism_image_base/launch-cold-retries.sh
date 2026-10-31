#!/usr/bin/env bash
# Thin launch orchestration. Every retry rebuilds all arms for one case on
# one dedicated host. Frozen v1 native source is reused byte for byte.
set -euo pipefail
image_bundle=${1:?path to frozen v1 base64 source}
image_receipts=${2:?new receipt directory}
mkdir "$image_receipts"
image_b64=$(cat "$image_bundle")
image_args=()
image_offset=0
for image_part in A B C D E F G H I J K L M N O P Q R S T; do
  image_args+=(--env "IMAGE_BUNDLE_$image_part=${image_b64:image_offset:100000}")
  image_offset=$((image_offset+100000))
done
for image_case in 229:1 229:2 241:0 241:1 241:2 337:1 337:2; do
  image_p=${image_case%:*}; image_seed=${image_case#*:}
  image_label="p$image_p-s$image_seed"
  bash taskq/deploy/ephemeral/ephemeral run --detach --spot --no-source --toolchain rust --type c7i.2xlarge --ttl 150m --name "twist-cold-$image_label" --env "COLD_PRIMES=$image_p" --env "COLD_SEEDS=$image_seed" "${image_args[@]}" -- "$(cat research/twist_index_calculus_20261020/endomorphism_image_base/run-cold.sh)" > "$image_receipts/$image_label.stdout" 2> "$image_receipts/$image_label.stderr"
  cat "$image_receipts/$image_label.stdout"
done
