# Graph-IC lane: unqueued leads, ranked (scout `scout-graph-ic`, 2026-11-18)

Every number below is a producer record, a replay by another round-1 lane,
or a model written here. All are [pcc:unverified]. No pcc claim was
registered. Nothing was built or run, and no paid compute was used. A lead is
a hypothesis with a falsification test, not a result. Accounting changes are
listed separately and are never counted as improvements.

## 0. Baseline the leads are ranked against

| Quantity (p1009 exposed fixture, Run50 deferred arm) | Value | Source |
|---|---:|---|
| Observed recorded subtotal R | 5,913,875,255,822 bitops | `runpod-deferred-pair-20261117-50/README.md` |
| Descent with the linear stage counted (replay) | 4,369,192,705,362 bitops for 3,415 residuals (1.2765e9 per residual) | `leads-20261118/complete-accounting/README.md` |
| Non-descent part of the w=0 lower bound | about 2.50e12 bitops | same (6.873e12 − 4.369e12) |
| `F_p`-solutions per Weil system | 3,419 / 3,415 = 1.001 | `descent-finder/runs/traced-seed389-n3415.summary.json` |
| Full decompositions | pooled 6 in 6,649 residuals (seeds 389 and 401 plus the original-IC stream), π̂ ≈ 9.0e-4 | `fresh-scaling/README.md` critic F4 |
| Expected descent at π̂ (2/π̂ residuals) | ≈ 2.84e12 bitops (the recorded 3,415 is one NB(2, π) draw) | arithmetic on the rows above |
| F4 share / linear-stage share of the counted descent | ≈ 0.78 / 0.22 | complete-accounting, descent-finder |
| Distance to rho (lower bounds only; §5a forbids ranking) | deferred / rho plain b16 = 0.0625 (w=0) to 0.0705 (w=1) | complete-accounting |
| Wall model: where IC passes rho on the fresh ladder | between p=383 and p=661. The descent is about 7.5 s per target at every p. | `fresh-scaling/estimate.csv` |

The descent costs about the same at every p: a fixed system shape, NB(2, π)
residuals with π independent of p, and only `x^p mod f` grows, as `log p`.
The rows that scale are relations and linear algebra. On the fresh ladder the
descent therefore decides where IC passes rho. A descent saving moves that
crossing point to smaller p. That is how the leads below translate into
distance to rho.

## Answer to the specific question: the discriminant-character screen

**It was done in Run49.** `runpod-discriminant-pair-20261117-49/README.md`
implements and measures the Swan–Stickelberger screen that
`runpod-quadratic-pair-20261117-48/NEXT.md` proposed. It removed 198,848 of
298,778 quartic requests and saved 205,148,763,450 observed bitops, 3.18% of
the unscreened pair subtotal. Both pair arms stayed above the
original/table subtotals. Run50's deferred arm already runs on the screened
pair stream (arm `pair-discriminant`). It is not an open lead. [pcc:unverified]

## Ranked leads

### L1. Use exact-log quadratic classes as a descent factor base, counting all three split patterns. Largest expected effect.

**Mechanism.** Each Weil system has about one `F_p`-solution. A solution
becomes a row only if the monic sextic `S` splits into 6 distinct linear
factors, probability 1/720 for a random sextic. If the degree-2 classes
`D=(U,V)` (U irreducible over `F_p`) whose discrete logs the certified kernel
fixes exactly are added to the descent factor base, then three more
factorization patterns also give affine rows:

| Pattern of S over F_p | Random-sextic probability | Ratio to 1/720 | Needs |
|---|---:|---:|---|
| (1^6) | 1/720 | 1 | as today |
| (1^4, 2) | 1/48 | 15 | 1 class in D_exact |
| (1^2, 2^2) | 1/16 | 45 | 2 classes in D_exact |
| (2^3) | 1/48 | 15 | 3 classes in D_exact |

