#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md section 4: one walk and two measure runs per
# manifest class, sequentially (measure parallelises solves itself). Create-only:
# an existing runs/<id>/ directory is skipped, never overwritten, so the script
# resumes after an interruption without touching finished classes.
# Usage: run_phase2.sh <isoclass-rho binary> <manifest.json> <runs dir> <TC> <TG>
set -uo pipefail
BIN=$1; MAN=$2; RUNS=$3; TC=$4; TG=$5
mkdir -p "$RUNS"
get() { sed -n "s/.*\"$1\":\"\{0,1\}\([^,\"}]*\).*/\1/p" <<<"$2"; }
grep '^{' "$MAN" | while IFS= read -r line; do
  id=$(get id "$line"); p=$(get p "$line"); a=$(get a "$line"); b=$(get b "$line"); s=$(get seed "$line")
  d="$RUNS/$id"
  if [ -e "$d/done" ]; then echo "skip $id"; continue; fi
  rm -rf "$d.partial"; mkdir -p "$d.partial"
  t0=$(date +%s)
  if "$BIN" walk --curve "$p,$a,$b" --ells 2,3,5,7 --radius 3 --max-vertices 400 --per-stratum 4 --hard 0 \
        --seed "$s" --out "$d.partial/walk" > "$d.partial/walk.log" 2>&1 \
     && "$BIN" measure --walk "$d.partial/walk/walk.json" --mode class --trials "$TC" --max-vertices 6 \
        --controls 2 --plants --folds --seed "$s" --out "$d.partial/class" > "$d.partial/class.log" 2>&1 \
     && "$BIN" measure --walk "$d.partial/walk/walk.json" --mode generators --generators 10 --trials "$TG" \
        --seed "$s" --out "$d.partial/gen" > "$d.partial/gen.log" 2>&1; then
    echo "$(( $(date +%s) - t0 ))" > "$d.partial/seconds"
    touch "$d.partial/done"; mv "$d.partial" "$d"; echo "ok $id $(cat "$d/seconds")s"
  else
    echo "FAILED $id (logs kept in $d.partial)"
  fi
done
