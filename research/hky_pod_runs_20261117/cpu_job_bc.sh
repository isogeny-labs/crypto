#!/usr/bin/env bash
# Stages B and C of cpu_job.sh again: the first attempt failed at once (exit 127), the image has no /usr/bin/time.
set -uo pipefail
W=/workspace; O=$W/out; B=$W/in/pilot_candidate; mkdir -p $O/t13 $O/t14
exec > >(tee -a $O/cpu_job_bc.log) 2>&1
CELL="--p 2013265921 --ladder-t none --planted 0 --engine hky2 --hky-max-cols 2000000 --hky-max-entries 14000000000 --kinds kummer --m 2 --controls tower"
rss() { while kill -0 $1 2>/dev/null; do echo "$(date -u +%T) $(grep VmHWM /proc/$1/status 2>/dev/null)"; sleep 60; done > $2; }
echo "=== B t = 13, draws 0 and 1, c <= 6 $(date -u +%FT%TZ)"; uptime
$B $CELL --random 2 --hky-cmax 6 --budget 14400 --t-min 13 --max-t 13 --hky-progress $O/t13/progress.jsonl --out $O/t13/hky2.jsonl > $O/t13/run.log 2>&1 & pid=$!; rss $pid $O/t13/rss.log & wait $pid; echo "t13 rc=$? $(date -u +%FT%TZ)"; tail -1 $O/t13/rss.log
echo "=== C t = 14, draw 0, c <= 6 $(date -u +%FT%TZ)"; uptime
$B $CELL --random 1 --hky-cmax 6 --budget 72000 --t-min 14 --max-t 14 --hky-progress $O/t14/progress.jsonl --out $O/t14/hky2.jsonl > $O/t14/run.log 2>&1 & pid=$!; rss $pid $O/t14/rss.log & wait $pid; echo "t14 rc=$? $(date -u +%FT%TZ)"; tail -1 $O/t14/rss.log
echo "=== ALL DONE"
