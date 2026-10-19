# Factor-base designs: preregistration (frozen before any data)

**Frozen:** 2026-10-15, before any run in this directory. Later changes go in
an appended, dated "Amendments" section. They never edit the text above it.
**Study:** `research/factor_base_designs_20261015/`. The design register and
the Lean proof map are in [`README.md`](README.md).
**Kind of work:** solver-axis stage diagnostics (AGENTS.md §2). Nothing here
prices a pipeline or reports `S`. No run here can show a speedup.

## 1. The quantity every experiment measures

Every arm is scored by the **slope of the solving degree** against the
factor-base entropy per summand. Write `b = log₂|V|` per summand. That is
`ℓ` for a binary subspace and `t` for a depth-`t` tower or a `t`-bit exponent
interval. The score is `σ = ΔD / Δb`, fitted by least squares over the
completed cells of one (arm, `m`) series.

Why the slope and not `D` at a point: under the repository's yield
conservation (`AR:KN-FIND-007`), yield per trial depends only on `|V|` and
`m`. One more bit per summand buys `m` bits of yield. At deployed sizes,
one more unit of `D` costs several bits of Gröbner work (README,
"Break-even", with integer instances checked in Lean, `BreakEven.lean`).

**Decision thresholds (fixed now):**

| reading | meaning |
|:--|:--|
| `σ ≤ 0.25` on at least 3 completed cells | **candidate**: degree grows slowly enough for an interior optimum. Replicate on a second prime before any further claim |
| `0.25 < σ < 0.5` | **weak**: reported. Follow up only if it beats the incumbent by ≥ 1 in `D` at equal `b` |
| `σ ≥ 0.5` | **no middle ground at this arity**. Kept and reported |
| fewer than 3 completed cells | **inconclusive** |

The **incumbent** for prime fields is the Kummer tower (PKM), measured with
`examples/pkm_tower_pilot.rs` (`research/pkm_tower_pilot_20260924/`).

## 2. Instrument

The instrument is the repository's degree-bounded F4 (`src/cryptanalysis/f4_fp.rs`)
in grevlex, with `solving_degree_max` as `D`, exactly as the PKM pilot defines
it. Every system is checked to vanish at its planted solution before it is
measured. A failing check aborts the run and is never recorded as a degree.
Limits: 300 s per system, `ulimit -v 13000000`. A system that hits a limit is
recorded with its reason and counts as a missing cell.

## 3. Experiments in this PR (prime field, `p₁ = 786433 = 3·2^18 + 1`)

`p₁` is the pilot's prime, so arm A0 must reproduce committed pilot rows,
which serves as the instrument check. Seeds are derived from `0x46424453`
("FBDS") combined with (arm, `m`, `t`, target index). Each cell has 2 planted
targets: the target is the sum of `m` random base points, so a solution
exists. `m ∈ {2, 3}`. `t` rises from 2 until a limit is hit.

