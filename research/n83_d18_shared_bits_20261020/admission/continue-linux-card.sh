#!/bin/bash
# Operational continuation only. The frozen run-card remains unchanged.
set -euo pipefail
root=/home/ec2-user/n83-shared-bits-20261024
source=/var/tmp/n83-shared-bits-20261024
card="$root/card-v1"
status="$root/continuation-status-v4.txt"
admission_started=$SECONDS
# The successful validator execs this continuation with its held lock on fd 9.
# Reopening the lock here would race other queued benchmark jobs.
flock -n 9
finish() {
    local code=$?
    if [ "$code" -ne 0 ]; then printf 'FAILED exit=%s\n' "$code" > "$status"; fi
}
trap finish EXIT
test "$(cat "$root/validation-status-v4.txt")" = NATIVE_AND_BOUNDED_CONSTRUCTION_PASS
test -z "$(sudo -n docker ps -q)"
printf '%s\n' PREPARING_HOST_CARD > "$status"
sh "$source/research/n83_d18_shared_bits_20261020/prepare-card.sh" \
    "$source" "$root/build-v4" "$root/target/release/examples" \
    "$root/construction" "$root/proof" "$card"
"$root/target/release/examples/n83_d18_shared_card_audit" \
    freeze "$card" "$source" "$card/host-card.json"
sha256sum "$card/host-card.json" > "$root/host-card.sha256"
# A two-field message is sent only after the actual host card is committed and pushed.
# Release the shared lock while the local session publishes the card. All
# admission waits share one four-hour limit; the frozen run has its own caps.
mkfifo "$root/commit-admission-v4.fifo"
exec 8<>"$root/commit-admission-v4.fifo"
flock -u 9
printf '%s\n' AWAITING_COMMITTED_HOST_CARD > "$status"
remaining=$((14400-SECONDS+admission_started))
if [ "$remaining" -le 0 ] || ! IFS=' ' read -r -t "$remaining" revision expected_hash <&8; then
    printf '%s\n' BLOCKED_HOST_CARD_NOT_COMMITTED > "$status"
    trap - EXIT
    exit 75
fi
[[ "$revision" =~ ^[0-9a-f]{40}$ ]]
[[ "$expected_hash" =~ ^[0-9a-f]{64}$ ]]
test "$(sha256sum "$card/host-card.json" | cut -d ' ' -f 1)" = "$expected_hash"
printf '%s %s\n' "$revision" "$expected_hash" > "$root/committed-host-card.txt"
remaining=$((14400-SECONDS+admission_started))
printf '%s\n' WAITING_FOR_EXECUTION_LOCK > "$status"
if [ "$remaining" -le 0 ] || ! flock -w "$remaining" 9; then
    printf '%s\n' BLOCKED_EXECUTION_LOCK > "$status"
    trap - EXIT
    exit 75
fi
test -z "$(sudo -n docker ps -q)"
printf '%s\n' RUNNING_FROZEN_CARD > "$status"
result=0
sh "$card/run-card.sh" "$card" > "$root/card.log" 2>&1 || result=$?
printf '%s\n' "$result" > "$root/card-exit.txt"
printf '%s\n' COLLECTING_CARD_RECEIPTS > "$status"
if [ -d "$card/runs" ] && [ -s "$card/card-status.txt" ]; then
    sh "$root/collect-card.sh" "$card" "$root/collected-v1" > "$root/collection.log" 2>&1
fi
if [ "$result" != 0 ]; then
    printf 'CARD_FAILED exit=%s; retained in collected-v1\n' "$result" > "$status"
    trap - EXIT
    exit "$result"
fi
printf '%s\n' COMPLETE_CARD_COLLECTED_PENDING_INDEPENDENT_AUDIT > "$status"
