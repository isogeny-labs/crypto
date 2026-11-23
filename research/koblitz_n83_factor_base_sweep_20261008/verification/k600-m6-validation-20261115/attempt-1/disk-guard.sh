#!/usr/bin/env bash
set -u
while ! test -f /private/tmp/n83-checkpoint-isolated-20261115.exit; do
  available_kib=$(df -Pk /private/tmp | awk 'END {print $4}')
  if test -n "$available_kib" && test "$available_kib" -lt 1572864; then
    printf 'STOP_DISK_PRESSURE available_kib=%s\n' "$available_kib" > /private/tmp/n83-checkpoint-isolated-20261115.disk-stop
    tmux send-keys -t n83-checkpoint-isolated-20261115 C-c
    exit 0
  fi
  sleep 15
done
