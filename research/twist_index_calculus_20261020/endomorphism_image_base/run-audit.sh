#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do image_var="IMAGE_BUNDLE_$image_part"; printf '%s' "${!image_var-}"; done | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do unset "IMAGE_BUNDLE_$image_part"; done
image_root="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020"
export CARGO_TARGET_DIR=/work/twist-image-target
cargo build --locked --release --manifest-path "$image_root/endomorphism_image_base/native/Cargo.toml" > "$EPHEMERAL_OUT/build.log" 2>&1
cargo test --locked --release --manifest-path "$image_root/endomorphism_image_base/native/Cargo.toml" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
export IMAGE_BINARY="$CARGO_TARGET_DIR/release/twist-endomorphism-image-study"
export IMAGE_TAPE="$image_root/quartic_twist_mixed_v1.ndjson"
export IMAGE_SHA=d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c
export IMAGE_AUDIT_INPUT="$EPHEMERAL_OUT/input"
export IMAGE_AUDIT_OUTPUT="$EPHEMERAL_OUT/audit"
aws s3 sync --only-show-errors "$IMAGE_REFERENCE_PREFIX/out/reference-first/" "$IMAGE_AUDIT_INPUT/reference/"
aws s3 sync --only-show-errors "$IMAGE_CANDIDATE_PREFIX/out/candidate-first/" "$IMAGE_AUDIT_INPUT/candidate/"
aws s3 sync --only-show-errors "$IMAGE_DOUBLE_PREFIX/out/reference-first/" "$IMAGE_AUDIT_INPUT/double-reference/"
aws s3 sync --only-show-errors "$IMAGE_DOUBLE_PREFIX/out/collect-first/" "$IMAGE_AUDIT_INPUT/double-candidate/"
mkdir -p "$IMAGE_AUDIT_OUTPUT"
seq 0 188 | xargs -P4 -n1 bash -c '
 image_shard=$1
 image_primes=(37 73 97 157 229 241 337)
 image_arms=(original psi-control phi-l2 phi-l0 union-l2 union-all union-l0 phi-l0-active l0-peel)
 image_p=${image_primes[image_shard/27]}; image_seed=$((image_shard/9%3)); image_arm=${image_arms[image_shard%9]}
 set +e
 prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" --image-audit "$IMAGE_TAPE" "$IMAGE_SHA" "$IMAGE_AUDIT_INPUT/reference/p$image_p-seed$image_seed/$image_arm.json" "$IMAGE_AUDIT_INPUT/candidate/shard$image_shard" "$IMAGE_AUDIT_OUTPUT/shard$image_shard" > "$IMAGE_AUDIT_OUTPUT/shard$image_shard.stdout" 2> "$IMAGE_AUDIT_OUTPUT/shard$image_shard.stderr"
 image_status=$?; printf "%s\n" "$image_status" > "$IMAGE_AUDIT_OUTPUT/shard$image_shard.exit"
 exit 0' _
for image_p in 37 73 97 157 229 241 337; do for image_seed in 0 1 2; do printf '%s %s\n' "$image_p" "$image_seed"; done; done |
 xargs -P4 -n2 bash -c '
 image_p=$1; image_seed=$2
 set +e
 prlimit --as=8589934592 -- timeout 1200s "$IMAGE_BINARY" --image-audit "$IMAGE_TAPE" "$IMAGE_SHA" "$IMAGE_AUDIT_INPUT/double-reference/p$image_p-seed$image_seed/double-l0-active.json" "$IMAGE_AUDIT_INPUT/double-candidate/p$image_p-seed$image_seed" "$IMAGE_AUDIT_OUTPUT/double-p$image_p-seed$image_seed" > "$IMAGE_AUDIT_OUTPUT/double-p$image_p-seed$image_seed.stdout" 2> "$IMAGE_AUDIT_OUTPUT/double-p$image_p-seed$image_seed.stderr"
 image_status=$?; printf "%s\n" "$image_status" > "$IMAGE_AUDIT_OUTPUT/double-p$image_p-seed$image_seed.exit"
 exit 0' _
printf 'all 210 separate set/rank/recovery audits attempted; read every exit and audit verdict\n' > "$EPHEMERAL_OUT/complete.txt"