| arm | design (README #) | `V` per summand | membership presentation |
|:--|:--|:--|:--|
| **A0** Kummer tower (incumbent) | — | `g·μ_{2^t}` | `y_{j+1} = y_j²`, `y_{t−1}² = 1` (`t` unknowns) |
| **A1z** exponent interval, ζ | #1 | the same set `g·μ_{2^t}` | `x = g·∏_j(1 + (ζ^{2^j} − 1)b_j)`, `b_j² = b_j`, partial products (`2t − 1` unknowns) |
| **A1g** exponent interval, generic | #1 | `{h·g^k : 0 ≤ k < 2^t}`, `ord(g)` not a power of 2 | as A1z |
| **A4** parallel tower | #4 | `{u + v}`, `u ∈ h·μ_{2^a}`, `v ∈ h'·μ_{2^b}`, `a = ⌈t/2⌉`, `b = ⌊t/2⌋` | two Kummer towers and `x = u + v` (`t + 1` unknowns) |
| **A11** norm chain | #11 | `g·μ_{2^t}` | Riemann–Roch function through `−R`, `G(gX) ∣ X^{2^t} − 1` as `t` squaring steps mod `G` |
| **N** null | control | as A0 | as A0, with `S₃`'s support given random coefficients and a planted zero |

A1z isolates the **presentation** from the **set**: it has the same `V` as
A0. A1g tests whether dropping the smoothness requirement costs extra
degree. A4's `|V|` is recorded exactly (`u + v` can collide). A cell with
`|V| < 2^t` reports its true `b`.

**Predictions recorded now:**
- **A1z**: `D` within ±1 of A0 at equal `b`, and `σ` no smaller. The extra
  variables make a win unlikely.
- **A1g**: equal to A1z within ±1. If so, the smoothness restriction on
  towers can be removed at no extra degree.
- **A4**: if `D` tracks elimination depth, A4 at `t` matches A0 at `⌈t/2⌉`
  plus at most 1. If `D` tracks the Bezout number, A4 matches A0 at equal `t`.
- **A11**: no prediction. This is the arm with the most room to differ.

## 4. Designed, not run in this PR

Each one gets its own amendment before running. The arms are in the README
register.

- E2 Lucas-ladder torus interval; E3 elliptic interval on an auxiliary curve;
  E5 radix-3 auxiliary towers; E6 torus tower at analogue primes
  `p ≡ −1 (mod 2^k)` (the P-521 shape); E7 fan-in roots; E8 lopsided depths;
  E9 weighted grevlex on towers; E10 split chain + towers, with treewidth
  recorded.
- E12 / E13: binary norm chain and algebraic large prime, through
  `research/nagao_relations/` (its open item 1).
- E14 / E15: learned frames and family-slope selection, in the
  ml-cryptanalysis CSD pipeline with a held-out field family.
- E16: arity portfolio (engineering).
- **E-K1**: Gauss-period window in `s = x + 1/x` on Koblitz analogues
  `n ∈ {11, 23}` (`2n + 1` prime, type-II normal basis), against the
  `x`-window and the `u`-frame bases of `koblitz_symmetrised.rs`.
- **E-K4** (MinRank modelling) and **E-K6** (shared-trunk towers): reopened,
  because no Lean-checkable statement disposes of them.

## 5. Stop rules

- If A0 fails to reproduce the pilot's committed `D` on the same cells, the
  study stops (instrument fault).
- An arm reaching **candidate** on `p₁` is replicated on `p₂ = 2013265921`
  before it is described as anything more than a single-prime reading.

## Amendments

### Amendment 1, 2026-10-15: implementation details, before the first data row

Registered by the experiment lane (`examples/fb_designs_slope.rs`) before any
row exists in `runs/`. Each item settles a detail §2–§3 leave open. None
changes an arm, a threshold or a prediction.

1. **Instrument version.** `f4_fp` has applied the Gebauer–Möller pair
   criteria by default since commit `3d4634596` (2026-10-11), after the pilot.
   This study runs the current default. Reason: before any data, the 16 planted
   systems of the pilot's Kummer cells (`m = 2`, `t ≤ 6`; `m = 3`, `t ≤ 3`)
   were rebuilt with the pilot's seed rule. Their `D`, `x_R`, curve, tower,
   unknown and equation counts, `inconsistent` and `planted_ok` equal every
   committed pilot row. Only `degree_reached` (the certification tail, not `D`)
   is lower. With `F4_FP_GM=0` it matches too. `degree_reached` is recorded,
   but it is not comparable with the pilot's.
2. **Instrument check (§5).** "The same cells" means A0 rebuilt with the
   pilot's seed rule and draw order (`repro`), compared row by row. This
   study's own seeds give other towers and curves, so the A0 series is also
   compared with the pilot cell by cell at matching (`m`, `t`). That
   comparison is reported but does not stop the study.
3. **Seeds.** The cell seed is `0x46424453 ⊕ arm≪40 ⊕ m≪32 ⊕ t≪16`, with arm
   index A0 = 0, N = 1, A1z = 2, A1g = 3, A4 = 4 and A11 = 5. It draws the set
   and the target curve, which both targets of a cell share. Target `k` is
   planted from `cell seed ⊕ (k+1)≪8`. N's random coefficients come from the
   same stream.
4. **Completed cell and fit.** A cell is completed when both of its planted
   systems finish within the budget and the planted solution satisfies F4's
   basis. The cell's `D` is the mean of the two. `σ` is the least-squares slope
   of the cell `D` on `b`. Any verdict other than "inconclusive" needs at
   least 3 completed cells.
5. **Limits and the stop.** `t` rises from 2 to at most 18, because
   `2^18 ∥ p₁ − 1`. Each system runs in its own child process under
   `ulimit -v 13000000`, with F4's 300 s budget. The child is killed at 420 s
   if F4 has not returned to its deadline check. A series stops after the
   first cell in which any system hits a limit. That cell is recorded with its
   reason and counts as missing.
6. **N at `m = 3`.** The pilot's null is defined only at `m = 2`. Here every
   `S₃` of the chain gets random coefficients on its own support, shifted to
   vanish at the planted solution.
7. **A1.** `x = π_{t−1}(1 + β_{t−1}b_{t−1})` is substituted into `S₃`; it is
   not an unknown. That gives `2t − 1` unknowns per summand. For A1g, `h` and
   `g` are drawn uniformly. `g` is redrawn when its order is a power of 2 or
   when the `2^t` elements repeat. `ord(g)` is recorded.
8. **A4.** `x` is an unknown, with the linear equation `x − h·y_0 − h'·z_0`,
   giving `t + 1` unknowns as in §3.
9. **A11.** The function passes through `−R`:
   `f = y − P(X)` with `P(X) = y(−R) + Σ_{i≥1} c_i (X^i − x_R^i)`. At `m = 2`
   this is the line `y = λ(X − x_R) + y(−R)` with `λ = c_1`. `G` comes from
   exact division of `N(X) = X³ + AX + B − P(X)²` by `X − x_R`, and the zero
   remainder is checked symbolically. The coefficient equations are
   `lead(G)·g^{m−i}·s_i − G_i = 0`. At `m = 2`, `lead = 1`, so they read
   `g·s_1 = e_1` and `g²·s_0 = e_0`. **`m = 3` is run:** `deg P = 2`, `G` is
   a cubic with leading coefficient `−c_2²`, and each `q_j` is linear. That
   gives `2 + 3 + 5t` unknowns and `3 + 5t + 3` equations. `m = 2` has
   `3 + 3t` unknowns and `3t + 4` equations. `b = t`. The presentation is
   checked in two ways. Every element of `V` is `g` times a `2^t`-th root of
   unity. On 128 random `m`-subsets, half of them with one element outside
   `V`, `X^{2^t} ≡ 1 mod ∏(X − v/g)` holds exactly when every element is in
   `V`.

### Amendment 2, 2026-10-15: A11 follow-up (F1–F4), before any of its rows

Registered after the p₁ run (`reports/run-20261015T1657Z`, review in
`reports/review-20261015T2030Z`) and before any row of F1–F4 exists. It adds
experiments. It changes no p₁ row, verdict, threshold or prediction. The
instrument, the per-system limits (300 s F4 budget, 420 s hard kill,
`ulimit -v 13000000`), the stop rule (Amendment 1 item 5), the completed-cell
rule and the σ fit are those of Amendment 1. The exceptions are stated below.

**Why.** On p₁, A11 at `m = 3` was the one arm below A0 on both D (4 vs 6) and
matrix width (≈ 4× narrower). That rests on 3 completed cells, one prime and
no null of its own shape (review, point 4). §5 requires replication. The
review lists F1–F4 as owed.

**Seeds.** As in Amendment 1 item 3, with a prime index:
`cell seed = 0x46424453 ⊕ prime≪48 ⊕ arm≪40 ⊕ m≪32 ⊕ t≪16`, where p₁ = 0
(so every p₁ seed is unchanged) and p₂ = 1. New arm indices: N11 = 6.

- **F1, replication on p₂ = 2013265921 = 15·2^27 + 1.** Arms A0 and A11, at
  `m = 2` and `m = 3`, with `t` rising from 2 (at most 27) until the stop
  rule. Before any p₂ row, check that `f4_fp` and the field arithmetic are
  exact at p₂ (`p₂ < 2^31`): rebuild one A0 and one A11 planted system at
  `t = 2`, check the planted solution, and record the check. If the
  instrument cannot run at p₂, F1 stops and the reason is recorded.
- **F2, A11-shape null (N11), p₁, `m = 2` and `m = 3`.** N11 is A11 with the
  curve removed, exactly as N is A0 with the curve removed. The equations
  that define `s_i` from the function's coefficients (Amendment 1 item 9,
  `lead(G)·g^{m−i}·s_i − G_i = 0`) are replaced by polynomials on the same
  monomial support, with independent uniform random coefficients, shifted
  by a constant so they vanish at the planted solution. The chain equations
  and `r_t = 1` are kept unchanged. The planted solution satisfies every
  equation, and this is checked. Unknown and equation counts equal A11's.
