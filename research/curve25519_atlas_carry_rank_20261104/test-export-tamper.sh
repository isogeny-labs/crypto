#!/usr/bin/env bash
# Metadata canaries only. Never modify the retained source run.
set -euo pipefail
controller=${1:?controller binary}
unit=${2:?completed unit-example2 directory}
out=${3:?fresh canary directory}
test ! -e "$out"
mkdir "$out"
controller=$(realpath "$controller")
unit=$(realpath "$unit")
out=$(realpath "$out")
for case in raw receipt counter source codec columns deadline; do
  scratch=$(mktemp -d /tmp/atlas-export-canary.XXXXXX)
  cp -a "$unit/inputs" "$unit/solver-run" "$scratch/"
  cp "$unit/protocol.json" "$scratch/protocol.json"
  rm "$scratch/solver-run/export-host.json"
  case "$case" in
    raw) printf '\nforged\n' >> "$scratch/solver-run/0000.solver.out" ;;
    receipt)
      jq '.solver_rlimit += 1' "$scratch/solver-run/0000.json" > "$scratch/changed.json"
      mv "$scratch/changed.json" "$scratch/solver-run/0000.json" ;;
    counter)
      jq '.rows[0].solver_rlimit += 1' "$scratch/solver-run/summary.json" > "$scratch/changed.json"
      mv "$scratch/changed.json" "$scratch/solver-run/summary.json"
      jq '.rows[0]' "$scratch/solver-run/summary.json" > "$scratch/solver-run/0000.json" ;;
    source|codec|columns)
      case "$case" in
        source) change='.source_revision = ("0" * 40)' ;;
        codec) change='.codec = "nia"' ;;
        columns) change='.columns += 1' ;;
      esac
      jq "$change" "$scratch/inputs/configuration.json" > "$scratch/changed.json"
      mv "$scratch/changed.json" "$scratch/inputs/configuration.json"
      jq --slurpfile c "$scratch/inputs/configuration.json" '.identity.configuration = $c[0]' "$scratch/solver-run/summary.json" > "$scratch/changed.json"
      mv "$scratch/changed.json" "$scratch/solver-run/summary.json"
      jq '.identity' "$scratch/solver-run/summary.json" > "$scratch/solver-run/identity.json" ;;
    deadline)
      jq '.identity.timeout_seconds += 1' "$scratch/solver-run/summary.json" > "$scratch/changed.json"
      mv "$scratch/changed.json" "$scratch/solver-run/summary.json"
      jq '.identity' "$scratch/solver-run/summary.json" > "$scratch/solver-run/identity.json" ;;
  esac
  status=0
  "$controller" export "$scratch/inputs" "$scratch/solver-run" "$scratch/protocol.json" "$scratch/export.json" > "$out/$case.stdout.txt" 2> "$out/$case.stderr.txt" || status=$?
  printf '%s\t%s\t%s\n' "$case" "$status" "$scratch" >> "$out/results.tsv"
  test "$status" = 2
  test ! -e "$scratch/export.json"
done
printf '7 metadata tamper canaries rejected (producer observation)\n'
