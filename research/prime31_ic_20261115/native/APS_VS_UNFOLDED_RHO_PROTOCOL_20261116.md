# Prime31 complete APS vs batched unfolded rho: prospective control (2026-11-16)

Status: preregistered BEFORE Rust source modifications or new GitHub Actions benchmark. [pcc:unverified]

## Objective, baseline, curve

Stress-test the reported APS advantage against a separate, well-formed **generic** collision search whose trajectory cannot be damaged by negation-folding fruitless cycles. This is **not** an attempt to relabel rho as index calculus: the APS arm remains a complete, published Amadori–Pintore–Sala (2018) S3 one-relation index-calculus-derived solver (explicit finite-pair scan, generic MITM-equivalent).

Reuse the Rust source in `examples/prime31_aps_one_relation.rs`, exactly `E:y²=x³+847514846x+2087444724` over `p=2147483647`, subgroup `n=2147446831`, cofactor one, identical `PRIME31.g`. Incumbent: joint-Jacobian APS `[384,384,16384]` with maximum six complete attempts, fresh labelled point setup and S3 candidate/verification costs all included. Prior source-specific 24-Q rho watchdog results (GitHub Actions 43909292156) were APS 64.741649 ms, folded negation-rho 125.261741 ms (32 lanes) / 140.040979 ms (64 lanes), all recovered 24/24. Prior plain BSGS and earlier censored folded rho remain separate controls.

## Prior art and searched leads (before choosing this experiment)

- Repository `src/cryptanalysis/ic_boundary.rs`: `rho.negation` is tuned look-ahead with cycle escape, and the repo's `research/ecbench_rho_optimized_20261014/METHODS.txt` distinguishes `rho.plain` from `rho.negation`. The present 16-jump batch watchdog comparator is NOT identical to that library implementation.
- `research/rho_walk_leads_20261014/README.md`: best-of-two +/- steps have fruitless cycles unless escaped; raw step counts are not a complete-cost speed advantage. Repeating that experiment unchanged is not useful.
- `APS_VS_NEG_RHO_WATCHDOG_RESULT_20261116.md` and `docs/session-logs/2026-11-16.md`: pilot failed 12/24 and 20/24 before watchdog; new held-out panel passed after restarts. A reliable generic denominator remains necessary.
- Bos, Kleinjung and Lenstra, *On the Use of the Negation Map in the Pollard Rho Method*, ANTS 2010, DOI 10.1007/978-3-642-14518-6_9: recurring fruitless cycles constrain realized negation speedups. Wiener and Zuccherato, SAC 1998, DOI 10.1007/3-540-48892-8_15: classical negation-fold acceleration and look-ahead.
- Petit/Kosters/Messeng PKC 2016, DOI 10.1007/978-3-662-49387-8_1 and Amadori/Pintore/Sala FFA 2018, DOI 10.1016/j.ffa.2018.01.009: APS is published prime-field index-calculus methodology, not a new non-generic algorithm.
- Other lead classes reviewed in existing PR: B31 partition sweeps after Jacobian (384 retained), anchored-six F4 supports <=3.7407% of targets, ordered m8 F4 censored; the unresolved mathematical lead remains target-conditioned original-ideal-equivalent symmetric S9/K-S5 rational-root extraction (B4/B8 then B31).
- Research-KB MCP/catalog and local Rust/Conductor are unavailable in this tool runtime. This is bounded GitHub plus primary-publisher fallback, **not** an absence/novelty finding.

## Changed hypothesis, comparator and frozen protocol

H0: when all costs are counted, folded negation-map rho's reported runtime may be worsened by cycle repair; an **unfolded**, same-batch-width r-adding rho can be a stronger control, despite more expected useful collision steps without automorphism quotient. The existing APS speed claim must be retested rather than promoted.

Implement an *opt-in* fold policy in the existing Rust rho routine; do not change the default folded watchdog results, the APS relation extractor, the public curve, or the known-scalar isolation. When fold=false, do not negate canonical representatives at starts/restarts/steps; track exact signed point labels; do not use x-only short-cycle detection as proof that an unfolded point repeated. Retain identical 16-jump public table and joint-Jacobian source, 32/64-lane batch inversion, DP x hash, exact point/collision labels, fresh individual cold setup, and final `[k]G=Q` replay. The map still keys distinguished points by x; either sign of a point at matching x is a valid collision only after canonicalizing labels or doing a signed collision check. **This must be implemented correctly**, not treated as a collision with equal signed labels. Cap 2,000,000 walk steps; all capped runs are censored, never wins.

Preselect 24 distinct fresh target scalars from fixed seed `0x52484f5f4e455733` (different from earlier 0x...5732); caller alone knows k. Four chronological rotating arms per Q: APS incumbent, unfolded rho32/DP4, unfolded rho64/DP5, unfolded rho32 A/A, fixed per-target rho seeds. Every arm is a separate cold call. Print full 24x4 raw records, cold wall nanoseconds, steps, watchdog restarts, DPs and censored outcomes. Recompute per-target scalars and replay. Do not tune against the test panel; a future improved map requires fresh seed and new protocol.

Success gate: complete 24/24 in each arm, verified recovered scalars, no truncated steps/uncounted setup, A/A drift <=5%, report aggregate total and paired wins. If any arm censors, label it as such; partial timing does not establish complete cost. Full speedup claim also requires an isolated same-host independent replay, an independently signed PCC, and the repository's actual tuned WZ look-ahead rho plus mature external index-calculus frontier. The present experiment tests a distinct control, NOT conclusively the fastest optimized rho.

No paid compute, PR merge, leaderboard promotion, history rewrite or weakened CI gates.