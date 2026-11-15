#!/usr/bin/env bash
set -euo pipefail
# ROOT and executable absolute paths are supplied explicitly. Never retry.
root=${1:?study root}
binary=${2:?frozen binary}
rep=${3:?replicate 0 or 1}
[[ "$rep" == 0 || "$rep" == 1 ]]
test ! -e "$root/runs/$rep"
mkdir -p "$root/runs/$rep"
out="$root/runs/$rep"
start=$(date -u +%FT%TZ)
cpu=$(awk '/Cpus_allowed_list/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
set +e
(ulimit -v 2097152; exec taskset -c "$cpu" timeout 900 "$binary" --panel) > "$out/panel.csv" 2> "$out/stderr.txt"
status=$?
set -e
finish=$(date -u +%FT%TZ)
jq -n --arg start "$start" --arg finish "$finish" --arg worker "local-cpu$cpu" --argjson code "$status" '{started_at:$start,finished_at:$finish,worker:$worker,exit_code:$code}' > "$out/receipt.json"
exit "$status"
