#!/usr/bin/env bash
# shellcheck disable=SC2329 # Trap handlers are invoked indirectly.
# Thin, signal-aware wrapper for a single bounded validation command.
set -u
if test "$#" -lt 4 || test "$3" != --; then
  echo 'usage: run-with-receipt.sh ABS_LOG ABS_RECEIPT -- PROGRAM [ARGS...]' >&2
  exit 2
fi
run_log=$1
run_receipt=$2
shift 3
case "$run_log:$run_receipt" in
  /*:/*) ;;
  *) echo 'absolute log and receipt paths required' >&2; exit 2 ;;
esac
if test -e "$run_log" || test -e "$run_receipt"; then
  echo 'log and receipt paths must be new' >&2
  exit 2
fi
started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
stop_signal=none
child_exit=unknown
child_pid=0

finish() {
  wrapper_exit=$?
  trap - EXIT
  ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
  if command -v sha256sum >/dev/null 2>&1; then
    log_sha256=$(sha256sum "$run_log" | awk '{print $1}')
  else
    log_sha256=$(shasum -a 256 "$run_log" | awk '{print $1}')
  fi
  log_bytes=$(wc -c < "$run_log" | tr -d ' ')
  printf 'schema=n83.validation-command-receipt/v1\nstarted_utc=%s\nended_utc=%s\nwrapper_exit_code=%s\nchild_exit_code=%s\nstop_signal=%s\nlog_bytes=%s\nlog_sha256=%s\n' \
    "$started_utc" "$ended_utc" "$wrapper_exit" "$child_exit" "$stop_signal" "$log_bytes" "$log_sha256" > "$run_receipt"
}
stop() {
  stop_signal=$1
  trap - HUP INT TERM
  if test "$child_pid" -gt 0; then
    kill -TERM -- "-$child_pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$child_pid" 2>/dev/null || true
    wait "$child_pid" 2>/dev/null || true
  fi
  case "$stop_signal" in
    HUP) exit 129 ;;
    INT) exit 130 ;;
    TERM) exit 143 ;;
  esac
}
trap finish EXIT
trap 'stop HUP' HUP
trap 'stop INT' INT
trap 'stop TERM' TERM
set -m
"$@" > "$run_log" 2>&1 &
child_pid=$!
wait "$child_pid"
child_exit=$?
exit "$child_exit"
