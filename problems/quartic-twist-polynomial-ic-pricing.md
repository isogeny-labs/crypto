# Quartic-twist mixed polynomial IC: S4/S5 completion and full cost

- **Status:** `investigating`
- **Kind:** native solver failure; implementation and measurement gaps; external-tool availability
- **Affected work:** [mixed polynomial report](../research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md), [larger-curve scaling and occupancy report](../research/twist_index_calculus_20261020/K1_SCALING_REPORT.md), [peeled-orbit reuse report](../research/twist_index_calculus_20261020/K1_PEELED_ORBIT_REUSE.md), [frozen protocol](../research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md)
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
historical S4 collector misses two verified relations at one failed system; the
p=97 paths are complete on their processed prefixes but sparse in exact
physical relation yield. Three small primes and incomplete cost fields
still prevent scaling or robust crossover claims.

The [exact S4/F4 follow-up](../research/twist_index_calculus_20261020/s4_twist_elimination/REPORT.md)
now repairs that S4 miss and matches every query's complete signed set on
all seven original processed prefixes in two byte-identical runs. The
remaining gaps include S5, the 21 thinned-base replays, complete cold
cost, external solvers and broader scaling; the historical misses and
their original costs remain preserved.

The [endomorphism-image study](../research/twist_index_calculus_20261020/endomorphism_image_base/PROTOCOL.md)
has now completed the resumed 189-case nine-arm polynomial panel and 21
generic-doubling controls, all on the 21 frozen p=37…337 instances.
All 210 independent processed-prefix signed-set and rank audits pass;
189 full-rank scalar/factor recoveries verify, and 21 inactive-column
controls remain censored as prescribed. The original arm matches all
621 earlier S4 prefix queries. Full-stream relation counts come from the
separate complete scalar/group oracle, not from unexecuted polynomial
suffixes. Its cold elapsed comparison now admits all 105 complete
eleven-execution rounds and 630 non-timing replays. Its msolve comparison
checks all 3,024 complete reduced-basis pairs for 378 fixed inputs. The
[final image-base report](../research/twist_index_calculus_20261020/endomorphism_image_base/README.md)
gives every paired per-target cost and its actual supplied-encoding clock
boundary. Complete GAE pricing, matched strongest rho and full external
DLP cost remain open; these image-base controls do not discharge them.

## Evidence and uncertainty

