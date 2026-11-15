#!/bin/sh
# Normalise an ecbench records.jsonl for an identity diff: drop only the
# wall-time fields (keys ending _ns/_ms, ns_per_*), timestamps,
# host/isolation, the binary hash and the session/record ids.
jq -cS 'del(.binary_sha256, .isolation, .record_id, .session_id, .time, .online.phases_ns)
        | walk(if type=="object" then with_entries(select(.key | test("(_ns|_ms)$|^ns_per_|^started_at$|^finished_at$") | not)) else . end)' "$1" | sort
