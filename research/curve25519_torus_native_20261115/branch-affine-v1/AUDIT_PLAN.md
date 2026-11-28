# Native post-call reader v1 — before calls

Read-only Rust aggregation of the already frozen core evidence, no new search,
timing, sampling or paid run. Inputs: exact controls.json SHA-256
de00f0bade0c248a8d771ee66a19791e3026246d8e3300c878c3513aa4b73748 and
NATIVE_ARCHIVE.json SHA-256
587963272adbb808ebaa90b164ac308af5fb71ac88ca46fad476d00e653610dd.

Require all retained raw member sizes/hashes. For every declared small/curve
case compare complete recorded old-v1/disabled answer, decisions, counters and
schedule; compare report summaries with the per-case artifacts. Native code
computes outcome totals and matching root node counts, preserving cap reasons
and per-input counts. A capped shorter trace never counts as a gain. Require
exactly28 curve/five small cases; root checking still belongs to the frozen
coefficient/lifetime/original/group controls, not this aggregation reader.

Compile with the existing locked serde_json/sha2 native rlibs, pin their actual
paths/digests plus rustc identity/source/binary before calls. Copy an immutable
executable. Output must be a NEW directory and retains pre-call/result plus a
whitespace-corrupted controls copy that must fail the hash guard. Native output
and full log are archived as one separate audit round; core bytes stay unchanged.
