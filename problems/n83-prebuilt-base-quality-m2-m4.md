# Primary N83 prebuilt-base quality for two to four summands

- **Status:** `open`
- **Kind:** empirical bottleneck; natural useful-rank and complete cold-cost measurements absent after bounded m=2/m=3 probes and capped m=4 SAT search
- **Affected work:** [prebuilt-base review](../research/n83_prebuilt_factor_base_20261016/REVIEW.md), [primary N83 sweep](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md), [solver gates](../research/koblitz_n83_factor_base_sweep_20261008/SOLVER_GATES.md), [K=16,627 construction](../research/n83_k16627_prebuild_20261017/RUN_REPORT.md), [m=4 model capacity](../research/n83_k16627_m4_capacity_20261017/RUN_REPORT.md), [planted control](../research/n83_k16627_m4_planted_20261017/RUN_REPORT.md), [complete-witness phases](../research/n83_k16627_m4_witness_20261018/RUN_REPORT.md), [root-phase control](../research/n83_k16627_m4_root_phase_20261018/RUN_REPORT.md), [search-progress control](../research/n83_k16627_m4_search_progress_20261018/RUN_REPORT.md), [selector-pinned positive control](../research/n83_k16627_m4_selector_positive_20261018/RUN_REPORT.md), [selector-only control](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md), [bounded natural query](../research/n83_k16627_m4_natural_query_20261018/RESULT.md), [dimension-18 direct-subgroup census](../research/n83_d18_subgroup_orbit_census_20261018/RESULT.md)
- **Next action owner:** N83 index-calculus workstream
- **Last updated:** 2026-10-20 UTC; [completed full-size U/D panel and native reuse accounting](../research/n83_prebuild_cost_m23_20261019/RESULT.md)

## Problem

A strong prebuilt factor base for `icv1-f2m83-tm6151469093347-debefd74`
at m = 2, 3 or 4 cannot yet be selected by measured full cost because the
stored bases lack matched natural relation, useful-rank and complete-pipeline
measurements with a full-width native oracle on this 81-bit subgroup.

The tested primary bases have 64, 256 or 600 signed-Frobenius columns and
10,624, 42,496 or 99,600 distinct points. A separate structured
dimension-12 closure has 2,001 columns and 332,166 points; a retained
explicit-orbit comparator has 2,048 columns and 339,968 points. Their
identities, membership representations, source rules and index costs differ.
Construction or orbit folding alone does not identify the lowest cold cost
or the best reusable setup for any of m = 2, 3 or 4.
The separately retained K=16,627 signed-orbit base has 2,760,082
points and a checked storage receipt. A constructed four-summand relation
and rank-one row have now been checked under exact representative and phase
pins. Natural-query relation yield and useful rank remain unmeasured. Its
larger size increases the rank and linear-algebra obligation as well as the
counting ceiling.

## Evidence and uncertainty

