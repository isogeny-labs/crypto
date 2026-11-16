#!/usr/bin/env bash
# Thin invocation glue only; proof arithmetic is Lean. No gate bypass.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
[ "$(git branch --show-current)" = codex/twist-chord-proof ]
[ -n "${CHECKPOINT_TOKEN_FILE:-}" ] && [ -s "$CHECKPOINT_TOKEN_FILE" ]
[ "$(git rev-parse HEAD:scripts/checkpoint.sh)" = "$(git rev-parse refs/remotes/origin/main:scripts/checkpoint.sh)" ]
[ -z "$(git status --porcelain --untracked-files=no)" ]
: "${PROOF_OUTPUT:?Set a fresh output directory outside the checkout}"
[ ! -e "$PROOF_OUTPUT" ]
case "$PROOF_OUTPUT" in "$root"/*) exit 2;; esac
bash "$root/scripts/checkpoint.sh" gate
mkdir -p "$PROOF_OUTPUT"
export ELAN_HOME=/workspace/twist-tooling/elan
export PATH=/workspace/twist-tooling/elan/bin:$PATH
# Existing pinned libraries are read only; do not build or alter old projects.
packages=/workspace/crypto-pr2381-current/research/quadratic_twist_evidence_20261021/lean-uniform/.lake/packages
[ "$(git -C "$packages/mathlib" rev-parse HEAD)" = d13f23b723b8a846827a245b89c10fc7d3f11612 ]
lean_paths=()
for package in "$packages"/*; do
  [ ! -d "$package/.lake/build/lib/lean" ] || lean_paths+=("$package/.lake/build/lib/lean")
done
export LEAN_PATH=$(IFS=:; printf '%s' "${lean_paths[*]}")
source_file="$root/research/quadratic_twist_chord_proof_20261105/Chord.lean"
sha256sum "$source_file" > "$PROOF_OUTPUT/source.sha256"
elan run leanprover/lean4:v4.34.1 lean --version > "$PROOF_OUTPUT/toolchain.txt"
set +e
timeout 600 elan run leanprover/lean4:v4.34.1 lean "$source_file" > "$PROOF_OUTPUT/lean.log" 2>&1
code=$?
set -e
printf '%s\n' "$code" > "$PROOF_OUTPUT/exit.txt"
# Review exact output/source and update truthful RESUME before finish checkpoint.
# An exit0 here is an elaboration observation, not a signed PCC verdict.
exit "$code"
