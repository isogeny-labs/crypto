#!/usr/bin/env bash
# Run tools/pari_check.gp for every landed rust_<label>.json (idempotent).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../data/cl"
for res in "$OUT"/rust_p*.json; do
  [ -s "$res" ] || continue
  lab="$(basename "$res" .json)"; lab="${lab#rust_}"
  [ -s "$OUT/pari_check_${lab}.json" ] && { echo "$lab: check exists"; continue; }
  grep -q '"accepted": true' "$res" || { echo "$lab: not accepted, skipping"; continue; }
  D="$(grep -o '"D_K": -[0-9]*' "$res" | cut -d' ' -f2)"
  N="$(grep -o '"h": [0-9]*' "$res" | head -1 | cut -d' ' -f2)"
  primes="$(grep -o '"h_factorisation": {"primes": \[[0-9, ]*\]' "$res" | grep -o '\[[0-9, ]*\]')"
  exps="$(grep -o '"h_factorisation": {"primes": \[[0-9, ]*\], "exponents": \[[0-9, ]*\]' "$res" | grep -o '\[[0-9, ]*\]$')"
  # pair primes and exponents into [[q,e],...]
  fac="$(paste -d, <(echo "$primes" | tr -d '[]' | tr ',' '\n' | tr -d ' ') <(echo "$exps" | tr -d '[]' | tr ',' '\n' | tr -d ' ') | sed 's/^/[/; s/$/]/' | paste -sd, | sed 's/^/[/; s/$/]/')"
  echo "check(\"$lab\", $D, $N, $fac)" | timeout 1200 gp -q -f "$HERE/pari_check.gp" 2>&1 | grep -v Warning > "$OUT/pari_check_${lab}.json" || true
  cat "$OUT/pari_check_${lab}.json"
done
