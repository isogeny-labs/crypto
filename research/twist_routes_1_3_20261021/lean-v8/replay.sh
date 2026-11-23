#!/usr/bin/env bash
set -euo pipefail
proof_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
repo_dir="$(cd -- "$proof_dir/../../.." && pwd)"
receipt_dir=${1:?fresh absolute receipt directory}
case "$receipt_dir" in /*) ;; *) exit 2;; esac
test ! -e "$receipt_dir"
mkdir -p "$receipt_dir"
tool_root=${TWIST_PROOF_TOOL_ROOT:-/workspace/shared/twist-tools-20261023}
export PATH="$tool_root/lean/bin:$PATH"
test "$(git -C "$tool_root/mathlib" rev-parse HEAD)" = d13f23b723b8a846827a245b89c10fc7d3f11612
git -C "$repo_dir" rev-parse HEAD > "$receipt_dir/source-commit.stdout"
git -C "$tool_root/mathlib" rev-parse HEAD > "$receipt_dir/mathlib-commit.stdout"
lean --version > "$receipt_dir/lean-version.stdout"
cd "$tool_root/mathlib"
lake env printenv LEAN_PATH > "$receipt_dir/lean-path.stdout"
base_path="$(cat "$receipt_dir/lean-path.stdout")"
cd "$proof_dir"
node run-lean.mjs "$receipt_dir/module" "$proof_dir" "$base_path" \
  -o "$receipt_dir/TwistQuotientCycle.olean" TwistQuotientCycle.lean
node run-lean.mjs "$receipt_dir/auditor-compile" "$proof_dir" "$tool_root/lean/lib/lean" \
  --root="$repo_dir" -c "$receipt_dir/AuditAxioms.c" "$repo_dir/tools/lean/AuditAxioms.lean"
timeout --signal=TERM --kill-after=5s 600s leanc -O2 \
  -o "$receipt_dir/axiom-auditor" "$receipt_dir/AuditAxioms.c" \
  > "$receipt_dir/auditor-link.stdout" 2> "$receipt_dir/auditor-link.stderr"
export LD_LIBRARY_PATH="$tool_root/lean/lib/lean${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
node run-lean.mjs "$receipt_dir/axiom-audit" "$proof_dir" "$receipt_dir:$base_path" \
  --native "$receipt_dir/axiom-auditor" TwistQuotientCycle
sha256sum TwistQuotientCycle.lean lakefile.toml lake-manifest.json lean-toolchain run-lean.mjs replay.sh \
  "$repo_dir/tools/lean/AuditAxioms.lean" "$receipt_dir/AuditAxioms.c" "$receipt_dir/axiom-auditor" \
  > "$receipt_dir/SOURCES.sha256"
