# Full-width relation recovery and probe correction

All results below are producer observations, not signed PCC verdicts. Earlier continuation-log uses of “passed”, “checked” or “verified” describe local command output or local comparisons; they do not assert independently verified execution. [pcc:unverified]

## Implementation

`WideRelationSystem` admits public-input relations only after checking the canonical Curve25519 group, subgroup membership, scalar ranges, row width and the group equation. It stores neither a target secret nor factor-base logarithms. `WideGauss::solve_coordinate` can recover a determined target coordinate without requiring every base logarithm to be determined; recovery checks the candidate against the public point. Matrix insertion and recovery retain separate scalar-field operation counters. [pcc:unverified]

The fast probe now converts stored Weierstrass abscissas to Montgomery coordinates before its differential recurrence and converts back before lookup. At identity/generator exceptions it continues with the affine loop. Its regression covers 18 deterministic cases, including identity crossings and the batch boundary. The legacy affine-equivalent tally excludes actual projective and reconstruction overhead and must not be treated as a complete native-operation cost ledger. [pcc:unverified]

## Development evidence

- Pre-repair source `20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a`: focused log reports 14 passing tests; final library rerun reports 6053 passed, zero failed and 130 ignored. Its first library attempt reported 27 missing-fixture failures, retained alongside 27 passing individual rechecks after tracked inputs were restored. [pcc:unverified]
- `source-replay-check.json` records local reconstruction of that source hash from the retained base/patch and local checks of 23 archived artifact hashes and byte counts. These are custody checks, not independent execution proofs. [pcc:unverified]
- The isolated probe diagnostic retained its original failed hit assertion, three additional identity failures, and an extended corrected prototype run reporting 18 matching cases. It used the repository Fp256 source, extracted ledger/trait definitions, and an unused panic-only Fp64 primality shim. [pcc:unverified]
- Repaired source `d9671086380789e83ed5cfb518cb054a53d3dfda4d7e2e3d957ad4ac48d6039a`: formatting and whitespace commands returned successfully; the actual-crate focused run reports 15 passed, zero failed, including the 18-case probe regression. The final full-library run reports 6054 passed, zero failed and 130 ignored in 1348.17 seconds. The pre-repair pass does not validate these later bytes. [pcc:unverified]

Raw logs, hashes and source transition are in `validation-wide-relations/`. Preserve failed attempts and append later terminal records instead of rewriting them.

## Publication and next integration

The repaired-source local library gate has completed; publication and current-head PR checks remain. Full-width Autolab/ecbench workload and result transport and the matched rho scalar interface remain implementation work. No comparative experiment was run for this change. [pcc:unverified]

Conductor rejected reservation of `pcc/claims` and `pcc/ledger.jsonl`: the latest readback reports task T-1649 holding both exclusively. Official claim registration is pending that ownership resolution; no record, signature or ledger standing is claimed. Keep this delivery text tagged as unverified, and do not bypass the reservation. This does not prevent the current code validation from completing. [pcc:unverified]

The next-integration/exact-scalar candidate remains unregistered in the library. Its separate native harness reports seven passing tests, including legacy-codec byte comparisons and 9612 order fields across 3204 retained records. Those retained fields are all at most u64; constructed boundaries separately cover u128 compatibility and full-width transport. [pcc:unverified]
