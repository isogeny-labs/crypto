#!/usr/bin/env bash
set -Eeuo pipefail
export PATH=/home/ec2-user/.cargo/bin:$PATH
fp_out=${1:?fresh gate directory}
fp_target=${2:?fresh separate Cargo target directory}
fp_root=research/twist_index_calculus_20261020/zero_fiber_collector/fp_residual_audit
fp_helper=research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe/run-phase.sh
test ! -e "$fp_out"
test ! -e "$fp_target"
mkdir -p "$fp_out/metadata" "$fp_out/source"
cp "$0" "$fp_out/metadata/executed-script.sh"
cp "$fp_helper" "$fp_out/metadata/run-phase.sh"
fp_finish() { local fp_code=$?; date -u +%FT%TZ > "$fp_out/metadata/finished-utc.txt"; printf '%s\n' "$fp_code" > "$fp_out/metadata/exit"; }
trap fp_finish EXIT
date -u +%FT%TZ > "$fp_out/metadata/started-utc.txt"
git rev-parse HEAD > "$fp_out/metadata/source-commit.txt"
git status --short -- "$fp_root" > "$fp_out/metadata/source-dirty-state.txt"
rustc --version > "$fp_out/metadata/rustc.txt"
cargo --version > "$fp_out/metadata/cargo.txt"
export CARGO_TARGET_DIR="$fp_target" CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
fp_phase() {
  local fp_tag=$1 fp_rc
  shift
  printf '%q ' "$@" > "$fp_out/metadata/$fp_tag.command.txt"
  if "$@" > "$fp_out/metadata/$fp_tag.stdout" 2> "$fp_out/metadata/$fp_tag.stderr"; then fp_rc=0; else fp_rc=$?; fi
  printf '%s\n' "$fp_rc" > "$fp_out/metadata/$fp_tag.exit"
  return "$fp_rc"
}
fp_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$fp_root/native/Cargo.toml"
sha256sum "$fp_root/native/Cargo.toml" "$fp_root/native/Cargo.lock" "$fp_root/native/src/main.rs" "$fp_root/PROTOCOL.md" "$fp_root/build-native.sh" "$fp_root/run.sh" "$fp_helper" > "$fp_out/metadata/sources.sha256"
cp "$fp_root/native/Cargo.toml" "$fp_root/native/Cargo.lock" "$fp_root/native/src/main.rs" "$fp_root/PROTOCOL.md" "$fp_root/build-native.sh" "$fp_root/run.sh" "$fp_out/source/"
fp_phase fmt cargo fmt --manifest-path "$fp_root/native/Cargo.toml" --check
fp_phase test-build timeout --signal=TERM --kill-after=10s 1200s cargo test --release --offline --locked --manifest-path "$fp_root/native/Cargo.toml" --no-run
mapfile -t fp_tests < <(find "$fp_target/release/deps" -maxdepth 1 -type f -executable -name 'twist_fp_residual_audit-*')
test "${#fp_tests[@]}" = 1
fp_phase tests python3 tools/isolated_bench.py busy -- bash "$fp_out/metadata/run-phase.sh" "$fp_out/metadata/tests-reserved" "${fp_tests[0]}" --test-threads=1
fp_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$fp_root/native/Cargo.toml"
cp "$fp_target/release/twist-fp-residual-audit" "$fp_out/auditor"
chmod 500 "$fp_out/auditor"
sha256sum "$fp_out/auditor" > "$fp_out/metadata/auditor.sha256"
sha256sum -c "$fp_out/metadata/sources.sha256" > "$fp_out/metadata/sources-unchanged.txt"