- **F3, the `m = 3`, `t = 5` cell, p₁.** Arms A0 and A11, 2 planted targets
  each, with F4 budget 3600 s and hard kill 3900 s (the only budget change).
  Memory limit unchanged. A system that still hits a limit is recorded as such.
- **F4, `m = 4`, p₁.** Arms A0 and A11, with `t` rising from 2 until the
  stop rule.
  - A0 at `m = 4` uses the `S₃` chain with two intermediate unknowns, as the
    pilot does at `m = 3` with one.
  - A11 at `m = 4` uses `f = Q(X)·y − P(X)` with `Q = X + q₀` and
    `deg P = 2`, passing through `−R` (3 free coefficients, `= m − 1`).
    `N(X) = Q²(X³ + AX + B) − P²` has degree 5. Exact division by `X − x_R`
    gives a quartic `G` with leading coefficient `1 − p₂²`, where `p₂` is the
    leading coefficient of `P`. Rows where that coefficient vanishes are
    recorded and redrawn. The chain uses `r_j` of degree 3 and `q_j` of
    degree 2.
  - Unknown and equation counts are recorded. The excess must be 1
    (`norm_chain_excess`), and this is checked.

**Decision rules, fixed now.**
- *A11 replicates* if, on p₂ at `m = 3`, A11's cell D ≤ A0's cell D − 1 and
  A11's `max_cols` < A0's at every shared completed `t`, with at least 3
  shared completed cells. *Fails to replicate* if any shared completed cell
  has A11's D ≥ A0's. Otherwise: *inconclusive*.
- *Shape, not structure* if N11's D equals A11's in every completed cell of
  F2. *Structure beyond shape* if N11's D exceeds A11's by ≥ 1 in at least 2
  completed cells. Otherwise: *mixed*, reported cell by cell.
