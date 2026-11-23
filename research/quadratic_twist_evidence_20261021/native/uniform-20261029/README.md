# Frozen uniform response census

Input head `0e88deaae36ac48eee24baaac21ef02bbc979c4d`; unchanged C++v2 producer,
independent versioned Rustv3 audit. All97 `cases/` JSON files stay byte-for-byte
as emitted; index.json and the auditor require exact individual filenames.
Each is under1MiB. The13 altered control payloads and all raw logs/exits are
restored from [validation](../../validation/uniform-20261029/README.md).

`audit.json` passes96arms,896source recoveries,210comparisons,536reachable spaces,
172failed queries, zero joint gains in both modes. All13 changed/rehashed controls
reject101/no receipt and22backend tests pass. Every producer file matches both
earlier attempts; their failures remain part of the record. These are finite
response and algebraic recurrence results, with no wall-time/cost claim.

Reproduce using `run-uniform-yield-v3.sh FRESH_DIRECTORY` after the official
same-branch checkpoint gate. For a later expected-byte replay supply the optional second positional
`EXPECTED_DIRECTORY` as documented by `--help`; do not claim an EXPECTED-mode
invocation from three record-mode executions. `INPUTS.sha256` is the exact final
runner source manifest; old source/proof/data remain unchanged.

[Report](../../reports/uniform-20261029/README.md) ·
[conditional Lean laws](../../lean-uniform/README.md).
