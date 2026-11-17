#!/usr/bin/env bash
# Validator driver (thin orchestration; arithmetic in the .jq files beside it).
#
#   research/fes_then_batched_20261112/validation/validate.sh [REPLAY_DIR]
#
# Without REPLAY_DIR: recompute from the committed raw reports
#   recompute.json, headline.json  (recompute.sh: recompute.jq, headline.jq)
#   band.json                      (band.jq)
#   counters.json                  (counters.jq, the AGENTS.md 5a listing)
#   instance-check.json            (instance_check.jq)
# With REPLAY_DIR (the output directory of
#   IC_BIN=<validator binary> research/fes_then_batched_20261112/run_e2e.sh REPLAY_DIR):
#   replay/<name>.json             copy of the replay report
#   replay/compare-<name>.json     compare_replay.jq, committed vs replay
#   replay/recompute-replay.json   recompute.jq on the replay reports (GAE of the
#                                  measured-calibration rows re-derived from counts
#                                  at the ratio the replay recorded)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
study="$(cd "$here/.." && pwd)"
names="koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23"
"$here/recompute.sh"
jq -f "$here/band.jq" "$here/recompute.json" >"$here/band.json"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
for n in $names; do
  jq --arg name "$n" -f "$here/counters.jq" "$study/results/$n.json" >"$tmp/c-$n.json"
  jq --arg name "$n" -f "$here/instance_check.jq" "$study/results/$n.json" >"$tmp/i-$n.json"
done
jq -s '.' $(for n in $names; do echo "$tmp/c-$n.json"; done) >"$here/counters.json"
jq -s '.' $(for n in $names; do echo "$tmp/i-$n.json"; done) >"$here/instance-check.json"
if [ $# -ge 1 ]; then
  replay="$1"
  mkdir -p "$here/replay"
  for n in $names; do
    cp "$replay/$n.json" "$here/replay/$n.json"
    jq -n --arg name "$n" --slurpfile a "$study/results/$n.json" --slurpfile b "$here/replay/$n.json" \
      -f "$here/compare_replay.jq" >"$here/replay/compare-$n.json"
    jq --arg name "$n" -f "$here/recompute.jq" "$here/replay/$n.json" >"$tmp/r-$n.json"
  done
  cp "$replay/ic-sha256.txt" "$here/replay/ic-sha256.txt"
  jq -s '.' $(for n in $names; do echo "$tmp/r-$n.json"; done) >"$here/replay/recompute-replay.json"
fi
echo "validate.sh: done"
