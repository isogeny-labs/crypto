# Version 2 source-check correction, 2026-10-28

Run n83-prime-m2-hilbert-20261028-r1 failed closed on the first original planted
source: the complete template omitted its existing witness map and check line.
Both producer and replay exited101; the launcher exited1. No input certificate
was accepted. The field/control/Hilbert calculation completed before that check,
and remains partial data, not a certified source result. All r1 raw files and
frozen source/protocol are preserved unchanged in their original S3 prefix and
separate r1 archive. Five later cases were not reached.

The v2 instrument is a new hilbert_v2.rs; the original hilbert.rs remains frozen.
It additionally requires exactly the planted witness suffix reconstructed from
54 Boolean witness values in the unchanged input metadata. Natural inputs must
have null witness and no suffix. No input polynomial is changed; extra executable
lines are still rejected. Two additional native tests exercise a complete valid
planted template and reject an extra ideal mutation or non-Boolean witness bit.

The algebra, rank formulas, controls, six input hashes, limits and full-data
retention policy are unchanged. The new version is filed after the failed r1
attempt and before any v2 data. No selective target or case replacement is made.
This is an explicit amended run, not an infrastructure retry under v1's no-retry
rule. The descriptive report must include r1 failure and r2 outcome together.

The original HILBERT_DESIGN.md, HILBERT_FORMAT.json and arithmetic proof remain
applicable, with protocol-hilbert-v2.json and hilbert_v2 executable names in r2.
