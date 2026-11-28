# Protocol: sparse linear algebra for arm D (arm D-S) against matched rho

Study `bdsl-20261127/v1`. Frozen before any evaluation data of arm D-S. The
machine-readable form is [`protocol.json`](protocol.json); its SHA-256 and
those of the harness sources are in [`SEEDS.sha256`](SEEDS.sha256) and in
the commit that freezes them. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container
(another agent runs on the same host). **Single target only** (owner
instruction): every solve is one cold solve of one public `Q`.

Builds on crypto PR #5261 (`research/binary-weil-descent-20261126/`, arm D,
study `bwd-20261126/v1`), itself stacked on #5253. #5261 found that its
incremental echelon linear algebra (LA) is 98% of arm D's cost at n = 48 and
named "structured Gaussian elimination plus block Wiedemann" as the next step.

## Question

On binary composite fields `F_{2^n}`, `n = 3n′`, with the subfield factor
base and the Gaudry–Diem k = 3 Weil-descent oracle of arm D:

1. If only the linear algebra is replaced by a sparse solver, how much does
   the **whole cold single-target solve** cost against matched negation-map
   rho and against arm D, per rung n = 18..54, in #5253's counted unit?
2. How does the sparse LA grow with the factor base F, and what does that
   imply for a crossover with rho (model, labelled as such)?
3. Does a reduced (balanced) factor base, declared here before data, help?

## Arm D-S (the instrument)

`harness/src/main.rs` (`run_ds`) and `harness/src/sparse_la.rs`.

**Relation phase: arm D's, unchanged.** #5253's field, curves, factor base,
A1 relation-target stream and group-law sign resolution, and #5261's
`descent.rs`, are included by `#[path]` without edits. `run_ds` repeats
`run_d`'s per-target loop (target, decomposition, resolution) with the same
counted-field objects per phase, so the relation stream is bitwise arm D's.
Only two things differ:

1. **Stopping rule.** D stops at the first useful dependency its echelon
   meets. D-S checks at relation counts `⌈0.70·F⌉, +⌈F/256⌉, …` whether the
   2-core of the column hypergraph (iterated removal of rows that hold a
   column of weight 1) has more rows than columns. At the first checkpoint
   where it does, the LA runs; if it returns no `d` (all dependencies found
   are useless, `Σ c_j b_j = 0`), collection continues to the next
   checkpoint and the LA reruns from scratch, and every rerun is counted.
   A 2-core with excess ≥ 1 contains a dependency, and any dependency lies in
   the 2-core, so D-S never stops before D: `relations(D-S) ≥ relations(D)`,
   with the same rows as a prefix. The overshoot is the price of the rule
   and is reported.
2. **Linear algebra** (`sparse_la::solve`):
   - *Structured Gaussian elimination (SGE).* Columns are taken from a
     min-weight heap. A column of weight 1 deletes its row (singleton
     removal). A column of weight k, 2 ≤ k ≤ 64, is pivoted on its
     shortest row: every other row r on it becomes `c_p·r − c_r·p`
     (cross-multiplication, no inversion; weight 2 is the classical merge),
     and the pivot row and column leave. A pivot is applied only if the
     Wiedemann cost model `N·(6·nnz + 4N)` of the remainder, plus the
     operation's own cost, decreases (the weight bound). Every row carries
     its right-hand side `(a, b)` through every operation, so no
     back-substitution is needed. A row that becomes empty is a dependency:
     with `b ≠ 0` it gives `d = −a/b` at once, with `b = 0` it is dropped.
   - *Excess pruning.* The heaviest rows are dropped until the remainder has
     at most 2 more rows than columns.
   - *Wiedemann mod ℓ (block size 1).* On `W = Mᵀ` (`C′ × R′`) stacked with
     `k − 1` dense random rows and one zero row (`k = R′ − C′ ≤ 2`), so the
     square matrix's kernel is a random line of `ker W`: sparse random
     projection `u` (16 non-zeros), sequence of length `2(C′ + k) + 4`,
     Berlekamp–Massey, `y = g(B)v`, then `B^j y` until zero. The dependency
     is checked exactly on the reduced matrix (counted), `d = −Σc a / Σc b`.
     Up to 4 attempts with fresh randomness; with `k = 1` a useless line is
     final for that call.

**Block size.** The task asked for block Wiedemann or block Lanczos. This
study uses block size 1 (scalar Wiedemann). In counted operations the
blocked variants cost the same to leading order (`~3N` products by the
matrix for block size 1, `~(1/m + 1/n)·N` block products of m vectors for
block size (m, n), so the same number of matrix entries times vectors) and
their matrix Berlekamp–Massey is more expensive, not less; blocking buys
parallelism and fewer passes, not fewer counted operations. This is
declared here, before data, as a deviation from the task's wording.

