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
"$IMAGE_BINARY" --image-identity "$EPHEMERAL_OUT/image-identity" > "$EPHEMERAL_OUT/identity.stdout" 2> "$EPHEMERAL_OUT/identity.stderr"
for image_pass in first aa; do
  export IMAGE_PASS_ROOT="$EPHEMERAL_OUT/reference-$image_pass"
  mkdir -p "$IMAGE_PASS_ROOT"
  for image_p in 37 73 97 157 229 241 337; do for image_seed in 0 1 2; do printf '%s %s\n' "$image_p" "$image_seed"; done; done |
    xargs -P4 -n2 bash -c 'image_p=$1; image_seed=$2; prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" --image-census "$IMAGE_TAPE" "$IMAGE_SHA" "$image_p" "$image_seed" "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed" > "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed.stdout" 2> "$IMAGE_PASS_ROOT/p$image_p-seed$image_seed.stderr"' _
done
diff -qr "$EPHEMERAL_OUT/reference-first" "$EPHEMERAL_OUT/reference-aa" > "$EPHEMERAL_OUT/aa.diff"
printf 'all 21 full-stream reference cases and nine arms completed twice; outputs equal\n' > "$EPHEMERAL_OUT/complete.txt"
