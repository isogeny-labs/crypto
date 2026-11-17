#!/usr/bin/env bash
# Thin capture, invoked through the common busy reservation.
set -Eeuo pipefail
export PATH=/home/ec2-user/.cargo/bin:$PATH
probe_out=${1:?fresh build output}
probe_target=${2:?separate Cargo target}
probe_root=research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe
test ! -e "$probe_out"
mkdir -p "$probe_out/metadata" "$probe_out/source/src"
cp "$0" "$probe_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$probe_out/metadata/started-utc.txt"
git rev-parse HEAD > "$probe_out/metadata/source-commit.txt"
git status --short -- "$probe_root" > "$probe_out/metadata/source-dirty-state.txt"
rustc --version > "$probe_out/metadata/rustc.txt"
cargo --version > "$probe_out/metadata/cargo.txt"
uname -a > "$probe_out/metadata/host.txt"
lscpu > "$probe_out/metadata/lscpu.txt"
export CARGO_TARGET_DIR="$probe_target" CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
probe_phase() {
  local probe_tag=$1 probe_rc
  shift
  printf '%q ' "$@" > "$probe_out/metadata/$probe_tag.command.txt"
  if "$@" > "$probe_out/metadata/$probe_tag.stdout" 2> "$probe_out/metadata/$probe_tag.stderr"; then probe_rc=0; else probe_rc=$?; fi
  printf '%s\n' "$probe_rc" > "$probe_out/metadata/$probe_tag.exit"
  return "$probe_rc"
}
probe_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$probe_root/native/Cargo.toml"
sha256sum "$probe_root/native/Cargo.toml" "$probe_root/native/Cargo.lock" "$probe_root/native/src/"*.rs "$probe_root/PROTOCOL.md" > "$probe_out/metadata/sources.sha256"
cp "$probe_root/native/Cargo.toml" "$probe_root/native/Cargo.lock" "$probe_root/PROTOCOL.md" "$probe_out/source/"
cp "$probe_root/native/src/"*.rs "$probe_out/source/src/"
probe_phase fmt timeout --signal=TERM --kill-after=10s 1200s cargo fmt --manifest-path "$probe_root/native/Cargo.toml" --check
probe_phase tests timeout --signal=TERM --kill-after=10s 1200s cargo test --release --offline --locked --manifest-path "$probe_root/native/Cargo.toml"
probe_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$probe_root/native/Cargo.toml"
cp "$probe_target/release/twist-degree-closure-probe" "$probe_out/probe"
chmod 500 "$probe_out/probe"
sha256sum "$probe_out/probe" > "$probe_out/metadata/probe.sha256"
sha256sum -c "$probe_out/metadata/sources.sha256" > "$probe_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$probe_out/metadata/finished-utc.txt"
printf '0\n' > "$probe_out/metadata/exit"
