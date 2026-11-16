#!/usr/bin/env bash
# Native polynomial replay only; the separate Lean gate checks actual point maps.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
out=${1:?usage: run-group-map.sh FRESH_OUTPUT_DIR}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
study=research/quadratic_twist_evidence_20261021
crate=$study/native/group-map-cert
frozen=$study/validation/group-map-20261023/certificates
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/start.utc"
rustc --version > "$out/toolchain.txt"
cargo --version >> "$out/toolchain.txt"
sha256sum "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/lean/TwistGroupMap.lean" "$study/lean/TwistEigenspace.lean" \
  src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs > "$out/INPUTS.sha256"
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/build.log" 2>&1
binary=$crate/target/release/quadratic-twist-group-map-cert
"$binary" issue "$root" "$out/certificates" > "$out/issue.log"
"$binary" check "$root" "$out/certificates" > "$out/check.log"
diff -ru "$frozen" "$out/certificates" > "$out/frozen-diff.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/certificates" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/end.utc"
printf '%s\n' 'PASS: source-bound group-map polynomial replay, exact frozen bytes, false identities/source bindings and backend tests'