- **Observed:** The [sweep result](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md) records stored, generically replayed primary objects and 1,728 dependent two-summand fixed-point probes with zero relations. Its bounded m=3 attempt did not finish a query. The [structured closure result](../research/koblitz_n83_factor_base_sweep_20261008/verification/structured-d12-orbit-20261010/RESULT.md) verifies 332,166 points and 2,001 columns; its later five-summand search reached a conflict cap. These receipts do not constitute an m=4 relation/rank comparison.
- **Observed:** The [K=16,627 construction](../research/n83_k16627_prebuild_20261017/RUN_REPORT.md) now produced two byte-identical 83,701,045-byte compressed objects with 16,627 columns and 2,760,082 points. Generic replay checked every point, and the native S3 upload/download hash check passed. The second construction's cgroup peak was 507,568,128 bytes; generic replay peaked at 834,940,928 bytes. The first construction peak was not recorded. This establishes stored-base feasibility at the four-summand counting threshold, not natural yield or useful rank.
- **Observed:** The [K=16,627 m=4 capacity gate](../research/n83_k16627_m4_capacity_20261017/RUN_REPORT.md) constructed the full Frobenius-orbit chained-S3 model with 201,598 SAT variables, 5,915,238 clauses and 3,901 XOR rows in a 4 GiB zero-swap container; cgroup peak was 2,661,126,144 bytes. Its one-variable negative control stopped at `UNKNOWN_variable_cap`. Neither arm searched, lifted a relation or measured rank. The specific full-model construction gap is closed at this object's tested settings.
- **Observed:** The [pinned planted-control arm](../research/n83_k16627_m4_planted_20261017/RUN_REPORT.md) used that exact stored object and a hard 1,800-second, 4 GiB, zero-swap container. It exited at the wall cap (`UNKNOWN_wall_cap`), without worker JSON or flushed stdout/stderr; peak memory was 2,660,335,616 bytes. The phase at timeout, conflicts, model status, group relation and rank are unknown. The unpinned arm was not started because the positive prerequisite did not complete.
- **Observed:** The [complete-witness arm](../research/n83_k16627_m4_witness_20261018/RUN_REPORT.md) added fsynced phase records. The same object passed group preflight with four distinct summand x values, affine partial sums, checked rank one and rejected target tampering. Full model construction and seven x-block pins completed; the worker entered SAT at 40.267 seconds. The 1,800-second outer cap ended the run before `sat_returned`, with no OOM and a 2,661,351,424-byte peak. Initial root propagation versus later conflict search, SAT status, checked solver lift and post-SAT rank remain unknown. The unpinned arm was not attempted.
- **Observed:** The [root-phase control](../research/n83_k16627_m4_root_phase_20261018/RUN_REPORT.md) kept the same stored object, full model and seven pinned x blocks. A separately checkpointed level-zero propagation returned `Fixpoint` in 185.023167 ms with zero decisions and conflicts; the worker entered SAT at 35.437 seconds. The 1,800-second outer cap then ended the run before `sat_returned`, without OOM and with a 2,661,863,424-byte peak. Completed post-root decisions, conflicts, SAT status, checked solver lift and useful rank remain unknown. This locates the observed delay after root propagation in this instrumented arm; it does not measure natural yield.
- **Observed:** The [search-progress control](../research/n83_k16627_m4_search_progress_20261018/RUN_REPORT.md) repeated the exact K=16,627 seven-block workload with an optional native atomic observer and ten-second fsynced samples. Root propagation returned `Fixpoint` in 199.977792 ms; SAT began at 39.623697 s. The last of 176 durable samples at 1,792.112144 s showed 322,255 completed decisions, 360 conflicts and two restarts; all 175 periodic phase samples landed in XOR propagation. The container exited at the 1,800-second cap without OOM, with a 2,662,309,888-byte peak and no SAT return. The samples establish repeated post-root search but do not measure an exact XOR CPU-time fraction or identify its inner cost. Later work, a returned model, checked lift and natural useful rank remain unknown.
- **Observed:** The [selector-pinned positive control](../research/n83_k16627_m4_selector_positive_20261018/RUN_REPORT.md) used the same stored object and full m=4 model, with four known representative-selector and 28 phase-zero units added after the seven x-block pins. It returned one SAT model, checked the original-field S3 chain and stored-point group equation, inserted a rank-one relation row and rejected a target tamper. A separate Rust replay independently parsed the object and receipt, repeated the field, group and row checks, and rejected changed-x and changed-row receipts. The run exited normally in 35,821.806933 ms with a 2,662,318,080-byte peak. Root propagation assigned all 201,598 variables with three completed XOR passes and 4,067 row scans; the SAT call took 8.861792 ms. The worker did not export SAT assignment bits, and replay shares the library's arithmetic, so this is a checked constructed-relation control with those stated replay limits. The earlier capped arm differs in source and constraints; their wall times are not a paired speed ratio.
- **Derived:** The [exact support screen](../research/koblitz_n83_factor_base_sweep_20261008/SUPPORT_MOMENTS.md) bounds a uniform nonidentity target's m=4 full-smooth hit probability for K=600 by 1.695987e-6, m=3 by 6.810989e-11 and m=2 by 2.051462e-15. The [capacity audit](../research/koblitz_n83_factor_base_sweep_20261008/S5_CAPACITY.md) counts 22,945,941,707 materialized root states at the K=16,627 point where the m=4 counting ceiling becomes vacuous. These are model-specific counts, not measured natural yield or a proof that a different oracle cannot work.
- **Suspected explanation:** Sum collisions, constrained source geometry, solver search cost and rank dependence may change the quality ordering of near-size-matched bases. Their respective contributions are unknown.
- **Missing capability:** The constructed-relation positive gate and free-phase selector-only control have checked group/rank receipts. Natural-query progress sampling, a returned natural relation, useful rank and full cost on K=16,627 and near-size-matched bases remain open. Comparable m=2/m=3 work needs an accepted relation event or base size under a feasible resource envelope.
- **Observed:** The [selector-only A1/B/A2 control](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md) used one frozen Linux worker and the exact retained object. All three arms returned the same checked constructed rank-one relation. B left 28 phase bits free, reached 517 decisions, 222 conflicts, 826 completed XOR passes and 1,124,379 row scans, and returned after a 6,487.732878 ms SAT call. A1/A2, with all phases pinned, used zero decisions and conflicts and 4,067 row scans each. Separate native relation replay passed for all three, while changed-x and changed-row receipts failed. The B worker wall was 42,786.035270 ms, and its cgroup peak was 2,661,019,648 bytes. This checks phase freedom for known selectors on one constructed witness; it does not measure natural-query yield.
- **Observed export and bounded cost:** The [native dimension-18 census and export](../research/n83_d18_subgroup_orbit_census_20261018/RESULT.md) enumerated all 262,144 low-bit polynomial-basis x values and 32,643 distinct full signed-Frobenius direct-subgroup classes, clearing the 16,627-class threshold by 16,016. The selected 2,760,082-point closure is a retained content-addressed object. Generic replay checked all representatives, phases, signs, labels, subgroup points and duplicates. The [same-host finite-query/cost result](../research/n83_prebuild_cost_m23_20261019/RESULT.md) records equal accepted D point/label sets for import and on-demand generation, a 27.914 s census, 39.373 s archive export, 20.702 s generic replay, and 36 complete paired D m=2/m=3 public probes with zero relation rows and rank zero under their exact finite policies. Their process-wall prebuilt/on-demand ratios are 3.567 and 3.919, with one-time setup excluded from those ratios. The U/D m=4 SAT card preserved progress and three `UNKNOWN_budget` arms before Spot interruption prevented U2. A compact source-x-plus-phase SAT representation, natural useful-rank yield beyond the bounded events, linear algebra and complete cost remain open. L0 measurements do not establish a qualified pipeline benefit.
- **Observed bounded natural gate:** The owner-selected SSH spot runner executed the [frozen P2/A1/B/A2 retry](../research/n83_k16627_m4_natural_query_20261018/reports/run-20261018T2052Z/RUN_REPORT.md). Both constructed controls passed independent replay. The unpinned B arm loaded the base and entered relation collection, then wrote `UNKNOWN_budget` at 1,790.010187 s with no OOM. It emitted no attempt, progress, relation or rank receipt. A2 was delayed about four hours by a local interruption. These are L0 observations on a shared x86_64 host, not quiet-host timing or natural yield. The prior [host-availability record](../research/n83_k16627_m4_natural_query_20261018/HOST_AVAILABILITY.md) remains a historical observation.
- **What this evidence does not establish:** It does not rule out a useful larger, implicit, partial-relation or selected-query construction, and it does not determine the best base under cold or amortized cost.

