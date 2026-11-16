#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 3 ]]; then
  echo "usage: analyze.sh ECBENCH OUT_DIR SESSION [SESSION ...]" >&2
  exit 2
fi

ecbench=$1
out=$2
shift 2
mkdir -p "$out"

session_number=0
for session in "$@"; do
  session_number=$((session_number + 1))
  prefix="$out/session-$session_number"
  "$ecbench" verify --dir "$session" --replay-all --exit-code \
    --out "$prefix-replay.json" >/dev/null
  "$ecbench" compare --dir "$session" --a incumbent --b candidate \
    --resamples 4000 --seed 2026103111 --json > "$prefix-candidate.json"
  "$ecbench" compare --dir "$session" --a incumbent --b incumbent-aa \
    --resamples 4000 --seed 2026103112 --json > "$prefix-aa.json"
  "$ecbench" compare --dir "$session" --a rho-neg --b rho-plain \
    --resamples 4000 --seed 2026103113 --json > "$prefix-positive.json"

  jq -e '
    .ops.status == "ok"
    and .ops.same_seeds == true
    and .ops.unpaired_runs == 0
    and .ops.ratio_b_over_a == 1
    and .ops.ci95 == [1, 1]
    and .wall.status == "admitted"
    and .wall.pairs_below_level == 0
    and .wall.total_wall_ratio < 1
    and .wall.total_wall_ci95[1] < 1
    and .wall.corrected_ratio < 1
    and .wall.corrected_ci95[1] < 1
    and .wall.outside_noise == true
    and .wall.aa_ci95[0] <= 1
    and .wall.aa_ci95[1] >= 1
    and .wall.aa_ci95[0] >= 0.9
    and .wall.aa_ci95[1] <= 1.1
  ' "$prefix-candidate.json" >/dev/null

  jq -e '
    .ops.status == "ok"
    and .ops.ratio_b_over_a == 1
    and .wall.status == "admitted"
    and .wall.ci95[0] <= 1
    and .wall.ci95[1] >= 1
    and .wall.ci95[0] >= 0.9
    and .wall.ci95[1] <= 1.1
  ' "$prefix-aa.json" >/dev/null

  jq -e '
    .ops.status == "ok"
    and .ops.same_seeds == true
    and .wall.status == "admitted"
    and .wall.total_wall_ratio > 1
    and .wall.total_wall_ci95[0] > 1
  ' "$prefix-positive.json" >/dev/null
done

jq -n --argjson sessions "$session_number" \
  '{schema:"ic.line-layout.analysis/v1", verdict:"supported_within_scope", sessions:$sessions,
    rule:"every independent L2 session passed replay, exact counted-cost parity, A/A, positive-control, corrected wall and full total-wall CI gates"}' \
  > "$out/verdict.json"
