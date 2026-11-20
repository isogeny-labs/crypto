# Checked affine-domain constructor: native component validation

The distinct `checked-codebook-affine-domain/v1` constructor consumes an authenticated CheckedMap, recognizes exact affine codebooks by canonical rank and cardinality, emits coordinate constraints using actual global wire IDs, and retains source/partial state on refusal. Non-affine codebooks return explicit unsupported status. The raw domain encoding is unchanged.

Release library compilation passed. Five exact native tests passed with unchanged source and binary: actual n23 parameters on both twists under 4,000,000-work / 8,000,000-byte local constructor caps; exhaustive small-domain membership; retained non-affine refusal; initial owner/cap refusal; deterministic source-bound ordered commitments. The n23 output has 158 generators and 115 original variables. These tests use a checked-map Ledger, not the actual dataset original owner through publication.

Preserved failures: rejected initial packaging included a rustfmt filename header; the corrected source then needed one explicit u32 annotation in an empty expected test term. The first compiler attempt exited101; corrected compilation exited0. No algorithm, input or cap changed for that correction.

The separate raw-domain integration remains54pass/7fail; this component does not resolve or replace those historical tests. Independent reconstruction, distinct row/receipt/replay adapters, actual producer publication, late failure controls and full release-suite validation remain before delivery. No dataset dispatch is activated, no scientific cloud job ran, and no fall-degree label follows from these tests.

Native replay: restore the S3 archive identified by native-s3-receipt.json; verify SHA256SUMS; run each exact test named in focused.receipt.json with the retained test-binary and --exact --test-threads=1. Compile and source manifests are inside the archive. The source commitment is tagged Map serialization, not the existing gate-wire digest.
