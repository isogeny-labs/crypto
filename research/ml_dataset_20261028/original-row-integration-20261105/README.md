# Original-owner Boolean row integration checkpoint

Status: compiled; runtime gate FAILED. This local checkpoint is not eligible for push under the required test gate. The previous SAT CLI PR remains at its separately tested source.

The reviewed11-file composition compiled with unchanged source. Exact61-test batch completed54pass/7fail, no unattempted tests, unchanged source/binary/inventory. All seven failures require genuine n23,m3 row expansion and stop with Expansion(Unknown(Cap)); they are not weakened or relabeled successes. Existing49shared regressions and5new nonpositive/receipt controls passed.

The failed actual map has115variables,46XORs,46units,129domain clauses and noAND gates. A native run of the unchanged sparse kernel on this map confirms the first local cap at Domain row5/literal13 after97equations:3994150work and1514872bytes; next comparison payment exceeds4000000work. This diagnostic uses an always-admitting outer ledger to isolate the local cap, not a new original-owner success. The full raw domain expansion diagnostic predicts76281monomials; current per-term allocations exceed the local8MB bound. Geometric growth alone is insufficient.

Next: cap-preserving compact domain encoding with authenticated equivalence and versioned generator binding. Do not claim the new encoding preserves the raw-domain unit span or degree filtration. Positive tests, original caps and full requested dataset scope remain unchanged. Reviewed post-ack deadline and paid-report telemetry fault controls remain external/unexecuted; dispatch proposal is uncompiled and mandatory replay/export remains outstanding.

Separately, isolated actual native XOR propagation passed7tests with rebuilt dependency hashes recorded. Source/dependencies unchanged. This is not production dataset routing. All failedrowfixtures, logs and receipts and XORsource/testbinary have verified S3 archives referenced here.
