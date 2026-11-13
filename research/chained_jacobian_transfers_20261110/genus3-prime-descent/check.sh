#!/usr/bin/env bash
# Thin integration driver. All arithmetic and proof verification are original
# Rust/Lean/PCC commands; this script records their actual process outcomes.
set -euo pipefail
test "$#" -eq 1
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/chained_jacobian_transfers_20261110
out=$(realpath -m "$1")
mkdir "$out"
reference="$study/genus3-prime-descent/reference.sha256"
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.txt"
finish() {
  status=$?
  trap - EXIT
  set +e
  sha256sum --check "$reference" > "$out/final-inputs.log" 2>&1
  guard=$?
  printf '%s\n' "$guard" > "$out/guard.exit"
  printf '%s\n' "$status" > "$out/original.exit"
  date -u +%Y-%m-%dT%H:%M:%SZ > "$out/finish.txt"
  if [ "$status" -ne 0 ]; then exit "$status"; fi
  exit "$guard"
}
trap finish EXIT
sha256sum --check "$reference" > "$out/initial-inputs.log" 2>&1
run_logged() {
  label=$1
  shift
  set +e
  "$@" 2>&1 | tee "$out/$label.log"
  codes=("${PIPESTATUS[@]}")
  set -e
  printf '%s %s\n' "${codes[0]}" "${codes[1]}" > "$out/$label.pipe-exits"
  test "${codes[0]}" -eq 0
  test "${codes[1]}" -eq 0
}
manifest="$study/genus3-prime-descent/Cargo.toml"
rustc -Vv > "$out/rustc.txt"
cargo -V > "$out/cargo.txt"
lean --version > "$out/lean.txt"
uname -a > "$out/host.txt"
run_logged fetch cargo fetch --locked --manifest-path "$manifest"
run_logged format rustfmt --edition 2021 --config skip_children=true --check \
  "$study/genus3-prime-descent/src/lib.rs" \
  "$study/genus3-prime-descent/src/main.rs" \
  "$study/genus3-prime-descent/src/tests.rs" \
  "$study/genus3-prime-descent/src/bin/ramification_panel.rs" \
  "$study/genus3-prime-descent/engine/src/lib.rs"
run_logged clippy cargo clippy --all-targets --offline --locked --manifest-path "$manifest" -- -D warnings
run_logged unit cargo test --release --offline --locked --manifest-path "$manifest" --lib -- --nocapture --test-threads=1
for q in 5 7 11; do
  run_logged "cli-q$q" cargo run --release --offline --locked --manifest-path "$manifest" --bin ghs-prime-descent -- \
    "$study/evidence/genus3-cli-fixtures-01/q$q-request.json" "$out/q$q-output.json"
  cmp "$out/q$q-output.json" "$study/evidence/genus3-cli-fixtures-01/q$q-output.json"
done
for source in Descent ModelTransport; do
  run_logged "lean-$source" tools/lean/proof-gate.sh \
    "$study/genus3-prime-descent/lean/$source.lean" --out "$out/proofs-$source"
done
while IFS= read -r claim; do
  run_logged "pcc-$(basename "$claim" .json)" python3 tools/pcc/pcc.py verify "$claim" --json
done < <(jq -r '.[].claim_path' "$study/evidence/genus3-lean-bindings-01/claims.json")
printf 'G3 focused replay complete:4 native tests,9 CLI points,2 proof modules,11 exact bindings\n'
