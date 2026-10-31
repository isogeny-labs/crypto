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
aws s3 sync --only-show-errors "$IMAGE_PREVIOUS_PREFIX/out/candidate-first/" "$EPHEMERAL_OUT/candidate-first/" > "$EPHEMERAL_OUT/checkpoint-fetch.log" 2>&1
export IMAGE_PRIOR_PARTIALS="$EPHEMERAL_OUT/prior-partials"
for image_pass in first aa; do
  export IMAGE_PASS_ROOT="$EPHEMERAL_OUT/candidate-$image_pass"
  export IMAGE_PASS_LABEL="$image_pass"
  mkdir -p "$IMAGE_PASS_ROOT"
  seq 0 188 | xargs -P4 -n1 bash -c '
    image_shard=$1
    image_dest="$IMAGE_PASS_ROOT/shard$image_shard"
    if test -s "$IMAGE_PASS_ROOT/shard$image_shard.exit" && test "$(cat "$IMAGE_PASS_ROOT/shard$image_shard.exit")" = 0 && test -s "$image_dest/summary.json"; then exit 0; fi
    if test -e "$image_dest"; then
      image_prior="$IMAGE_PRIOR_PARTIALS/$IMAGE_PASS_LABEL/shard$image_shard"
      mkdir -p "$image_prior"
      mv "$image_dest" "$image_prior/partial"
      for image_suffix in exit stdout stderr; do if test -f "$IMAGE_PASS_ROOT/shard$image_shard.$image_suffix"; then mv "$IMAGE_PASS_ROOT/shard$image_shard.$image_suffix" "$image_prior/"; fi; done
    fi
    set +e
    prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" --image-collect-shard "$IMAGE_TAPE" "$IMAGE_SHA" "$image_shard" "$image_dest" > "$IMAGE_PASS_ROOT/shard$image_shard.stdout" 2> "$IMAGE_PASS_ROOT/shard$image_shard.stderr"
    image_status=$?
    printf "%s\n" "$image_status" > "$IMAGE_PASS_ROOT/shard$image_shard.exit"
    exit 0' _
done
if diff -qr "$EPHEMERAL_OUT/candidate-first" "$EPHEMERAL_OUT/candidate-aa" > "$EPHEMERAL_OUT/aa.diff"; then
  printf 'all nine-arm shards attempted; complete non-timing output trees equal; independent audit remains required\n' > "$EPHEMERAL_OUT/complete.txt"
else
  printf 'non-timing output trees differ; all partial records retained\n' > "$EPHEMERAL_OUT/incomplete.txt"
  exit 1
fi
