#!/usr/bin/env bash
# Plan the weekly exhaustive replay from the weights of ecbench-replay-weights.sh.
#
# Every week: the sessions the pre-rotation lane replayed (the "core": every
# session under research/ecbench_*/sessions, research/ghs_curve_comparison_20261009/
# sessions and the two research/notes/ecc2k130/*_compact_orbit_ic_20261003/
# sessions directories).  One quarter of the rest: each remaining session is in
# exactly one of four quarters, assigned greedily (heaviest recorded solver time
# first, ties by path, into the lightest quarter so far, ties to the lower one),
# so a session is replayed once every 4 weeks.  This week's quarter is the ISO
# week number mod 4, or QUARTER (0-3), or every quarter with QUARTER=all.
# Never planned: the pinned incomplete and defective sessions (no weights are
# given for them) and the pinned long sessions (LONG_SESSIONS.tsv; the
# long_replay job replays those on request).
# The selected sessions go to SHARDS shards by the same greedy rule on weight.
# Usage: [QUARTER=0-3|all] [WEEK=ISO week] [SHARDS=12] ecbench-replay-plan.sh WEIGHTS OUTDIR [LONG_TSV]
# Writes OUTDIR/plan.tsv (shard, weight_ms, directory) and prints a summary.
set -euo pipefail
weights=${1:?usage: ecbench-replay-plan.sh WEIGHTS OUTDIR [LONG_TSV]}
out=${2:?}
long=${3:-docs/ecbench/LONG_SESSIONS.tsv}
shards=${SHARDS:-12}
week=${WEEK:-$(date -u +%V)}
quarter=${QUARTER:-}
if [[ -z "$quarter" ]]; then quarter=$((10#$week % 4)); fi
if [[ ! "$quarter" =~ ^([0-3]|all)$ ]]; then echo "::error::QUARTER must be 0-3 or all, got $quarter" >&2; exit 1; fi
test -s "$weights" || { echo "::error::$weights is empty" >&2; exit 1; }
mkdir -p "$out"
{ cut -f1 "$long" | { grep -v '^#' || true; } | { grep -v '^$' || true; } | sort -u; } > "$out/long.txt"
# Rotation quarters and shards, in awk (no sort order is left to chance: the
# input is ordered by weight descending then path, by `sort` in the C locale).
LC_ALL=C sort -t $'\t' -k2,2nr -k1,1 "$weights" | awk -F'\t' \
  -v quarter="$quarter" -v shards="$shards" -v longfile="$out/long.txt" -v plan="$out/plan.tsv" -v summary="$out/summary.txt" '
  BEGIN { while ((getline l < longfile) > 0) isl[l] = 1 }
  function core(d) {
    return d ~ /^research\/ecbench_[^\/]+\/sessions\/[^\/]+$/ || d ~ /^research\/ghs_curve_comparison_20261009\/sessions\/[^\/]+$/ \
      || d ~ /^research\/notes\/ecc2k130\/(cold|full_rank)_compact_orbit_ic_20261003\/sessions\/[^\/]+$/
  }
  function lightest(arr, n,   i, b) { b = 0; for (i = 1; i < n; i++) if (arr[i] < arr[b]) b = i; return b }
  { d = $1; w = $2 + 0
    if (d in isl) { nlong++; next }
    if (core(d)) { cn++; cwt += w; sel[++ns] = d; selw[ns] = w; selq[ns] = -1; next }
    qi = lightest(ql, 4); ql[qi] += w; qc[qi]++
    sel[++ns] = d; selw[ns] = w; selq[ns] = qi }
  END {
    # selected: core plus the chosen quarter, or all quarters; already in weight order
    for (i = 1; i <= ns; i++) {
      if (selq[i] >= 0 && quarter != "all" && selq[i] != quarter + 0) continue
      k = lightest(load, shards); load[k] += selw[i]; cnt[k]++
      printf "%d\t%d\t%s\n", k, selw[i], sel[i] > plan
      tot += selw[i]; nsel++
    }
    close(plan)
    printf "replay rotation: quarter %s ran (%d sessions, %.1f h recorded solver time)\n", quarter, nsel, tot / 3.6e6 > summary
    printf "- core, replayed every week: %d sessions, %.1f h\n", cn, cwt / 3.6e6 > summary
    line = "- rotation quarters (each session once every 4 weeks):"
    for (i = 0; i < 4; i++) line = line sprintf(" Q%d%s %d sessions %.1f h;", i, (quarter == "all" || quarter + 0 == i && quarter != "all") ? "*" : "", qc[i], ql[i] / 3.6e6)
    print line > summary
    printf "- long sessions, not in the rotation (long_replay, workflow_dispatch long=true): %d\n", nlong > summary
    mx = 0; mn = 1e30; for (k = 0; k < shards; k++) { if (load[k] > mx) mx = load[k]; if (load[k] < mn) mn = load[k] }
    printf "- %d shards; heaviest %.1f h of recorded solver time, lightest %.1f h\n", shards, mx / 3.6e6, mn / 3.6e6 > summary
  }'
: > "$out/plan.sorted"
LC_ALL=C sort -t $'\t' -k1,1n -k2,2nr -k3,3 "$out/plan.tsv" > "$out/plan.sorted" && mv "$out/plan.sorted" "$out/plan.tsv"
cat "$out/summary.txt"
if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then cat "$out/summary.txt" >> "$GITHUB_STEP_SUMMARY"; fi
