#!/usr/bin/env bash
# Bounded correctness validation; no comparative timing or speed claims.
set -euo pipefail
cd "$(dirname "$0")"
out=${EPHEMERAL_OUT:?Run through the managed ephemeral runner}
export PATH="/usr/local/cuda/bin:$PATH"
exec > >(tee "$out/validation.log") 2>&1
finish() {
    rc=$?
    printf '%s\n' "$rc" > "$out/validation.exit"
    if [ -d build ]; then
        find build -maxdepth 1 -type f -executable -exec sha256sum {} + > "$out/binaries.sha256"
    fi
}
trap finish EXIT
git rev-parse HEAD
sha256sum service.cpp host.hpp curve.cuh device.cuh paper_driver.rs Cargo.toml Cargo.lock ../../src/cryptanalysis/hashed_ecdl.rs > "$out/source.sha256"
nvidia-smi --query-gpu=name,uuid,driver_version,memory.total,compute_cap --format=csv | tee "$out/gpu.csv"
expected_gpu=${ER_EXPECT_GPU:-RTX PRO 6000}
case "$expected_gpu" in
    'RTX PRO 6000'|'RTX 5090') required_arch=sm_120; expected_cc=12.0 ;;
    'RTX 4090') required_arch=sm_89; expected_cc=8.9 ;;
    *) echo "unsupported validation GPU: $expected_gpu" >&2; exit 2 ;;
esac
test "$(wc -l < "$out/gpu.csv")" = 2
grep -Fq -- "$expected_gpu" "$out/gpu.csv"
arch=${ER_CUDA_ARCH:-$required_arch}
test "$arch" = "$required_arch"
awk -F, -v expected="$expected_cc" 'NR == 2 {gsub(/[[:space:]]/, "", $5); if ($5 != expected) exit 1}' "$out/gpu.csv"
nvcc --version
c++ --version
rustc --version
rustup component add clippy
make test
make ARCH="$arch" test-cuda
cargo clippy --locked --release --manifest-path Cargo.toml --target-dir build/cargo -- -D warnings
timeout 600 compute-sanitizer --tool memcheck --error-exitcode 86 build/explicit-relations-cuda --self-test
echo "$expected_gpu $arch build, host reference, CUDA differential, reduction replay and memcheck passed"
