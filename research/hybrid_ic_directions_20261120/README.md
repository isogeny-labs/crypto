# Hybrid index-calculus directions — design + grounding sweep (2026-11-20)

**Status:** design-only. Nothing was run or measured. Every statement here is
`[pcc:unverified]`; quantitative predictions by the design forks (f_share, Y_pair,
the D4 crossover discriminant) are *derivations*, not data. No leaderboard row,
no `benchmarks/` row, no bound record follows from this directory.

**Provenance.** Produced from a parallel subagent sweep: four Fable design forks
(one per direction) and four Sonnet codebase-grounding agents, cross-checked. The
owner's request was to "think of a hybrid index-calculus algorithm with a graph
approach for relation solving, and assess auxiliary data, twists, and SAT." The
four directions below are the result.

**Knowledge-base receipts.** `research-kb` and the lead atlas were NOT queried in
the design/grounding pass. The receipts (`source_id`/`repo:path`, novelty verdict
per direction) are compiled separately in [`SOURCES.md`](SOURCES.md) (sibling
fork). No "novel"/"not tried" wording here should be treated as settled until
those receipts are in.

**Files in this directory**
- [`SYNTHESIS.md`](SYNTHESIS.md) — the condensed cross-direction synthesis.
- [`D1_transport_graph.md`](D1_transport_graph.md) … [`D4_auxiliary.md`](D4_auxiliary.md) — the four full design memos.
- [`SOURCES.md`](SOURCES.md) — KB receipts + novelty verdicts (sibling fork).
- `prereg/` — the D1 n=31 cheap-cell preregistration + resume draft (sibling fork).

---

## Meta-finding (the signal across all four directions)

Relation **collection is 84–89%** of an `ic logs` run; **linear algebra is
0.11–1.9%**, and matrix `row_ops` are **unpriced in the ranked `S` metric**
(verified from code: `research/ic_linalg_pipeline_20261019/`,
`docs/ic/ECDLP_RESEARCH_TECHNIQUES.md:1151-1153`). Consequently each lever the
owner named either attacks the wrong stage (D3 selection, D4 amortization), has no
mechanism to beat the **equal-envelope control** that already reduced the
large-prime relation graph to 2.1× (`docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md`
§1), or produces disjoint, unusable work (D2 twist). The only measured wins that
exist in-repo are constant-factor **base-structure** wins in the decomposition
oracle (norm=N compact-orbit 9.2× over a plain scan at n=41/m=3,
`research/ic_leads_20261010/RESULTS_ROUND2.md` §A1; double-LP 2.1× over equal
envelope). A productive new idea should target **cost-per-relation in the
decomposition oracle structurally**, not re-order the search or post-process the
matrix.

---

## Per-direction design, grounding and verdict

### D1 — Frobenius-transport relation graph + A\* + cross-target frontier sharing — **LIVE (narrowed)**

**Mechanism.** Reformulate the decomposition oracle as a search over a graph whose
nodes are canonicalized residual points `R − Σ(partial factor-base sum)` under the
signed-Frobenius + negation orbit action, whose edges are (a) subtract a
factor-base orbit rep, (b) apply a small-norm `Z[τ]` multiplier (the measured
norm=N compact-orbit move), (c) a MITM meet-join, ordered by an A\* heuristic =
ONB Hamming weight of `x(ρ)` against the factor-base subspace; plus the relation
selection stage as a min-cost rank-|F| basis posed jointly with the frontier.

**Grounding (verified from code).** No A\*/beam/best-first/priority search over
partial decomposition states exists in `src/` — genuinely new as a search
mechanism, though the KB-fallback sweep (research-kb was unavailable; see
`SOURCES.md`) found two **rho-dominated** conceptual neighbors —
`research/graph_cycle_ic_20261007/` (graph/cycle IC, boundary negative) and
`research/single_relation_reduction_20261115/` (Wagner k-sum tree) — neither of
which has an A\*/frontier. But: `residual.rs`
already implements "nodes = residuals, edges = subtract a base point, MITM meet"
as a *random collision walk* whose own doc puts it at ≈1 rho solve/relation; the
A\* heuristic candidate is exactly the Python selector feature set
(`ecc2k130/codegen/indexcalc_selector.py`) already shown to carry **no routing
signal** (`docs/ic/SOLVER_SELECTION.md`, no headroom on 288 queries); and there is
**no n=131 decomposition path** (u64 `FastPoint` < n=64, `MAX_MODULUS_BITS=63`),
so it is testable only at n≤41.

