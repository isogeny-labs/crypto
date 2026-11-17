#!/usr/bin/env bash
set -euo pipefail
proof_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
repo_dir="$(cd -- "$proof_dir/../../.." && pwd)"
receipt_dir="${1:?fresh absolute persistent receipt directory}"
test ! -e "$receipt_dir"
mkdir -p "$receipt_dir" "$proof_dir/.lake/build/lib/lean"
export PATH="/tmp/twist-lean/bin:$PATH"
test "$(git -C /tmp/twist-mathlib rev-parse HEAD)" = d13f23b723b8a846827a245b89c10fc7d3f11612
/tmp/twist-lean/bin/lean --version > "$receipt_dir/lean-version.stdout"
git -C "$repo_dir" rev-parse HEAD > "$receipt_dir/source-commit.stdout"
git -C /tmp/twist-mathlib rev-parse HEAD > "$receipt_dir/mathlib-commit.stdout"
work_dir="/workspace/shared/twist-tools-20261023/proofs/$(basename -- "$receipt_dir")"
test ! -e "$work_dir"
mkdir -p "$work_dir"
cd -- "$proof_dir/../lean"
/tmp/twist-lean/bin/lake env printenv LEAN_PATH > "$receipt_dir/base-lean-path.txt"
base_path="$(cat "$receipt_dir/base-lean-path.txt")"
node "$proof_dir/run-lean.mjs" "$receipt_dir/prerequisite" "$PWD" "$base_path" \
  -o "$work_dir/TwistRoutes.olean" TwistRoutes.lean
cd -- "$proof_dir"
module_path="$proof_dir/.lake/build/lib/lean:$work_dir:$base_path"
node "$proof_dir/run-lean.mjs" "$receipt_dir/module" "$proof_dir" "$module_path" \
  -o .lake/build/lib/lean/TwistFollowup.olean TwistFollowup.lean
node "$proof_dir/run-lean.mjs" "$receipt_dir/auditor-compile" "$proof_dir" /tmp/twist-lean/lib/lean \
  --root="$repo_dir" -c "$work_dir/AuditAxioms.c" ../../../tools/lean/AuditAxioms.lean
timeout --signal=TERM --kill-after=5s 600s /tmp/twist-lean/bin/leanc \
  -O2 -o "$work_dir/axiom-auditor" "$work_dir/AuditAxioms.c" \
  > "$receipt_dir/auditor-link.stdout" 2> "$receipt_dir/auditor-link.stderr"
node "$proof_dir/run-lean.mjs" "$receipt_dir/axiom-audit" "$proof_dir" "$module_path" \
  --native "$work_dir/axiom-auditor" TwistFollowup
sha256sum "$proof_dir/TwistFollowup.lean" "$proof_dir/../lean/TwistRoutes.lean" \
  "$proof_dir/lake-manifest.json" "$proof_dir/lean-toolchain" "$proof_dir/run-lean.mjs" \
  "$proof_dir/replay.sh" "$repo_dir/tools/lean/AuditAxioms.lean" "$work_dir/AuditAxioms.c" \
  "$work_dir/axiom-auditor" > "$receipt_dir/SOURCES.sha256"
