#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/cm8-transfer-audit
frozen=$study/native/cm8-transfer-20261025
out=${1:?usage: run-cm8-transfer.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
sha256sum -c "$frozen/source-inputs.sha256" > "$out/source-check.log"
sha256sum "$study/native/cm8-transfer.cpp" "$study/native/phase-adaptive-prior.hpp" "$study/native/reduced-s3-prior.hpp" "$study/native/full.cpp" "$study/native/json.hpp" "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" "$study/CM8_TRANSFER_20261025_PROTOCOL.md" "$study/lean-cm8/CM8Transfer.lean" src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/cm8-transfer.cpp" -lcrypto -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-cm8-transfer-audit
timeout 600 "$out/producer" "$root" "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$root" "$out/cases" "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certificate check "$root" "$frozen/certificate" > "$out/certificate.log" 2>&1
mkdir "$out/controls"
for control in false-map false-subgroup false-source missing-arm missing-target false-degree false-witness false-row false-log missing-query false-scalar; do
  cp -a "$out/cases" "$out/controls/$control"
  "$auditor" corrupt "$control" "$out/controls/$control"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" audit "$root" "$out/controls/$control" "$out/controls/$control-receipt.json" > "$out/controls/$control.log" 2>&1
  result=$?
  set -e
  printf '%s\n' "$result" > "$out/controls/$control.exit"
  test "$result" -eq 101
  test ! -e "$out/controls/$control-receipt.json"
done
# The frozen experiment has no failed online shifts. Missing-query removes an
# actual successful query; no missing-failed-query rejection is fabricated.
jq -e '.failed_online_shifts==0 and .missing_failed_query_control=="null-no-failed-shift"' "$out/audit.json" > "$out/null-control.log"
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
printf '%s\n' 'PASS:12 arms/72 verified source scalars/196 actual addition pairs, exact witnesses, prior bindings and eleven controls'
