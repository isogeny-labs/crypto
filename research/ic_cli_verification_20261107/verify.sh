#!/usr/bin/env bash
# Check the `ic` install/update Lean proofs and write a receipt.
set -euo pipefail
cd "$(dirname "$0")"
log=proof_check.log
: > "$log"
status=0
for file in PairCursor VersionSelection AssetSelection InstallLayout DiskBudget BatchInversion; do
  printf '== %s.lean ==\n' "$file" >> "$log"
  if lean "$file.lean" >> "$log" 2>&1; then
    printf 'OK %s.lean\n' "$file"
  else
    printf 'FAIL %s.lean\n' "$file" >&2
    status=1
  fi
done
if grep -Eq "sorry|admit|error:" "$log"; then
  echo "proof check failed: unfinished or erroneous proof" >&2
  status=1
fi
if grep -q "sorryAx" "$log"; then
  echo "proof check failed: a theorem depends on sorryAx" >&2
  status=1
fi
axioms=$(grep -c "depends on axioms:" "$log" || true)
if [ "$axioms" -eq 0 ]; then
  echo "proof check failed: no axiom receipts found" >&2
  status=1
fi
printf '\n%d theorem axiom receipts; log: %s\n' "$axioms" "$log"
exit "$status"
