#!/usr/bin/env bash
# Bounded correctness replay; no elapsed-time or solver-speed comparison.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
frozen=$study/native/pairing-20261024T0442Z
crate=$study/native/pairing-audit
out=${1:?usage: run-pairing.sh FRESH_OUTPUT_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/start.utc"
uname -a > "$out/environment.txt"
c++ --version >> "$out/environment.txt"
rustc --version >> "$out/environment.txt"
cargo --version >> "$out/environment.txt"
sha256sum "$study/native/pairing.cpp" "$study/native/full.cpp" "$study/native/json.hpp" \
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/lean/PairingFixture.lean" "$study/lean/PairingTransfer.lean" \
  src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs \
  docs/curves/registry.json > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/pairing.cpp" \
  -lcrypto -o "$out/pairing" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-pairing-audit
timeout 600 "$out/pairing" docs/curves/registry.json "$out/results.json" > "$out/producer.log" 2>&1
timeout 600 "$auditor" "$out/results.json" docs/curves/registry.json "$out/audit.json" > "$out/audit.log" 2>&1
cmp "$frozen/results.json" "$out/results.json"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate issue "$root" "$out/certificate" > "$out/certificate-issue.log" 2>&1
"$auditor" certificate check "$root" "$out/certificate" > "$out/certificate-check.log" 2>&1
diff -ru "$frozen/certificate" "$out/certificate" > "$out/certificate-diff.log"
mkdir "$out/controls"
for control in false-value missing-model false-count false-ic-coefficient; do
  "$auditor" corrupt "$control" "$out/results.json" "$out/controls/$control.json"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" "$out/controls/$control.json" docs/curves/registry.json \
    "$out/controls/$control-audit.json" > "$out/controls/$control.log" 2>&1
  control_exit=$?
  set -e
  printf '%s\n' "$control_exit" > "$out/controls/$control.exit"
  test "$control_exit" -ne 0 && test "$control_exit" -ne 124
  test ! -e "$out/controls/$control-audit.json"
done
rg -q 'pairing value mismatch' "$out/controls/false-value.log"
rg -q 'missing model coverage' "$out/controls/missing-model.log"
rg -q 'declared summary differs' "$out/controls/false-count.log"
rg -q 'false IC coefficient' "$out/controls/false-ic-coefficient.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
sha256sum "$out/results.json" "$out/audit.json" "$out/controls/"*.json \
  "$out/certificate/"*.json > "$out/OUTPUTS.sha256"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/end.utc"
printf '%s\n' 'PASS: complete pairing sweep, independent arithmetic and IC certificates, exact bytes, corruption controls and backend tests'
