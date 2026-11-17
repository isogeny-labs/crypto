#!/usr/bin/env bash
# Exact signed-cache request replay; no time/GAE or cross-method speed claim.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/signed-cache-audit
source_cases=$study/native/reduced-s3-20261025/cases
prior_cases=$study/native/adaptive-s3-20261025/cases
baseline_cases=$study/native/known-roots-20261025/cases
frozen=$study/native/signed-cache-20261025
out=${1:?usage: run-signed-cache.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
sha256sum -c "$frozen/source-inputs.sha256" > "$out/source-check.log"
sha256sum "$study/native/signed-cache.cpp" "$study/native/phase-adaptive-prior.hpp" \
  "$study/native/reduced-s3-prior.hpp" "$study/native/full.cpp" "$study/native/json.hpp" \
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/SIGNED_CACHE_20261025_PROTOCOL.md" \
  "$study/lean-signed-cache/SignedCache.lean" src/cryptanalysis/identity_certificate.rs \
  src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/signed-cache.cpp" -lcrypto \
  -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-signed-cache-audit
timeout 600 "$out/producer" "$source_cases" "$prior_cases" "$baseline_cases" "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$source_cases" "$prior_cases" "$baseline_cases" "$out/cases" "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate issue "$root" "$out/certificate" > "$out/certificate.log" 2>&1
"$auditor" certificate check "$root" "$out/certificate" >> "$out/certificate.log" 2>&1
diff -ru "$frozen/certificate" "$out/certificate" > "$out/certificate-diff.log"
mkdir "$out/controls"
for control in false-sign false-point missing-entry false-demand false-source missing-failed-query false-recovery; do
  cp -a "$out/cases" "$out/controls/$control"
  "$auditor" corrupt "$control" "$out/controls/$control"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" audit "$source_cases" "$prior_cases" "$baseline_cases" \
    "$out/controls/$control" "$out/controls/$control-receipt.json" > "$out/controls/$control.log" 2>&1
  result=$?
  set -e
  printf '%s\n' "$result" > "$out/controls/$control.exit"
  test "$result" -eq 101
  test ! -e "$out/controls/$control-receipt.json"
  rg -q 'false signed-cache point, sign, demand, coverage, query, scalar or source' "$out/controls/$control.log"
done
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
printf '%s\n' 'PASS: 80 setups/800 verified targets, equal ordered lifts/certificates, charged cache deltas and seven rehashed controls'
