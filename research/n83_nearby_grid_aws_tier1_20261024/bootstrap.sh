#!/bin/bash
# Cloud-init body. launch_spot.py prepends the four pinned N83_* variables.
set -euo pipefail

deadline_seconds="$(date -u -d "$N83_SHUTDOWN_DEADLINE_UTC" +%s)"
remaining_seconds="$((deadline_seconds - $(date -u +%s)))"
if (( remaining_seconds <= 0 )); then
  shutdown -h now
  exit 1
fi
shutdown -h +"$((remaining_seconds / 60))"
mkdir -p /opt/n83/logs /opt/n83/src
exec > >(tee -a /opt/n83/logs/bootstrap.log) 2>&1

upload() {
  local status=$?
  trap - EXIT
  if command -v aws >/dev/null 2>&1; then
    printf '%s\n' "$status" > /opt/n83/logs/bootstrap_exit_code.txt
    aws s3 sync /opt/n83/results "$N83_RESULTS_S3_URI/results/" --only-show-errors || true
    aws s3 sync /opt/n83/logs "$N83_RESULTS_S3_URI/logs/" --only-show-errors || true
  fi
  shutdown -h now || true
}
trap upload EXIT

export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq awscli zstd git curl build-essential pkg-config libssl-dev python3

aws s3 cp "$N83_BUNDLE_S3_URI" /opt/n83/source.tar.zst --only-show-errors
printf '%s  %s\n' "$N83_BUNDLE_SHA256" /opt/n83/source.tar.zst | sha256sum -c -
zstd -dc /opt/n83/source.tar.zst > /opt/n83/source.tar
observed_commit="$(git get-tar-commit-id < /opt/n83/source.tar)"
test "$observed_commit" = "$N83_COMMIT"
tar -xf /opt/n83/source.tar -C /opt/n83/src
rm /opt/n83/source.tar
cd /opt/n83/src

export AWS_EXPECTED_GIT_SHA="$N83_COMMIT"
export AWS_BUNDLE_SHA256="$N83_BUNDLE_SHA256"
export AWS_BUNDLE_S3_URI="$N83_BUNDLE_S3_URI"
export PYTHONDONTWRITEBYTECODE=1
export CARGO_TARGET_DIR=/opt/n83/target
export CARGO_BUILD_JOBS=4
export RUST_TEST_THREADS=8

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal
source /root/.cargo/env
rustup toolchain install 1.99.1 --profile minimal
rustc +1.99.1 --version | tee /opt/n83/logs/rustc.txt
uname -a | tee /opt/n83/logs/uname.txt

cargo +1.99.1 test --release --locked --lib -- --test-threads=8 \
  > /opt/n83/logs/libtest.log 2>&1
cargo +1.99.1 test --release --locked --example koblitz_orbit_dlp_fast \
  > /opt/n83/logs/ic_example_test.log 2>&1
cargo +1.99.1 test --release --locked --example n83_rho_optimization_eval \
  > /opt/n83/logs/rho_example_test.log 2>&1
cargo +1.99.1 build --release --locked \
  --example koblitz_orbit_dlp_fast --example n83_rho_optimization_eval \
  > /opt/n83/logs/build.log 2>&1

sync_output() {
  while sleep 300; do
    aws s3 sync /opt/n83/results "$N83_RESULTS_S3_URI/results/" \
      --only-show-errors || true
    aws s3 sync /opt/n83/logs "$N83_RESULTS_S3_URI/logs/" \
      --only-show-errors || true
  done
}
sync_output &
sync_pid=$!
timeout --signal=TERM --kill-after=30s 145m \
  python3 -m research.n83_nearby_grid_aws_tier1_20261024.run \
    --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
    --rho-binary "$CARGO_TARGET_DIR/release/examples/n83_rho_optimization_eval" \
    --output /opt/n83/results \
  > /opt/n83/logs/run.log 2>&1
kill "$sync_pid" 2>/dev/null || true