**Falsifiable claim / prediction.** Design fork self-refuted the headline:
cross-target forward-sharing fraction `f_share ≈ b·E·n/r ≈ 7.6e-4` at n=41 (≈0.18
only at toy n=31) — sharing dies exponentially in n, **preregistered as a null**.
The surviving live hypothesis is the A\* ordering cutting forward expansions ≥2× at
matched yield.

**Cheapest discriminating test.** Masked 2×2 {sharing}×{heuristic}; run the **n=31
sharing-null cell FIRST** (near-zero cost), then spend on n=41 only if the A\* arm
beats uniform-cost by ≥2× at matched yield — **gated on beating the equal-envelope
control at all three §5a price points (0, 0.1, 1 GAE)**.

**Falsifier.** Reject if the n=31 sharing gain is within noise of the predicted
`f_share`, OR the A\* arm cuts expansions <2× at matched yield, OR it fails to beat
the equal-envelope control anywhere in the band.

**Expected gain / ceiling.** Most likely collapses to batched MITM + compact-orbit
(already on `main`). Ceiling even if it survives: constant factor, one rung, no
exponent; does not reach n=131.

### D2 — Shared-x twist twin collector (binary Koblitz a=0/a=1) — **NEAR-NULL (no payoff scenario)**

**Mechanism.** The quadratic twist of a binary Koblitz curve is the a=0/a=1 pair
sharing x-line arithmetic; one x-only S3/S4 solve is consumed by both curves'
relation sets, raising relations-per-field-op (not per-relation).

**Grounding (verified from code).** `binary_semaev.rs:36-41`: **S3 is
a-independent**; the entire twist difference is one Artin-Schreier bit
`Tr(s1+a)=0` plus a disjoint factor-base partition (for odd n every x lies on
exactly one twin). So there is **one system, not two** — which refutes the "two
different Semaev systems" risk, but the same fact removes the leverage. A shared
x-solve's roots split into *disjoint* subsets: each root triple lifts on only one
curve, so it is two *separate* DLPs sharing solve cost, not 2× relations/solve for
one target. Two additional killers from grounding: (1) the subgroups are wildly
unbalanced — at n=83, a=0 is 82-bit but a=1 is a 53-bit prime (rho ≈ 2^26,
trivial) — so the twin DLP is uninteresting; shared cost only pays if both DLPs
are of comparable interest, which the Koblitz twist pair never is; (2) binary
field-op counting is **not wired into ecbench** (`BinaryGroup` does not override
`counts_field_ops`; `F2mCounts` exists in `binary_velu.rs` but is unwired), so
measuring the claim needs new instrumentation first.

