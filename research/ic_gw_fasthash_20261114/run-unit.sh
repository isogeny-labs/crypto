#!/usr/bin/env bash
# usage: run-unit.sh BIN Q SEED  -- runs one ic unit, prints its JSONL, then an rss record (peak VmHWM, kB,
# polled every 0.2 s from /proc/<pid>/status; the last value seen before exit; the driver is single-threaded).
"$1" ic "$2" "$3" D 3.0 1.7 &
pid=$!; peak=0
while kill -0 $pid 2>/dev/null; do
  r=$(awk '/VmHWM/{print $2}' /proc/$pid/status 2>/dev/null); [ -n "$r" ] && peak=$r; sleep 0.2
done
wait $pid; rc=$?
echo "{\"rec\":\"rss\",\"vmhwm_kb\":$peak,\"rc\":$rc}"