- *F3*: reported as recorded (D, width, time). It adds a fourth A11 and A0
  cell to the `m = 3` p₁ σ fits only if both of the cell's systems complete.
- *F4*: σ verdicts as in §1. The threshold was derived at `m = 4`, so here it
  is used as designed.
- **Secondary, descriptive (not a verdict):** the per-bit growth of the
  matrix width, `Δ log₂ max_cols / Δb`, by least squares over completed
  cells, per (arm, `m`, prime). It is recorded because D alone misread A1 on p₁.

**Predictions.**
- F1: A11 replicates.
- F2: shape, not structure. The p₁ null N equalled A0, so the same is
  expected here.
- F3: both systems of A11 complete within 3600 s. A0's may not.
- F4: A0's σ ≥ 0.5. No prediction for A11.

### Amendment 3, 2026-10-15: implementation details of F1–F4, before any of their rows

Registered by the experiment lane (`examples/fb_designs_slope.rs`) before any
row of F1–F4 exists. It changes no arm, threshold, decision rule or
prediction. Items 1 and 2 settle what Amendment 2 leaves open; item 3
corrects a statement in it that cannot hold.

1. **N11's random coefficients** come from the target stream after the
   target is planted, as N's do (Amendment 1 item 3). The `m` replaced
   equations are the first `m` of the A11 system, in the same order.
2. **A cell's `max_cols`** in the F1 rule and the secondary slope is the mean
   over the cell's two systems, as a cell's D is (Amendment 1 item 4).
   Amendment 2 does not say how a cell's width is formed.
3. **The leading coefficient of `G` at `m = 4`.** Amendment 2 says `G` has
   leading coefficient `1 − p₂²` (`p₂` the leading coefficient of `P`). With
   `Q = X + q₀` monic and `deg P = 2`,
   `N(X) = Q²(X³ + AX + B) − P² = X⁵ + (2q₀ − p₂²)X⁴ + …`: `Q²X³` reaches
   `X⁵` and `P²` only `X⁴`. So `N` is monic and `G = N/(X − x_R)` is a monic
   quartic. The harness checks symbolically that `lead(G)` is the constant 1
   in every built system. Consequences: the coefficient equations read
   `g^{4−i}·s_i − G_i = 0`, and the "leading coefficient vanishes" redraw
   cannot trigger. The redraw that can trigger is the other degeneracy: when
   no `f` with `Q` monic passes through the four summands (the 4 × 4 linear
   system for `(q₀, P)` is singular), the target is replanted from the same
   stream, as A11 replants on a vanishing lead at `m = 3`. The free
   coefficients are ordered `q₀, c₁, c₂`, with
   `P(X) = Q(x_R)·y(−R) + c₁(X − x_R) + c₂(X² − x_R²)`. The excess
   (`norm_chain_excess`) is asserted to be 1 at every `m`.

### Amendment 4, 2026-10-16: cost score, a third m = 4 cell, a small-prime instrument, and an ablation of A11 (C1–C4), before any of their rows

Registered after Amendment 2's run (`reports/run-20261015T2052Z`, review in
`reports/review-20261016T0130Z`) and before any row of C1–C4 exists. It adds
experiments and one score. It changes no existing row, verdict, threshold or
prediction. Instrument, per-system limits, stop rule, completed-cell rule and
seeds are those of Amendments 1–3, except as stated. Seeds take a new prime
index p₃ = 2 and new arm indices H1 = 7, H2 = 8.

**Why.** D misread A1 on p₁, and on A11 it hides that the width grows at least
as fast per bit as A0's (Amendment 2, secondary). The F4 trace was identical on
p₁ and p₂ in every completed system, so the trace is a function of the
system's shape. m = 4, where the §1 threshold was derived, has only 2 completed
cells per arm. Nothing yet says which part of A11 makes it cheaper.

- **C1, cost score.** Every row records `field_ops`, F4's count of `F_p`
  multiplications in row reduction (`F4Report::field_ops`; symbolic
  preprocessing is not counted). Every committed completed system of the first
  run, F1, F2, F3 and F4 is rebuilt from its seed and measured again
  (`rescore`), and the regenerated row must equal the committed one on D,
  `max_cols`, `max_rows` and `steps`. A mismatch is recorded and that system's
  cost is not used. The score is **κ = Δlog₂ field_ops / Δb**, the least-squares
  slope of log₂ of the cell's mean `field_ops` on b, over completed cells, per
  (arm, m, prime). Reasoning: one more bit per summand buys m bits of yield per
  trial and needs one more bit of relations (§1), so relation collection gets
  cheaper exactly when κ < m − 1 (linear algebra not counted).
  *Verdicts:* **cost-favourable** if κ < m − 1 on ≥ 3 completed cells;
  **cost-unfavourable** if κ ≥ m − 1 on ≥ 3; otherwise **inconclusive**. These
  are read at the sizes measured and are not extrapolated.