**Rejected during engineering (on self-test instances only).** A padded
square `B = [Mᵀ; 0]` without the dense rows (the shape of #5253's
`kernel()`) can return only kernel vectors in its image, which are
supported on the first C′ rows; on one self-test case it locked onto a
sparse useless dependency for 100 checkpoints. The dense rows fix this. The
SGE cost-model weight (2, 3, 6, 12 tried) changed the LA by < 3%; 6 is kept.

## Arm D-S-B: reduced factor base (declared before data)

`DSB_l1.41` and `DSB_l2`: the factor base is reduced to the classes whose
keyed hash (`fbsub|n|ci|label|x`) is below `2⁶⁴/λ`, λ = √2 and 2, so about
F/λ classes. The oracle is unchanged (its cost per target does not depend
on the base) and its decompositions are filtered to the reduced base, one
counted hash probe per point. Model: decompositions per target fall by λ³,
relations needed by λ, so the relation phase grows by about λ² and the LA
falls by about λ². Rungs n = 42..54.

## Instances, references and rungs

- **Cases:** exactly #5261's: `curve|n|ci`, `target|n|ci|ti`, 4 curves ×
  3 targets per rung, 2 × 2 at n = 33 and 48. New rungs: n = 51 (4 × 3) and
  n = 54 (2 × 2).
- **D (reference):** #5261's records `data/runs_d.jsonl`, reused unchanged.
  Not run at n = 51, 54 (about 2 h per case).
