#!/usr/bin/env bash
# Thin native/proof record/replay glue; no research arithmetic in shell/Python.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
out=$(realpath -m -- "${1:?usage: run-cm8-dual.sh FRESH_OUTSIDE_DIRECTORY}")
case "$out/" in "$root/"*) echo 'output must be outside checkout' >&2; exit 2;; esac
test ! -e "$out"
test "$(git branch --show-current)" = codex/quadratic-twist-evidence-import
test -s "${CHECKPOINT_TOKEN_FILE:?secure scoped checkpoint selector required}"
git show origin/main:scripts/checkpoint.sh | cmp - scripts/checkpoint.sh
sources=("$study/native/cm8-dual.cpp" "$study/native/cm8-transfer-prior.hpp" "$study/native/cm8-dual-audit/main.rs" "$study/native/cm8-dual-audit/Cargo.lock" "$study/CM8_DUAL_20261029_PROTOCOL.md" "$study/lean-dual/CM8Dual.lean" "$study/run-cm8-dual.sh")
git ls-files --error-unmatch "${sources[@]}" >/dev/null
git diff --quiet HEAD -- "${sources[@]}"
mkdir -p "$out"
bash scripts/checkpoint.sh gate > "$out/checkpoint.log" 2>&1
git rev-parse HEAD > "$out/input-head.txt"
git ls-files "$study/native" "$study/lean" "$study/lean-cm8" "$study/lean-dual" |
  rg '\.(cpp|hpp|rs|toml|lock|lean)$|lean-toolchain$' |
  xargs sha256sum > "$out/INPUTS.sha256"
sha256sum "${sources[@]}" src/hash/sha256.rs src/cryptanalysis/identity_certificate.rs >> "$out/INPUTS.sha256"
export CARGO_BUILD_JOBS=2
run() {
  local name=$1; shift
  printf '%q ' "$@" > "$out/$name.command"; printf '\n' >> "$out/$name.command"
  set +e
  timeout 600 "$@" > "$out/$name.log" 2>&1
  local result=$?
  set -e
  printf '%s\n' "$result" > "$out/$name.exit"
  return "$result"
}
old=$study/native/cm8-transfer-audit
new=$study/native/cm8-dual-audit
run producer-build c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/cm8-dual.cpp" -lcrypto -o "$out/producer"
run old-build cargo build --release --locked --manifest-path "$old/Cargo.toml"
run new-build cargo build --release --locked --manifest-path "$new/Cargo.toml"
olda=$old/target/release/quadratic-twist-cm8-transfer-audit
newa=$new/target/release/quadratic-twist-cm8-dual-audit
run producer "$out/producer" "$root" "$out/cases"
run reference-audit "$olda" audit "$root" "$out/cases/reference" "$out/reference-audit.json"
run candidate-audit "$olda" audit "$root" "$out/cases/candidate" "$out/candidate-audit.json"
run dual-audit "$newa" audit "$root" "$out/cases" "$out/audit.json"
run frozen-reference diff -ru "$study/native/cm8-transfer-20261025/cases" "$out/cases/reference"
cmp "$out/reference-audit.json" "$out/candidate-audit.json"
run old-certificate "$olda" certificate check "$root" "$study/native/cm8-transfer-20261025/certificate"
mkdir "$out/controls"
for kind in false-dual false-multiplier false-composition false-kernel missing-point false-count false-subgroup false-scalar false-source; do
  cp -a "$out/cases" "$out/controls/$kind"
  run "$kind-mutate" "$newa" corrupt "$kind" "$out/controls/$kind"
  if cmp -s "$out/cases/dual.json" "$out/controls/$kind/dual.json"; then echo "no-op control: $kind" >&2; exit 2; fi
  set +e
  RUST_BACKTRACE=0 timeout 600 "$newa" audit "$root" "$out/controls/$kind" "$out/controls/$kind-receipt.json" > "$out/$kind-reject.log" 2>&1
  result=$?
  set -e
  printf '%s\n' "$result" > "$out/$kind-reject.exit"
  test "$result" = 101
  test ! -e "$out/controls/$kind-receipt.json"
  rg -q 'false dual' "$out/$kind-reject.log"
done
run backend-tests cargo test --release --locked --manifest-path "$new/Cargo.toml"
sha256sum -c "$out/INPUTS.sha256" > "$out/final-inputs.log"
printf '0\n' > "$out/runner.exit"
printf '%s\n' 'PASS: full F13 dual, frozen S3 parity, nine controls and backend tests'
