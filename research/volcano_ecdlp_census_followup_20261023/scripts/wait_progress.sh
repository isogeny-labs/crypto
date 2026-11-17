#!/bin/bash
# usage: TARGET=<n> bash scripts/wait_progress.sh
# Blocks until runs/progress.jsonl has at least TARGET distinct done specs, or the driver is not alive,
# or 500 s have passed; then prints the one JSON line of scripts/phase_b_status.sh and exits 0.
# The 500 s cap leaves margin under the 540 s a foreground command may run.
STUDY=${STUDY:-/home/user/crypto/research/volcano_ecdlp_census_followup_20261023}
CAP=500; STEP=10
case "${TARGET:-}" in ''|*[!0-9]*|0) echo "WAIT_ERROR: TARGET must be a positive integer, got '${TARGET:-}'" >&2; exit 2;; esac
SECONDS=0
while :; do
  line=$(bash "$STUDY/scripts/phase_b_status.sh") || exit 2
  done_n=$(jq -r .sessions_done <<<"$line"); alive=$(jq -r .driver_alive <<<"$line")
  if [ "$done_n" -ge "$TARGET" ] || [ "$alive" != true ] || [ $SECONDS -ge $CAP ]; then break; fi
  left=$((CAP-SECONDS)); [ $left -lt $STEP ] && sleep $left || sleep $STEP
done
echo "$line"
