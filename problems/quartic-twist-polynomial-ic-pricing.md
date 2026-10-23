# Quartic-twist mixed polynomial IC: S4/S5 completion and full cost

- **Status:** `investigating`
- **Kind:** native solver failure; implementation and measurement gaps; external-tool availability
- **Affected work:** [mixed polynomial report](../research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md), [larger-curve scaling and occupancy report](../research/twist_index_calculus_20261020/K1_SCALING_REPORT.md), [frozen protocol](../research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md)
- **Next action owner:** quartic twist IC study
- **Last updated:** 2026-10-23

## Problem

The canonical `p=37,b0=2,r=1,873,633` quartic twist has a complete
mixed-pattern `k=1` polynomial one-target solve, but a complete
line-union S5 relation finder and an admitted IC/rho cost comparison
cannot yet be established. The native S5 solver misses a verified
`k=0` relation at the frozen residual index 2, and hash/allocator CPU,
peak bytes, isolated online intervals, calibrated GAE and same-input
external solver comparisons remain absent.

The original broader request asks whether twist symmetry speeds relation
finding and index calculus across relevant regimes. The p=73/p=97
three-seed `k=1` panel is now independently audited. Its p=73 seed-one
S4 collector misses two verified relations at one failed system; the
p=97 paths are complete on their processed prefixes but sparse in exact
physical relation yield. Three small primes and incomplete cost fields
still prevent scaling or robust crossover claims.

## Evidence and uncertainty

- **Observed:** The [pilot audit](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_audit_p37_v1.json) enumerated all `r` subgroup logs and found five complete signed relations among the first four residuals. `k=1,2,3` reproduce their entire pattern-specific sets; `k=0` misses one and `k=4` has one system failure. The [one-target k=1 audit](../research/twist_index_calculus_20261020/quartic_twist_mixed_polynomial_ic_k1_audit_p37_v1.json) independently reconstructs all eight processed sets, rank four, all physical factor logs, scalar 271, and the same-target twist24 rho replay. The native producer counts 116,050,118 `Fp M` and 3,936 group operations against rho's 348,941 `Fp M` and 2,745 group operations, in separate units.
- **Bounded S5 retry:** [Degree 30](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d30_p37_v1.json) and [degree 31](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d31_p37_v1.json) still return “border unreachable at this degree”; the `k=4` control does likewise. The 8 GiB and 1,200 s caps were observed. A repeated quotient dimension of 4,090 is recorded; its cause is unknown.
- **Derived:** Four `k=1` rows suffice only after the 24-action folds the 48 physical factors to four columns and the public generator log anchors one of them. Flat and cubic-only encodings have 48 and 16 anchored unknowns respectively, so these same four rows cannot solve either. This statement is specific to this base and trace.
- **Larger-curve panel:** [Six independent audits](../research/twist_index_calculus_20261020/K1_SCALE_RUNS.md) verify six one-target scalar recoveries, five exact processed-prefix signed sets and one partial p=73 seed-one set (11 exact, nine polynomial; two missing at query 18). The [post-run difference record](../research/twist_index_calculus_20261020/quartic_twist_k1_missing_diagnostic_v1.json) isolates both missing relations to signed `L2` peel `(54,+1)` and finds no false polynomial relation. Counted IC field multiplications are 1,788–17,249 times matched twist24 rho's on these runs; hash/allocator and calibrated timing remain absent. The [full-stream exact census](../research/twist_index_calculus_20261020/K1_CENSUS_RUNS.md) finds 167 signed `k=1` relations in 359 p=73 residuals and 79 in 1,497 p=97 residuals. An independent BSGS-based scalar oracle matches all 1,856 per-query set digests; the p=97 sparsity is a physical occupancy observation, not a hidden polynomial miss. The [orbit-row audit](../research/twist_index_calculus_20261020/quartic_twist_k1_orbit_row_audit_v1.json) verifies that the 24 images of every selected folded row span only that row.
- **Suspected explanation:** The S5 closure failure may be a low-degree Macaulay border or specialization defect in the native solver; neither was established by the retries. A different order, degree, Gröbner engine or exact root algorithm could change the outcome.
- **Missing capability:** A solver that returns complete signed sets for all five patterns on the frozen p=37 stream and repairs the p=73 seed-one S4 miss; native hash/tree/allocator and rho hash CPU accounting; peak memory; isolated ABAB wall intervals; a same-input msolve comparison (Magma if available); and tests on larger curves beyond p=97.
- **Not established:** Impossibility of complete S5 solving, polynomial IC superiority to rho, fastest solver status, large-size asymptotics, or an effect on deployed curves.

## What would unblock it

- [ ] Reproduce every `k=0…4` signed decomposition on the frozen 61-residual stream with no failed algebraic systems, retaining all extraneous, duplicate, and invalid roots and a scalar-keyed independent audit.
- [ ] Export identical Weil-restricted S4/S5 systems to the native engine and msolve on the same host, compare complete roots, memory and all solver phases; include Magma if available and name its absence otherwise.
- [ ] Emit native hash/tree/allocator and rho-partition CPU/work counters, requested and observed peak bytes, exclusive setup and online phases; run at least five interleaved ABAB repetitions under the repository's isolation tool, A/A controls, same target and resource envelope, and price `w=0,0.1,1` GAE.
- [x] Run a preregistered p=73/p=97 one-target panel with independent signed-set/rank/scalar audits and a cap that records censored outcomes. Five processed-prefix relation sets pass; one is retained as partial.
- [ ] Repair and independently audit the two missing p=73 seed-one S4 relations, keeping the initial failure and its complete counted cost; extend the polynomial panel beyond p=97 where feasible.
- [ ] For any priced leaderboard result, provide the replay certificate, validator report and strongest matched rho receipt in `VALIDATION.json` under `docs/VALIDATION_GATE.md`.

The first item would establish a complete tagged-line-union polynomial
finder on the tested stream; it would not by itself establish a practical
or asymptotic advantage. The other items address ranking and transfer.

## Next discriminating step

- **Action:** Export the frozen failed `k=0` residual-2 S5 system and the successful `k=1` S4 systems to msolve's accepted format; compare roots and border behavior on the same host and exact curve model.
- **Inputs and controls:** Existing frozen p=37 residuals and signed base; scalar-keyed exact oracle; failed degree-29/30/31 native records; no change to target or factor signs.
- **Limits and stop:** Retain the 8 GiB/1,200 s cap per system and record timeout/OOM separately. A successful S5 repair proceeds to full-stream completeness; another failure motivates algebraic-system diagnosis, not an impossibility conclusion.
- **Recheck trigger:** Solver repair or external solver installation, completion of cost instrumentation, or new evidence beyond p=97.

## Resolution and history

2026-10-22: The restricted `k=1` one-target path and independent replay
landed as evidence; S5 degree 29–31 retries remained partial. Unresolved.

2026-10-23: The six-run p=73/p=97 polynomial panel, twelve-run
point/scalar-keyed full-stream census, and deterministic 24-action row
audit were added. One p=73 S4 system remains partial; complete costing
and external comparisons remain unresolved.
