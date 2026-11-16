# Pre-registration: does guessing k variables, then closing at degree 4, beat the degree-5 closure?

Registered 2026-10-24 (Conductor T-3), before any new branch below was run.
Class: **stage diagnostic with a derived row**. It measures closure matrices on
m = 3 chained `S₃` systems and derives what they imply for the m = 4 row of
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` (§6–§9). No `S`,
no speed claim, no row beats rho.

## Question

The unspecialised MutantXL closure stops refuting at degree 4 once ℓ = 7
(m = 3, N = 40 and 42; 8 of 8 draws). The degree-5 closure there is blocked on
memory (760k columns; capped at 169 GB; this host has 15 GB). The specialise
study (`research/dreg_closure_specialise_20261017/`) found that fixing one
variable lets degree 4 refute on 6 of 8 draws, and both values of variable 6 on
5 of 8. The hybrid: fix k variables, run all 2^k branches at degree 4, charge the
sum, and compare with the unspecialised degree-5 closure.

Q1. For each draw, what is the smallest k (k\*) in {1, 2, 3} for which all 2^k
branches refute at degree ≤ 4?
Q2. At (m, ℓ) = (3, 7), is Σ branch cost at k\* below the projected degree-5 cost?
Q3. What does the measured k\* imply for the m = 4 row (+7.62 bits over rho at
degree 4, +10.32 at degree 5), and does it move the lead?

## Cells, and which are infeasible on a 15 GB host

Closure memory is the dense basis: upper envelope `3·cols²/8` bytes, with
`cols = C(N−k, ≤4)`. Measured: 3.07 GB at 92,171 columns (this study's
predecessor), 6.5 GB at 112,792, and 25.8 GB at 341,537 (the (4, 6) run, 0.59×
the envelope). Time scales as cols^2.2 from 360 s at 92,171 columns. N is the
unknown count `(m−2)·n + m·ℓ`; surplus S = (m−1)n − N.

| cell (m, n, ℓ) | S | N | cols at k = 0 / 1 / 3 | envelope GB at k = 0 / 1 / 3 | status |
|---|--:|--:|---|---|---|
| (3, 19, 7) | −2 | 40 | 102k / 92k / 75k | 3.9 / 3.2 / 2.1 | **core** |
| (3, 21, 7) | 0 | 42 | 124k / 113k / 92k | 5.8 / 4.8 / 3.2 | **core** |
| (3, 22, 8) | −2 | 46 | 179k / 164k / 137k | 12.1 / 10.1 / 7.0 | **stretch E8** (measured scale about 0.6×: 4–7 GB; 14–26 min a branch) |
| (4, 15, 6) | −9 | 54 | 343k / 318k / 272k | 44 / 38 / 28 | **infeasible**: at k = 3 still above 15 GB (the measured 0.59× gives 16 GB). It also needs no guess: unspecialised degree 4 already refuted both of its draws (`dreg_closure_m4l6_20261018`), so k\* = 0 there. |
| (4, 8, 7) | −20 | 44 | 150k / 137k / 113k | 8.4 / 7.0 / 4.8 | **fits but cannot refute**: at S = −20 every draw is satisfiable (unsatisfiable draws at S = −9 were 2 in 251). It is soundness-only and is not run. |
| (4, 28, 7) | 0 | 84 | 2.03M / 1.93M / 1.75M | 1,540 / 1,400 / 1,150 | **infeasible**: the first S ≈ 0 cell with ℓ = 7. Even k = 8 leaves 1.36M columns (690 GB). |
| (4, 30, 7) | +2 | 88 | 2.45M / 2.34M / 2.13M | 2,240 / 2,050 / 1,700 | **infeasible**, same reason |

So (4, 7) is **not run**: an unsatisfiable (4, 7) system needs N ≈ 84 unknowns, and
no k ≤ 3 brings its matrix within 15 GB. The m = 4 consequence is therefore a
derivation from m = 3 measurements (Q3), marked as such, and the m = 4 row is
**not measured**.

Draws:

- **Old draws (8):** the specialise study's, `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`
  draws 0, 3, 5, 7 and `m3-21-7.jsonl` draws 0, 2, 3, 6. All unsatisfiable, and
  unspecialised degree 4 saturates without refuting on each.
- **Fresh draws (up to 4):** `dreg_ladder --cells 19:7:3 --m 3 --unsat 2 --ffd-max 3 --seed 20261024`
  and the same at `21:7:3`, written to `draws/m3-19-7f.jsonl` and `draws/m3-21-7f.jsonl`
  before any branch of them is run. They count as additional draws; a fresh draw
  whose unspecialised degree-4 baseline refutes is **dropped from the analysis and
  listed** (it is not a degree-5 case).
- **Satisfiable controls (3):** `m3-19-7.jsonl` draws 1 and 2 (6 solutions each)
  and `m3-21-7.jsonl` draw 1 (12 solutions).

## Variable-choice rules (fixed now)

Summand i's coordinate variables are `i·ℓ … i·ℓ+ℓ−1`; the intermediate point's are
`m·ℓ … m·ℓ+n−1`. Branch b ∈ [0, 2^k) assigns the j-th rule variable the bit
`(b >> j) & 1`.

- **R1 (primary): the top coordinate of each summand in turn**, `c_{0,ℓ−1}`, `c_{1,ℓ−1}`, `c_{2,ℓ−1}`.
  At ℓ = 7 these are variables **6, 13, 20**; at ℓ = 8, 7, 15, 23. The rule is
  nested (k = 2 refines k = 1), uses no draw-specific information, and is the
  predecessor's arm A (k = 1) and B (k = 3, branch 0).
- **R2 (sensitivity only): the lowest intermediate coordinates**, `mℓ, mℓ+1, mℓ+2` (21, 22, 23 at ℓ = 7). The predecessor's
  arm C is its k = 1, branch 0. R2 carries no part of the verdict.

No variable is chosen, replaced or dropped after any outcome is seen.

## Measurements per branch

One `examples/dreg_closure --to 4 --fix … --draw K` process per branch (the
unchanged shared tool; it already takes any number of `--fix v=b` pairs), run as
`prlimit --as=15000000000 timeout 3600`, under the heavy-compute lock, 4 threads.
Per branch:

- **refutes:** the closure refutes at degree 3 or 4 (`closure_degree` not null), or
  **does not** (degree 4 saturated, `1` absent), or **unmeasured** (killed);
- **dimensions:** columns (monomials of degree ≤ 4 in the variables that remain) and closure rank at the last degree built;
  the Macaulay row count is not emitted by the tool and is not reported;
- **wall seconds** (sum over degrees built), **peak RSS** (the process's VmHWM), **`closure_word_ops`** (64-bit word operations the elimination is charged),
  and **`closure_products`** (fall × monomial products inserted).

## Branch order and the adaptive rule (frozen)

1. Branches of a (draw, k) run in order b = 0, 1, …. The first branch that
   does not refute **fails the (draw, k)** and its remaining branches are not run
   (the hybrid at that k cannot be completed at degree 4). Their cost is simply
   absent, and the failed attempt's cost is still counted in the "with failed
   attempts" column.
2. For each unsatisfiable draw, k = 1 runs first, then k = 2 **only if k = 1
   failed**, then k = 3 only if k = 2 failed.
3. **Probes.** On two draws where k = 1 succeeds (the lowest-index ones in
   queue order: (19,7) draw 3 and (21,7) draw 0, if both succeed at k = 1), k = 2 and k = 3 run in full
   too, to measure how the branch cost falls with k. These are the only
   draws that run k > k\* on purpose.
4. The predecessor's records are **reused**, not rerun, for the R1 branches they
   contain: k = 1, b = 0 and 1 (its A0, A1) and k = 3, b = 0 (its B), and for R2 k = 1, b = 0 (its C).
   Their command line is identical, and a reproducibility check (control N1 below) re-measures one of them.
5. Tier order, with the time estimate from the table above. A tier not reached is **unmeasured** and is
   reported as such (no extrapolated stand-in):

| tier | content | branches | est. |
|--:|---|--:|--:|
| 0 | build; plant check; N1 placebo and A/A | 2 | 15 min |
| 1 | satisfiable controls, R1, k = 1, 2 on (19,7) draws 1, 2; k = 1 on (21,7) draw 1 | 14 | 80 min |
| 2 | old draws, R1, escalation per rule 2 | ≈ 20–30 | 2–3 h |
| 3 | fresh draws: k = 0 baseline, then R1 per rule 2 | ≈ 20 | 2 h |
| 4 | probes (rule 3) | 22 | 1.5 h |
| 5 | R2, k = 1, branch 1 on the 8 old draws | 8 | 1 h |
| 6 | stretch E8, (3, 22, 8): draw generation, k = 0 baseline, R1 per rule 2, two unsat draws | ≈ 12 | 4 h |

Every heavy item runs under `flock …/heavy.lock`, one branch per lock hold (each
branch is 2–30 min, so a hold is at most about 30 min), and a branch whose output
is non-empty is skipped on restart. Interruptions are recorded in `runs/queue.log`.

## Controls

- **Positive control P (a planted solution survives).** On each satisfiable draw above,
  `dreg_hybrid plant` enumerates every solution exactly and checks that the assignment
  satisfies every equation of the built system (instrument check). Then R1 branches
  at k = 1 and k = 2 (k = 1 only on (21,7)) are run with `dreg_closure --satisfiable`.
  - **Pass:** every branch whose guessed values agree with some solution does **not** refute, and not
    every branch of a satisfiable draw refutes.
  - **Fail:** any such branch refutes. The closure is then unsound at this scale and the study is
    **void** (reported with every run shown, no Q1–Q3 reading).
  - Branches that contain no solution may refute or not; they are reported and carry no verdict.
- **Null control N1, repaired (placebo that must reproduce).** The predecessor's null (arm C) predicted
  "an intermediate-coordinate fix does not refute" from H_max, and 6 of 8 refuted, so the control failed for a
  reason about the hypothesis, not about the instrument. The replacement states only logical
  expectations that can fail through an instrument or harness fault:
  - **N1a (placebo).** Fix variable 60, which occurs in no equation (N ≤ 46), on (19,7) draw 3 and (21,7) draw 0.
    Pass: columns, closure rank, refutation outcome and `closure_products` equal the
    unspecialised scale-study records exactly, `closure_word_ops` within 1 %.
    (The predecessor's pre-registration check did this on a (16, 6) draw.)
  - **N1b (A/A).** Rerun the predecessor's (19,7) draw 3, `--fix 6=0`. Pass: outcome, columns and rank equal its record, word ops within 1 %,
    wall time within 25 %.
  - **N2 (no hybrid on a satisfiable draw).** On each satisfiable draw the
    "all 2^k branches refute" status must not occur at the k run (part of P above).
  Failure of N1a or N1b means the reused records or the harness drifted: the study is
  **uninformative**, with every run shown. R2 vs R1 is reported as a descriptive comparison with no pass condition.

## Cost model, unit, and the §5a band

- **Unit: `closure_word_ops`**, the tool's counted 64-bit word operations, summed over
  the branches of a (draw, k) and over all of a draw's attempts. It is the same counter on
  every arm. Wall seconds are reported as a practicality note, never as the metric.
- **Hybrid cost** `H_k(d) = Σ_b ops(d, k, b)` over the 2^k branches (all must refute for the cell to count);
  `H_total(d)` adds the failed attempts at smaller k (a lower bound, since a failed attempt stopped at its first
  failing branch).
- **Degree-5 arm, projected (never measured here).** `D5(d, e) = ops4(d) · (C(N, ≤5) / C(N, ≤4))^e`, with
  `ops4(d)` the draw's own measured unspecialised degree-4 word ops and
  `e ∈ {3.0, 4.1, 4.25}`:
  - 3.0 is the dense-elimination floor (a deliberate lower-cost choice: it favours the degree-5 arm);
  - 4.1–4.25 is the exponent across degrees 4 → 5 measured in the committed records at N ≥ 19
    (`dreg_closure_mutantxl_20261016` and later satisfiable-control scans, cells up to N = 25, 4.10–4.25;
    N = 16–18 gave 3.9–4.5). It is *larger* than the 2.2 within degree 4 across cells; this study uses
    the measured across-degree value, not 2.2.
  - Because the projection comes from satisfiable-system scans, a refuting degree-5 run could stop earlier;
    and degree 5 may itself fail to refute. Both make the projection an **optimistic-for-degree-5** estimate.
- **§5a band.** Native work the counters do not charge: `closure_products` (each builds a row of
  `⌈cols/64⌉` words), the branch substitution, and, for the rho arm of the m = 4 row, the canonicalisations,
  partition hashes and table operations that rho's counted unit leaves out (the reference's counters are
  not emitted: an **implementation gap**, so the rho column is a lower bound).
  Priced at g ∈ {0, 0.1, 1}: `cost_g = ops + g · Σ products · ⌈cols/64⌉` for both the hybrid and degree-5 arms
  (the degree-5 arm scales its products by the same projection). Two arms are ranked only where the order is the same at every g and every e.
  For the rho comparison, the rows are in unconverted touches (as the note), the distance remaining is
  reported as bits over rho at g = 0, 0.1, 1 with rho charged `(1 + g·u)`, u = 1 unpriced operation per step as the band's unit,
  and the break-even `log₂ K` (touches per group operation) that would close the distance is stated.
- **Model for the m = 4 row.** `dreg_hybrid model` is `descent_degree_cost`'s model
  (relations 2^ℓ; targets m!·2^{n−(m−1)ℓ}; algebra m·2^{2ℓ}; N = (m−2)·n + m·ℓ at n = 131) with the oracle
  `2^k · C(N−k, ≤4)^w` per target, minimised over integer ℓ in 4..70, at w = 1 and 2, with κ = the largest k\* measured
  and three scenarios for k(ℓ):
  - **S_const:** k = κ at every ℓ (optimistic);
  - **S_lin:** k = κ·(ℓ − 6) for ℓ > 6 (the rise begins at ℓ = 7, as at m = 3 and by the (4, 6) result);
  - **S_linm:** k = κ·m·(ℓ − 6) (the predecessor's per-summand reading).
  The model reproduces the note's degree-4 (`2^68.43`, +7.62) and degree-5 (`2^71.13`, +10.32) m = 4 rows at k = 0, w = 1; if it does not, the
  model is wrong and nothing is derived from it.

## Decision rule (before data)

Let U be the unsatisfiable draws (old plus fresh) measured to completion under R1. Controls P and N1
must pass, else the verdict is **uninformative** (every run shown).

**Q1/Q2 verdict at (3, 7)**, using draws for which the escalation ran to a result (k\* ∈ {1, 2, 3} or "> 3"):

| outcome | condition | reading |
|---|---|---|
| **V-small** | k\* ≤ 2 on at least 75 % of U, none > 3 | a small constant number of guesses restores degree 4 at ℓ = 7 |
| **V-large** | k\* > 3 (fails at k = 3) on at least 25 % of U | up to 3 guesses do not restore degree 4; degree ≥ 5 needed, and the hybrid is not a way around it |
| **V-mixed** | neither | reported per draw |

Needs |U| ≥ 6 old draws complete; fewer is **inconclusive**, listed.

Cost, per draw with k\* found: **hybrid cheaper at (3,7)** if `H_{k*}(d) < D5(d, e)` at every e and g in the band; **degree 5 cheaper** if the reverse holds at all of them; otherwise unresolved at that boundary.

**Q3: the implied m = 4 row**, in bits over rho and distance remaining (not pass/fail):

- Row bounds, from the model, for every κ ≤ 3: S_const, S_lin, S_linm at w = 1 and 2.
- **The hybrid improves the m = 4 degree-5 row (+10.32) iff** V-small holds **and** the S_lin row (w = 1) is below +10.32.
  Otherwise the degree-5 row stays the best-supported optimistic row and the hybrid is not an improvement at the exponent level.
- If E8 ran: **κ(ℓ = 8) > κ(ℓ = 7)** on the same rule (maximum over completed draws) reads as **k grows with ℓ** (S_lin favoured over S_const);
  κ(8) = κ(7) reads as "S_const not rejected", which is one more point on a flat line and not a proof. If E8 did not run, the const/lin choice is **unresolved** and both rows are reported.
- In every outcome the row is **above rho**: the degree-4 floor with k = 0 is +7.62 bits (w = 1), and every k ≥ 0 hybrid is at or above it.
  Hence the lead does not close below rho at degree 4 with a guess, and degree ≥ 6 only raises the cost. The hybrid can at most recover the 2.7 bits between the degree-5 and degree-4 rows.

## Predictions (recorded, not tuned to)

- P and N1 pass: confidence 0.9.
- k\* = 1 on 5 of the 8 old draws (known from the predecessor); k\* ≤ 3 on 7 of 8 (confidence 0.6; draw (19,7) 5 resists single fixes).
- Fresh draws: k\* ≤ 2 on at least half.
- At (3, 7) the hybrid is cheaper than the projected degree-5 closure by a factor above 10 at every e: 0.8.
- The break-even k at the m = 4 row is a parameter: the oracle-only break-even against degree 5 (computed by `dreg_hybrid model` before any branch was run) is k ≤ 6 at w = 1 and k ≤ 12 at w = 2 at N = 290–398, so V-small + S_const is the only branch that can improve the row.

## Scope, class, disclosures

- **Class:** stage diagnostic. No `S`, no rho ratio from measurement. The row is derived and marked so.
- **Host:** 4-core, 15 GB, shared, **not isolated**; four agents share it, heavy runs under the lock. Timings are L0 cloud-VM readings.
- **System:** m = 3 chained `S₃` with b = 1, random ℓ-dimensional subspaces of GF(2^n), n = 19, 21 (and 22). n = 19 and 21 are prime (no proper subfield); n = 22 has GF(2²) and GF(2^11), unused since the subspaces are random (AGENTS.md §8b). Nothing here is evidence at m = 83 or 131 (§8a).
- **Instrument:** `examples/dreg_closure` (unchanged) for every branch, so the shared files changed are none. New files: `examples/dreg_hybrid.rs` (plant, score, model) and this directory.
- **Closure check:** the dense closure's soundness is held by the predecessor's satisfiable replays and `sparse_closure_matches_the_dense_closure`, and re-held here by P.

## Note added before data (model check)

`dreg_hybrid model` reproduces the note's degree-4 and degree-5 rows exactly at k = 0
(m = 4: `2^68.43` / +7.62 and `2^71.13` / +10.32; m = 3: `2^82.27` and `2^85.18`), so the
model is accepted. Scenarios whose k(ℓ) is zero for ℓ ≤ 6 may take their minimum at ℓ ≤ 6, where no
guess is charged and the target-search term dominates; that is the model's honest minimum
and is reported as such, not removed.

## Amendment 1 (2026-10-24, after the N1 runs, before any outcome branch)

N1 as written required `closure_products` to equal the committed record exactly. The first three N1
runs (two placebos, one A/A) show columns, closure rank and refutation outcome identical, `closure_word_ops`
within 0.007 %, and `closure_products` within 0.3–0.7 %: the product count depends on thread scheduling, an
assumption of determinism this pre-registration did not check. Wall time varied by 0.8× to 2.05× (the host is
shared; N1a had no time criterion).

- **Registered reading (as written):** N1 FAILS on products, so the verdict is "uninformative".
  `dreg_hybrid score --prod-tol 0` prints it.
- **Amended reading:** product tolerance 2 %, everything else unchanged. Under it N1 passes.
- **Both are reported.** Q1–Q3 are read under the amended reading and labelled post-amendment; the
  amendment concerns a control's tolerance on a counter that enters the §5a band only at the < 0.3 % level
  (see RESULTS.md), not any outcome branch. A reader who holds the registered text strictly reads the
  whole study as uninformative.

## Amendment 2 (2026-10-24, before any outcome branch): queue order

With four agents sharing one heavy-compute lock, a branch that computes in 3–8 minutes took 15–25 minutes of
wall time, so the registered tier order (all of tier 1 before tier 2) would have run for about a day. The order is
changed to value order, **before any unsatisfiable-draw outcome branch ran**, with every item and every rule above unchanged:

1. controls at k = 1 (all three satisfiable draws; the two (19,7) draws and the (21,7) draw);
2. escalation on the three old draws whose k = 1 failed ((19,7) 0 and 5, (21,7) 3);
3. the k = 2 control on (19,7) draw 1 (the k = 2 control on draw 2 is dropped);
4. fresh draws (k = 0 baselines, then escalation);
5. probes (tier 4), then R2 (tier 5);
6. E8 (tier 6) only if everything above is done.

A tier or item not reached is reported unmeasured. The soundness control P is evaluated on what ran.
`run2.sh` is this order; `run.sh` holds the same functions and tiers 0–5 as registered.

## Amendment 3 (2026-10-24, before any unsatisfiable-draw outcome branch): scope under lock contention

Measured pace after Amendment 2 was about 40 minutes of wall time per locked branch. The queue is cut and split
in two, with no rule changed:

- **Chain A (locked, 4 threads):** (21,7) draw 3 escalation; control P: (19,7) draw 2 branch 1 at k = 1, then (19,7) draw 1 at k = 2.
- **Chain B (no lock, `RAYON_NUM_THREADS=2`):** the (19,7) escalations (draws 0, 5) and then the fresh (19,7) draws. Allowed
  by the shared rules because those branches use 2 cores and at most about 3.1 GB (peak RSS is recorded per branch). **Their wall
  times are not comparable with locked ones and are marked `unlocked`**; the metric (`closure_word_ops`) does not depend on wall time.
- **Dropped, unmeasured:** the (21,7) draw 1 control, the fresh (21,7) draws, the probes, R2 branch 1, E8.
  The (21,7) cell is therefore represented by old draws 0, 2 (k = 1) and 3 only.

## Amendment 4 (2026-10-24, after the (19,7) and (21,7) d3 escalations and the fresh (19,7) draws had run)

The fresh (21,7) draws are not run (their k = 0 baselines exceed the 4 GB unlocked limit and the lock pace is 30–40 min a branch).
Remaining items, in order and both rules unchanged: chain C (unlocked, 2 threads) the probes on (19,7) draw 3 and R2 k = 1 branch 1 on the
(19,7) old draws; chain D (locked) the k = 2 soundness control on (19,7) draw 1. A stray control branch (19,7 draw 2, k = 1, b1) that was
already queued behind the lock when chain A was stopped ran to completion and is reported.