- **C2, the third m = 4 cell on p₁.** A0 and A11 at m = 4, t = 4, 2 targets,
  F4 budget 3600 s and hard kill 3900 s, memory limit unchanged
  (`ulimit -v 13000000`). Amendment 2 asked for a bigger machine. The lab
  runner (isolab) has no CLI or credentials in this container, so the cell
  runs here, and that blocker is recorded. If both systems of an arm complete,
  the cell joins F4's σ and C1's κ for that arm.
- **C3, small-prime instrument p₃ = 40961 = 5·2^13 + 1.** Below 2^16, so F4
  takes its narrow 32-bit path, which uses half the memory per entry. *Check
  first:* every completed cell measured on p₁ (first run, F2, F3, F4, C2, and
  H1/H2 below) is rebuilt with p₃ seeds and compared system by system on D,
  `max_cols`, `max_rows` and `steps`. **p₃ is admitted** only if every
  compared system matches. A cell whose t exceeds 13 (`2^13 ∥ p₃ − 1`) is not
  compared. *If admitted:* A0 m = 3, t = 5; A11 m = 3, t = 6; A0 and A11 m = 4,
  t = 4 are run on p₃ with budget 3600 s and hard kill 3900 s, 2 targets each,
  as single cells. They are reported as p₃ readings beside the p₁ series. They
  extend a p₁ fit only through the admission rule (same trace) and are labelled
  as such. If p₃ is not admitted, the extension is not run, and the mismatches
  are reported.
- **C4, ablation of A11 (p₁, m ∈ {2, 3}, t rising from 2).** Two hybrids on
  the coset `g·μ_{2^t}`, each with excess 1 (checked):
  - **H1, A11's curve relation with Kummer towers on the roots.** The
    unknowns are A11's function coefficients and a Kummer tower `z_i = y_{i,0}`,
    `y_{i,j+1} = y_{i,j}²`, `y_{i,t−1}² = 1` per summand. The equations are A11's
    coefficient equations `lead(G)·g^{m−i}·s_i − G_i = 0` with `s_i` replaced by
    `(−1)^{m−i}·e_{m−i}(z_1, …, z_m)` (Vieta for `G_s = ∏(X − z_i)`), plus the
    towers. No summation polynomial and no chain.
  - **H2, A0's summation with A11's chain.** The unknowns are `z_i` (`x_i =
    g·z_i`), A0's intermediates `u`, and the chain. The equations are A0's `S₃`
    summation in the `x_i`, and the squaring chain `r_0 = X`,
    `r_j² = q_j·G_s + r_{j+1}`, `r_t = 1` modulo `G_s = ∏(X − z_i)` (Vieta in
    the `z_i`). No Kummer tower and no function.

  *Decision rule (m = 3, over t where A0, A11, H1 and H2 all completed, at
  least 2 such t, else inconclusive), applied to cell D and, separately, to
  log₂ of the cell's mean `field_ops`:* **the curve relation carries it** if
  H1 ≤ A11 and H2 ≥ A0 at every such t; **the chain carries it** if H2 ≤ A11
  and H1 ≥ A0 at every such t; otherwise **mixed**, reported cell by cell.
  m = 2 is reported descriptively.

**Predictions.** C1: every arm cost-unfavourable at m ∈ {2, 3}. C2: A11's
cell completes and A0's does not. C3: p₃ admitted. C4: the curve relation
carries it.

### Amendment 6, 2026-10-19: E-K1, a Gauss-period window in `s = x + 1/x` on Koblitz analogues, before any of its rows

Registered by the E-K1 lane (`examples/fb_koblitz_ek1.rs`) before any row of
`runs/K1-*` exists. §4 left E-K1 "designed, not run; gets its own amendment
before running"; this is that amendment. **Numbering and the score.**
Amendment 5, appended in another checkout and merged with this one by the
coordinator, makes κ = Δlog₂ field_ops/Δb the primary score for every later
round. This amendment follows that rule: κ is primary here, σ is secondary.
The text of this amendment does not depend on Amendment 5's wording; it
restates κ, the break-even `m − 1` and Amendment 4's thresholds below. It
changes no earlier row, verdict, threshold or prediction.

**Why E-K1 needs a design of its own.** The instrument of §2 is `f4_fp` over
`F_p`. E-K1 lives in characteristic 2, on the Weil restriction of a
summation polynomial over `F_{2^n}`, so the engine is the repository's Boolean
F4 (below). The prime-field arms, seeds and budgets are not reused, but the
rules of §1, Amendment 1 item 4–5 (completed cell, stop at the first limit) and
Amendment 4 (κ, trace replay) are, with the changes stated here.

**1. Exact facts established before any measurement** (class: exact
computation; the harness subcommand `check` proves each one or aborts, and its
log is `runs/K1-check.txt`).

