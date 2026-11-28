#!/usr/bin/env bash
# Thin shell transport only. Arithmetic, controls and output ownership are Rust.
set -Ceu
if [ "$#" -ne 4 ]; then
  echo 'usage: run-control.sh IMMUTABLE_BINARY MODE NEW_RAW_JSONL NEW_LOG_PREFIX' >&2
  exit 2
fi
query_binary=$1
query_mode=$2
query_raw=$3
query_logs=$4
set +e
timeout --signal=TERM 180s "$query_binary" "$query_mode" "$query_raw" > "${query_logs}.stdout.log" 2> "${query_logs}.stderr.log"
query_exit=$?
set -e
printf '%s\n' "$query_exit" > "${query_logs}.exit.txt"
exit "$query_exit"
