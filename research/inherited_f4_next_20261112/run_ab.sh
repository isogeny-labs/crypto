#!/usr/bin/env bash
# Paired per-call comparison of `inherited-f4` and `inherited-f4-next` (same
# work, faster kernels), with `fes-f2` as the reference engine and the wall
# yardstick, on the same seeded Weil-descent targets of every cell,
# interleaved per target, engine order rotating per repetition
# (ic descent --solver).  One ic invocation per cell and family.  Run under
# tools/benchrec.py run.
#
#   research/inherited_f4_next_20261112/run_ab.sh [results-dir]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
TARGETS="${TARGETS:-6}"; REPEATS="${REPEATS:-3}"; BUDGET="${BUDGET:-60}"
CELLS="${CELLS:-K:13:7:2 R:13:7:2 K:15:8:2 R:15:8:2 K:17:9:2 R:17:9:2 K:19:10:2 R:19:10:2 R:21:11:2 K:23:12:2 R:23:12:2 K:25:13:2 K:27:14:2 K:29:15:2 K:31:16:2}"
sha256sum "$repo/target/release/ic" >"$out/ic-sha256.txt"
for c in $CELLS; do
  fam=${c%%:*}; cell=${c#*:}; tag="$fam-$(echo "$cell" | tr ':' '-')"
  f="$out/cell-$tag.json"
  if [ -s "$f" ]; then echo "skip $tag (exists)"; continue; fi
  echo "run $tag $(date -u +%FT%TZ)"
  "$repo/target/release/ic" descent --json --out "$f" --families "$fam" --cells "$cell" \
    --targets "$TARGETS" --repeats "$REPEATS" --budget-seconds "$BUDGET" \
    --solver inherited-f4 --solver inherited-f4-next --solver fes-f2 >"$out/cell-$tag.log" 2>&1 \
    || echo "cell $tag failed (exit $?)"
done
ls -la "$out"
