#!/usr/bin/env bash
# Per-round ratios (a/c, a/q) for both rows and metrics, as TSV, from runs/.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
v() { jq -r "$med"' "\([.repetitions[].setup_phases_ns.collect] | med) \([.repetitions[].setup_ns] | med)"' "$1"; }
printf "row\tround\tab_collect\taa_collect\tab_setup\taa_setup\n"
for row in k0n53 k0n61; do for r in $(seq 1 42); do
  f="$here/runs/$row"; [ -s $f/a-r$r.json ] && [ -s $f/q-r$r.json ] && [ -s $f/c-r$r.json ] || continue
  read ac as < <(v $f/a-r$r.json); read qc qs < <(v $f/q-r$r.json); read cc cs < <(v $f/c-r$r.json)
  echo -e "$row\t$r\t$(echo "$ac/$cc" | bc -l)\t$(echo "$ac/$qc" | bc -l)\t$(echo "$as/$cs" | bc -l)\t$(echo "$as/$qs" | bc -l)"
done; done
