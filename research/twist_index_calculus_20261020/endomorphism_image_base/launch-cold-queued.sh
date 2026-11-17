#!/usr/bin/env bash
# Retain quota refusals and wait for capacity; never change instance class,
# frozen source, cases, arm ordering, repetitions or measurement caps.
set -euo pipefail
image_bundle=${1:?frozen v1 base64 source}
image_receipts=${2:?new receipt directory}
mkdir "$image_receipts"
image_b64=$(cat "$image_bundle")
image_args=()
image_offset=0
for image_part in A B C D E F G H I J K L M N O P Q R S T; do
  image_args+=(--env "IMAGE_BUNDLE_$image_part=${image_b64:image_offset:100000}")
  image_offset=$((image_offset+100000))
done
for image_case in ${IMAGE_CASES:-337:1 337:2 241:2 37:0 37:1 73:2 97:0}; do
  image_p=${image_case%:*}; image_seed=${image_case#*:}
  image_label="p$image_p-s$image_seed"
  image_attempt=0
  while :; do
    image_stem="$image_receipts/$image_label-attempt$image_attempt"
    set +e
    bash taskq/deploy/ephemeral/ephemeral run --detach --spot --no-source --toolchain rust --type c7i.2xlarge --ttl 150m --name "twist-cold-$image_label" --env "COLD_PRIMES=$image_p" --env "COLD_SEEDS=$image_seed" "${image_args[@]}" -- "$(cat research/twist_index_calculus_20261020/endomorphism_image_base/run-cold.sh)" > "$image_stem.stdout" 2> "$image_stem.stderr"
    image_status=$?
    printf '%s\n' "$image_status" > "$image_stem.exit"
    set -e
    if test "$image_status" = 0; then cat "$image_stem.stdout"; break; fi
    if ! grep -q 'MaxSpotInstanceCountExceeded' "$image_stem.stderr"; then cat "$image_stem.stderr" >&2; exit "$image_status"; fi
    printf 'capacity refusal retained for %s attempt %s; waiting for existing runners\n' "$image_label" "$image_attempt"
    image_attempt=$((image_attempt+1))
    sleep 30
  done
done