1. *The s-line polynomial is Semaev's polynomial in `x`.* Write
   `w = u² + u` with `u = 1/(x + 1)`, so `s = x + 1/x = 1/w`. Eliminating the
   linear `s`-variable of `koblitz_symmetrised::symmetrised_terms` (σ in the
   code: `F = P(w) + σ·Q(w)`, `σ² + σ = Σw`) gives
   `G = P² + PQ + (Σw)Q²`. The check verifies
   `G(1/s₁,…,1/s_{m+1})·(Π s_i)^k = S_{m+1}(s₁,…,s_{m+1})`, the polynomial of `plain_terms(m)`
   (`b = 1`; `k = 2` at `m = 2`, `k = 4` at `m = 3`) on a full 6-per-variable
   grid at `n = 11` and `n = 23`. Both sides have degree ≤ 4 in each variable,
   so 5 values per variable already prove the identity; 6 are used. **So the
   `S` and `X` arms below have the same equations.** They differ only in which
   target field elements occur (`s_R` against `x_R`) and in which curve points
   the window contains.
2. *The set.* `n ∈ {11, 23}`: `p = 2n + 1 ∈ {23, 47}` is prime, `p | 2^n − 1`
   (`2` has order `n` mod `p`), so `ζ` of order `p` lies in `F_{2^n}` and
   `c_i = ζ^i + ζ^{−i}`, `1 ≤ i ≤ n`, are checked to be independent
   (rank `n`), Frobenius-permuted (`c_i² = c_{2i}`), to satisfy
   `c_i c_j = c_{i+j} + c_{|i−j|}` for all `i, j ≤ n` (the Lean `dickson_mul`)
   and to sum to 1. `ζ` is `g^{(2^n−1)/p}` for the least `g = 2, 3, …` with
   `ζ ≠ 1`. This fixes one of the two window classes (`ζ ↦ ζ^k`, `k` a
   non-residue mod `p`, gives the other); only the first is run.
3. *The curve.* `K_0: y² + xy = x³ + 1` over `F_{2^n}` (`a = 0`), with the
   repository's `find_irreducible_sparse(n)` modulus. `KoblitzCurve::new`
   rejects `K_0` at `n = 11` because `#E = 4·23²` (`r² | #E`), so the
   `KoblitzCurve` is built directly. Nothing here uses its prime-order
   subgroup. The model is confirmed at both `n` by counting every abscissa
   (`x = 0` gives `T`, other `x` give 0 or 2 points) and comparing
   `1 + count` with the Koblitz-recurrence `#E` (2116 and 8383412).
4. *Word field.* The harness's 64-bit field arithmetic equals `F2mElement` on
   200 random products and inverses per `n`; it is used for counts and for a
   second, independent evaluation of every polynomial at every planted point.

