# Native projection delivery and bounded terminal output

2026-10-31. Integration after source 6bbc3fb18ec8 passed 15 native-event tests and 40 capture tests. Those source and terminal records have verified S3 readback (source df91e022c9d723bb1248d844ab9a44310f7274760cccf9e05f3e5732a03de1a1; validation 644e5d3bb7f0b6a801bb5813c1537cb1c10aca14a37121cdf5e7e1246973fce4).

## Change and purpose

A private native projection receipt moves with the acknowledged canonical JSONL. It binds original capture identity, native record count, last global capture sequence and byte length. Parent retention writes the actual root artifact without republishing that entire projection as a second live payload. Central verification authenticates the capture stream, reconstructs and hashes exact raw native payload bytes, checks sequence/count/length/binding and compares the actual root manifest/file. A sequence gap for another event is legitimate. Pending unacknowledged bytes are excluded. No capture acknowledgement is substituted for root storage.

All storage quote coefficients remain unchanged. Actual duplicate hashing, file publication and payload-event work is removed; no smaller storage tariff or measured instruction saving is claimed.

The terminal reservation is narrowed from 64 KiB to 32 KiB using the closed worker constructor domain: fixed scalar keys, static diagnostic structure, at most twelve bounded root descriptors, and two fixed error strings. Structural and bounded serialization guards run before cloning/publication and after finalization. New guard work is charged: net future modeled work reservation falls by 782336 and trace reservation by 131072. Memory and ParentDelivery's 16 KiB allowance are unchanged. There are no refunds and no changes to original workload or limits.

Unexpected arbitrary Values injected directly into the private finalization helper are outside the constructor proof. On such a schema invariant failure, full diagnostic and root acknowledgement custody is retained, capture stops, and completion is censored. The earliest finalization stop remains authoritative. This is a scoped current-source bound, not a universal claim about arbitrary serde_json Values.

## Review and controls

Independent source/accounting review is recorded in RETENTION_DEDUP_REVIEW.md. Reviewed external patch digests were projection 1b23aa7025208a889a0e91808f9d36c8a2e769ccc2e9e914c44b5722cf86e956 and terminal ec1bc0aec7ad713756c12423d75d5bcafed5accfc6ab024b732f317c7d262cec. Integration only separated their overlapping end-of-file test additions and clarified the constructor-scoped comment; rustfmt normalized formatting.

Controls cover both engines, exact canonical bytes, absent duplicate payload, actual reached root collision, receipt mutations, empty acknowledged projection with pending bytes, same-length disk corruption, valid forbidden generic publication, and an actual root write followed by a late parent clock. The late-write test covers storage plus the parent clock wrapper, not the full worker. Terminal controls cover maximal diagnostics/enums/receipts, refusal without mutation, retained acknowledgements and expired-entry stop precedence with central verification.

## Validation and next gate

Compilation and scoped runtime validation are pending at this commit. Earlier wider-worker failures remain evidence; no completion is inferred from this design. Run the unchanged Koblitz suite, preserve any failing fixtures and exact remaining-work diagnosis, and check canonical/partial central replay. Full-library, CLI and scientific cloud dataset gates remain pending. No new cloud jobs or generation budget renewal occurred.

## Terminal validation, 2026-10-31

Source 8708d456d3e13f5501c3f1e92ecf7442168470f3 completed the scoped Koblitz release suite: 135 passed, 2 failed, 0 ignored, exit101. All seven source snapshot hashes still matched after the run. The original session51143 was observed live through a long period of host contention; increasing rustc CPU counters and running LTO threads confirmed progress, so no duplicate build was launched.

All new projection controls and all four terminal-cap controls passed. The four positive original-bound F4/F5/plain/twist witness controls now pass their explicit assertion that parent storage reaches the intended witness root collision. Earlier Work refusal had prevented that collision from being exercised; this result fixes that coverage gap without changing the cells or limits.

The two frozen wide tests still fail on their first n47/F4/plain cell. Native work4605360 and checking9232450 leave2939406 of original16777216 modeled work; the first parent native-events storage quote is3282368 for100526bytes. All192group additions completed, producer first_stop is null, and capture finalization acknowledges both event and finish. Parent retention is Work; ordinary completion remains false. The failing fixtures are retained at /tmp/verified-worker-3429243-1793461585177207015 and /tmp/verified-worker-3429243-1793461585621342467. Later cells in those test functions were not reached.

Full terminal logs, exact source receipt and both restricted raw fixtures were uploaded and freshly downloaded with all hashes verified:

s3://setec-astronomy-artifacts/ml-dataset/20261031/validation/koblitz-projection-terminal-failure/d02d96f8d31fe7e2a772e794700e390ccd92f04e3fc40303d087219a50c7d46c/checkpoint.tar.gz

The next candidates remain external: a closed-constructor terminal bound tighter than the generic32KiB envelope, and exact native lengths with a single serialization pass and explicitly charged length calculator. No further reservation reduction is accepted merely because it would make a test pass. Full-library/CLI gates and scientific dataset production remain pending. POST_M3_ROADMAP.md records current capabilities/export drift and all retained corpus axes.
