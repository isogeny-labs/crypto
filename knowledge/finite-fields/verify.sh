#!/usr/bin/env bash
# Local proof verification, not an experiment or a benchmark. No timing claims.
set -euo pipefail
kb_root=$(cd "$(dirname "$0")" && pwd)
repo_root=$(cd "$kb_root/../.." && pwd)
cd "$repo_root"
cargo test --release --locked --manifest-path tools/finite-field-kb/Cargo.toml > "$kb_root/receipts/kb-tests.log" 2>&1
cargo build --release --locked --manifest-path tools/finite-field-kb/Cargo.toml
tools/finite-field-kb/target/release/finite-field-kb prepare "$kb_root"
run_recorded() {
  local name=$1; shift
  local rc=0
  "$@" > "$kb_root/receipts/$name.log" 2>&1 || rc=$?
  printf '%s\n' "$rc" > "$kb_root/receipts/$name.exit"
  if [ "$rc" -ne 0 ]; then cat "$kb_root/receipts/$name.log"; return "$rc"; fi
}
cd "$kb_root/lean"
run_recorded build lake build
run_recorded audit lake env lean --run "$repo_root/tools/lean/AuditAxioms.lean" -- FiniteFieldHandbook
run_recorded axioms lake env lean PrintAxioms.lean
run_recorded usage lake env lean Usage.lean
run_recorded dependencies lake env lean --run ExportDependencies.lean
cp "$kb_root/receipts/dependencies.log" "$kb_root/receipts/dependencies.json"
cd "$repo_root"
tools/finite-field-kb/target/release/finite-field-kb seal "$kb_root"
if [ "$#" -gt 0 ]; then
  tools/finite-field-kb/target/release/finite-field-kb check "$kb_root" "$1"
else
  tools/finite-field-kb/target/release/finite-field-kb check "$kb_root"
fi
