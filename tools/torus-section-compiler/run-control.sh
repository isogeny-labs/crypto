#!/usr/bin/env bash
# Thin bounded transport. Rust chooses controls and writes the full raw stream.
set -Ceu
if [ "$#" -ne 5 ]; then
  echo 'usage: run-control.sh IMMUTABLE_BINARY MODE NEW_RAW NEW_EXPORT_DIR NEW_LOG_PREFIX' >&2
  exit 2
fi
compiler_binary=$1
compiler_mode=$2
compiler_raw=$3
compiler_exports=$4
compiler_logs=$5
ulimit -v 2097152
set +e
timeout --signal=TERM 600s "$compiler_binary" "$compiler_mode" "$compiler_raw" "$compiler_exports" > "${compiler_logs}.stdout.log" 2> "${compiler_logs}.stderr.log"
compiler_exit=$?
set -e
printf '%s\n' "$compiler_exit" > "${compiler_logs}.exit.txt"
exit "$compiler_exit"
