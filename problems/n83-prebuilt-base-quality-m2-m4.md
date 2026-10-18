# Primary N83 prebuilt-base quality for two to four summands

- **Status:** `open`
- **Kind:** implementation gap and empirical bottleneck; resource limit for a materialized large m=4 root index
- **Affected work:** [prebuilt-base review](../research/n83_prebuilt_factor_base_20261016/REVIEW.md), [primary N83 sweep](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md), [solver gates](../research/koblitz_n83_factor_base_sweep_20261008/SOLVER_GATES.md), [K=16,627 construction](../research/n83_k16627_prebuild_20261017/RUN_REPORT.md)
- **Next action owner:** N83 index-calculus workstream
- **Last updated:** 2026-10-17

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
The separately retained K=16,627 signed-orbit base now has 2,760,082
points and a checked storage receipt, but has no four-summand relation or
rank result. Its larger size increases the rank and linear-algebra
obligation as well as the counting ceiling.

## Evidence and uncertainty

- **Observed:** The [sweep result](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md) records stored, generically replayed primary objects and 1,728 dependent two-summand fixed-point probes with zero relations. Its bounded m=3 attempt did not finish a query. The [structured closure result](../research/koblitz_n83_factor_base_sweep_20261008/verification/structured-d12-orbit-20261010/RESULT.md) verifies 332,166 points and 2,001 columns; its later five-summand search reached a conflict cap. These receipts do not constitute an m=4 relation/rank comparison.
- **Observed:** The [K=16,627 construction](../research/n83_k16627_prebuild_20261017/RUN_REPORT.md) now produced two byte-identical 83,701,045-byte compressed objects with 16,627 columns and 2,760,082 points. Generic replay checked every point, and the native S3 upload/download hash check passed. The second construction's cgroup peak was 507,568,128 bytes; generic replay peaked at 834,940,928 bytes. The first construction peak was not recorded. This establishes stored-base feasibility at the four-summand counting threshold, not natural yield or useful rank.
- **Derived:** The [exact support screen](../research/koblitz_n83_factor_base_sweep_20261008/SUPPORT_MOMENTS.md) bounds a uniform nonidentity target's m=4 full-smooth hit probability for K=600 by 1.695987e-6, m=3 by 6.810989e-11 and m=2 by 2.051462e-15. The [capacity audit](../research/koblitz_n83_factor_base_sweep_20261008/S5_CAPACITY.md) counts 22,945,941,707 materialized root states at the K=16,627 point where the m=4 counting ceiling becomes vacuous. These are model-specific counts, not measured natural yield or a proof that a different oracle cannot work.
- **Suspected explanation:** Sum collisions, constrained source geometry, solver search cost and rank dependence may change the quality ordering of near-size-matched bases. Their respective contributions are unknown.
- **Missing capability:** The primary retained-object adapter still gates chained-S3 commands at m=5 or 6. It needs a source-pinned full-width m=4 path with a degree-83 planted relation control, checked group lifting, exact rank and memory receipts for the K=16,627 and near-size-matched bases. Comparable m=2/m=3 work needs an accepted relation event or base size under a feasible resource envelope.
- **What this evidence does not establish:** It does not rule out a useful larger, implicit, partial-relation or selected-query construction, and it does not determine the best base under cold or amortized cost.

## What would unblock it

- [ ] Freeze one primary-curve query corpus and both retained object hashes, then run a native m=4 oracle on the K=2,048 explicit-orbit base and the 2,001-column dimension-12 closure, preserving all attempts and independently replaying each returned group relation and exact rank increment.
- [ ] Gate the newly stored K=16,627 object's m=4 adapter under a hard memory cap, including a degree-83 planted relation positive control; then include it in a frozen costed comparison if the producer returns valid natural rows.
- [ ] Measure base import, root/index preparation, failed and successful query work, rank/linear algebra, target extraction and verification on matched instances and resources; include A/A controls, current IC baseline and same-point rho before claiming a pipeline improvement.
- [ ] If m=2 or m=3 is promoted as a candidate despite the small-base support bounds, freeze its accepted relation event and resource-feasible base or partial-relation policy and independently measure natural yield and rank.

The first condition would establish comparative relation and rank behavior for
two specific bases under a specific oracle. The second would determine a
complete cost ordering at that workload. It would not establish a universal
best base, asymptotic crossover or transfer to a different curve.

## Next discriminating step

- **Action or proof obligation:** Extend and validate the retained primary-object adapter for m=4, then preregister capacity and planted-relation gates on the K=16,627 object and the two near-size-matched bases before a paired rank/cost run.
- **Inputs and controls:** Exact primary curve and subgroup above; byte-pinned K=2,048, structured dimension-12 and K=16,627 archives; common public query points and coefficients; duplicate-index and planted relation controls; unchanged baseline and A/A repeats.
- **Execution limits and stop condition:** Freeze the time, memory, query and solver caps before execution under the repository's admission rules. A cap or infrastructure failure remains a preserved bounded result.
- **Outcomes and next actions:** Valid natural relations and rank lead to the full cold comparison; zero completions at a cap lead to a separately justified oracle or resource change; a lift mismatch blocks comparison pending repair; host failure leads to a new admitted host, not a mathematical verdict.
- **Recheck trigger:** The full-width primary m=4 adapter, a planted positive control or a new matched retained-base experiment lands.

A planned step is not a completed experiment.

## Resolution and history

Unresolved. This record names the missing comparison; it does not close the
underlying factor-base selection task.