- **RHO (reference):** reused unchanged for n ≤ 48 (#5253 `data/runs.jsonl`
  on the panel, #5261 `data/runs_ext_ref.jsonl` on n = 36..48). New for
  n = 51, 54 by the #5253 binary unchanged (sha256 `bf3f431f…`), 16 seeds
  per target. Lanes: W = argmin of the #5253 pilot rule on held-out pilot
  curves at n = 51 (`data/pilot_rho_51.jsonl`); n = 54 uses the same W
  (no n = 54 pilot, host budget).
- **Resource rule:** one process per rung under `prlimit --as` = 10 GiB and
  a wall budget, ≤ 2 threads. A killed process is a resource limit, never a
  negative result; the run goes on.

## Cost unit and band

- **Unit:** #5253's Meq unchanged: field m and s at 1, an inversion at its
  Itoh–Tsujii census, and 1 per table map, hash probe, product mod ℓ (`z`)
  and addition mod ℓ (`za`). These non-field units are priced at u.
- **ℓ-arithmetic, exactly:** one product mod ℓ = one `z`, one addition = one
  `za`, an inversion mod ℓ = `2·bits(ℓ)` `z` (#5253's `inv_mod`). This is
  #5261's convention for D's echelon. A product with an operand equal to ±1
  is a sign change and is not charged; this is the rule of #5253's own
  Wiedemann (`kernel()`). Index bookkeeping (adjacency lists, heaps,
  weights, the 2-core check) is not charged, as D's echelon does not charge
  its pivot maps.
- **Sensitivity readings** (reported next to the primary): `pm1_charged`
  (every ±1 product charged as one `z`) and `pm1_and_structural_charged`
  (also every counted bookkeeping operation at 1).
- **Band (§5a):** u ∈ {0, 0.1, 1} and s ∈ {1, 0.25}, five combinations. At
  u = 0 all LA is free for both D and D-S. An order is stated only where it
  holds across the band.
- **Indicative calibration** (not a band point): `bdsl_harness wcal` times
  one product mod ℓ (u128 `%`) against one `F_{2^n}` multiplication on this
  host, best of 7, labelled indicative (no PMU, shared host).
- Wall time is recorded and indicative only; the shadow echelon and the
  kernel cross-check are excluded from it (`check_wall_ns`).

## Correctness checks (frozen)

- **Logs:** `[d]G = Q` inside the counted solve; `d = d_true` by the
  evaluator, every solve.
- **Shadow identity (n ≤ 48):** an uncounted copy of #5253's echelon is fed
  D-S's rows exactly as arm D feeds them. At its stop, its `(d, targets,
  relations, columns)` and the seven relation-phase counter maps (fb, setup,
  targets, descent, rootfind, lift, resolve) must equal #5261's D record of
  the same case, bitwise. This shows the relation phase is D's, row for row.
- **Prefix:** `relations(D-S) ≥ relations(D)` on every case.
- **Kernel cross-check (n ≤ 30):** the dependency SGE + Wiedemann found is
  tracked back to the input rows (uncounted) and checked exactly:
  `Σ c_j row_j = 0` and `Σ c_j (a_j + b_j d) = 0`; its d equals the shadow
  echelon's d.
- **Replay:** n = 39, ci = 0, ti = 0 rerun from the frozen manifest, every
  non-wall field compared bitwise.

## Analysis (frozen; `bdsl_harness analyze`)

- Paired per-case ratios DS/rho, D/rho, DSB/rho (rho = mean of its 16 runs
  on the same Q), DS/D, DSB/DS; per rung the geometric mean with a 95%
  hierarchical bootstrap (curves, then targets, then rho seeds), B = 10000
  for the primary key and 2000 for the band and diagnostics.
- Rho A/A floor (seeds 0–7 against 8–15).
- DS relation phase / rho, DS LA / rho, DS relation phase / D relation
  phase, DS LA / D LA; LA share; census (core size, remainder rows, columns
  and weight, pivots, Wiedemann attempts, LA calls, LA/F²).
- Exponent fits: OLS of log₂ cost on log₂ ℓ (rho, D, DS, DS relation phase)
  and of log₂ LA on log₂ F (DS and D, n ≥ 30), bootstrap within rung.
- Crossover projection (below), labelled as a model.

## Predictions (written before D-S evaluation data)

The pre-freeze engineering runs were on self-test instances only
(`bdsl-selftest-*` streams, n = 18..45): no evaluation case was touched.
They informed these numbers, and this is said so the predictions are not
read as blind.

- **LA model:** after the 2-core (about 0.64·F rows, the random 3-uniform
  core at rows/F ≈ 0.92) and SGE, the remainder has about ρ·F rows,
  ρ ≈ 0.12, of weight w ≈ 13, mostly ±1. Wiedemann costs about
  `3·(ρF)·(w·ρF)` charged entry operations plus `≈ 4(ρF)²` for the
  projections, Berlekamp–Massey and evaluation, so
  **LA_DS ≈ c·F² with c = ρ²(3w + 4) ≈ 0.6–0.8**, against D's echelon
  ≈ 38·F² at n = 48 (#5261, growing like F^2.79).
- **H1 (exactness):** every log correct; shadow identity on every case
  n ≤ 48; prefix everywhere; every kernel cross-check exact.
- **H2 (LA constant):** LA_DS/F² in [0.4, 1.5] at every rung n ≥ 36; LA
  exponent in F of 1.8–2.2.
- **H3 (against D):** DS/D in [0.9, 1.1] for n ≤ 30; in [0.02, 0.10] at
  n = 48.
- **H4 (gap to rho):** DS/rho in [12, 40] at n = 48; within 15% of D/rho for
  n ≤ 33; no parity at any rung.
- **H5 (shares):** LA share at n = 48 in [0.3, 0.7]; DS relation phase / D
  relation phase in [1.0, 1.03] everywhere.
- **H6 (crossover, model):** with LA = c·F² the full base does not cross rho
  (LA grows like q′², rho like q′^{3/2}); the relation phase alone crosses
  near n′ ≈ 23–25; a balanced base levels off 10–40× above rho, no
  crossing.
- **H7 (reduced base):** DSB_l1.41/DS < 1 at n ≥ 48 and > 1 at n = 42;
  DSB_l2/DS > DSB_l1.41/DS at every rung n ≤ 48.

**Falsifier.** Parity at a rung if the upper 95% bound of DS/rho is ≤ 1
there; it must then carry the rho A/A floor and full verification.

## Crossover formula (used by the analysis; a model, not a measurement)

```text
rho(n′)    = S_top · √(2^{3n′} / h̄)                 S_top: rho Meq/√ℓ at the top measured rung
DS_rel(n′) = κ · 2^{n′} · (α + β n′)                 κ = targets/q′; α, β: OLS of relation Meq/target on n′
LA_fit(F)  = 2^{a} F^{γ}                             OLS on DS rungs with F ≥ 256
LA_c2(F)   = c₂ F²                                   c₂ = mean LA/F² of the top three rungs
F          = (F/q′) · 2^{n′}
balanced:  λ = max(1, (LA_c2/DS_rel)^{1/4}),  DS = DS_rel·λ² + LA_c2/λ²
```

The analysis reports the first n′ ≤ 64 (step 1/4) where each ratio is ≤ 1,
and each minimum ratio.

## Deviations and what is not tested

- Any change after the freeze is an amendment frozen before its own data.
  Every record is kept, failures included.
- Not tested: multi-target amortisation (excluded); double large prime
  (another lane); block size > 1; D (echelon) at n = 51, 54; Koblitz rungs;
  isolab / L2.
