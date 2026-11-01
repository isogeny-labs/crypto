#!/bin/bash
set -euo pipefail
root=/home/ec2-user/n83-shared-bits-20261024
source=/var/tmp/n83-shared-bits-20261024
build="$root/build-v4"
mkdir "$build"
exec 9>/tmp/crypto-bench.lock
printf "%s\n" WAITING_FOR_BENCHMARK_LOCK > "$root/validation-status-v4.txt"
wait_start=$SECONDS
if ! flock -w 14400 9; then
    printf "%s\n" BLOCKED_BENCHMARK_LOCK > "$root/validation-status-v4.txt"
    exit 75
fi
printf '%s\n' WAITING_FOR_OTHER_BENCHMARK > "$root/validation-status-v4.txt"
while [ -n "$(sudo -n docker ps -q)" ]; do
    if [ "$((SECONDS-wait_start))" -ge 14400 ]; then
        printf '%s\n' BLOCKED_HOST_STILL_BUSY > "$root/validation-status-v4.txt"
        exit 75
    fi
    sleep 60
done
printf '%s\n' VALIDATING > "$root/validation-status-v4.txt"
cd "$source"
test -z "$(git status --porcelain)"
git rev-parse HEAD > "$build/source-commit.txt"
git status --porcelain > "$build/source-status.txt"
rustc --version > "$build/rustc-version.txt"
cargo --version > "$build/cargo-version.txt"
uname -a > "$build/host.txt"
lscpu > "$build/cpu.txt"
export CARGO_TARGET_DIR="$root/target"
export CARGO_BUILD_JOBS=2
export RAYON_NUM_THREADS=1
export RUST_TEST_THREADS=1
mkdir -p "$root/tmp"
export TMPDIR="$root/tmp"
date -u +%FT%TZ > "$build/start.utc"
finish() {
    local code=$?
    if [ "$code" -ne 0 ]; then printf 'FAILED exit=%s\n' "$code" > "$root/validation-status-v4.txt"; fi
}
trap finish EXIT
run_gate() {
    label=$1
    shift
    printf '%q ' "$@" > "$build/$label-command.txt"
    printf '\n' >> "$build/$label-command.txt"
    result=0
    "$@" > "$build/$label.log" 2>&1 || result=$?
    printf '%s\n' "$result" > "$build/$label-exit.txt"
    test "$result" = 0
}
run_gate notification bash "$root/test-notification.sh" "$root/notification-tests-v4"
cp "$root/notification-tests-v4/wait-status" "$root/wait-status"
# One bounded event notification; do not hold the benchmark lock in the notifier.
timeout 14400 sh -c 'printf "READY\n" > /home/ec2-user/n83-shared-bits-20261024/notification-ready.fifo' 9>&- &
echo $! > "$root/notification-ready.pid"
run_gate lib cargo test --offline --release --lib -- --test-threads=1
run_gate tests cargo test --offline --release --example n83_d18_shared_bits --example n83_d18_shared_card_audit --example n83_prebuild_cost_m23 --example n83_small_m4_replay --example n83_full_m4_audit --example n83_xor_cache_audit --example n83_full_m4_card_check --example n83_prebuild_reuse_accounting
run_gate build cargo build --offline --release --example n83_d18_shared_bits --example n83_d18_shared_card_audit --example n83_prebuild_cost_m23 --example n83_small_m4_replay
binaries="$CARGO_TARGET_DIR/release/examples"
sha256sum "$binaries/n83_d18_shared_bits" "$binaries/n83_d18_shared_card_audit" "$binaries/n83_prebuild_cost_m23" "$binaries/n83_small_m4_replay" > "$build/binaries.sha256"
sh research/n83_d18_shared_bits_20261020/run-construction.sh "$binaries/n83_d18_shared_bits" research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv "$root/construction"
cmp "$root/construction/construction.json" research/n83_d18_shared_bits_20261020/construction.json
date -u +%FT%TZ > "$build/end.utc"
printf '%s\n' NATIVE_AND_BOUNDED_CONSTRUCTION_PASS > "$root/validation-status-v4.txt"

exec bash "$root/continue-linux-card-v4.sh"
