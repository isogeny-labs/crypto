#!/usr/bin/env bash
# Exact finite S3 replay and controls; no runtime or speed comparison.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/s3-yield-audit
frozen=$study/native/s3-yield-20261024
out=${1:?usage: run-s3-yield.sh FRESH_OUTPUT_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/start.utc"
uname -a > "$out/environment.txt"
c++ --version >> "$out/environment.txt"
rustc --version >> "$out/environment.txt"
cargo --version >> "$out/environment.txt"
sha256sum "$study/native/s3-yield.cpp" "$study/native/full.cpp" "$study/native/json.hpp" \
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/lean/S3System.lean" "$study/lean/S3DegreeFixture.lean" \
  docs/curves/registry.json src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/s3-yield.cpp" \
  -lcrypto -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-s3-yield-audit
timeout 600 "$out/producer" docs/curves/registry.json "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$out/cases" docs/curves/registry.json "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate issue "$root" "$out/certificate" > "$out/certificate-issue.log" 2>&1
"$auditor" certificate check "$root" "$out/certificate" > "$out/certificate-check.log" 2>&1
diff -ru "$frozen/certificate" "$out/certificate" > "$out/certificate-diff.log"
mkdir "$out/controls"
for control in false-rank false-degree missing-case false-scaling; do
  "$auditor" corrupt "$control" "$out/cases" "$out/controls/$control"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" audit "$out/controls/$control" docs/curves/registry.json \
    "$out/controls/$control-receipt.json" > "$out/controls/$control.log" 2>&1
  s3_control_exit=$?
  set -e
  printf '%s\n' "$s3_control_exit" > "$out/controls/$control.exit"
  test "$s3_control_exit" -eq 101
  test ! -e "$out/controls/$control-receipt.json"
done
rg -q 'false useful rank' "$out/controls/false-rank.log"
rg -q 'false degree profile' "$out/controls/false-degree.log"
rg -q 'missing case coverage' "$out/controls/missing-case.log"
rg -q 'false scaling' "$out/controls/false-scaling.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" "$out/certificate" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/end.utc"
printf '%s\n' 'PASS: 124 S3 comparisons / 1904 systems, independent matrices/points/ranks, exact bytes, scalar recoveries and corruption controls'
