#!/usr/bin/env bash
set -u
session=n83-root-lib-warm-20261115
receipt=/private/tmp/n83-root-lib-warm-20261115.receipt
marker=/private/tmp/n83-root-lib-warm-20261115.disk-stop
while ! test -e "$receipt"; do
  boot_kib=$(df -Pk /private/tmp | awk 'END{print $4}')
  ssd_kib=$(df -Pk /Volumes/SSD990-2 | awk 'END{print $4}')
  if test "$boot_kib" -lt 1572864 || test "$ssd_kib" -lt 1572864; then
    printf 'STOP_DISK_PRESSURE utc=%s boot_kib=%s ssd_kib=%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" "$boot_kib" "$ssd_kib" > "$marker"
    tmux send-keys -t "$session" C-c
    exit 0
  fi
  if ! tmux has-session -t "$session" 2>/dev/null; then
    printf 'MISSING_SESSION_WITHOUT_RECEIPT utc=%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" > "$marker"
    exit 1
  fi
  sleep 15
done
