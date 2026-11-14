#!/usr/bin/env bash
# Thin orchestration of phase4/PROTOCOL.md section 4: per class of classes.json,
# rho-spec -> ecbench run -> ecbench verify. Create-only and resumable: a finished
# class (runs/<id>/done) is skipped; a failed one keeps its logs in runs/<id>.partial
# and is reported FAILED, never retried.
# Usage: run_phase4.sh <ecbench> <fp31-class-census> <classes.json> <runs dir>
set -uo pipefail
E=$1; T=$2; CL=$3; RUNS=$4
mkdir -p "$RUNS"
for id in $(grep -o '"id": "c[0-9]*"' "$CL" | sed 's/.*"\(c[0-9]*\)"/\1/'); do
  d="$RUNS/$id"
  if [ -e "$d/done" ]; then echo "skip $id"; continue; fi
  rm -rf "$d.partial"; mkdir -p "$d.partial"
  t0=$(date +%s)
  if "$T" rho-spec --classes "$CL" --class "$id" --targets 64 --rounds 2 --seed 20261112 --control rho.negation \
        --out "$d.partial/spec.json" > "$d.partial/spec.log" 2>&1 \
     && "$E" run --wait --spec "$d.partial/spec.json" --out "$d.partial/session" > "$d.partial/run.log" 2>&1 \
     && "$E" verify --dir "$d.partial/session" --replay 4 --out "$d.partial/audit.json" > "$d.partial/verify.log" 2>&1; then
    echo "$(( $(date +%s) - t0 ))" > "$d.partial/seconds"
    touch "$d.partial/done"; mv "$d.partial" "$d"; echo "ok $id $(cat "$d/seconds")s"
  else
    echo "FAILED $id (logs kept in $d.partial)"
  fi
done
