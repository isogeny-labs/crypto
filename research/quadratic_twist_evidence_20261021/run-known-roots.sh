#!/usr/bin/env bash
# Exact checked-root request replay; no time/GAE or cross-method speed claim.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/known-roots-audit
source_cases=$study/native/reduced-s3-20261025/cases
prior_cases=$study/native/adaptive-s3-20261025/cases
baseline_cases=$study/native/phase-demand-20261025/cases
frozen=$study/native/known-roots-20261025
out=${1:?usage: run-known-roots.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
sha256sum -c "$frozen/source-inputs.sha256" > "$out/source-check.log"
sha256sum "$study/native/known-roots.cpp" "$study/native/phase-adaptive-prior.hpp" \
  "$study/native/reduced-s3-prior.hpp" "$study/native/full.cpp" "$study/native/json.hpp" \
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/KNOWN_ROOTS_20261025_PROTOCOL.md" "$study/KNOWN_ROOTS_20261025_ORDER_CORRECTION.md" \
  "$study/lean-known-roots/KnownRoots.lean" src/cryptanalysis/identity_certificate.rs \
  src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/known-roots.cpp" -lcrypto \
  -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-known-roots-audit
timeout 600 "$out/producer" "$source_cases" "$prior_cases" "$baseline_cases" "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$source_cases" "$prior_cases" "$baseline_cases" "$out/cases" "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate issue "$root" "$out/certificate" > "$out/certificate.log" 2>&1
"$auditor" certificate check "$root" "$out/certificate" >> "$out/certificate.log" 2>&1
diff -ru "$frozen/certificate" "$out/certificate" > "$out/certificate-diff.log"
mkdir "$out/controls"
for control in omitted-root extra-root false-membership false-setup false-matrix missing-failed-query false-base-log false-recovery false-source; do
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
  rg -q 'false known-root trace, demand, coverage, root, polynomial, log, scalar or source' "$out/controls/$control.log"
done
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
printf '%s\n' 'PASS: 80 setups/800 verified targets, equal certificates, checked-root demands and nine rehashed controls'