- **Observed:** The [pilot audit](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_audit_p37_v1.json) enumerated all `r` subgroup logs and found five complete signed relations among the first four residuals. `k=1,2,3` reproduce their entire pattern-specific sets; `k=0` misses one and `k=4` has one system failure. The [one-target k=1 audit](../research/twist_index_calculus_20261020/quartic_twist_mixed_polynomial_ic_k1_audit_p37_v1.json) independently reconstructs all eight processed sets, rank four, all physical factor logs, scalar 271, and the same-target twist24 rho replay. The native producer counts 116,050,118 `Fp M` and 3,936 group operations against rho's 348,941 `Fp M` and 2,745 group operations, in separate units.
- **Bounded S5 retry:** [Degree 30](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d30_p37_v1.json) and [degree 31](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d31_p37_v1.json) still return “border unreachable at this degree”; the `k=4` control does likewise. The 8 GiB and 1,200 s caps were observed. A repeated quotient dimension of 4,090 is recorded; its cause is unknown.
- **Derived:** Four `k=1` rows suffice only after the 24-action folds the 48 physical factors to four columns and the public generator log anchors one of them. Flat and cubic-only encodings have 48 and 16 anchored unknowns respectively, so these same four rows cannot solve either. This statement is specific to this base and trace.
- **Larger-curve panel:** [Six independent audits](../research/twist_index_calculus_20261020/K1_SCALE_RUNS.md) verify six one-target scalar recoveries, five exact processed-prefix signed sets and one partial p=73 seed-one set (11 exact, nine polynomial; two missing at query 18). The [post-run difference record](../research/twist_index_calculus_20261020/quartic_twist_k1_missing_diagnostic_v1.json) isolates both missing relations to signed `L2` peel `(54,+1)` and finds no false polynomial relation. Counted IC field multiplications are 1,788–17,249 times matched twist24 rho's on these runs; hash/allocator and calibrated timing remain absent. The [full-stream exact census](../research/twist_index_calculus_20261020/K1_CENSUS_RUNS.md) finds 167 signed `k=1` relations in 359 p=73 residuals and 79 in 1,497 p=97 residuals. An independent BSGS-based scalar oracle matches all 1,856 per-query set digests; the p=97 sparsity is a physical occupancy observation, not a hidden polynomial miss. The [orbit-row audit](../research/twist_index_calculus_20261020/quartic_twist_k1_orbit_row_audit_v1.json) verifies that the 24 images of every selected folded row span only that row.
- **Exact peeled-target orbit diagnostic:** The [full-stream six-run test and subgroup ablation](../research/twist_index_calculus_20261020/K1_PEELED_ORBIT_REUSE.md) check 142,248 finite signed L2 peels. Identity, cubic-only, transported-Frobenius-only and combined-action caches find 20, 107, 243 and 790 duplicate targets respectively. Only two combined-action collisions occur within one residual; 788 more occur across residuals within their separate streams. Even an ideal unlimited combined cache yields only a 1.005585× aggregate S4 call-count ratio here, before cache overhead. This is no native speed measurement or general obstruction.
- **Native base-shape panel:** The [complete 21-case report](../research/twist_index_calculus_20261020/K1_THIN_NATIVE_REPORT.md) and [machine validator](../research/twist_index_calculus_20261020/K1_THIN_NATIVE_VALIDATION.json) show full anchored rank and independently verified scalar 271 on every preregistered prefix choice. Seventeen processed signed relation sets are exact; four p=73 seed-one choices remain partial from the same one failed S4 system. All 21 first/A/A non-timing records and independent audit outputs match, and six full-base controls reproduce the older scaling source. The largest observed proper-subset same-seed full/candidate counted-field ratios are 3.653× at p=73 and 3.575× at p=97; p=97 seed-one one-orbit regresses to 0.745×. This comparison charges full-base and subset-map construction but omits native hash/allocator CPU and a calibrated whole-method unit.
- **Suspected explanation:** The S5 closure failure may be a low-degree Macaulay border or specialization defect in the native solver; neither was established by the retries. A different order, degree, Gröbner engine or exact root algorithm could change the outcome.
- **Exact S4 repair:** [Seven complete first/A/A collectors](../research/twist_index_calculus_20261020/s4_twist_elimination/results/prefixes.json) execute 46,200 finite systems per run with no undetermined cases. Every signed set matches a post-run group-pair oracle; all seven anchored matrices and target/factor logarithms verify. The old p73/seed1 prefix's nine relations become the exact eleven and full rank first occurs at query 18 rather than 19. Eight separately frozen systems also match complete `Fp^3` root enumeration. The 49-term and 18-term resultant identities have zero exact integer remainders, separate IDC records and two checked Lean theorems. All 46,200 component-coefficient matches pass; simple linear equation reduction retains degree profile `[4,4,4,4]`. F4's counter covers row reduction only, so this is a completeness result, not a cold-cost improvement.
- **Missing capability:** A solver that returns complete signed sets for all five patterns on the frozen p=37 stream; F4 replay on the 21 thinned bases; complete F4 and cold-pipeline accounting including native hash/tree/allocator and rho hash CPU; peak memory; isolated ABAB wall intervals; a same-input msolve comparison (Magma if available); and tests on larger curves beyond p=97.
- **Not established:** Impossibility of complete S5 solving, polynomial IC superiority to rho, fastest solver status, large-size asymptotics, or an effect on deployed curves.
- **Image-base attribution control:** On the same 9,658-query full reference streams, phi(L0) supplies 1,865 unique signed tuples and 1,698 distinct projective augmented rows; [2]L0 supplies 1,804 and 1,705 respectively. Both active-column variants attain full rank on all 21 cases. These distinct rows are not all linearly independent. The psi control reproduces the original's exact sets; phi(L2) gives 1,472 tuples against the original's 1,473 with equal peel counts. All 210 candidate-prefix audits pass, both A/A trees match, and hash/omission negative controls reject. All 105 whole cold rounds and 630 non-timing records are admitted; original deadlines, isolation refusals and Spot reclaims are retained. Through first full rank, phi consumes 5.868804% more systems than doubling. Per-record paired doubling/phi median elapsed ratios span 0.405438–5.524357, with both gains and losses. The exact fixture boundary includes encoded public points for alignment while recomputing and charging every lift. The same-input GB control completes 378 fixed systems in eight passes, with 3,024 matching native/msolve bases and generally slower native timers. Complete operation pricing, strongest rho, S5 and full external DLP comparisons remain open.

