# Four-summand chained-S3 entry point for retained factor bases

**Status:** native solver integration and correctness validation. This change
does not report a degree-83 relation yield, useful rank, or runtime comparison.
The pre-merge release suite passed. After the current PR #2100 head was merged
into this branch, [inherited library compile errors](../../problems/current-main-library-build-20261017.md)
blocked current-head validation; this code remains unverified on that merged
head until the base build is repaired and rerun.

## Question and scope

Can the existing full-width chained-S3 encoding be reached through the
index-calculus decomposition and rank path with four summands? The motivating
curve is `icv1-f2m83-tm6151469093347-debefd74`, whose primary subgroup order
exceeds 64 bits. The prior [prebuilt-base review](../n83_prebuilt_factor_base_20261016/REVIEW.md)
identified a missing retained-base m=4 oracle and paired rank comparison.

The [circuit](../../src/cryptanalysis/binary_semaev_chain_sat.rs) already
supports m=4. The [index-calculus entry points](../../src/cryptanalysis/koblitz_index_calculus.rs)
rejected arity below five. This change permits m=4 in both guarded dispatch
and the producer. It retains the existing finite-domain preflight, conflict
and model caps, original-field equation check, group lift, and exact
relation-rank machinery. See the [path diagram](diagram.svg).

## Validation

The release library tests exercise m=4, 5, and 6 witnesses on degree-7
fixtures with direct group-sum replay. Both signed-Frobenius domain modes
exercise m=4, including ordered first-pair constraints and rejection of an
incomplete orbit. The collection test checks that m=4 reaches verified
relations, exact rank, and the final known-answer check; a default-cap
control must remain an unknown rather than a false negative.

Degree-83 arithmetic preflight asserts 63,578 maximum circuit variables
for m=4, and 21,012 extra variables, 697,808 clauses, and 2,656 XOR rows
for a 2,001-representative signed-Frobenius domain. Those are derived
construction bounds, not measured allocations or solve costs.

The test commands and exit statuses are recorded in [VALIDATION.md](VALIDATION.md).

## Remaining gate

The historical primary adapter still admits chained S3 only for m=5 or 6,
and it imports the frozen primary panel rather than the two near-size-matched
retained bases. A follow-on change must load and authenticate the K=2,048
explicit-orbit and 2,001-column structured objects, enable a bounded m=4
query against a common corpus, replay every returned group relation, then
measure rank and all charged cold phases under a frozen protocol. The
[problem record](../../problems/n83-prebuilt-base-quality-m2-m4.md) remains open.

There is no new measured cost or finding for the canonical scoreboard or
progress graphs; those were checked and left unchanged.

| Requested work | Status after this change | Evidence or gap |
|---|---|---|
| Native m=4 chained-S3 solver path | implemented and validated on small-degree known-answer fixtures | release library tests and source diff |
| Degree-83 retained-base m=4 producer | open | authenticated dual-base adapter and bounded query runner |
| Natural yield and useful rank comparison | open | matched query corpus and independent replay |
| Complete cold cost against IC baseline and rho | open | all pipeline phases and matched references |
