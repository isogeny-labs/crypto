#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md section 4: for every class of classes.json and
# m in {3, 2} (m = 3 first), p2tool spec -> ecbench run -> ecbench verify -> p2tool analyze.
# Create-only and resumable: a finished cell (runs/<id>_m<m>/done) is skipped; a failed
# cell keeps its logs in runs/<id>_m<m>.partial and is reported FAILED, never retried.
# Usage: run_phase3.sh <ecbench> <p2tool> <classes.json> <runs dir>
set -uo pipefail
E=$1; P=$2; CL=$3; RUNS=$4
mkdir -p "$RUNS"
ids=$(grep -o '"id": "c[0-9]*"' "$CL" | sed 's/.*"\(c[0-9]*\)"/\1/')
for m in 3 2; do
  Y=0.02; [ "$m" = 3 ] && Y=0.1
  for id in $ids; do
    d="$RUNS/${id}_m$m"
    if [ -e "$d/done" ]; then echo "skip $id m=$m"; continue; fi
    rm -rf "$d.partial"; mkdir -p "$d.partial"
    t0=$(date +%s)
    if "$P" spec --classes "$CL" --class "$id" --m "$m" --y "$Y" --rounds 2 --targets 2 --aa 0 --isolation L1 \
          --seed 20261112 --out "$d.partial/spec.json" > "$d.partial/spec.log" 2>&1 \
       && "$E" run --wait --allow-busy --spec "$d.partial/spec.json" --out "$d.partial/session" > "$d.partial/run.log" 2>&1 \
       && "$E" verify --dir "$d.partial/session" --replay 4 --out "$d.partial/audit.json" > "$d.partial/verify.log" 2>&1 \
       && "$P" analyze --classes "$CL" --class "$id" --m "$m" --session "$d.partial/session" --out "$d.partial/analysis.json" > "$d.partial/analyze.log" 2>&1; then
      echo "$(( $(date +%s) - t0 ))" > "$d.partial/seconds"
      touch "$d.partial/done"; mv "$d.partial" "$d"; echo "ok $id m=$m $(cat "$d/seconds")s"
    else
      echo "FAILED $id m=$m (logs kept in $d.partial)"
    fi
  done
done
