#!/usr/bin/env bash
# Exact phase-request replay; no time/GAE or cross-method speed claim.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/phase-demand-audit
source_cases=$study/native/reduced-s3-20261025/cases
prior_cases=$study/native/adaptive-s3-20261025/cases
frozen=$study/native/phase-demand-20261025
out=${1:?usage: run-phase-demand.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
sha256sum -c "$frozen/source-inputs.sha256" > "$out/source-check.log"
sha256sum "$study/native/phase-demand.cpp" "$study/native/phase-adaptive-prior.hpp" \
  "$study/native/reduced-s3-prior.hpp" "$study/native/full.cpp" "$study/native/json.hpp" \
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock" \
  "$study/PHASE_DEMAND_20261025_PROTOCOL.md" "$study/PHASE_DEMAND_20261025_COUNTERS.md" \
  "$study/lean-phase/PhaseRecovery.lean" src/cryptanalysis/identity_certificate.rs \
  src/hash/sha256.rs > "$out/INPUTS.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/phase-demand.cpp" -lcrypto \
  -o "$out/producer" > "$out/producer-build.log" 2>&1
cargo build --release --locked --manifest-path "$crate/Cargo.toml" > "$out/auditor-build.log" 2>&1
auditor=$crate/target/release/quadratic-twist-phase-demand-audit
timeout 600 "$out/producer" "$source_cases" "$prior_cases" "$out/cases" > "$out/producer.log" 2>&1
timeout 600 "$auditor" audit "$source_cases" "$prior_cases" "$out/cases" "$out/audit.json" > "$out/audit.log" 2>&1
diff -ru "$frozen/cases" "$out/cases" > "$out/cases-diff.log"
cmp "$frozen/audit.json" "$out/audit.json"
mkdir "$out/controls"
for control in false-setup false-matrix missing-failed-query false-base-log false-recovery false-source; do
  cp -a "$out/cases" "$out/controls/$control"
  "$auditor" corrupt "$control" "$out/controls/$control"
  set +e
  RUST_BACKTRACE=0 timeout 600 "$auditor" audit "$source_cases" "$prior_cases" \
    "$out/controls/$control" "$out/controls/$control-receipt.json" > "$out/controls/$control.log" 2>&1
  result=$?
  set -e
  printf '%s\n' "$result" > "$out/controls/$control.exit"
  test "$result" -eq 101
  test ! -e "$out/controls/$control-receipt.json"
  rg -q 'false phase trace, demand, coverage, log, scalar or source' "$out/controls/$control.log"
done
cargo test --release --locked --manifest-path "$crate/Cargo.toml" > "$out/backend-tests.log" 2>&1
find "$out/cases" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
printf '%s\n' 'PASS: 40 setups/400 verified targets, exact demand vectors/certificates and six rehashed semantic controls'