**2. Instrument (same on every arm).**
`pq_f4_f2::groebner_basis_f4`, the repository's Boolean F4 over
`F_2[v]/(v² − v)` with the Gebauer–Möller criteria and field pairs, on the
Weil restriction of each decomposition system, `F4_F2_*` environment unset.
`D = F4Stats::solving_degree` (highest step degree at which the basis gained an
element), `max_cols = matrix_cols_max`, `max_rows = matrix_rows_max`,
`steps = steps`, `field_ops = word_xors` (64-bit XORs the row-by-row
eliminations perform; the engine's counted unit; the matrix construction is not
in it). Budget 300 s per system (F4's cooperative deadline), a child process per
system under `ulimit -v 13000000`, killed at 420 s. A system is *ok* only if F4
returned, did not set `timed_out` or `oversize`, did not derive 1, and every
element of its basis vanishes at the planted point. The unit of `field_ops`
(Boolean word XORs) is not that of Amendment 4 (`F_p` multiplications); κ is a
slope on a log scale, compared with `m − 1` and across arms of this
amendment only.

**3. Arms** (`m` planted summands; the *window* `W_ℓ` has dimension `ℓ`).
`|F|` is the exact number of affine curve points whose frame coordinate lies in
the window (counted for the whole window, so every target of a cell shares it),
and `b = log₂|F|` is the per-summand entropy used by the scores.

| arm | set | frame coordinate | equations |
|:--|:--|:--|:--|
| **S** | `P` with `s(P) = x + 1/x ∈ G_ℓ = span(c_1…c_ℓ)` | `s` | `plain_terms(m)` in `(s₁…s_m, s_R)`, direct, `mℓ` unknowns |
| **X** | `x(P) ∈ G_ℓ` | `x` | the same polynomial in `(x₁…x_m, x_R)`, `mℓ` unknowns |
| **XR** | `x(P) ∈ R_ℓ`, a random `ℓ`-dimensional subspace | `x` | as X (the repository's standard `x`-window) |
| **U** | `u(P) ∈ V`, `V ∋ 1` random of dimension `ℓ` | `u = 1/(x+1)` | `build_symmetrised_system`, `m(ℓ−1)+1` unknowns (the repository's `u`-frame) |
| **NS** | as S | `s` | S's polynomial with an independent uniform non-zero `F_{2^n}` coefficient on every non-constant term of `plain_terms(m)`, plus a constant chosen so the planted point is a zero (as N and N11) |
| **PC** | as X | `x` | positive control, `m = 3` only: `build_decomposition_system`, two chained `S₃` links with `n` intermediate unknowns (the repository's production `x` system) |

`G_ℓ` is the Gauss-period window of E-K1. Its *set* equals a scaled power
window (Lean `window_eq`); what E-K1 tests is the *presentation* in `s`
(S) against the same window in `x` (X), a random window in `x` (XR), the
`u`-frame (U), and the null (NS). The coordinate basis in which the output
equations are written cannot change F4's trace (its initial echelon form of
the inputs is the same), so equations are written in the repository's
polynomial basis for every arm. The window's unknown basis is `c_1…c_ℓ` for S,
X, NS, PC; a basis of `R_ℓ` or `V` drawn from the cell seed for XR and U.
**Not run:** a `u`-frame with a Gauss-derived `V`, `a = 1`, the other window
class, `m ≥ 4`.

**4. Cells, ranges, seeds.** `n ∈ {11, 23}`, `m ∈ {2, 3}`, `ℓ` rising from 3
(from 4 at `m = 3`, see §10) until a limit (at most `n`; at most 64 Boolean unknowns, the repository's
`MAX_VARS`). 2 planted targets per cell. A cell where the window holds no
planted decomposition with `m` distinct summands (20000 draws) is not a cell
(row status `no_plant`, skipped, not a limit). Seeds:
`cell = 0x454B31 ⊕ n≪48 ⊕ arm≪40 ⊕ m≪32 ⊕ ℓ≪16` with arm indices
S=0, X=1, XR=2, U=3, NS=4, PC=5; target `k` is planted from
`cell ⊕ (k+1)≪8`. NS plants from S's seed and PC from X's (so each pair
shares its planted decompositions); NS's coefficients come from
`cell(NS) ⊕ (k+1)≪8`; XR's and U's window from `cell ⊕ 0x5B5≪32`. A planted
decomposition is `m` random window values with distinct coordinates (distinct
`w` in the `u`-frame), one of the 2 or 4 curve points over each chosen at
random, `R` their sum, with `x_R ∉ {0, 1}` (so `u_R` is finite and non-zero);
a redraw is counted in the row. Every series stops after the first cell in
which any system hits a limit or fails (`timeout`, `oversize`, `aborted`,
`layout_limit`); that cell is recorded and counts as missing. Series are run
in order `n = 11` before `n = 23`, `m = 2` before `m = 3`.

**5. Checks before any measurement** (a failure aborts the child; never a
measurement). Window basis independent (and `1 ∈ V` for U); every planted point
on the curve; its frame coordinate equals the window combination of the planted
bits; the points sum to `R`; the polynomial vanishes at the planted coordinates
in the word field; the Weil-restricted Boolean equations all vanish at the
planted bit-vector (a second evaluation through the repository's arithmetic);
for PC the intermediate point `P₁ + P₂` is the planted intermediate unknown. The
exact point count `|F|` equals the count by `points_with_x` on `F2mElement`
wherever that is cheap (`check`).

**6. Scores, per (arm, `n`, `m`).** A cell is *completed* when both targets are
ok (and, once the replay file exists, reproduced; below). The cell's `D`,
`field_ops` and `max_cols` are the means of its two systems, as in Amendment 1
item 4. **κ** is the least-squares slope of `log₂` cell `field_ops` on `b` over
completed cells; **σ** that of cell `D` on `b`. Break-even `m − 1`.
*Verdict on κ* (Amendment 4): **cost-favourable** if κ < `m − 1` on ≥ 3
completed cells; **cost-unfavourable** if κ ≥ `m − 1` on ≥ 3; else
**inconclusive**. *σ* is read by §1's table. Secondary and descriptive:
κ over the completed cells with `excess_bits = (unknowns) − n ≥ 0`, the regime
where a planted target has more than one expected decomposition.

**7. Trace replay (standard, Amendment 4's check).** After the series, every
completed system is rebuilt from its seed and measured once more
(`replay`). Its `D`, `max_cols`, `max_rows` and `steps` must equal the row's.
A mismatch is recorded, and that system is excluded from every score. The
replay of `field_ops` is reported (equal or not) but is not a gate.

**8. Decision rules, fixed now** (`analyze` prints each one).

- **R1 (cost verdict):** κ as in §6, per (arm, `n`, `m`).
- **R2 (S against X, same equations):** **tied** if, over ≥ 3 shared completed
  cells, `|D_S − D_X| ≤ 1` at every shared cell and `|κ_S − κ_X| < 0.25`; else
  **differs**; fewer than 3 shared cells: inconclusive.
- **R3 (S against XR, and against U):** with ≥ 3 completed cells in both
  series, **S lower** if κ_S ≤ κ_other − 0.25, **S higher** if
  κ_S ≥ κ_other + 0.25, else **tied**. Descriptive beside it: at the shared
  `ℓ`, the number of cells where the other arm's `field_ops` is ≥ 2× S's, and
  where it is ≤ ½.
- **R4 (outcome per (`n`, `m`)):** **no advance** if S is cost-unfavourable on
  ≥ 3 cells, or if S is not slope-lower than both XR and U. **Advances** only
  if S is cost-favourable and slope-lower than both XR and U. A series with
  fewer than 3 completed cells makes the outcome **inconclusive**. An
  "advances" at one `n` is a single-field reading until the other `n` agrees
  (§5's replication rule).
- **R5 (null, over shared completed cells of S and NS, ≥ 3):** **shape, not
  structure** if NS's `D` equals S's in every shared cell; **structure beyond
  shape** if NS's `D` exceeds S's by ≥ 1 in ≥ 2 shared cells; else **mixed**
  (Amendment 2's rule). Descriptive: cells where NS's `field_ops` is ≥ 2× or
  ≤ ½ S's.
- **R6 (positive control):** *PC1*, the known difference between the
  repository's two presentations at `m = 3` (direct `S₄`, Boolean degree 6, on
  a random `x`-window, against the `u`-frame, degree 4; the gap the
  `koblitz_symmetrised` module documents): over completed cells that XR and U
  share at `m = 3` (both `n` pooled), the instrument **detects** if XR's
  `field_ops` is ≥ 2× U's in ≥ 2 of them. If there are ≥ 2 shared cells and it
  does not detect, **the instrument is blind to this difference and every
  verdict above is invalid**. With fewer than 2 shared cells, PC1 is *not
  evaluable* and every verdict stands as a reading only. *PC2*, descriptive:
  PC against X on the same window and planted points (chained against
  direct), the number of shared cells where `|Δlog₂ field_ops| ≥ 1` or
  `|ΔD| ≥ 1`.
- **R7 (window structure, descriptive, not a verdict):** X against XR at equal
  `ℓ`: the number of shared cells where XR's `field_ops` is ≥ 2× X's, and where
  it is ≤ ½.

**9. Predictions, recorded now.**
P1 the identity of §1.1 holds (an exact fact; a failing `check` would have
stopped this amendment). P2 S is **tied** with X in every (`n`, `m`) where R2 is
evaluable (same equations, so equal `D` within ±1, and equal slope up to the
target distribution). P3 no arm is cost-favourable on ≥ 3 completed cells
(Amendment 4's C1 prediction, carried over). P4 PC1 detects, with XR dearer than
U in every shared cell. P5 R4 is **no advance** for S wherever evaluable. P6 R5
reads **structure beyond shape**. P7 the Gauss `x`-window is cheaper than the
random one (`c_i c_j = c_{i+j} + c_{|i−j|}` keeps products in `G_{2ℓ}`): XR
≥ 2× X's `field_ops` in at least half of the shared cells where R7 is evaluable.
P8 at `m = 3` the series of S, X, XR and NS at `n = 11` have fewer than 3 completed cells. No prediction is made for the size of any κ.

**10. Disclosure, and scope.** Before this amendment a dry run on scratch
output (not in `runs/`; one system per line, 1 target, budget 60–300 s)
measured: n = 11, `m = 2`, `ℓ = 6`: S `D = 3`, 6 steps, `field_ops` 86406; X `D = 3`, 85321; U `D = 3`, 52528 (milliseconds each).
n = 11, `m = 3`, `ℓ = 5`: S `D = 7`, 14 steps, `field_ops` 2.1·10¹⁰, 26 s; XR `D = 7`, 2.1·10¹⁰, 24 s; U `D = 5`, 6.3·10⁷, 0.27 s; PC `D = 4`, 4.3·10⁹, 57 s.
n = 11, `m = 3`, `ℓ = 6`, S: no result in the 300 s budget (9 steps, peak resident memory 11.9 GB).
n = 23, `m = 2`: S `ℓ = 10` `D = 3`, 0.1 s; S `ℓ = 13` and `ℓ = 16` no result in 60 s budgets; U `ℓ = 13` `D = 4`, 25 s.
It showed that at `m = 3` the direct degree-6 systems
(S, X, XR, NS) complete at `ℓ ≤ 5` and hit the limit at `ℓ = 6`
(`n = 11`), and, in `check`, that no arm has a planted decomposition with `m = 3`
distinct summands at `ℓ = 3` at either `n` (so `m = 3` series start at `ℓ = 4`),
and that at `m = 2` only U at `n = 23` has none at `ℓ = 3`. Expected from this:
at `m = 3` the series of S, X, XR and NS have at most 2 completed cells (`ℓ = 4,
5`) at `n = 11`, so their κ is inconclusive there by the rule of §6; the
m = 3 runs are kept because PC1 needs them. These are the only measurements
seen. The conclusions
licensed are per (`n`, `m`) series at toy sizes (`n ≤ 23`, `ℓ ≤ 13`); they are
solver-axis stage diagnostics of a Boolean F4, not costs of a pipeline, and say
nothing about `n = 131`, other `a`, other window classes or other engines.
Nothing here prices a pipeline or reports `S` in the repository's sense.
