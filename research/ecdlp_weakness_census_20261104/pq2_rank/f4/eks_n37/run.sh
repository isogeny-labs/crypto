#!/bin/bash
# Runs msolve (F4, grevlex, -g 2 -v 2) on /opt/msolve/input.ms inside the isolab
# job, logs memory every 30 s, and uploads everything to $ISOLAB_RESULTS_URI,
# with a partial upload every 10 minutes so an OOM kill still leaves a record.
# argv: run.sh THREADS
set -u
threads=${1:-1}
D=${MSOLVE_DIR:-/opt/msolve}
OUT=${MSOLVE_OUT:-/work/out}
mkdir -p "$OUT"
upload() { aws s3 cp "$OUT/" "${ISOLAB_RESULTS_URI}" --recursive --only-show-errors || echo "upload failed" >&2; }
{
  date -u +%FT%TZ
  echo "threads=$threads"
  sha256sum "$D/msolve" "$D/input.ms" "$D/run.sh" "$D"/lib/*
  grep -E "MemTotal|MemAvailable" /proc/meminfo
  nproc
} > "$OUT/env.txt" 2>&1
start=$(date +%s)
"$D/lib/ld-linux-x86-64.so.2" --library-path "$D/lib" "$D/msolve" \
  -f "$D/input.ms" -g 2 -v 2 -t "$threads" -o "$OUT/gb.out" > "$OUT/msolve.stdout" 2> "$OUT/msolve.stderr" &
pid=$!
last_up=$start
while kill -0 "$pid" 2>/dev/null; do
  now=$(date +%s)
  rss=$(awk '/VmRSS|VmHWM/{printf "%s=%s ", $1, $2}' "/proc/$pid/status" 2>/dev/null)
  avail=$(awk '/MemAvailable/{print $2}' /proc/meminfo)
  echo "$((now - start)) $rss MemAvailable_kB=$avail" >> "$OUT/mem.log"
  if (( now - last_up >= 600 )); then upload; last_up=$now; fi
  sleep 30
done
wait "$pid"; code=$?
end=$(date +%s)
printf '{"exit_code": %d, "seconds": %d}\n' "$code" "$((end - start))" > "$OUT/exit.json"
sha256sum "$OUT"/* > "$OUT/SHA256SUMS" 2>/dev/null
upload
exit "$code"