## What would unblock it

The [completed smaller-base m=4 card](../research/n83_prebuild_cost_m23_20261019/reports/run-small-m4-natural-20261019/RUN_REPORT.md)
adds four normally exited conflict-capped searches at public target 0.
K1,D1,D2,K2 all reached 100,000 conflicts with durable final progress;
the native receipt audit checks 132 archived files. Repeated search
counters match within each base. Natural yield and rank remain censored.
Recorded XOR elapsed was 97.508–97.561% of each worker interval, motivating
a separately tested cache of satisfied, fully assigned XOR rows. Broader
corpus coverage, useful rank and complete cost remain open below.

- [ ] Freeze one primary-curve query corpus and both retained object hashes, then run a native m=4 oracle on the K=2,048 explicit-orbit base and the 2,001-column dimension-12 closure, preserving all attempts and independently replaying each returned group relation and exact rank increment.
- [x] Construct the stored K=16,627 object's full-width m=4 orbit-domain model under a hard 4 GiB, zero-swap cap, with a negative variable-cap control.
- [x] Complete a degree-83 selector-pinned constructed relation positive control on that same object, with a checked group lift and rank receipt. This does not establish natural yield or useful rank.
- [x] Remove phase pins while retaining four known representative selectors under one source revision, with A1/B/A2 constructed controls and separate relation replay. The [selector-only run](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md) passed; its B SAT call took 6,487.732878 ms versus 10.045541 and 17.313917 ms for the fully pinned controls. This is a constructed stage measurement, not natural yield or a pipeline improvement.
- [x] Remove the known selectors and x blocks for one natural public query under the [separately frozen protocol](../research/n83_k16627_m4_natural_query_20261018/PROTOCOL.md). The [B arm](../research/n83_k16627_m4_natural_query_20261018/spot-arms/b-natural-v2/run/cap.json) reached its internal budget without a returned row; relation/rank remain open.
- [x] Execute a versioned equivalent P2/A1/B/A2 policy on the owner-selected x86_64 SSH spot runner, with source-attested binary, hard memory/CPU/network controls, raw cap receipt and separate A1/A2 replay. This closes the earlier access blocker for an L0 bounded run.
- [ ] Obtain a quiet or independently isolated host for an admitted timing comparison; the shared spot run and delayed A2 do not meet that timing gate.
- [ ] Measure base import, root/index preparation, failed and successful query work, rank/linear algebra, target extraction and verification on matched instances and resources; include A/A controls, current IC baseline and same-point rho before claiming a pipeline improvement.
- [x] Compare D prebuilt import and on-demand generation of the same accepted point/label set under one native worker revision, identical eight-point corpus and m=2/m=3 finite-query policies. Record census, serialization/export, full-object replay, per-query loading/indexing, unsuccessful finite queries, peak memory and explicit one/eight-query derived budgets. The [L0 cost result](../research/n83_prebuild_cost_m23_20261019/RESULT.md) closes this bounded preparation comparison, but not full IC cost or admitted timing.
- [x] Run the separately frozen 18-dimensional direct-subgroup-x census. It counted 32,643 distinct full signed-Frobenius classes from 262,144 x words, clearing the 16,627-class size premise; the two native executions returned byte-identical CSVs but share one implementation. See the [census result](../research/n83_d18_subgroup_orbit_census_20261018/RESULT.md).
- [x] Export the deterministic first 16,627 classes as an exact native point/coefficient object and independently replay all 2,760,082 rows; [export receipts](../research/n83_d18_subgroup_orbit_census_20261018/export-v1/RESULT.md).
- [x] Freeze common U/D m=2/m=3 finite-query policies on eight public points and record completed relation/rank counts with fresh replay; [finite panel](../research/n83_prebuild_cost_m23_20261019/reports/run-20261019T0536Z/RUN_REPORT.md). These zero counts apply to the exact exhaustive m=2 and `2^20`-pair m=3 events only.
- [x] Run K2048 and D12 under their separately frozen m=2 exhaustive and m=3 `2^20`-pair policies on eight public points, with 36 complete public receipts, fresh replay, constructed/general-field controls and preserved Spot interruption/retry; [small-base run](../research/n83_prebuild_cost_m23_20261019/reports/run-small-20261019/RUN_REPORT.md). Their finite zero-row/rank outcomes do not order the bases.
- [x] Complete the separately frozen same-host full-size U/D m=4 public-query card, preserving every attempt. The [18-attempt card](../research/n83_prebuild_cost_m23_20261019/reports/run-full-m4-natural-20261020/RUN_REPORT.md) completed and its 329-file audit passed; all attempts were capped.
- [ ] Obtain independently replayed natural rows and useful-rank increments for U/D. The completed card returned no accepted natural rows; its retained matrices have rank zero and its yield/rank-completion estimates remain null. The [first interrupted four-arm card](../research/n83_d18_subgroup_orbit_census_20261018/natural-v1/NATURAL_RESULT.md) remains preserved.
- [x] Freeze a resource-feasible natural m=4 policy for K=2,048 and D12, retaining caps, relation/rank receipts and independent replay. The [native smaller-base protocol](../research/n83_prebuild_cost_m23_20261019/SMALL_M4_PROTOCOL.md) was committed before implementation; execution and checked natural rank remain separate open obligations.
- [x] Pass exact-base m=4 constructed controls for K2048 and D12 with original-field SAT verification, stored-point group/row checks, rank one and separate general-field replay on x86_64 and arm64. The [control report](../research/n83_prebuild_cost_m23_20261019/reports/run-small-m4-controls-20261019/RUN_REPORT.md) records 84,966 / 84,590 model variables and preserved 4 GiB zero-swap host receipts. The common-public-point K1,D1,D2,K2 card entered search after these controls; its natural outcomes are not supplied by the control result.
- [ ] Measure natural relation yield and useful rank outside the completed finite m=2/m=3 events, or a separately frozen partial-relation event, before selecting a prebuilt IC base.

