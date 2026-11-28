#!/usr/bin/env bash
# Thin bounded transport; all arithmetic, control selection and raw output are Rust.
set -Ceu
if [ "$#" -ne 4 ]; then
  echo 'usage: run-control.sh IMMUTABLE_BINARY MODE NEW_RAW_JSONL NEW_LOG_PREFIX' >&2
  exit 2
fi
radical_binary=$1
radical_mode=$2
radical_raw=$3
radical_logs=$4
ulimit -v 2097152
set +e
timeout --signal=TERM 180s "$radical_binary" "$radical_mode" "$radical_raw" > "${radical_logs}.stdout.log" 2> "${radical_logs}.stderr.log"
radical_exit=$?
set -e
printf '%s\n' "$radical_exit" > "${radical_logs}.exit.txt"
exit "$radical_exit"
