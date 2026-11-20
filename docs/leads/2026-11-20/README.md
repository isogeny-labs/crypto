# Research-lead entry — hybrid IC directions, 2026-11-20 UTC

Snapshot date in the user timezone: 20 November 2026. [pcc:unverified]

This entry records four hybrid index-calculus directions proposed and grounded on
2026-11-20 via a parallel subagent sweep (four Fable design forks + four Sonnet
codebase-grounding agents). Nothing was run or measured; every finding is an
attributed assertion or an inspected source fact, all `[pcc:unverified]`. The
design forks' quantitative predictions are derivations, not data. Full memos and
synthesis: [`research/hybrid_ic_directions_20261120/`](../../../research/hybrid_ic_directions_20261120/).
KB receipts for the novelty claims below: that directory's `SOURCES.md`.

## Observation plan

Each direction states a mechanism, the exact falsifiable claim, the cheapest
discriminating test, the falsifier, and a status. No direction may enter a charged
claim or a leaderboard/`benchmarks/` row without the full validation gate
(replay certificate, validator report, matched rho with the §5a band) and a pcc id.

## Priorities (interpretive)

The cross-direction meta-finding governs the ranking: relation **collection is
84–89%** of an `ic logs` run, **linear algebra 0.11–1.9%**, and `row_ops` are
**unpriced in `S`**. A lever that is not cost-per-relation in the decomposition
oracle, and that cannot beat the equal-envelope control, cannot move a ranked row.
Suggested allocation if any of this is pursued: 70% D1 (the one live question,
cheapest-first), 20% a new cost-per-relation oracle direction (not yet designed),
10% the D2 near-null and the D4 one-day discriminant check. This is a proposed
allocation, not a measured success probability. [pcc:unverified]

### D1-TRANSPORT: Frobenius-transport relation graph with A\* ordering (P1, LIVE)

**Mechanism.** Decomposition-as-graph-search: nodes = residuals `R − Σ(partial
sum)` canonicalized by the signed-Frobenius + negation orbit; edges = factor-base
peel / small-norm `Z[τ]` multiplier (the measured norm=N compact-orbit move) / MITM
meet; A\* ordering by ONB Hamming weight of `x(ρ)` vs the factor-base subspace;
relation selection as a min-cost rank-|F| basis posed jointly with the frontier.
Genuinely new as a search mechanism vs the large-prime relation graph (adds no
columns) and vs MITM (its depth-1 instance); no best-first search over partial
states exists in `src/`. Two rho-dominated conceptual neighbors exist
(`research/graph_cycle_ic_20261007/`, boundary negative; `research/single_relation_reduction_20261115/`,
Wagner k-sum tree), neither with an A\*/frontier.

**Exact falsifiable claim.** Cross-target frontier sharing is a null
(`f_share ≈ b·E·n/r ≈ 7.6e-4` at n=41). The surviving claim: an A\* ordering cuts
forward node-expansions ≥2× at matched relation yield and beats the equal-envelope
control across the §5a band.

**Next discriminating action.** Masked 2×2 {sharing}×{heuristic} at n=31 then n=41
on K₀; run the n=31 sharing-null cell FIRST (near-zero cost), spend on n=41 only if
the A\* arm clears the ≥2× bar. Preregistration drafted under
`research/hybrid_ic_directions_20261120/prereg/`.

**Falsifier / acceptance boundary.** Reject if the n=31 sharing gain is within noise
of the predicted `f_share`, the A\* arm cuts expansions <2×, or it fails the
equal-envelope control anywhere in the band. Accept only a constant-factor,
single-rung gain; there is no n=131 decomposition path, and the heuristic reuses
features previously shown to carry no routing signal, so the expected outcome is a
recorded null.

Sources: `research/hybrid_ic_directions_20261120/D1_transport_graph.md`;
`docs/ic/SOLVER_SELECTION.md`; `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` §1;
`src/cryptanalysis/ic_framework/residual.rs`.

### D2-TWINX: Shared-x twist twin collector, binary Koblitz a=0/a=1 (P3, NEAR-NULL)

**Mechanism.** One x-only S3/S4 solve consumed by both curves of the a=0/a=1 twist
pair (shared x-line arithmetic), scored in relations per field-op, not per-relation.

**Exact falsifiable claim.** Pair throughput `Y_pair` ≈ 1.0–1.3× (possibly <1) vs
two independent single-curve collectors on the same x-base; nothing for a single
target.

