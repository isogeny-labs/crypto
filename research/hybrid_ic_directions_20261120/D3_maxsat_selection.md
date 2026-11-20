# D3 — MaxSAT on relation *selection* (not Semaev decomposition)

> **Prior-art reconciliation (see `SOURCES.md`):** a MaxSAT-for-filtering pilot was
> **already run and killed** in `research/sat_smt_leads_20261114/12-maxsat-filtering/`
> (independent `SKEPTIC.md`: "kill-criterion-met", scoped to uniform random weight-4
> matrices; open only on skewed/merge). This memo's no-go verdict agrees with that
> prior; it is not an untried lead.

**Status:** design memo, preregistration-grade. No experiment run, nothing
measured, no claim. Recommendation up front: **NO-GO for the single-target
ECC2K-130 goal.** The stage this attacks (linear algebra after filtering) is
measured at **0.11–1.9 % of end-to-end wall**, and the incumbent (CADO
purge/merge + Markowitz) is already "a few per cent, not a factor" from optimal
in row operations. The arithmetic ceiling on any gain is below the noise floor
of the thing it would improve. Details below so the no-go is falsifiable rather
than asserted.

## 0. What is being proposed and why it is new

Every closed SAT result in this repo uses SAT as a **decomposition oracle** —
solving the Semaev / Boolean point-decomposition system per target to *produce*
a relation. That is harvested and lost: SOLVER_SELECTION found no routing
headroom (prepared `pairs` beats `sat-short`/`sat-medium` on all 288 queries at
equal verified rank gain); native-XOR SAT reads 0.854× rho at n=17 but 7.34× by
the next rung; FES is the strongest arm.

D3 proposes a **structurally different** use: SAT/MaxSAT not on the algebra of a
single decomposition, but on the **combinatorial selection** of which already-
collected relations enter the matrix — "choose a min-weight, rank-|F| subset of
the relation pool." This is the job currently done by `FilterStrategy::Cado`
(purge + Markowitz spanning-tree merge) in `koblitz_sparse_la.rs`. It consumes
the *output* of the oracle, never calls the oracle, and so none of the closed
SAT-as-oracle evidence bears on it. CryptoMiniSat is already installed with
native XOR Gauss–Jordan (`pycryptosat` adapter; the in-house CDCL in
`sat.rs`/`semaev_sat.rs` also has XOR-GJ), and min-weight exact-rank cover is
exactly a cardinality + XOR-constraint problem that family targets.

That novelty is real but, as §7 shows, it buys access to a stage too small to
matter at current scale.

## 1. Exact encoding

Pool of `R` collected relations over factor base `F` (|F| = c columns), each
relation `i` a sparse row `rᵢ ∈ (Z/r)^c` plus the target column, carrying weight
`wᵢ` = nnz(rᵢ) (its contribution to sparse-LA fill), all rows group-verified
(`audit_rank`, `ic-useful-rank/v1`) before entry.

- **Selection variables.** Boolean `sᵢ`, i=1..R: relation i is kept iff `sᵢ=1`.
- **Rank constraint (the hard part).** We need the kept rows to span a rank-c
  subspace over `Z/r` (r the 129-bit ECC2K-130 order). SAT/XOR engines live in
  GF(2), not Z/r, so the rank-over-Z/r condition **cannot be encoded natively**.
  Two honest options, both lossy:
  (a) Encode a GF(2) *surrogate*: a sufficient structural condition — every
      column j must be covered by ≥1 kept row (`⋁_{i: rᵢ[j]≠0} sᵢ`), plus a
      spanning-forest/acyclicity condition on the row–column incidence
      bipartite graph so that covered columns are independently determined.
      This is a matroid-ish relaxation; it does **not** certify rank over Z/r,
      so the selected set still needs a Z/r rank check afterward (the same
      elimination we were trying to avoid).
  (b) Encode exact rank via CMS XOR clauses only for the GF(2) *image* of the
      matrix (reduce mod 2). Full rank mod 2 is necessary but **not sufficient**
      for full rank mod r; false "independent" sets pass.
  Either way the clean "SAT decides rank" story fails: rank is an
  integer-lattice property, SAT is Boolean. This is the first structural strike.
- **Objective (MaxSAT).** Soft clauses `(¬sᵢ)` with weight `wᵢ` (prefer dropping
  heavy rows) under the hard cover/forest constraints; equivalently minimize
  `Σ wᵢ sᵢ` subject to rank-c. Minimizing total kept weight ≈ minimizing initial
  sparse-LA fill, a proxy for block-Wiedemann/Lanczos SpMV cost.
- **XOR structure CMS exploits.** The incidence constraints and any GF(2)-image
  independence clauses are XORs; CMS's Gauss–Jordan handles them without CNF
  blow-up. But see §1(a/b): the useful constraint (rank over Z/r) is precisely
  the one that is not an XOR.

## 2. Why genuinely different from the closed SAT work

Closed work: SAT ⟶ one relation, per target, inside the hot loop, competing
with `pairs`/FES on relations-per-second. D3: SAT ⟶ a subset choice, once,
offline, after collection, competing with CADO merge on matrix row-ops. Disjoint
problem, disjoint cost model, disjoint competitor. Nothing transfers.

## 3. Exact falsifiable claim

> On an identical collected relation pool, a MaxSAT-selected relation matrix has
> strictly lower **total** cost `C_sel + C_LA` than `FilterStrategy::Cado`
> `C_merge + C_LA`, where `C_LA` is counted `row_ops` of the subsequent block
> solve priced in GAE (§5a band: unpriced encode/solve ops at 0, 0.1, 1 GAE),
> and `C_sel` / `C_merge` are the selector's own counted work. The order must
> hold across the price band to count.