The expected number of residuals falls by
`m(f) = 1 + 15f + 45f² + 15f³`, where `f = |D_exact| / |D_universe|`.
CANDIDATES.md C2 and `LEAD_ANCHORED_QUADRATIC_DESCENT.md` (variants A and B)
cover the `(1^4,2)` term; variant B lists the two-quadratic case without a
yield figure. **New here:** the `f²` and `f³` terms, a measured-input
estimate of `f`, and the scaling of `f` with p (L3).

**Estimate of f from recorded data (model).**
- `D_exact`: Run50 deferred graph at stop, 20,510 anchored classes (Run50
  README). The replay of all 108,058 Run49 sources gives 32,408
  (`deferred-graph50/recorded49-graph-replay.json`).
- `D_universe`: about 508,536 monic irreducible U over `F_1009`, each with on
  average one admissible V, so about 2.5e5 classes up to sign. The recorded
  vertex saturation gives an independent check. In the replay trajectory the
  rate of new vertices per source falls from 1.91 (598 sources) to 0.90
  (≈105k sources, 139,030 vertices). A uniform-draw model `0.90 = 1.91·(1 − V/U)`
  then gives U ≈ 2.6e5.
- So `f ≈ 0.079` at Run50's stop, giving `m ≈ 2.47`, and `f ≈ 0.125` at 108k
  sources, giving `m ≈ 3.6`. The `(1^4,2)` term alone gives 2.18 and 2.9.

At `f = 0.079` the expected descent would go from about 2.84e12 to about
1.15e12 bitops. The expected deferred-arm lower bound would go from about
5.3e12 to about 3.65e12, roughly 31% lower. Combined with C1 trace replay
(1.68x on the descent, conservative) the descent would be about 0.69e12 and
the total about 3.2e12. These are models, not measurements. [pcc:unverified]

**Already tried.**
- Run42 (`runpod-anchored-descent-20261116-42`) was rejected for
  foreign-thread counters.
- Run43 (`runpod-owned-descent-20261117-43`) found 26 valid `(1^4,2)`
  partials and 0 matches against 11,206 retained classes. Those were the
  stage41 quadratic-table vertices (`runpod-extension-pipeline-20261116-41/README.md`),
  not the deferred graph's exact-log set. 26 × 11,206 / 2.6e5 ≈ 1.1 expected
  matches, so 0 is not informative (P≈0.33).
- C2 in `descent-finder/CANDIDATES.md` and M2 in
  `descent-finder/PROTOCOL_DRAFT.md` (domain count) are queued but not run.
- No lane has counted the `(1^2,2^2)` or `(2^3)` patterns.

**Exact correctness condition.** As in C2, for every class used:
- a signed canonical match in the graph's encoding;
- a replayed anchor witness chain inside the certified-kernel system;
- target coefficient −b and RHS a kept;
- a group identity check, kernel projection, and the elliptic target replay.

For `(1^2,2^2)` the two classes must be distinct, or else the
coefficient is doubled exactly. Unmatched partials are discarded and
counted. The stream changes acceptance, so a descent comparison needs at
least 100 pooled successes per arm (fresh-scaling critic F4, item 3).

**Cheapest local falsification** (finite algebra; no timing; minutes on one
core):
1. *Pattern tally on the existing stream.* Extend
   `descent-finder/traced_descent_check.rs` (mini-build, about 32 s on
   seed389) so that for each `F_p`-solution it factors `S` over `F_p` and
   tallies the factorization pattern. Prediction for 3,419 solutions:
   `(1^4,2)` ≈ 71, `(1^2,2^2)` ≈ 214, `(2^3)` ≈ 71, `(1^6)` ≈ 5 (observed 2).
   Stage43's 26 of 1,508 solutions (1/58) agrees for `(1^4,2)`. If
   `(1^4,2)` ≤ 35 or `(1^2,2^2)` ≤ 110, the 15x and 45x multipliers are
   wrong by at least 2x.
