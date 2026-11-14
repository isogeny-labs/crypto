#!/usr/bin/env bash
# L4 measured run (frozen PROTOCOL.md). One `ic descent` invocation per cell
# (arms I = inherited-f4 and F = fes-f2 on the seeded targets), then arm G
# (l4bench gcd) on exactly the targets each JSON records, then the summary.
#   IC=<path to ic> L4BENCH=<path to l4bench> research/.../L4-baselines/run.sh
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
raw="$here/raw"; mkdir -p "$raw"
IC="${IC:?path to ic}"; L4="${L4BENCH:?path to l4bench}"
SEED="${SEED:-1}"; TARGETS="${TARGETS:-8}"; REPEATS="${REPEATS:-2}"; BUDGET="${BUDGET:-120}"
CELLS="${CELLS:-11:6:2 13:7:2 19:10:2 29:15:2 37:19:2 29:6:2 29:8:2 29:10:2 29:12:2 29:14:2 11:4:3 13:4:3 13:5:3 19:6:3}"
{ echo "ic_sha256 $(sha256sum "$IC" | cut -d' ' -f1)"; echo "l4bench_sha256 $(sha256sum "$L4" | cut -d' ' -f1)";
  echo "crypto_head $(git -C "$here/../../.." rev-parse HEAD)"; echo "protocol_sha256 $(sha256sum "$here/PROTOCOL.md" | cut -d' ' -f1)";
  echo "started $(date -u +%FT%TZ)"; } > "$raw/run-identity.txt"
for c in $CELLS; do
  tag="K-$(echo "$c" | tr ':' '-')-s$SEED"
  f="$raw/ic-$tag.json"
  if [ ! -s "$f" ]; then
    echo "ic $tag $(date -u +%FT%TZ)"
    timeout 3600 prlimit --as=2000000000 "$IC" descent --json --families K --cells "$c" \
      --targets "$TARGETS" --repeats "$REPEATS" --seed "$SEED" --budget-seconds "$BUDGET" \
      --solver inherited-f4 --solver fes-f2 > "$f" 2> "$raw/ic-$tag.log" || echo "ic $tag exit $?"
  fi
  if [ -s "$f" ] && [ ! -s "$raw/g-$tag.jsonl" ]; then
    echo "G $tag $(date -u +%FT%TZ)"
    timeout 3600 prlimit --as=2000000000 "$L4" gcd "$f" "$raw/g-$tag.jsonl" 2> "$raw/g-$tag.log" || echo "G $tag exit $?"
  fi
done
echo "finished $(date -u +%FT%TZ)" >> "$raw/run-identity.txt"
"$L4" summarize "$raw"/g-*.jsonl --out "$here/summary"
