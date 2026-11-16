# Raw row stream format

The worker's file is named rows.jsonl, but its jq enrichment emitted pretty
printed consecutive JSON objects. Preserve the exact raw bytes. Native
analyze.cpp reads a stream of JSON values with operator>>, accepting both
single-line and whitespace-formatted objects. Equivalent hand-written input
formats give byte-equal summaries in the development check.

A parser failure does not establish an algorithm failure or a miss. An
unfinished final JSON object is explicitly flagged if detected; malformed
input otherwise causes a nonzero analysis exit. No raw file is rewritten.
The process-times.jsonl companion uses compact JSON lines as named.
