#!/usr/bin/env bash
# Exact generator-preprocessing replay; no timing or speed claim.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/reduced-s3-audit
source_cases=$study/native/s3-yield-20261024/cases
frozen=$study/native/reduced-s3-20261025
out=${1:?usage: run-reduced-s3.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
sha256sum -c "$frozen/source-cases.sha256" > "$out/source-check.log"
sha256sum "$study/native/reduced-s3.cpp" "$study/native/reduced-s3-prior.hpp" \
  "$study/native/full.cpp" "$study/native/json.hpp" "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/lean-reduced/ReducedS3.lean" src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/reduced-s3.cpp" -lcrypto \
  -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-reduced-s3-audit
timeout 600 "$out/producer" "$source_cases" "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$source_cases" "$out/cases" "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate issue "$root" "$out/certificate" > "$out/certificate-issue.log" 2>&1
"$auditor" certificate check "$root" "$out/certificate" > "$out/certificate-check.log" 2>&1
diff -ru "$frozen/certificate" "$out/certificate" > "$out/certificate-diff.log"
mkdir "$out/controls"
for control in false-remainder false-degree false-rank missing-case; do
  cp -a "$out/cases" "$out/controls/$control"
  "$auditor" corrupt "$control" "$out/controls/$control"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" audit "$source_cases" "$out/controls/$control" \
    "$out/controls/$control-receipt.json" > "$out/controls/$control.log" 2>&1
  result=$?
  set -e
  printf '%s\n' "$result" > "$out/controls/$control.exit"
  test "$result" -eq 101
  test ! -e "$out/controls/$control-receipt.json"
done
rg -q 'case semantic mismatch' "$out/controls/false-remainder.log"
rg -q 'case semantic mismatch' "$out/controls/false-degree.log"
rg -q 'case semantic mismatch' "$out/controls/false-rank.log"
rg -q 'case coverage' "$out/controls/missing-case.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" "$out/certificate" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
printf '%s\n' 'PASS: 98 cases, 188 comparisons, 1384 targets; independent division/points/ranks/degrees, exact bytes and controls'
