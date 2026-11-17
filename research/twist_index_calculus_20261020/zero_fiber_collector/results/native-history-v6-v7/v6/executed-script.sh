#!/usr/bin/env bash
# Thin build/test orchestration. Invoke through tools/isolated_bench.py busy.
set -Eeuo pipefail
trap 'printf "native gate stopped at line %s with exit %s\n" "$LINENO" "$?" >&2' ERR
zero_out=${1:?fresh gate output directory}
zero_target=${2:?EFS Cargo target directory}
zero_study=research/twist_index_calculus_20261020/zero_fiber_collector
zero_native=research/twist_index_calculus_20261020/endomorphism_image_base/native
test ! -e "$zero_out"
mkdir -p "$zero_out/metadata" "$zero_out/bin"
cp "$0" "$zero_out/metadata/executed-script.sh"
sha256sum "$zero_out/metadata/executed-script.sh" > "$zero_out/metadata/executed-script.sha256"
date -u +%FT%TZ > "$zero_out/metadata/started-utc.txt"
git rev-parse HEAD > "$zero_out/metadata/source-commit.txt"
rustc --version > "$zero_out/metadata/rustc.txt"
cargo --version > "$zero_out/metadata/cargo.txt"
{
  if command -v rg >/dev/null 2>&1; then
    rg --files -uuu "$zero_native/src" "$zero_study/native" research/twist_index_calculus_20261020/joint_image_elimination/native -g '*.rs' -g '!**/target/**'
  else
    find "$zero_native/src" "$zero_study/native" research/twist_index_calculus_20261020/joint_image_elimination/native -type f -name '*.rs' -not -path '*/target/*'
  fi
  printf '%s\n' "$zero_native/Cargo.toml" "$zero_native/Cargo.lock" "$zero_study/fixtures/p37-seed0-public.json" \
    src/cryptanalysis/f4_fp.rs src/cryptanalysis/fx_hash.rs src/cryptanalysis/identity_certificate.rs \
    docs/identity-certificates/schoenhage-10mult.json \
    examples/quartic_twist_s4_integer.rs examples/quartic_twist_s4_structure.rs tools/isolated_bench.py
} | LC_ALL=C sort -u | xargs sha256sum > "$zero_out/metadata/sources.sha256"
export CARGO_TARGET_DIR="$zero_target" CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
zero_phase() {
  local zero_id=$1 zero_rc
  shift
  printf '%q ' "$@" > "$zero_out/metadata/$zero_id.command.txt"
  printf 'Starting native gate phase %s\n' "$zero_id" >&2
  if "$@" > "$zero_out/metadata/$zero_id.stdout" 2> "$zero_out/metadata/$zero_id.stderr"; then
    zero_rc=0
  else
    zero_rc=$?
  fi
  printf '%s\n' "$zero_rc" > "$zero_out/metadata/$zero_id.exit"
  printf 'Native gate phase %s exited %s\n' "$zero_id" "$zero_rc" >&2
  return "$zero_rc"
}
zero_phase compile-tests timeout --signal=TERM --kill-after=10s 1200s cargo test --release --locked --manifest-path "$zero_native/Cargo.toml" --no-run
zero_phase build-runner timeout --signal=TERM --kill-after=10s 1200s cargo build --release --locked --manifest-path "$zero_native/Cargo.toml"
zero_phase native-tests prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s cargo test --release --locked --manifest-path "$zero_native/Cargo.toml" -- --test-threads=1
cp "$zero_target/release/twist-endomorphism-image-study" "$zero_out/bin/study"
chmod a-w "$zero_out/bin/study"
sha256sum "$zero_out/bin/study" > "$zero_out/metadata/runner.sha256"
sha256sum -c "$zero_out/metadata/sources.sha256" > "$zero_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$zero_out/metadata/finished-utc.txt"
printf '0\n' > "$zero_out/metadata/exit"
