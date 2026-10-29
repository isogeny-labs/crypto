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
grep -Eq 'RTX PRO 6000' "$out/gpu.csv"
nvcc --version
c++ --version
rustc --version
rustup component add clippy
make test
make ARCH=sm_120 test-cuda
cargo clippy --locked --release --manifest-path Cargo.toml --target-dir build/cargo -- -D warnings
timeout 600 compute-sanitizer --tool memcheck --error-exitcode 86 build/explicit-relations-cuda --self-test
echo 'RTX PRO 6000 build, host reference, CUDA differential, reduction replay and memcheck passed'
