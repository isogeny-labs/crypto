#!/usr/bin/env bash
# Native provenance check; one busy reservation covers compile and replay.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_study/native/pure_zero_verify/"{Cargo.toml,Cargo.lock,src/historical_match.rs} "$joint_repo/examples/quartic_twist_tagged_s5.rs" "$joint_repo/src/cryptanalysis/gaudry_quartic.rs" "$joint_study/../endomorphism_image_base/native/src/main.rs" > "$joint_out/metadata/sources.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR="$joint_study/../endomorphism_image_base/native/target"
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
joint_phase compile timeout --kill-after=10s 1200s cargo build --locked --offline --release --manifest-path "$joint_study/native/pure_zero_verify/Cargo.toml" --bin twist-historical-s5-match
cp "$CARGO_TARGET_DIR/release/twist-historical-s5-match" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
joint_phase match prlimit --as=8589934592 -- timeout --kill-after=10s 60s "$joint_out/runner" "$joint_repo" "$joint_out/verified"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Native source/record binding terminal; read the historical runtime-seal limitation. No collector, BSGS or timing comparison.\n' > "$joint_out/complete.txt"
