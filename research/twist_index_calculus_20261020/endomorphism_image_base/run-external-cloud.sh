#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do image_var="IMAGE_BUNDLE_$image_part"; printf '%s' "${!image_var-}"; done | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do unset "IMAGE_BUNDLE_$image_part"; done
image_source="$EPHEMERAL_OUT/source"
image_study="$image_source/research/twist_index_calculus_20261020/endomorphism_image_base"
export CARGO_TARGET_DIR=/work/twist-image-target
export CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=1
python3 "$image_source/tools/isolated_bench.py" busy -- cargo build --locked --release --manifest-path "$image_study/native/Cargo.toml" > "$EPHEMERAL_OUT/build.log" 2>&1
python3 "$image_source/tools/isolated_bench.py" busy -- cargo test --locked --release --manifest-path "$image_study/native/Cargo.toml" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
python3 "$image_source/tools/isolated_bench.py" busy -- dnf install -y docker > "$EPHEMERAL_OUT/docker-install.log" 2>&1
systemctl start docker
python3 "$image_source/tools/isolated_bench.py" busy -- docker build --progress=plain -f "$image_study/msolve.Dockerfile" -t twist-image-msolve:0.11.0 "$image_study" > "$EPHEMERAL_OUT/msolve-build.log" 2>&1
python3 "$image_source/tools/isolated_bench.py" busy -- docker build --progress=plain -f "$image_study/msolve-precise.Dockerfile" -t twist-image-msolve:0.11.0-precision "$image_study" > "$EPHEMERAL_OUT/msolve-precision-build.log" 2>&1
image_binary="$CARGO_TARGET_DIR/release/twist-endomorphism-image-study"
image_cpus=$(cat /sys/devices/system/cpu/cpu1/topology/thread_siblings_list)
docker image inspect twist-image-msolve:0.11.0-precision > "$EPHEMERAL_OUT/image-inspect.json"
docker run --rm twist-image-msolve:0.11.0-precision sh -c 'cat /opt/msolve-source-commit.txt; sha256sum /usr/local/bin/msolve /usr/local/lib/libmsolve.so.4.0.0; gcc --version | head -1; git -C /opt/msolve diff --stat' > "$EPHEMERAL_OUT/versions.txt"
sha256sum "$image_binary" "$image_study/msolve-precision.patch" > "$EPHEMERAL_OUT/native-and-patch.sha256"
for image_attempt in 0 1 2 3 4; do
  set +e
  python3 "$image_source/tools/isolated_bench.py" run --wait --cpus "$image_cpus" --out "$EPHEMERAL_OUT/attempt$image_attempt.isolation.json" -- \
    docker run --rm --cpuset-cpus "$image_cpus" --memory=8g --env RAYON_NUM_THREADS=1 \
    --mount "type=bind,src=$image_study/runs/external-inputs,dst=/inputs,readonly" \
    --mount "type=bind,src=$EPHEMERAL_OUT,dst=/output" \
    --mount "type=bind,src=$image_binary,dst=/native/twist-image,readonly" \
    twist-image-msolve:0.11.0-precision /native/twist-image --image-external-benchmark /inputs /usr/local/bin/msolve "/output/attempt$image_attempt" > "$EPHEMERAL_OUT/attempt$image_attempt.stdout" 2> "$EPHEMERAL_OUT/attempt$image_attempt.stderr"
  image_status=$?
  printf '%s\n' "$image_status" > "$EPHEMERAL_OUT/attempt$image_attempt.exit"
  set -e
  if test "$image_status" = 0 && jq -e '.run.contended == false and (.left_on_reserved.user_threads | length == 0)' "$EPHEMERAL_OUT/attempt$image_attempt.isolation.json" >/dev/null; then
    printf '%s\n' "$image_attempt" > "$EPHEMERAL_OUT/admitted-attempt.txt"
    printf 'all external basis and deterministic checks passed; resource receipt admitted\n' > "$EPHEMERAL_OUT/complete.txt"
    exit 0
  fi
  if test -d "$EPHEMERAL_OUT/attempt$image_attempt" && test "$image_status" != 0; then exit "$image_status"; fi
done
printf 'no admitted external attempt; all failures retained\n' > "$EPHEMERAL_OUT/incomplete.txt"
exit 1
