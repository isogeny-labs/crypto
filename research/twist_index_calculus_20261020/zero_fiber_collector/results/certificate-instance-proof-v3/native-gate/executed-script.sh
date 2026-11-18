#!/usr/bin/env bash
set -Eeuo pipefail
export PATH=/home/ec2-user/.cargo/bin:$PATH
instance_out=${1:?fresh gate output}
instance_target=${2:?separate Cargo target}
instance_root=research/twist_index_calculus_20261020/zero_fiber_collector/certificate_instance_proof
instance_prefix=research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/proof/RepeatedFactor.lean
test ! -e "$instance_out"
mkdir -p "$instance_out/metadata" "$instance_out/source"
cp "$0" "$instance_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$instance_out/metadata/started-utc.txt"
git rev-parse HEAD > "$instance_out/metadata/source-commit.txt"
git status --short -- "$instance_root" > "$instance_out/metadata/source-dirty-state.txt"
rustc --version > "$instance_out/metadata/rustc.txt"
cargo --version > "$instance_out/metadata/cargo.txt"
export CARGO_TARGET_DIR="$instance_target" CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
instance_phase() {
  local instance_tag=$1 instance_rc
  shift
  printf '%q ' "$@" > "$instance_out/metadata/$instance_tag.command.txt"
  if "$@" > "$instance_out/metadata/$instance_tag.stdout" 2> "$instance_out/metadata/$instance_tag.stderr"; then instance_rc=0; else instance_rc=$?; fi
  printf '%s\n' "$instance_rc" > "$instance_out/metadata/$instance_tag.exit"
  return "$instance_rc"
}
instance_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$instance_root/native/Cargo.toml"
sha256sum "$instance_root/native/Cargo.toml" "$instance_root/native/Cargo.lock" "$instance_root/native/src/main.rs" "$instance_root/PROTOCOL.md" "$instance_prefix" src/cryptanalysis/identity_certificate.rs > "$instance_out/metadata/sources.sha256"
cp "$instance_root/native/Cargo.toml" "$instance_root/native/Cargo.lock" "$instance_root/native/src/main.rs" "$instance_root/PROTOCOL.md" "$instance_prefix" src/cryptanalysis/identity_certificate.rs "$instance_out/source/"
instance_phase fmt cargo fmt --manifest-path "$instance_root/native/Cargo.toml" --check
instance_phase tests timeout --signal=TERM --kill-after=10s 1200s cargo test --release --offline --locked --manifest-path "$instance_root/native/Cargo.toml" -- --test-threads=1
instance_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$instance_root/native/Cargo.toml"
cp "$instance_target/release/twist-certificate-instance-proof" "$instance_out/verifier"
chmod 500 "$instance_out/verifier"
sha256sum "$instance_out/verifier" > "$instance_out/metadata/verifier.sha256"
sha256sum -c "$instance_out/metadata/sources.sha256" > "$instance_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$instance_out/metadata/finished-utc.txt"
printf '0\n' > "$instance_out/metadata/exit"
