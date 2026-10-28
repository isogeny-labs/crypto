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
