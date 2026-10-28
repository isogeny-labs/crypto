# Initial implementation checkpoint

Runtime implementation follows DESIGN.md (f74588c3be4). This checkpoint is
uncompiled; no test or lint pass is claimed. The next steps are focused native,
dataset/export/persistence tests, strict lint, read review, and parent-controlled
immutable full-library/CLI validation and publication.

Changed paths: relations_generic.rs producer/strict v2 replay and tests;
relations.rs explicit dispatch/helpers; mod.rs new v2 cell outcome/config and
batch payload binding; export.rs v2 schema gates, restricted certificate refs
and feature-equivalence/tamper tests. V1 replay code and Koblitz algorithm remain
unchanged. Source/row/logical units are not machine or full-field cost.

Lowered trace policies in software tests are private helper arguments; public
v2 replay accepts only the frozen production policy. The missing-provenance
legacy export regression now explicitly converts its generated software fixture
to v1; a missing v2 transcript with new verified-trace claims is rejected.
No historical artifact is changed.

Recorded preparation errors: initial design git-add needed --sparse; initial
lookups for rust-toolchain.toml, build.rs and live_capture.rs were absent and
corrected using the actual inherited toolchain/capture.rs layout. Those lookup
errors are not validation results. No Cargo execution preceded the design.

Initial source-bound dataset run at 2bcaf449fe9 compiled successfully and reported
162 passed, 2 failed, 1 ignored. Both failures were new software-fixture byte
assumptions: native Begin serializes the trace policy, so reducing the byte
allowance changes decimal-width bytes. Corrected fixtures independently encode
the changed Begin and solve its small decimal-width fixed point. One retains
exactly the pre-Finish prefix; the cumulative-byte fixture exhausts exactly one
full batch. Production admission behavior was unchanged. Original failure log
is retained externally in generic-filter-trace-validation/dataset-first.log.

First strict lint at 89f0ed80647 reported only large_enum_variant for completed
batch storage (240 bytes vs one-byte stop). Boxed the completed batch field;
JSON schema and values stay unchanged. Also addressed the statically identified
32MiB reader limit using bounded compact serialization for new v2 transcript
and batch payloads, with exact/one-byte-below/zero-cap software tests and an
actual 32MiB read_json boundary check. This prevents a producer-success artifact
that the unchanged reader cannot admit. Initial lint diagnostics remain in
clippy-first.log. Final rebuild pending.