2. *Exact universe count.* Enumerate monic irreducible U over `F_1009` and
   count V with `V² ≡ H mod U`, using the cover H from the mini-build's
   `jv_cover`. This is seconds of Rust. If |D_universe| > 5e5, f halves and
   `m(0.079)` drops below 1.7.
3. *Exact-log set and match count (M2 plus matching).* Rerun the deferred
   collection locally, without descent (Run50 inner wall about 25 s, scoped
   build), or obtain Run50's LOCAL witnesses. Export the anchored canonical
   classes at the kernel checkpoint, and look up the step-1 partial classes.
   **Kill criterion:** matched rows per solution < 1.5x the full-split
   rate. **Promote criterion:** ≥ 2x, which justifies an M3-style paired
   protocol on fresh targets.

**Expected value.** Largest of all leads: a descent residual reduction of
about 2.5x at p1009, conditional on f. The added work per solution is
post-processing (about 0.17% of descent now) plus hash lookups.

### L2. Keep collecting after rank readiness to enlarge D_exact (variant D, quantified)

**Mechanism.** Under L1, each extra exact-log class lowers the expected
descent cost. The question is whether it lowers it by more than the extra
sources cost.

**Data.** The anchor trajectory in `recorded49-graph-replay.json` is
accelerating. Marginal anchors per source:
- 0.10 at 598–9,983 sources;
- 0.43 at 77–87k;
- 0.56 at 87–96k;
- 0.72 at 96–106k.

Rank 508 is reached at about 90k sources. The observed relation phase is
about 2.25e7 bitops per source (Run50: 2.04e12 / 90,877). The marginal cost
of an exact class is therefore about 3.1e7 bitops and falling.

**Model.** `E_desc(f) = 2c/(π·m(f))`. At `f = 0.079`,
`dE/d|D_exact| ≈ −E·m'(f)/(m·U) ≈ −4.0e7` bitops per class at the current
per-residual cost (`c = 1.2765e9`). With C1 applied it is about −2.4e7. So
at Run50's stop, extra collection is about break-even to mildly favourable.
The rising marginal rate argues for stopping later than rank readiness.
[pcc:unverified]

**Already tried.** Listed as variant D in `LEAD_ANCHORED_QUADRATIC_DESCENT.md`
without numbers. `RANK_STOP_REVIEW.md` and Amendment 26 optimise the stop for
the homogeneous rank only.

**Cheapest local falsification.** Pure arithmetic on the recorded trajectory
and Run50 phase costs (done above, as a model). Next, a local deferred
collection that records `|D_exact|` at every checkpoint up to 2x the rank-stop
source count, with the L1 match rate from step 3. The lead dies if the measured
marginal descent saving per added source falls below the measured marginal
source cost at every checkpoint past rank readiness. It depends on L1.

**Expected value.** Second order (≈5–15% on top of L1 at p1009). It is larger
at small p (L3).

### L3. The L1 gain grows as p shrinks, which is where IC is still behind rho

**Mechanism.** `|D_universe| ~ p²/4`, while the collection needed for rank
grows roughly like p (sources scale with the base size, about p/2), and
`|D_exact|` grows with it. So f scales roughly like 1/p and saturates near 1
at small p. There `m(1) = 76`: almost every `F_p`-solution whose sextic has
only linear and quadratic factors becomes a row. The descent is p-independent
and decides the crossover with rho (section 0). L1 at small p therefore
shrinks the fresh ladder's crossing point.

In the `estimate.csv` model the descent is about 7.5 s per target. Rho-neg
b16 is 0.8 s at p=223 and 4.2 s at p=383. If the descent fell to about 1.8 s
(C1 × L1 at p1009-level f), IC would pass rho at about p≈380 instead of
≈450–660. With f near 1 at p ≤ 223 the descent becomes negligible, and the
crossing point is set by T_nd. Illustrative model only; no ranking or
parity claim. [pcc:unverified]

