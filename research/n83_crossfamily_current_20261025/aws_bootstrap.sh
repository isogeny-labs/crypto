#!/usr/bin/env bash
# EC2 user data. The launcher supplies N83_* variables and presigned URLs.
set -uo pipefail

base=/var/tmp/n83-crossfamily
repository="$base/crypto"
bundle="$base/bundle"
mkdir -p "$base" "$repository" "$bundle"
export PYTHONDONTWRITEBYTECODE=1
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

# This timer survives cloud-init. The EC2 launch sets instance-initiated
# shutdown behavior to terminate. The local controller also terminates both
# instance IDs at its independent absolute deadline.
systemd-run --unit=n83-hard-stop --on-active="${N83_CAP_SECONDS}s" \
  /sbin/shutdown -h now > "$base/deadline-guard.txt" 2>&1 || exit 75
systemctl list-timers n83-hard-stop.timer --no-pager \
  >> "$base/deadline-guard.txt" 2>&1 || exit 75

curl --fail --retry 5 --silent --show-error "$N83_SOURCE_GET_URL" \
  -o "$bundle/source.tar.gz" || exit 65
curl --fail --retry 5 --silent --show-error "$N83_MANIFEST_GET_URL" \
  -o "$bundle/manifest.json" || exit 65
tar -C "$repository" -xzf "$bundle/source.tar.gz" || exit 65
cd "$repository" || exit 65
python3 -m research.n83_crossfamily_current_20261025.source_bundle \
  --verify "$bundle" --root "$repository" > "$base/source_check.json" || exit 66
python3 -m research.n83_crossfamily_current_20261025.run \
  --preflight --stratum "$N83_STRATUM" > "$base/preflight.json" || exit 67

python3 - "$N83_ALLOCATION_JSON" "$bundle/allocation.json" <<'PY'
import json
import sys
from research.n83_crossfamily_current_20261025 import aws_runner

allocation = json.loads(sys.argv[1])
identity = aws_runner.imds_identity()
document = identity['document']
allocation.update({'instance_id': document['instanceId'],
                   'state': 'running'})
aws_runner.check_instance(allocation, identity)
with open(sys.argv[2], 'x') as stream:
    json.dump(allocation, stream, sort_keys=True, indent=2)
    stream.write('\n')
PY
test "$?" -eq 0 || exit 67

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
cargo +1.99.1 test --release --locked --example koblitz_orbit_dlp_fast \
  > "$base/ic_tests.log" 2>&1 || exit 70
cargo +1.99.1 test --release --locked --example n83_rho_optimization_eval \
  > "$base/rho_tests.log" 2>&1 || exit 71
cargo +1.99.1 build --release --locked \
  --example koblitz_orbit_dlp_fast --example n83_rho_optimization_eval \
  > "$base/build.log" 2>&1 || exit 72

cpus=$(python3 - <<'PY'
import os
ids = sorted(os.sched_getaffinity(0))[:4]
if len(ids) != 4:
    raise ValueError('four available CPU IDs required')
print(','.join(map(str, ids)))
PY
) || exit 73
printf '%s\n' "$cpus" > "$base/cpu_affinity_ids.txt"
timeout --signal=TERM --kill-after=30s 18300 \
  taskset -c "$cpus" \
  python3 -m research.n83_crossfamily_current_20261025.aws_runner \
    --stratum "$N83_STRATUM" \
    --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
    --rho-binary "$CARGO_TARGET_DIR/release/examples/n83_rho_optimization_eval" \
    --allocation "$bundle/allocation.json" --source-bundle "$bundle" \
    --output "$base/results" > "$base/producer.log" 2>&1
producer_code=$?
printf '%s\n' "$producer_code" > "$base/producer.exit"
if [ -d "$base/results" ]; then
  python3 -m research.n83_crossfamily_current_20261025.verify \
    "$base/results" --stratum "$N83_STRATUM" \
    > "$base/replay.log" 2>&1
  replay_code=$?
  printf '%s\n' "$replay_code" > "$base/replay.exit"
else
  replay_code=98
fi
if [ "$producer_code" -ne 0 ] || [ "$replay_code" -gt 2 ]; then
  exit 74
fi
