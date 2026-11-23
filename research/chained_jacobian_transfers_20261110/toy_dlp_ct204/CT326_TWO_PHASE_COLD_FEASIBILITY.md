# CT326 — Two-phase complete-cold Pareto feasibility on frozen F103 transfer data

2026-11-23 UTC. **Standing [pcc:unverified].** This is a *source-bound record arithmetic and conditional inequality*, not a second native benchmark, independently replayed ECDLP solver, or new acceleration. It does not modify the actual F103 collector.

## Source, reservation and independent calculation

The source is the unchanged 15 genuine paired cold Q73 Rust records from CT241, Git blob
`61d8aa3f86ef5672faf09c9fc638e5fe9594f429`, original timing [GitHub Actions #44592813955](https://github.com/isogeny-labs/crypto/actions/runs/44592813955), frozen exact-row audit [#44602035872](https://github.com/isogeny-labs/crypto/actions/runs/44602035872). Original [CT241 protocol](CT241_F103_NATIVE_REDUCED_NORM_LP_PROTOCOL.md) fixes E/F103, B=12,16,18,20,22, three alternating pairs per base, and target Q=[73]G. Original [CT243 negative](CT243_F103_NATIVE_REDUCED_NORM_LP_NEGATIVE_ADJUDICATION.md) already establishes that complete IC is slower than tiny direct BSGS. **Do not count this analysis as new timed evidence.** Unchanged [CT241 record auditor](ct241_f103_native_cost_audit.py) remains separate.

New standalone [CT326 auditor](ct326_f103_cold_frontier.py) independently checks the exact Git blob in bytes, each of 15 arm/round slots, cold phase sums, original B/LP ranks, the known relation budgets, per-row baseline/candidate ratios and unchanged unverified standing. It rejects 4 altered-source-byte controls and 7 parsed-data mutations. It reports *medians of paired rowwise thresholds*, never a ratio of marginal medians.

Reproduce on the branch, without network access or cloud compute:

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct326_f103_cold_frontier.py
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct326_f103_cold_frontier.py
```

No bytes of the original panel, associated Rust source, Jacobian model or timing receipt were altered.

## Exact cost inequality and quantifiers

For **each original paired cold observation** let `B>0` be that row's direct BSGS time, `R>0` the already-measured candidate `relation_online_la_ns` phase, and `O=C-R>0` all remaining candidate phases (setup, backsolve, recovery, verification). Model an *unchanged-workload stage acceleration* by multiplicative `s_R,s_O>0`:

```text
T_candidate(s_R,s_O) = R/s_R + O/s_O .
Strict cold win iff T_candidate(s_R,s_O) < B.

Necessary, but not jointly sufficient:
  s_R > R/B       (even if the nonrelation phase were free)
  s_O > O/B       (even if the relation phase were free)

For any s_R > R/B, exact simultaneous feasibility requires
  s_O > O / (B - R/s_R).
```

Proof: both summands are nonnegative; strict total below B implies each summand separately below B. Rearranging gives the first two strict inequalities; fixing `s_R` and isolating `s_O` gives the exact final inequality, provided the denominator is positive. No random-oracle hypothesis, ECDLP lower bound, or asymptotic claim is involved. An algorithmic redesign may change workload and phase boundaries, so these numbers are **not lower bounds on all transfer or index-calculus algorithms**.

## Evaluated original-pair gates

The factors below are dimensionless. Each is the median across **three same-base paired raw rows**, and each 'minimum' comes from one of those three rows. Numbers are necessary constraints only on the above counterfactual model. There is no measured treatment implementing them.

| B columns | Median R/B | Median O/B | Minimum R/B | Minimum O/B | Median O, with R hypothetically free |
|---:|---:|---:|---:|---:|---:|
| 12 | 10,839.326× | 679.634× | 9,675.470× | 646.051× | 1,499,780 ns |
| 16 | 7,165.894× | 708.530× | 7,136.608× | 690.447× | 1,482,953 ns |
| 18 | 7,130.221× | 706.911× | 6,977.890× | 670.577× | 1,433,720 ns |
| 20 | 6,915.932× | 682.590× | 6,838.431× | 655.431× | 1,441,773 ns |
| 22 | 7,104.928× | 683.051× | 6,667.721× | 641.186× | 1,429,625 ns |

The necessary R/B gate arises even if setup/recovery/verification are made **literally zero**; the O/B gate arises even if the relation+online-LA phase becomes **literally zero**. Both one-phase-free idealizations still lose to the paired baseline in all 15 original rows. Conversely, crossing both separate gates by a tiny amount cannot establish a win: the two residual costs must *sum* below B.

## Interpretation and next discriminating steps

This **strengthens the engineering stop rule** in CT243 for its fixed F103 reduced-base norm collector: improving *only* the relation collector or *only* the other phases is inadequate at this original instance. Targeting a few percent in the same representation is therefore a poor next run. It does **not** establish an unconditional obstruction to DLP transfer, prohibit structural relation-finding changes, or justify extrapolating these F103 numbers to Prime31.

Priority remains original norm reuse's independently authenticated 695/701 population and physical matched `C169/C13a/C13b/N2_actual/D` pricing, then an entirely charged cold single-target transferred solver and strongest rho/BSGS/IC. Those are **not** supplied here. An alternate collector or genus/cover should first preregister its changed workload and a new cost decomposition rather than apply CT326's fixed-phase parameters outside scope.

## Ownership, execution limits and proof standing

Research checks examined original CT241/243, original transfer [#3877](https://github.com/isogeny-labs/crypto/pull/3877), cost-frontier [#4215](https://github.com/isogeny-labs/crypto/pull/4215), existing norm replay from cryptanalysis #1523, current CT325 #1930, the shared AGENTS/ACTIVE/current log and queue #3904. The algebra is elementary, not a new geometric theorem; no Lean build or signed PCC was performed. Research-KB/conductor and native workers were unavailable through this runtime. The coordinator performed three disjoint read-only roles using parallel tools: map-obstruction review, Rust/Cantor source inspection (no edits), and frozen-row audit. No subagents, independent native executions or cross-worker proof agreement are claimed. All shared writes occur in this separate draft branch only. No paid compute, privileged S3/VLT1, branch-protection bypass or protected merge. [pcc:unverified]
