#!/usr/bin/env bash
# Thin orchestration of phase5/PROTOCOL.md section 4. Create-only and resumable: a finished
# session (runs/<id>/<kind>_s<s>/done) is skipped; a failed one keeps its logs in
# <dir>.partial and is reported FAILED, never retried.
# Usage: run_phase5.sh <ecbench> <fp31-class-census> <classes.json> <runs dir>
set -uo pipefail
E=$1; T=$2; CL=$3; RUNS=$4
for id in $(grep -o '"id": "c[0-9]*"' "$CL" | sed 's/.*"\(c[0-9]*\)"/\1/'); do
  for spec in rho:8:8 ic3:4:2 ic2:4:2; do
    kind=${spec%%:*}; rest=${spec#*:}; S=${rest%%:*}; TG=${rest##*:}
    for ((s=0; s<S; s++)); do
      d="$RUNS/$id/${kind}_s$s"
      if [ -e "$d/done" ]; then echo "skip $id $kind s$s"; continue; fi
      rm -rf "$d.partial"; mkdir -p "$d.partial"
      t0=$(date +%s)
      if "$T" p5-spec --classes "$CL" --class "$id" --kind "$kind" --session "$s" --targets "$TG" --seed 20261113 --warmup 0 \
            --out "$d.partial/spec.json" > "$d.partial/spec.log" 2>&1 \
         && "$E" run --wait --spec "$d.partial/spec.json" --out "$d.partial/session" > "$d.partial/run.log" 2>&1 \
         && "$E" verify --dir "$d.partial/session" --replay 2 --out "$d.partial/audit.json" > "$d.partial/verify.log" 2>&1; then
        echo "$(( $(date +%s) - t0 ))" > "$d.partial/seconds"
        touch "$d.partial/done"; mv "$d.partial" "$d"; echo "ok $id $kind s$s $(cat "$d/seconds")s"
      else
        echo "FAILED $id $kind s$s (logs kept in $d.partial)"
      fi
    done
  done
done