**Already tried.** None. Fresh-scaling designs the ladder (`fresh-scaling/`),
with frozen-pipeline arms only. Its dry-run targets at p=127/223/383 are not
evidence; they may serve as local positive controls.

**Cheapest local falsification.** On a dry-run p=127 target (a control, not
evidence):
1. Count |D_universe| exactly (about 8k U).
2. Run the local deferred collection to rank readiness, and record |D_exact|.

If `f(127) < 4 · f(1009)`, the 1/p scaling is wrong and L3 collapses into
L1.

**Expected value.** High for distance to rho, because it acts in the regime
where IC currently loses. It is contingent on L1's step-1 and step-3
outcomes.

### L4. Integrate C1 trace replay (already queued; listed for ranking only)

**Mechanism.** Joux–Vitse CT-RSA 2011 F4 trace replay. 2.29x fewer F4
multiplications. Descent is 1.68x lower under conservative pricing with the
linear stage counted (`complete-accounting/README.md`). 0 disagreements on
4,921 residuals.

**Status.** Queued as M1 in `descent-finder/PROTOCOL_DRAFT.md`. It composes
with L1: same solves, different post-processing. It is not a new lead.

**Expected value.** Certain at the counted level, about 1.7x on the descent.
The C5 wall null warns that the wall gain is unproven.

### L5. Measure the linear stage's per-kernel split before investing in C4 (triage of a queued lead)

**Mechanism.** The counted linear stage on the Run50 stream is:
- 653.1M `mac`;
- 37.9M `mul` and 44.6M `add`;
- 28.4M words and 4.8M probes.

(`complete-accounting/runs/recount-seed389-n3415.json`.)

C4 ("replay the normal-form pattern") removes pattern discovery: divisibility
tests, probes and map lookups. On these counts that is at most about 5% of
the linear stage, or about 1% of the descent, unless the replay also removes
`mac`s. The `mac`s are split between the NF construction of `M`, the
transposed Krylov pass (2δ = 128 sparse matrix-vector products), BM,
`x^p mod f` (Karatsuba with lazy reduction already: `zero_dim_la.rs` lines
15–50 and 1154) and the read-out. Krylov alone, at about 1.5k nonzeros
per product, would account for most of the 199k `lin_muls` per residual. The
split has not been recorded.

After C1 the linear stage is about 39% of a traced residual's
multiplications. The split decides whether any linear-stage lead is worth
having.

**Already tried.** C4 is open in CANDIDATES.md. C3 (early abort) was rejected
there after reading `fast_stage`.

**Cheapest local falsification.** Add per-kernel `mac` tallies to the
complete-accounting counted patch (`FastStats`) and rerun
`linear_stage_recount` (13 s). If Krylov plus BM is more than 70% of the
`mac`s, demote C4 to "≤ 1% of descent" and close it. If NF is more than 50%,
test border-basis direct read-off from the replayed reduced basis.

**Expected value.** Low on its own. Its value is avoiding a wasted C4
implementation.

### L6. Collection-side work is the next bottleneck once L1, L2 and L4 land (pointer)

With L1 and L4 applied the model puts descent at about 0.7e12 against about
2.5e12 for everything else. Relations, graph containers (uncounted, #10 in
complete-accounting) and linear algebra would then dominate. Existing
unselected collection leads:
- `COVER_SIEVE_REVIEW.md`: the Joux–Vitse §3.2–3.3 parameter sieve with
  two large primes;
- `quadratic-remainder-component/NEXT.md`: the useful-rank cohort.

Not new. Recorded here so that the ranking stays honest about what follows
L1.

## Checked and rejected (so nobody re-queues them without new information)