## What would unblock it

- [ ] Reproduce every `k=0…4` signed decomposition on the frozen 61-residual stream with no failed algebraic systems, retaining all extraneous, duplicate, and invalid roots and a scalar-keyed independent audit.
- [ ] Export identical Weil-restricted S4/S5 systems to the native engine and msolve on the same host, compare complete roots, memory and all solver phases; include Magma if available and name its absence otherwise.
- [ ] Emit native hash/tree/allocator and rho-partition CPU/work counters, requested and observed peak bytes, exclusive setup and online phases; run at least five interleaved ABAB repetitions under the repository's isolation tool, A/A controls, same target and resource envelope, and price `w=0,0.1,1` GAE.
- [x] Run a preregistered p=73/p=97 one-target panel with independent signed-set/rank/scalar audits and a cap that records censored outcomes. Five processed-prefix relation sets pass; one is retained as partial.
- [x] Run every preregistered orbit-complete L2 prefix choice with native polynomial collection, full-base construction, independent exact signed-set and rank replay, full-base source parity and A/A. All 21 recover the target; four retain the known relation miss.
- [x] Repair and independently audit both missing p=73 seed-one S4 relations, retaining the historical failure and cost record; complete on all seven original processed prefixes in first/A/A.
- [ ] Extend the repaired collector to all 21 thinned bases and the unused frozen-stream suffixes; extend the polynomial panel beyond p=97 where feasible.
- [x] Execute all 189 image-base cases plus 21 doubling controls through rank or their frozen cap on all seven primes, with complete independently checked processed sets, rank progress, source parity and A/A.
- [x] Reconcile and admit all 105 complete same-origin cold rounds and 630 non-timing replays for the ten image-base arms; check complete elapsed phase sums and retain every censored, interrupted and refused record. Complete GAE prices are still missing.
- [x] Compare complete native and msolve reduced bases on all 378 preselected image-study systems in eight passes, preserving losing rows and internal timer ranges. This is a GB component comparison, not all S5 roots or external DLP cost.
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

2026-10-23: The 21-case native orbit-thinning grid and separate
identity/cubic/Frobenius/full peeled-target cache ablation were added.
The restricted S4 path has counted base-shape improvements on selected
frozen cases and a regression on another; the p=73 algebraic miss,
full pricing and external comparison remain unresolved.

2026-10-23: Exact coefficient derivation and two Lean identities, an
eight-system exhaustive field-root comparison, and complete seven-prefix
F4 first/A/A collectors repair the S4 miss. Full cold costing and external
comparisons are still open. The proposed endomorphism-sum image base is
recorded as draft card
[`XP-20261023-quartic-twist-image-base`](../backlog/XP-20261023-quartic-twist-image-base/README.md),
with an unimplemented harness and explicit prerequisites; no image-base
result or launch is implied.
