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
for image_pass in first aa; do
  export IMAGE_PASS_ROOT="$EPHEMERAL_OUT/candidate-$image_pass"
  mkdir -p "$IMAGE_PASS_ROOT"
  seq 0 188 | xargs -P4 -n1 bash -c 'image_shard=$1; set +e; prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" --image-collect-shard "$IMAGE_TAPE" "$IMAGE_SHA" "$image_shard" "$IMAGE_PASS_ROOT/shard$image_shard" > "$IMAGE_PASS_ROOT/shard$image_shard.stdout" 2> "$IMAGE_PASS_ROOT/shard$image_shard.stderr"; image_status=$?; printf "%s\n" "$image_status" > "$IMAGE_PASS_ROOT/shard$image_shard.exit"; exit 0' _
done
if diff -qr "$EPHEMERAL_OUT/candidate-first" "$EPHEMERAL_OUT/candidate-aa" > "$EPHEMERAL_OUT/aa.diff"; then
  printf 'all candidate shards attempted in both passes; output trees equal; scientific verdicts require the separate audit\n' > "$EPHEMERAL_OUT/complete.txt"
else
  printf 'candidate output trees differ; all partial records retained\n' > "$EPHEMERAL_OUT/incomplete.txt"
  exit 1
fi
