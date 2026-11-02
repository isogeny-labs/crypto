#!/usr/bin/env bash
# Thin capture. Invoke through the repository's shared busy reservation.
set -Eeuo pipefail
cert_out=${1:?fresh build output}
cert_target=${2:?separate Cargo target directory}
cert_root=research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate
test ! -e "$cert_out"
mkdir -p "$cert_out/metadata" "$cert_out/source"
cp "$0" "$cert_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$cert_out/metadata/started-utc.txt"
git rev-parse HEAD > "$cert_out/metadata/source-commit.txt"
rustc --version > "$cert_out/metadata/rustc.txt"
cargo --version > "$cert_out/metadata/cargo.txt"
export CARGO_TARGET_DIR="$cert_target" CARGO_BUILD_JOBS=1
cert_phase() {
  local cert_tag=$1 cert_rc
  shift
  printf '%q ' "$@" > "$cert_out/metadata/$cert_tag.command.txt"
  if "$@" > "$cert_out/metadata/$cert_tag.stdout" 2> "$cert_out/metadata/$cert_tag.stderr"; then cert_rc=0; else cert_rc=$?; fi
  printf '%s\n' "$cert_rc" > "$cert_out/metadata/$cert_tag.exit"
  return "$cert_rc"
}
cert_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$cert_root/native/Cargo.toml"
sha256sum "$cert_root/native/Cargo.toml" "$cert_root/native/Cargo.lock" "$cert_root/native/src/main.rs" "$cert_root/PROTOCOL.md" "$cert_root/proof/RepeatedFactor.lean" "$cert_root/proof/lean-toolchain" > "$cert_out/metadata/sources.sha256"
cp "$cert_root/native/Cargo.toml" "$cert_root/native/Cargo.lock" "$cert_out/source/"
cp "$cert_root/native/src/main.rs" "$cert_out/source/main.rs"
cp "$cert_root/PROTOCOL.md" "$cert_root/proof/RepeatedFactor.lean" "$cert_root/proof/lean-toolchain" "$cert_out/source/"
cert_phase tests timeout --signal=TERM --kill-after=10s 1200s cargo test --release --offline --locked --manifest-path "$cert_root/native/Cargo.toml"
cert_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$cert_root/native/Cargo.toml"
cp "$cert_target/release/twist-collision-certificate" "$cert_out/certificate"
chmod 500 "$cert_out/certificate"
sha256sum "$cert_out/certificate" > "$cert_out/metadata/certificate.sha256"
cert_lean=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
"$cert_lean" --version > "$cert_out/metadata/lean-version.txt"
cert_phase lean timeout --signal=TERM --kill-after=10s 1200s "$cert_lean" "$cert_root/proof/RepeatedFactor.lean"
sha256sum -c "$cert_out/metadata/sources.sha256" > "$cert_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$cert_out/metadata/finished-utc.txt"
printf '0\n' > "$cert_out/metadata/exit"
