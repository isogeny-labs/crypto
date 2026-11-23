# Post-call aggregation provenance

The core solver, certificate checks, truth controls and qualifier were Rust
launched by thin shell. Its 98-input source and 247-file raw archive stay frozen.

One post-call Python diagnostic grouped existing status strings for inspection.
That diagnostic is excluded from report aggregation and acceptance evidence.
The separate native `audit-v1.rs` reader instead binds the raw control and member
manifest hashes, rechecks all raw member bytes, compares old/disabled answer,
full decision and counter artifacts, and computes the report totals in Rust.
Do not relabel the historical diagnostic as native or remove old receipts.

The new reader is a separately frozen POST-CALL audit, not part of the old
98-input pre-call source snapshot. Its source, compiler/dependency identities,
binary and pre-call receipt are recorded separately before it runs. This is
producer arithmetic/data readback, not independent signing, formal validation
or complete accounting. No solver search is rerun for the audit.