**Falsifiable claim / prediction.** Y_pair (pair throughput, relations per shared
field-op vs two independent single-curve collectors on the same x-base) ≈
1.0–1.3×, possibly <1 (≈3/4 of S3 solutions are cross-curve "mixed" and unusable);
**nothing for a single ECC2K-130 target**. The prime-field negative result ("a
twist factor cannot add an independent column for an E target") stands, and the
binary **m=2** case is **already measured** — `research/twist_ecdlp_20261027/E4-koblitz-twist-pair/`
preregistered and ran it, verdict `prediction_confirmed`: "shared base is a pure
cost increase (accounting, not a gain)". Only **m≥3** is unmeasured, and is
predicted null.

**Cheapest test / falsifier.** Four interleaved masked arms {shared vs
equal-envelope two-search control vs A/A vs per-curve matched rho}, in field
muls/sqrs/invs behind GAE. Reject the mechanism if Y_pair ≤ 1 or if the gain
disappears once charged against two single-curve collectors. **Blocked** until
binary field-op instrumentation lands.

**Status.** Low priority; worth running only to convert the twist lever from
"maybe" to "measured".

### D3 — MaxSAT on relation selection — **NO-GO**

**Mechanism.** Given a collected relation pool, use MaxSAT/CryptoMiniSat (XOR +
cardinality) to choose a min-weight rank-|F| subset minimizing sparse-LA fill —
distinct from every closed SAT-as-oracle result (those lose at decomposition).

**Grounding (verified from code).** Attacks a stage that is 0.11–1.9% of a run;
`row_ops` are unpriced in `S`, so a selection win cannot register in the ranked
unit; greedy CADO purge is near-optimal on the one measured instance (538 vs 582
row_ops, `ic_framework/linalg.rs:35-44`); the rank constraint is over `Z/r` while
SAT is GF(2) (no clean encoding; a post-hoc `Z/r` rank check is still required);
min-weight exact-rank cover is NP-hard, so CMS overhead likely exceeds the
Gaussian elimination it replaces; and no cardinality/MaxSAT tooling — or even
pysat/CMS — is on the host. A MaxSAT-for-filtering pilot was **already run and
killed** — `research/sat_smt_leads_20261114/12-maxsat-filtering/`, independent
`SKEPTIC.md` verdict "kill-criterion-met", scoped to uniform random weight-4
matrices; open only on skewed/merge matrices.

**Verdict.** End-to-end ceiling ≤1.9% (the cost of making LA free), inside the
collection stage's A/A noise. Revisit only as a bounded "quantify the greedy gap"
study at LA-dominant distributed scale, reusing an existing pool
(`research/la_engines_20261014/`), no new collection.

### D4 — Auxiliary data — **DROP (ranked lowest)**

**(A) Cheon / static-DH auxiliary input** is a *different oracle domain*:
`FRONTIER.md:7` + `docs/bounds/README.md:114` forbid an oracle-fed method from
displacing a non-oracle one, surfaced as its own `oracle_queries` dominance axis.
In-scope only if the deployment exposes a static-DH oracle, and then as a separate
result on a separate domain. The repo's own note finds no IC algorithm that uses
auxiliary inputs (`research/notes/ecdlp-general/RESEARCH_TORSION_AUXILIARY_INPUTS.md`).

**(B) "Auxiliary data" = cross-target precomputation** is what IC already is (the
`ic logs` factor-base log store, stamped `cost_basis: amortized` vs `cold`).
`FRONTIER.md` has **no multi-target row** (different domain by rule); the existing
`benchmarks/20261112-mt-fieldops-v5/` row is **unranked** across the §5a band (IC
dearer at h=0, tied at 0.1, cheaper at 1), its lone h=1 win rests on a fragile "2
native ops per DP point" convention, and wall-time reruns put IC behind at
2^32–2^36 with the gap *growing* in group size (`research/mt_ic_scale_20261115/`).

**Crossover model.** `IC_pt = C_pre/T + c_solve` vs `Rho_pt ≈ κ√(r/(AT))`; discriminant
`κ²r/A > 4·c_solve·C_pre` must hold across h∈{0,0.1,1}. Structurally both scale
~√(n/T) per target, so the ratio is ~flat in T — **no guaranteed crossover**.

**Sabotage trap (already fenced by `AGENTS.md:2196`).** Comparing amortized IC
against *non-batched* rho manufactures a spurious √T win; the matched reference
must be batch rho on the same T targets. Only salvageable act: read `C_pre`,
`c_solve`, `κ` from existing records and check the discriminant (~a day; likely
"no band").

---

## Ranked recommendation

1. **D1** — the only live question. Run the cheapest form (n=31 sharing-null cell
   first, then the A\* node-expansion count), expecting a null that is itself worth
   recording. Gate on the equal-envelope control across the §5a band.
2. **D2** — cheap near-null, but needs binary field-op instrumentation first and the
   mechanism has no payoff scenario in the Koblitz twist pair. Low priority.
3. **D3** — no-go; at most a bounded greedy-gap study at LA-dominant scale.
4. **D4** — drop; at most the one-day discriminant check.

---

## Process gates still required before ANY run

(crypto + cryptanalysis `CLAUDE.md` / `AGENTS.md`) Conductor task + scope
reservation; `research-kb` + lead-atlas receipts; a frozen `experiment.protocol/v1`
with positive control + A/A + masked, interleaved arms; `.resume/RESUME.md`
checkpoint pushed to S3; `isolab-eks` execution (L2 admission); `bench.record/v1`;
`VALIDATION.json` (replay certificate + validator report + matched rho with the
§5a band); a pcc claim id. Default route is EKS, not local. The D1 n=31 cell
preregistration is drafted under `prereg/`.