A completed matched relation/rank experiment would establish behavior for
the named bases under that oracle. The separate full-cost gate would determine
their cost ordering at that workload. Neither would establish a universal
best base, asymptotic crossover or transfer to a different curve.

## Next discriminating step

- **Action or proof obligation:** Obtain checked natural relation and useful-rank increments under a separately frozen m=4 search strategy or a separately defined partial-relation event, then measure rank/linear algebra, individual logarithm, verification and matched same-point rho with full cold/reuse charges. The full-size replacement card is complete; its cap receipts remain censored.
- **Inputs and controls:** Exact primary curve and subgroup above; byte-pinned K=2,048, structured dimension-12 and K=16,627 archives; common public query points and coefficients; duplicate-index and planted relation controls; unchanged baseline and A/A repeats.
- **Execution limits and stop condition:** Freeze the time, memory, query and solver caps before execution under the repository's admission rules. A cap or infrastructure failure remains a preserved bounded result.
- **Outcomes and next actions:** A checked natural relation with an independent useful-rank increment would justify a larger common-query rank comparison and complete-cost measurement. A cap with progress but no row identifies solver effort only; it directs one sound propagation or branching change rather than a base ranking. An interrupted arm is repeated only under a newly frozen same-host design. A cap never proves absence of a relation.
- **Recheck trigger:** The smaller-base finite panel, a new m=4 or partial-relation policy, or a complete matched IC/rho session lands.

