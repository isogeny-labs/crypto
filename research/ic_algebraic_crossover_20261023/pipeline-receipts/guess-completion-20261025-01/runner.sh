#!/usr/bin/env bash
# Read-only end-of-run audit. No cloud allocation, Git push or publication.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
run="$study/runs/guess-cap200k-20261025-01"
pid=${1:?registered queue process id}
bin=${IC_CHECKPOINT_BIN:?frozen native checkpoint executable}
meta="$study/pipeline-receipts/guess-completion-20261025-01"
test ! -e "$meta"
mkdir -p "$meta"
cp "$0" "$meta/runner.sh"
sha256sum "$bin" "$0" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/watching.txt"
start=$(awk '{print $22}' "/proc/$pid/stat")
while test -r "/proc/$pid/stat"; do
  current=$(awk '{print $22}' "/proc/$pid/stat")
  if test "$current" != "$start"; then break; fi
  sleep 30
done
if ! test -f "$run/ended.txt"; then
  printf '%s\n' 'Queue ended without final metadata; raw tree may remain mutable. No complete archive or verdict claimed.' > "$meta/INCOMPLETE.txt"
  exit 1
fi
"$bin" "$study" "$run" > "$meta/receipt.json"
"$bin" "$study" "$run" > "$meta/receipt-repeat.json"
cmp "$meta/receipt.json" "$meta/receipt-repeat.json"
(
  cd "$study"
  tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs/guess-cap200k-20261025-01 | xz -6 -T1 > evidence/runs-guess-cap200k-complete-20261025-01.tar.xz
  cd evidence
  sha256sum runs-guess-cap200k-complete-20261025-01.tar.xz > runs-guess-cap200k-complete-20261025-01.tar.xz.sha256
)
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'Native all-branch audit and raw archive prepared. Visual report inspection and publication remain required; no mathematical or cost qualification is promoted automatically.' > "$meta/REVIEW_REQUIRED.txt"
