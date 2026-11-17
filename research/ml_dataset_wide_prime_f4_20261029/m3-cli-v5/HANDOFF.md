# Merged-source V5 software replay handoff

## What was done

Committed the frozen six-cell protocol before the first V5 draw. Runtime source `4d5a0a65bbe699e399a06579a444b6a21c87c194` passed 285 focused tests and 5,444 full-library tests, with zero failures. Release CLI build and strict library/binary/test Clippy passed. All 32 CLI commands passed across n=47/53/61, m=3 and both twists.

The complete inputs, intermediate artifacts, exports, reports and binary were uploaded to S3 and restored by a fresh ETag-bound download. Archive bytes, contained hashes and binary identity passed verification. The downloaded binary verified all six downloaded cells, with stdout equal to the originals; all six artifact/export checks passed again. Receipts and exact content-addressed S3 URI are adjacent.

## Decisions made

V4 remains historical evidence. V5 repeats the same field sizes, twists, seed, factor base and caps against the source merged with main. No retry or resampling occurred. All six cells retain partial status, native unit-derived output and completed bounded group checking; these are software fixtures, not ECDLP recoveries, scientific corpus production or independent native-basis derivation proofs.

## Open threads

Publish the merge and evidence to PR #2928 and inspect hosted checks. Broader families, arities, factor-base policies, extraction capabilities, scientific corpus production and instruction-count claims remain unfinished.

## Errors

The initial merged-source compile reused a stale path dependency. That failed attempt is separately preserved; the dependency was rebuilt before the successful gates.
