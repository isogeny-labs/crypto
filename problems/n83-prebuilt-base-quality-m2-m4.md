# Primary N83 prebuilt-base quality for two to four summands

- **Status:** `open`
- **Kind:** implementation gap and empirical bottleneck; resource limit for a materialized large m=4 root index
- **Affected work:** [prebuilt-base review](../research/n83_prebuilt_factor_base_20261016/REVIEW.md), [primary N83 sweep](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md), [solver gates](../research/koblitz_n83_factor_base_sweep_20261008/SOLVER_GATES.md), [K=16,627 construction](../research/n83_k16627_prebuild_20261017/RUN_REPORT.md), [m=4 model capacity](../research/n83_k16627_m4_capacity_20261017/RUN_REPORT.md), [planted control](../research/n83_k16627_m4_planted_20261017/RUN_REPORT.md), [complete-witness phases](../research/n83_k16627_m4_witness_20261018/RUN_REPORT.md), [root-phase control](../research/n83_k16627_m4_root_phase_20261018/RUN_REPORT.md), [search-progress control](../research/n83_k16627_m4_search_progress_20261018/RUN_REPORT.md), [selector-pinned positive control](../research/n83_k16627_m4_selector_positive_20261018/RUN_REPORT.md), [selector-only control](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md)
- **Next action owner:** N83 index-calculus workstream
- **Last updated:** 2026-10-18; [selector-only control](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md)

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
- **Missing capability:** The constructed-relation positive gate is complete with completed XOR-pass timing and row-work counters in its pinned arm. The model's behavior when phase pins or selector pins are removed remains to be measured under a common source. Natural-query group lifting, useful rank and full cost on K=16,627 and near-size-matched bases remain open. Comparable m=2/m=3 work needs an accepted relation event or base size under a feasible resource envelope.
- **Observed:** The [selector-only A1/B/A2 control](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md) used one frozen Linux worker and the exact retained object. All three arms returned the same checked constructed rank-one relation. B left 28 phase bits free, reached 517 decisions, 222 conflicts, 826 completed XOR passes and 1,124,379 row scans, and returned after a 6,487.732878 ms SAT call. A1/A2, with all phases pinned, used zero decisions and conflicts and 4,067 row scans each. Separate native relation replay passed for all three, while changed-x and changed-row receipts failed. The B worker wall was 42,786.035270 ms, and its cgroup peak was 2,661,019,648 bytes. This checks phase freedom for known selectors on one constructed witness; it does not measure natural-query yield.
- **What this evidence does not establish:** It does not rule out a useful larger, implicit, partial-relation or selected-query construction, and it does not determine the best base under cold or amortized cost.

## What would unblock it

- [ ] Freeze one primary-curve query corpus and both retained object hashes, then run a native m=4 oracle on the K=2,048 explicit-orbit base and the 2,001-column dimension-12 closure, preserving all attempts and independently replaying each returned group relation and exact rank increment.
- [x] Construct the stored K=16,627 object's full-width m=4 orbit-domain model under a hard 4 GiB, zero-swap cap, with a negative variable-cap control.
- [x] Complete a degree-83 selector-pinned constructed relation positive control on that same object, with a checked group lift and rank receipt. This does not establish natural yield or useful rank.
- [x] Remove phase pins while retaining four known representative selectors under one source revision, with A1/B/A2 constructed controls and separate relation replay. The [selector-only run](../research/n83_k16627_m4_selector_only_20261018/RUN_REPORT.md) passed; its B SAT call took 6,487.732878 ms versus 10.045541 and 17.313917 ms for the fully pinned controls. This is a constructed stage measurement, not natural yield or a pipeline improvement.
- [ ] Remove the known selectors and x blocks for natural public queries under the [separately frozen protocol](../research/n83_k16627_m4_natural_query_20261018/PROTOCOL.md).
- [ ] Measure base import, root/index preparation, failed and successful query work, rank/linear algebra, target extraction and verification on matched instances and resources; include A/A controls, current IC baseline and same-point rho before claiming a pipeline improvement.
- [ ] Compare prebuilding and reusing the retained signed-Frobenius base against generating the same accepted point set on demand under one native source revision, identical query corpus and membership policy. Charge construction, serialization, loading, indexing, failed queries and memory in a cold run, then report setup and per-query cost separately for each explicit reuse count.
- [ ] If m=2 or m=3 is promoted as a candidate despite the small-base support bounds, freeze its accepted relation event and resource-feasible base or partial-relation policy and independently measure natural yield and rank.

A completed matched relation/rank experiment would establish behavior for
the named bases under that oracle. The separate full-cost gate would determine
their cost ordering at that workload. Neither would establish a universal
best base, asymptotic crossover or transfer to a different curve.

## Next discriminating step

- **Action or proof obligation:** Execute the frozen natural-query P/A1/B/A2 gate on the same K=16,627 object, with no known summand x, selector or phase pins in B. Preserve completed XOR and row-work counters and independently replay any returned relation. Then freeze common natural queries for K=16,627 and the two near-size-matched bases before a paired rank/cost run.
- **Inputs and controls:** Exact primary curve and subgroup above; byte-pinned K=2,048, structured dimension-12 and K=16,627 archives; common public query points and coefficients; duplicate-index and planted relation controls; unchanged baseline and A/A repeats.
- **Execution limits and stop condition:** Freeze the time, memory, query and solver caps before execution under the repository's admission rules. A cap or infrastructure failure remains a preserved bounded result.
- **Outcomes and next actions:** A checked natural B relation and rank increment lead to a larger frozen common-query comparison. A capped B arm retains completed inner-work counters and motivates one frozen sound propagation or branching change; a cap never proves absence of a relation. A preflight or constructed control failure blocks interpretation pending repair. A lift mismatch blocks comparison pending repair.
- **Recheck trigger:** The natural-query gate or a new matched retained-base experiment lands.

A planned step is not a completed experiment.

## Resolution and history

Unresolved. This record names the missing comparison; it does not close the
underlying factor-base selection task.
