#!/bin/bash
# Compare the before/after audits: only the audit time and the auditor binary hash may differ.
cd "$(dirname "$0")"
for f in before/*.json; do n=$(basename $f)
  d=$(diff <(jq -S 'del(.audited_unix_ms, .auditor_binary_sha256)' before/$n) <(jq -S 'del(.audited_unix_ms, .auditor_binary_sha256)' after/$n) | wc -l)
  echo "$n: $(jq -r '"\(.records) records, \(.verified_records) verified, \([.replays[]|select(.reproduced)]|length)/\(.replays|length) replays reproduced, problems=\(.problems|length)"' after/$n); diff lines (excluding time and auditor hash) = $d"
done
