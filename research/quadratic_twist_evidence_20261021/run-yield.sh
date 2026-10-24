#!/usr/bin/env bash
# Polynomial replay only. The separate Lean gate proves counts, rank and inequalities.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
out=${1:?usage: run-yield.sh FRESH_OUTPUT_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
study=research/quadratic_twist_evidence_20261021
crate=$study/native/yield-cert
frozen=$study/validation/yield-20261024/certificates
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/start.utc"
rustc --version > "$out/toolchain.txt"
cargo --version >> "$out/toolchain.txt"
sha256sum "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/lean/RelationCost.lean" src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs > "$out/INPUTS.sha256"
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/build.log" 2>&1
binary=$crate/target/release/quadratic-twist-yield-cert
"$binary" issue "$root" "$out/certificates" > "$out/issue.log" 2>&1
"$binary" check "$root" "$out/certificates" > "$out/check.log" 2>&1
diff -ru "$frozen" "$out/certificates" > "$out/frozen-diff.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/certificates" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/end.utc"
printf '%s\n' 'PASS: source-bound cleared cost polynomial, exact bytes, false identities/source hashes and native backend tests'
