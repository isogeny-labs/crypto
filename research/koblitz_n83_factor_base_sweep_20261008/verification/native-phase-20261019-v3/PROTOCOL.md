# Native retained hash-policy phase diagnostic v3

Frozen before v3 execution. V2's K2048 and K4096 cards both failed before
search: the runner requested public_x_sequential but each exact retained
manifest contains public_x_hash, seed2026100801, a=0. Both worker errors,
nonzero exits, cleanup evidence, card receipts and relay assessments remain
unchanged. Neither launch supplied solver observations or spent its full
600-second search allowance. These failures are configuration errors, not
search failures or censored trials.

Correction: use public_x_hash with that same seed and the same immutable
K2048/K4096 archives. Native preflight now requires exactly one matching
manifest row before launch, and terminal auditing checks policy and seed.
A negative native unit test rejects the former policy and ambiguous rows.
No solver, factor base, query generator, limits or core instrumentation change.

Repeat the distinct v2 positive control card with the newly pinned runner:
exact small-field original-group test, trace checker (four SAT returns),
fully specified N83 selector-witness model and independent group/phase replay.
The previous v2 selector-phase forgery was rejected after a coherent custody
hash update. Keep v1's fixed-coordinate/free-selector 180s censoring separate;
this control change does not resolve unpinned N83 witness search.

After controls pass, execute K2048 then K4096, one natural public trial each,
unpinned selectors, m5 orbit-chain-S3, 32 models, 150000 variables, 2000000
domain clauses, 1000 conflicts and 600s wall each. Fresh write-once paths
under the already ignored ../native-phase-20261019-v2/exec/v3/ keep tracked
source clean. No automatic retries. K4096 remains independently requested.
All v2 source/input/binary/image freezes, strict native cards, director relay,
Docker isolation, native custody, independent query replay and cleanup apply.
Each command has 660s allowance, positive control240s, small-field120s.
Added container envelope is180+600+600=1380s, never substituted for previous
spent work. Tests/builds/audits/storage/rendering are separate overhead.

Audit admits supported zero-relation solver-cap terminals or wall-censored
observations only; unsupported positive relation/scalar results stay failures
until full native replay is implemented. Final counters remain null when
censored. solver_enter includes preprocessing. This is an L0 diagnostic,
not a full cold measurement, yield estimate, A/A comparison, external-tool
benchmark, fastest-base selection or nonexistence result. All historical
failures are retained, source-linked visual reports/PDFs lie outside audited
run trees, and no performance promotion follows from this diagnostic alone.