**Next discriminating action.** Four interleaved masked arms {shared / equal-envelope
two-search / A/A / per-curve matched rho} in field muls/sqrs/invs behind GAE —
**blocked** until binary field-op counting is wired into ecbench.

**Falsifier / acceptance boundary.** `binary_semaev.rs:36-41` shows S3 is
a-independent (one system); shared roots split into disjoint per-curve subsets, so a
shared solve serves two separate DLPs, not one target twice. The Koblitz twist
subgroups are wildly unbalanced (82-bit vs 53-bit at n=83), so the shared-cost
mechanism has no payoff scenario. Reject the mechanism if `Y_pair ≤ 1` or the gain
vanishes against two single-curve collectors. The **m=2** case is **already
measured** (`research/twist_ecdlp_20261027/E4-koblitz-twist-pair/`,
`prediction_confirmed`: "shared base is a pure cost increase"); only **m≥3** is
unmeasured and is predicted null.

Sources: `research/hybrid_ic_directions_20261120/D2_twist_twin.md`;
`src/cryptanalysis/binary_semaev.rs`; `src/cryptanalysis/binary_velu.rs`;
`research/twist_index_calculus_20261020/`;
`research/twist_ecdlp_20261027/E4-koblitz-twist-pair/` (m=2 prior, prediction_confirmed).

### D3-MAXSAT: MaxSAT on the relation-selection stage (P4, NO-GO)

**Mechanism.** Given a collected relation pool, use MaxSAT/CryptoMiniSat (XOR +
cardinality) to choose a min-weight rank-|F| subset minimizing sparse-LA fill —
distinct from the closed SAT-as-oracle results.

**Exact falsifiable claim.** A MaxSAT-selected matrix has lower (encode + solve + LA)
cost than CADO merge on the same pool.

**Next discriminating action.** Only if ever revisited at LA-dominant distributed
scale: reuse an existing pool (`research/la_engines_20261014/`), feed CADO merge vs a
MaxSAT selector to the same block solver — no new collection.

**Falsifier / acceptance boundary.** LA is 0.11–1.9% of a run and `row_ops` are
unpriced in `S`, so any gain is ≤1.9% end-to-end, inside collection-stage A/A noise,
and unrankable; greedy CADO is near-optimal on the one measured instance; the rank
constraint (`Z/r`) does not map cleanly to GF(2) SAT; no cardinality tooling or
pysat/CMS on the host. A MaxSAT-for-filtering pilot was **already run and killed**
(`research/sat_smt_leads_20261114/12-maxsat-filtering/`, independent `SKEPTIC.md`
"kill-criterion-met", uniform random weight-4; open only on skewed/merge). No-go
for the single-target goal.

Sources: `research/hybrid_ic_directions_20261120/D3_maxsat_selection.md`;
`src/cryptanalysis/koblitz_sparse_la.rs`; `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md:1133`;
`research/sat_smt_leads_20261114/12-maxsat-filtering/` (prior pilot, kill-criterion-met).

### D4-AUX: Auxiliary data (Cheon / static-DH; cross-target precomputation) (P4, DROP)

**Mechanism.** (A) Cheon/static-DH auxiliary-input attacks; (B) precomputation
amortized across many targets.

**Exact falsifiable claim.** Neither helps the plain single-target ECDLP goal. (A) is
a different oracle domain; (B) is already what IC is and already charged. The only
preregisterable question: the target-count crossover of shared-log-DB IC vs matched
batch rho, with discriminant `κ²r/A > 4·c_solve·C_pre` across h∈{0,0.1,1}.

**Next discriminating action.** Read `C_pre`, `c_solve`, `κ` from existing records and
evaluate the discriminant (~a day; likely "no band"). Nothing else.

**Falsifier / acceptance boundary.** `FRONTIER.md:7` + `docs/bounds/README.md:114`
forbid an oracle-fed method from displacing a non-oracle one; `FRONTIER.md` has no
multi-target row (different domain by rule); `benchmarks/20261112-mt-fieldops-v5/` is
unranked across the band and loses in wall time at 2^32–2^36. Matched reference MUST
be batch rho on the same T targets (`AGENTS.md:2196`); anything else is a spurious √T
win. Drop for the single-target goal.

Sources: `research/hybrid_ic_directions_20261120/D4_auxiliary.md`;
`docs/bounds/FRONTIER.md`; `benchmarks/20261112-mt-fieldops-v5/`;
`research/mt_ic_scale_20261115/`; `src/bin/ic/log_store.rs`.
