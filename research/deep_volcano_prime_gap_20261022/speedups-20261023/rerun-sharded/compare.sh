#!/usr/bin/env bash
# Compare the sharded re-run's records with the original run's, by
# (arm, workload, target, round): counted cost, recovered value, counters.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
orig="$here/../../run/sessions"
key='[.arm, .workload.workload_id, .workload.target_index, .round, .warmup, .cost.total_gae, .outcome.recovered, (.counters|tojson)] | @tsv'
zcat "$orig"/{spec,spec-aa,spec-bsgs}/records.jsonl.gz | jq -r "$key" | sort > "$here/original.tsv"
cat "$here"/sessions/*/records.jsonl | jq -r "$key" | sort > "$here/sharded.tsv"
echo "original: $(wc -l < "$here/original.tsv") records; sharded: $(wc -l < "$here/sharded.tsv") records"
if cmp -s "$here/original.tsv" "$here/sharded.tsv"; then echo "IDENTICAL"; else echo "DIFFERENT"; diff "$here/original.tsv" "$here/sharded.tsv" | head; exit 1; fi