| Idea | Why rejected (arithmetic) | Source |
|---|---|---|
| Collide two descent partials on the same D (descent-side GTTD double large prime) | About 59 `(1^4,2)` partials per Run50 stream. Birthday collisions on U ≈ 2.6e5 classes: 59²/(2·2.6e5) ≈ 0.007 per target. Even the 214 `(1^2,2^2)` partials give < 0.1. L1 dominates. | this note |
| Match partials to live, unanchored graph components (variant C) | The deferred graph's 20,510 anchors make any large component anchored, so live components are small trees. The chance that two partials share one is negligible at p1009. | `deferred-graph50/README.md`, variant C in `LEAD_ANCHORED_QUADRATIC_DESCENT.md` |
| Need only one descent row | The kernel fixes logs up to a scale t. Each row gives `t·(h·s) = a + b·x`, with two unknowns (t, x). An element with a known `log_G` needs a descent, and a base element's `log_G` is unknown. Two rows are necessary. | this note |
| Decompose `R − B_j` into 5 points, sieving over 505 base points | 6 equations in 5 unknowns: about 1/p `F_p`-solutions per system, so P ≈ 1/(120p) ≈ 8e-6 per system against 1/720. Each overdetermined system would have to be about 168x cheaper. | JV 2012 §3 (lane's recorded read) |
| Re-descend an unmatched partial's D | A degree-2 D needs `L` of degree 2 for a square 6×6 system, with the same shape and the same 1/720. No better than the next residual. | this note |
| Larger descent factor base of degree-1 points | Run50 base = 509, which is already all `F_p`-abscissa points up to sign (≈ p/2). | `original-ic.raw.json` `base: 509` |
| Lazy or SIMD F4 arithmetic | Already present (`f4_fp.rs` `add_lazy_avx2/avx512`, lines 2518–2600). It does not change the counted unit. | code |
| Discriminant-character screen | Done in Run49 (see above). | Run49 README |
| NB(2, π) descent variance | Already handled: fresh-scaling critic F4 (paired streams, pooled π̂, ≥100 successes). This is accounting, not an improvement. | `fresh-scaling/README.md` |

## Sources checked

- **research-kb unavailable.** `scripts/research-kb status` prints
  `research-kb: no executable found`, and no research-kb MCP tools are loaded
  in this runtime. No novelty claim rests on the KB.
- Repository files read:
  - Run50 `NEXT.md` and README; Run48 `NEXT.md`; Run49 README;
  - `quadratic-remainder-component/NEXT.md`;
  - `LEAD_ANCHORED_QUADRATIC_DESCENT.md`, `AMENDMENT_37_ANCHORED_DESCENT.md`;
  - Run42 and Run43 READMEs; `RESUME_UNIT51.md`; `nagao-e2e/README.md`;
  - `deferred-graph50/README.md` and `recorded49-graph-replay.json`;
  - `COVER_SIEVE_REVIEW.md` and grep of all `*_REVIEW.md`;
  - round-1 `descent-finder/{README,CANDIDATES,PROTOCOL_DRAFT}.md` and
    `runs/*summary.json`;
  - `complete-accounting/README.md` and `runs/recount-seed389-n3415.json`;
  - `fresh-scaling/README.md` and `estimate.csv`; `run51-outcome/README.md`;
  - `src/cryptanalysis/zero_dim_la.rs` (header and `x_pow_p`) and
    `src/cryptanalysis/f4_fp.rs` (lazy kernels).
- Primary literature: not re-downloaded in this session. Citations rely on
  the lane's recorded reads:
  - Joux–Vitse EUROCRYPT 2012, author PDF SHA-256 `7ca88c54…b38`;
  - Swan 1962, SHA-256 `db8a7349…bdc2`, in the Run48 NEXT;
  - Joux–Vitse CT-RSA 2011, Nagao ANTS IX, and Gaudry–Thomé–Thériault–Diem
    2007, cited by CANDIDATES.md.

  The pattern probabilities `1/(∏ k^{a_k} a_k!)` are the standard
  cycle-type densities of random polynomials over `F_p`, used as heuristics
  that step 1 of L1 measures.
