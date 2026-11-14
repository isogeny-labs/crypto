#!/usr/bin/env bash
# PLAN.md's runs: 5 processes per row, k0n53 then k0n61 alternating, one thread.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; repo="$(cd "$here/../../../.." && pwd)"
: "${BIN:?}" "${ISO:?}"; runs="$here/runs"; suite="$repo/research/ic_tool_program/suite/v1"
echo "$(sha256sum "$BIN" | cut -d' ' -f1)  ic" | cmp - "$runs/ic.sha256"
for r in 1 2 3 4 5; do for row in k0n53 k0n61; do
  out="$runs/$row-r$r.json"; [ -e "$out" ] && continue
  seed=$(jq -r --arg id "$row-M1-T01" '.rows[] | select(.id == $id) | .rho_seed' "$suite/SUITE.json")
  until RAYON_NUM_THREADS=1 "$ISO" run --wait --cpus 2 --out "$runs/isolation.jsonl" --label "bs/$row/r$r" -- \
      "$BIN" price --params "$suite/params/S/$row/M1-T01.json" --json --out "$out.tmp" \
      --single-target --rho-seed "$seed" > /dev/null 2> "$runs/last.err"; do
    grep -qi 'busy\|refus' "$runs/last.err" || { cat "$runs/last.err" >&2; exit 1; }; sleep 10
  done
  rm -f "$runs/last.err"; mv "$out.tmp" "$out"
done; done
echo "runs complete"
