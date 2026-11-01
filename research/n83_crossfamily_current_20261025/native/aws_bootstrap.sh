#!/usr/bin/env bash
# EC2 user data: build and execute the sealed native N83 panel on one Spot host.
set -uo pipefail

base=/var/tmp/n83-crossfamily
repository="$base/crypto"
bundle="$base/bundle"
mkdir -p "$base" "$repository" "$bundle"
export CARGO_TARGET_DIR="$base/cargo-target"
export CARGO_BUILD_JOBS=2
export RUST_TEST_THREADS=1

finish() {
  code=$?
  printf '%s %s\n' "$code" "$(date -u +%FT%TZ)" > "$base/bootstrap.exit"
  if [ -d "$base/results" ]; then
    tar -C "$base" -czf "$base/raw_results.tar.gz" results
    sha256sum "$base/raw_results.tar.gz" > "$base/raw_results.sha256"
    curl --fail --retry 5 --retry-delay 5 --silent --show-error \
      --upload-file "$base/raw_results.tar.gz" "$N83_RESULTS_PUT_URL" \
      > "$base/upload.stdout" 2> "$base/upload.stderr" || true
  fi
  tar -C "$base" --exclude=crypto --exclude=bundle --exclude=cargo-target \
    --exclude=results --exclude=raw_results.tar.gz -czf \
    /var/tmp/n83-crossfamily-status.tar.gz . || true
  if [ -f /var/tmp/n83-crossfamily-status.tar.gz ]; then
    curl --fail --retry 5 --retry-delay 5 --silent --show-error \
      --upload-file /var/tmp/n83-crossfamily-status.tar.gz \
      "$N83_STATUS_PUT_URL" > /dev/null 2>&1 || true
  fi
}
trap finish EXIT

systemd-run --unit=n83-hard-stop --on-active="${N83_CAP_SECONDS}s" \
  /sbin/shutdown -h now > "$base/deadline-guard.txt" 2>&1 || exit 75
systemctl list-timers n83-hard-stop.timer --no-pager \
  >> "$base/deadline-guard.txt" 2>&1 || exit 75

curl --fail --retry 5 --silent --show-error "$N83_SOURCE_GET_URL" \
  -o "$bundle/source.tar.gz" || exit 65
curl --fail --retry 5 --silent --show-error "$N83_MANIFEST_GET_URL" \
  -o "$bundle/manifest.json" || exit 65
printf '%s  %s\n' "$N83_SOURCE_SHA256" "$bundle/source.tar.gz" \
  | sha256sum --check - > "$base/source_checksum.txt" 2>&1 || exit 65
printf '%s  %s\n' "$N83_MANIFEST_SHA256" "$bundle/manifest.json" \
  | sha256sum --check - >> "$base/source_checksum.txt" 2>&1 || exit 65
tar -C "$repository" -xzf "$bundle/source.tar.gz" || exit 65
cd "$repository" || exit 65

if ! command -v gcc >/dev/null || ! command -v pkg-config >/dev/null || \
   ! pkg-config --exists openssl || ! command -v curl >/dev/null; then
  apt-get update > "$base/apt.log" 2>&1 || exit 68
  DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential pkg-config libssl-dev curl >> "$base/apt.log" 2>&1 || exit 68
fi
if ! command -v rustup >/dev/null; then
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
    | sh -s -- -y --profile minimal > "$base/rustup.log" 2>&1 || exit 68
fi
export PATH=/root/.cargo/bin:$PATH
rustup toolchain install 1.99.1 --profile minimal > "$base/toolchain.log" 2>&1 || exit 69
rustc +1.99.1 --version > "$base/rustc.txt" || exit 69
native_manifest=research/n83_crossfamily_current_20261025/native/Cargo.toml
cargo +1.99.1 build --release --locked --manifest-path "$native_manifest" \
  > "$base/native-build.log" 2>&1 || exit 69
native="$CARGO_TARGET_DIR/release/n83-crossfamily-current-native"
"$native" bundle-verify "$bundle" "$repository" > "$base/source_check.json" || exit 66
"$native" preflight "$repository" "$N83_STRATUM" > "$base/preflight.json" || exit 67
printf '%s\n' "$N83_ALLOCATION_JSON" > "$bundle/allocation-template.json"
"$native" bind-instance "$bundle/allocation-template.json" \
  "$bundle/allocation.json" > "$base/instance-bind.json" || exit 67

cargo +1.99.1 test --release --locked --example koblitz_orbit_dlp_fast \
  > "$base/ic-tests.log" 2>&1 || exit 70
cargo +1.99.1 test --release --locked --example n83_rho_optimization_eval \
  > "$base/rho-tests.log" 2>&1 || exit 71
cargo +1.99.1 build --release --locked \
  --example koblitz_orbit_dlp_fast --example n83_rho_optimization_eval \
  > "$base/build.log" 2>&1 || exit 72

cpus=$("$native" cpu-list) || exit 73
printf '%s\n' "$cpus" > "$base/cpu_affinity_ids.txt"
timeout --signal=TERM --kill-after=30s 18300 \
  taskset -c "$cpus" "$native" run "$repository" "$N83_STRATUM" \
  "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
  "$CARGO_TARGET_DIR/release/examples/n83_rho_optimization_eval" \
  "$bundle/allocation.json" "$bundle" "$base/results" \
  > "$base/producer.log" 2>&1
producer_code=$?
printf '%s\n' "$producer_code" > "$base/producer.exit"
if [ "$producer_code" -ne 0 ]; then exit 74; fi
"$native" replay "$repository" "$N83_STRATUM" "$base/results" \
  > "$base/replay.log" 2>&1 || exit 76