A planned step is not a completed experiment.

## Resolution and history

### XOR cache implementation and experiment

The [XOR cache continuation](../research/n83_prebuild_cost_m23_20261019/xor-cache-v2/STATUS.md)
adds a tested implementation and a completed sixteen-attempt matched
component card with public target 1 as a fresh holdout. Four constructed
controls passed independent replay. All sixteen natural searches capped
at 10,000 conflicts, and the 294-file native audit passes all four exact
matched endpoints. Checked row visits fall by 82.960–85.361%; L0 worker
off/on ratios are 1.321160–1.345360. Natural useful rank and qualified
complete IC cost remain open. Both launch failures and their separately
frozen fixes are retained.

The [full-base common-public panel](../research/n83_prebuild_cost_m23_20261019/full-m4-v1/STATUS.md)
ran from 02:06:17 to 06:48:53 UTC on 2026-10-20 after four full-base
controls, four independent replays and two native pair checks passed.
All eighteen natural attempts over eight public points completed, each
base with 16,627 columns / 2,760,082 points. The 329-file native audit
passes. D has nine conflict caps; U has four conflict caps and five wall
caps. Summed worker intervals are 2,636.872934 s and 14,284.411736 s
respectively; different endpoints do not justify a speed ratio.
The retained natural-row matrices are empty, with rank zero; useful-rank
completion and natural yield remain null.

The [reuse addendum](../research/n83_prebuild_cost_m23_20261019/reports/reuse-20261020/RUN_REPORT.md)
completes q=1,8,64 projections and explicitly charges observed fresh
replay/container intervals omitted from the earlier worker-only view.
This is an accounting addition from the retained finite-policy runs;
no 64-query card was executed. Full cold IC accounting remains open.

### Catalog evidence drift blocking the current PR

The N83 continuation PR also contains earlier catalog maintenance commits.
Its [CI/provenance record](../research/n83_prebuild_cost_m23_20261019/CI_STATUS_20261020.md)
identifies failed native count assertions, a removed nonexistent source
reference, eleven stale artifact digests, and 28 remaining independent
assessment-verifier problems. The
[versioned reconciliation](../research/cover_transfer_assessment_20261013/catalog-refresh-20261020.md)
now passes the independent catalog verifier, exact generation replay,
2,572-model native verification, 77 affected example tests and the
4,401-pass release-library suite. The catalog transition checks all
2,098 retained model and 664 retained EC1 identities, with two removed
non-EC1 models explicitly recorded. Original failed logs remain retained.
This closes the local reconciliation gap. Successful applicable checks
and required review at the final PR head remain the merge gate; N83
tests alone do not substitute for those checks.

Unresolved. This record names the missing comparison; it does not close the
underlying factor-base selection task.