## 4. Cheapest discriminating test (no new collection)

Reuse an **existing** pool — the pipeline already persists relation logs (`ic
logs`), and the measured studies `research/la_engines_20261014/`,
`research/ic_linalg_pipeline_20261019/`, `research/sparse_la_timing_20261018/`
each hold real pools at |F| = 1,024 … 262,144 on K₀ (n=31/41 and wider). Feed the
**same** pool to three selectors, then the **same** block solver:
`cado` (default), `keep-all`, and the MaxSAT selector (via `pycryptosat` with the
GF(2)-image independence encoding of §1b plus a post-hoc Z/r rank check). This
needs zero new oracle calls — the 84–89 % collection cost is shared and
cancels.

## 5. Baselines, controls, ledger

- **Primary baseline:** `FilterStrategy::Cado` (purge over ≤50 steps by Ω₃₁
  weight + Markowitz spanning-tree merge). Measured: 10.40× [9.99, 11.06] faster
  than legacy at 262,144 cols, core 26,750 → 3,424 rows.
- **Lower/upper frame:** `keep-all` (no selection) and `structured-gauss`
  Markowitz-only.
- **A/A:** two seeds of each selector on the same pool for the noise floor; CADO
  purge is randomized in component order, so its own run-to-run spread must be
  measured before any MaxSAT delta is believed.
- **Honest overhead control (decisive):** the LA-stage saving
  `C_LA^cado − C_LA^maxsat` must exceed `C_sel^maxsat − C_merge^cado`. If the
  MaxSAT encode+solve costs more than the entire LA stage it shrinks, the arm is
  dead on arrival.
- **Ledger:** instrument (same `row_ops` counter, same block solver, same
  completion rule `FullRank`), host, pool identity (SHA-256), relation order
  (shuffle-invariance check), seeds, masked arm labels during analysis.
- **Matched rho:** not directly comparable (this is an intra-LA-stage study);
  report the end-to-end delta as a fraction of a full `ic logs` run so the gap-
  to-rho context is preserved, never as a standalone speedup.

## 6. Falsification threshold

Reject D3 if **any** of:
(i) `C_sel^maxsat − C_merge^cado > C_LA^cado` (overhead ≥ the stage it improves);
(ii) the end-to-end delta `(C_sel+C_LA)^maxsat` vs `^cado` is within the A/A
     noise band of CADO; or
(iii) MaxSAT's GF(2)-image selection produces a set that fails the Z/r full-rank
     check more often than CADO (correctness regression).
Accept to "engineering candidate" only if the end-to-end delta separates from
A/A across the whole price band **and** the overhead control passes.

## 7. Expected gain and honest ceiling — the quantitative no-go

- LA is **0.11–1.9 %** of a real `ic logs` run (`la_engines_20261014`); setup is
  84–89 % (`ic_linalg_pipeline_20261019`). So the **absolute** ceiling on any
  selection-stage improvement to end-to-end work is **≤ 1.9 %**, and that is the
  limit of making LA *free*, not of making it merely lighter.
- CADO merge is already "a few per cent, not a factor" from the dense row-op
  optimum (`structured-gauss` note). If "optimal selection" beats CADO by even a
  generous 20 % of LA row-ops, end-to-end that is `0.20 × 1.9 % ≈ 0.4 %`, almost
  certainly inside the A/A band of the 84–89 % collection stage.
- Min-weight exact-rank cover is NP-hard; CMS on an R-variable instance (R = core
  rows, thousands to hundreds of thousands) with the independence XOR block is
  plausibly **comparable to or costlier than the Gaussian elimination itself**,
  so (i) is the likely verdict.

Net: best realistic end-to-end gain < 0.5 %, expected overhead > gain, and the
clean encoding does not even exist (rank is over Z/r, not GF(2)).

## 8. Novelty vs prior art

- vs closed SAT-as-oracle (SOLVER_SELECTION, native-XOR, WDSat, crossbred):
  different problem entirely (selection, not decomposition). Novel, but the
  novelty is in a cul-de-sac.
- vs CADO filtering (`FilterStrategy::Cado`, Bouvier 2013): same objective
  (min-weight sparse core), CADO solves it greedily/heuristically and cheaply;
  MaxSAT would solve it more globally but at NP-hard cost. No evidence the global
  optimum is materially better than CADO's greedy one — and the stage is 2 % of
  wall regardless.

## 9. Sharpest risk

CADO merge is already within ε of optimal on the only metric that matters
(sparse-core row-ops), and the stage it governs is ~2 % of total work. MaxSAT's
encode+solve overhead then dominates any conceivable gain, and the rank-over-Z/r
/ GF(2) mismatch means it cannot even guarantee a valid selection without
re-running the elimination it meant to replace. The experiment would most likely
measure a wash-or-worse at real cost.

## Recommendation

**NO-GO** for ECC2K-130 / single-target. Do not spend collection or solver
budget here. The *only* regime that would revive it is one where LA becomes the
dominant stage — a genuinely distributed/GPU solve at |F| ≳ 10⁸ where filter
quality changes block-Krylov iteration counts materially — and even there CADO
is the incumbent to beat and the test above (reusing a pool, no new collection)
is the right cheapest probe to run *first*, before writing any MaxSAT encoder.
If anything, the finding reinforces D1: the leverage is in **relation
collection** (84–89 % of wall), i.e. deciding what to search for, not in
post-hoc selection of what was already found.
