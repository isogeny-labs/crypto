#!/usr/bin/env bash
# Thin capture; invoke through the shared busy reservation.
set -Eeuo pipefail
cert_out=${1:?fresh output}
cert_target=${2:?separate Cargo target}
cert_root=research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate
cert_source="$cert_root/identity_native"
test ! -e "$cert_out"
mkdir -p "$cert_out/metadata" "$cert_out/source"
cp "$0" "$cert_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$cert_out/metadata/started-utc.txt"
git rev-parse HEAD > "$cert_out/metadata/source-commit.txt"
export CARGO_TARGET_DIR="$cert_target" CARGO_BUILD_JOBS=1
cert_phase() {
  local cert_tag=$1 cert_rc
  shift
  printf '%q ' "$@" > "$cert_out/metadata/$cert_tag.command.txt"
  if "$@" > "$cert_out/metadata/$cert_tag.stdout" 2> "$cert_out/metadata/$cert_tag.stderr"; then cert_rc=0; else cert_rc=$?; fi
  printf '%s\n' "$cert_rc" > "$cert_out/metadata/$cert_tag.exit"
  return "$cert_rc"
}
cert_phase lockfile cargo generate-lockfile --offline --manifest-path "$cert_source/Cargo.toml"
sha256sum "$cert_source/Cargo.toml" "$cert_source/Cargo.lock" "$cert_source/src/main.rs" src/cryptanalysis/identity_certificate.rs "$cert_root/proof/RepeatedFactor.lean" > "$cert_out/metadata/sources.sha256"
cp "$cert_source/Cargo.toml" "$cert_source/Cargo.lock" "$cert_source/src/main.rs" "$cert_out/source/"
cp src/cryptanalysis/identity_certificate.rs "$cert_root/proof/RepeatedFactor.lean" "$cert_out/source/"
cert_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$cert_source/Cargo.toml"
cp "$cert_target/release/twist-collision-identity" "$cert_out/identity"
sha256sum "$cert_out/identity" > "$cert_out/metadata/identity.sha256"
cert_phase issue-and-check timeout --signal=TERM --kill-after=10s 1200s "$cert_out/identity" "$cert_root/proof/RepeatedFactor.lean" "$cert_out/artifacts"
sha256sum -c "$cert_out/metadata/sources.sha256" > "$cert_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$cert_out/metadata/finished-utc.txt"
printf '0\n' > "$cert_out/metadata/exit"
