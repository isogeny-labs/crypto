#!/usr/bin/env bash
# Serial integration checks after merging main; no scientific timing claim.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
if test "${2:-}" = --execute; then
  export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
  ulimit -c 0
  joint_phase() {
    local joint_id=$1 joint_rc
    shift
    printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
    set +e
    "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
    joint_rc=$?
    set -e
    printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
    return "$joint_rc"
  }
  joint_phase library-compile timeout --kill-after=10s 1200s cargo test --locked --offline --release --lib --no-run --message-format=json
  joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null and (.target.kind|index("lib") != null)) | .executable' "$joint_out/metadata/library-compile.stdout" | tail -1)
  test -x "$joint_test"
  joint_phase library-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
  joint_phase examples-compile timeout --kill-after=10s 1200s cargo test --locked --offline --release --example quartic_twist_s4_integer --example quartic_twist_s4_structure --no-run --message-format=json
  for joint_example in quartic_twist_s4_integer quartic_twist_s4_structure; do
    joint_binary=$(jq -r --arg name "$joint_example" 'select(.reason=="compiler-artifact" and .profile.test==true and .target.name==$name and .executable!=null) | .executable' "$joint_out/metadata/examples-compile.stdout" | tail -1)
    test -x "$joint_binary"
    joint_phase "$joint_example" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_binary" --test-threads=1
  done
  joint_phase curve-id-selftest timeout --kill-after=10s 1200s python3 scripts/curve_id.py selftest
  joint_phase curve-names timeout --kill-after=10s 1200s python3 scripts/check_curve_names.py
  joint_phase validation-gate-tests timeout --kill-after=10s 1200s python3 scripts/test_validation_gate.py
  joint_phase validation-gate timeout --kill-after=10s 1200s python3 scripts/validation_gate.py check --base origin/main
  joint_phase large-files timeout --kill-after=10s 1200s bash scripts/check-large-files.sh origin/main
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
  printf 'Integration checks terminal; each exit and test summary remains authoritative. No solver timing or scientific price.\n' > "$joint_out/complete.txt"
  exit 0
fi
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
git diff -- src Cargo.toml Cargo.lock examples/quartic_twist_s4_integer.rs examples/quartic_twist_s4_structure.rs > "$joint_out/metadata/touched-source-diff.patch"
git ls-files -z -- src Cargo.toml Cargo.lock rust-toolchain.toml .cargo/config.toml examples/quartic_twist_s4_integer.rs examples/quartic_twist_s4_structure.rs | xargs -0 sha256sum > "$joint_out/metadata/sources.sha256"
rustc --version > "$joint_out/metadata/rustc.txt"
cargo --version > "$joint_out/metadata/cargo.txt"
uname -a > "$joint_out/metadata/host.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
# Let the current scientific replay and its already owned dependent census
# finish before reserving the machine for the longer library suite.
while systemctl is-active --quiet twist-joint-fp-pruning-audit-v1.service || systemctl is-active --quiet twist-joint-five-zero-v1.service; do sleep 10; done
python3 "$joint_repo/tools/isolated_bench.py" busy -- bash "$joint_out/metadata/executed-script.sh" "$joint_out" --execute
