#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do image_var="IMAGE_BUNDLE_$image_part"; printf '%s' "${!image_var-}"; done | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do unset "IMAGE_BUNDLE_$image_part"; done
image_root="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020"
image_manifest="$image_root/endomorphism_image_base/native/Cargo.toml"
export CARGO_TARGET_DIR=/work/twist-image-target
cargo build --locked --release --manifest-path "$image_manifest" > "$EPHEMERAL_OUT/build.log" 2>&1
cargo test --locked --release --manifest-path "$image_manifest" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
export IMAGE_BINARY="$CARGO_TARGET_DIR/release/twist-endomorphism-image-study"
export IMAGE_TAPE="$image_root/quartic_twist_mixed_v1.ndjson"
export IMAGE_SHA=d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c
for image_mode in reference collect; do
  for image_pass in first aa; do
    export IMAGE_PASS_ROOT="$EPHEMERAL_OUT/$image_mode-$image_pass"
    export IMAGE_MODE="$image_mode"
    mkdir -p "$IMAGE_PASS_ROOT"
    for image_p in 37 73 97 157 229 241 337; do for image_seed in 0 1 2; do printf '%s %s\n' "$image_p" "$image_seed"; done; done |
      xargs -P4 -n2 bash -c '
        image_p=$1; image_seed=$2
        set +e
        prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" "--image-$IMAGE_MODE-arm" "$IMAGE_TAPE" "$IMAGE_SHA" "$image_p" "$image_seed" double-l0-active "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed" > "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed.stdout" 2> "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed.stderr"
        image_status=$?
        printf "%s\n" "$image_status" > "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed.exit"
        exit 0' _
  done
  diff -qr "$EPHEMERAL_OUT/$image_mode-first" "$EPHEMERAL_OUT/$image_mode-aa" > "$EPHEMERAL_OUT/$image_mode-aa.diff"
done
printf 'all 21 doubling-control instances attempted twice for the reference and native candidate; equal non-timing trees require separate correctness audit\n' > "$EPHEMERAL_OUT/complete.txt"
