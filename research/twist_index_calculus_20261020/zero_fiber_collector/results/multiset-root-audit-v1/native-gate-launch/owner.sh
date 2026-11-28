#!/usr/bin/env bash
set -Eeuo pipefail
cd /home/ec2-user/crypto-twist-zero-fiber
multiset_receipt=research/twist_index_calculus_20261020/zero_fiber_collector/results/multiset-root-audit-v1
multiset_gate=research/twist_index_calculus_20261020/zero_fiber_collector/runs/multiset-native-gate-v1
multiset_command=(python3 tools/isolated_bench.py busy -- prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s bash research/twist_index_calculus_20261020/zero_fiber_collector/multiset_root_audit/run-native-gate.sh "$multiset_gate")
test ! -e "$multiset_receipt/native-gate.exit"
printf '%q ' "${multiset_command[@]}" > "$multiset_receipt/native-gate.command.txt"
date -u +%FT%TZ > "$multiset_receipt/native-gate.queued-utc.txt"
if "${multiset_command[@]}" > "$multiset_receipt/native-gate.stdout" 2> "$multiset_receipt/native-gate.stderr"; then multiset_rc=0; else multiset_rc=$?; fi
printf '%s\n' "$multiset_rc" > "$multiset_receipt/native-gate.exit"
exit "$multiset_rc"
