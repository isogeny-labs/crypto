#!/usr/bin/env bash
# Correctness gates on a managed host; these durations are not speed evidence.
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
aws s3 cp --only-show-errors "$IMAGE_GATE_SOURCE_URI" /work/gate-source.tgz
printf '%s  /work/gate-source.tgz\n' "$IMAGE_GATE_SOURCE_SHA" | sha256sum -c - > "$EPHEMERAL_OUT/source-check.log"
tar -xzf /work/gate-source.tgz -C "$EPHEMERAL_OUT/source"
cd "$EPHEMERAL_OUT/source"
export CARGO_TARGET_DIR=/work/twist-root-gate-target
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
rustc --version > "$EPHEMERAL_OUT/rust-version.txt"
for image_gate in library examples native; do
  case "$image_gate" in
    library) image_command=(cargo test --locked --release --lib -- --test-threads=1) ;;
    examples) image_command=(cargo test --locked --release --example quartic_twist_s4_integer --example quartic_twist_s4_structure -- --test-threads=1) ;;
    native) image_command=(cargo test --locked --release --manifest-path research/twist_index_calculus_20261020/endomorphism_image_base/native/Cargo.toml -- --test-threads=1) ;;
  esac
  # Compilation is not a measured cryptanalytic arm. The first managed gate
  # exhausted LLVM's virtual address space before running any tests. Retain
  # that failure and build separately; cap each actual test invocation below.
  image_compile=("${image_command[@]:0:${#image_command[@]}-2}" --no-run)
  printf '%s\n' "${image_compile[*]}" > "$EPHEMERAL_OUT/$image_gate.compile.command.txt"
  set +e
  python3 tools/isolated_bench.py busy -- timeout 1200s "${image_compile[@]}" > "$EPHEMERAL_OUT/$image_gate.compile.log" 2>&1
  image_status=$?
  set -e
  printf '%s\n' "$image_status" > "$EPHEMERAL_OUT/$image_gate.compile.exit"
  if test "$image_status" != 0; then exit "$image_status"; fi
  printf '%s\n' "${image_command[*]}" > "$EPHEMERAL_OUT/$image_gate.command.txt"
  set +e
  python3 tools/isolated_bench.py busy -- prlimit --as=8589934592 -- timeout 1200s "${image_command[@]}" > "$EPHEMERAL_OUT/$image_gate.log" 2>&1
  image_status=$?
  set -e
  printf '%s\n' "$image_status" > "$EPHEMERAL_OUT/$image_gate.exit"
  if test "$image_status" != 0; then exit "$image_status"; fi
done
printf 'library, touched examples and native study test gates passed\n' > "$EPHEMERAL_OUT/complete.txt"
