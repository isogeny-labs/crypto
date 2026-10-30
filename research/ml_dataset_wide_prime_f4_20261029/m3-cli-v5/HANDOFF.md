# Merged-source V5 software replay handoff

## What was done

Frozen the unchanged six-cell m3 CLI protocol before any V5 draw. Runtime source4d5a0a65bbe699e399a06579a444b6a21c87c194 passed285 focused tests and5444 full-library tests, with zero failures. Release CLI and strict library/binary/test Clippy passed. Exact terminal logs and hashes are bound by validation-gates.json.

## Decisions made

Preserve V4 as historical evidence. V5 repeats the same field sizes, twists, seed, factor base and caps against the source merged with main. No resampling, retries, scientific corpus or performance claim.

## Open threads

V5 execution, artifact checks, S3 readback and restore replay remain pending. A frozen protocol is not an execution result.

## Errors

The initial merged-source compile reused a stale path dependency. That failed attempt is separately preserved; the dependency was rebuilt before the successful gates.
