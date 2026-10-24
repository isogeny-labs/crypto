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

### Amendment 5 (registered as “Amendment 4”), 2026-10-16: the `m = 4` series to three cells; distinct targets

*Numbering note, added 2026-10-19 when the lanes were merged.* Two
amendments were registered independently as "Amendment 4" on 2026-10-16,
each before any of its own rows. This one (G1, G2) was registered on the
experiment lane at 05:58Z, in commit `113f31659`. Amendment 4 above (C1–C4)
was registered on experiments-2 at 06:16Z, in commit `025746fd5`. Neither
lane saw the other. The study's main line already carries C1–C4 as
Amendment 4, so this one is renumbered 5 here. Below this note, its text is
the registered text, unchanged; where it says "Amendment 4" it means itself.
Its G1 and Amendment 4's C2 measure the same systems (A0 and A11, `m = 4`,
`t = 4`) under different memory limits (15 GB and 13 GB). The rows are
compared in `runs/G1-vs-C2-crosscheck.txt`.


Registered by the experiment lane before any G1 row exists. It finalises the
draft committed as `AMENDMENT-4-DRAFT.md` (`1b44b48cc`). The draft's three
open questions were settled with the study's owner, as items 1–3 below. It
adds one experiment (G1) and one construction rule (G2). It changes no
committed row, verdict, threshold or decision rule.

**Why.** F4 left both `m = 4` series inconclusive with 2 completed cells each.
Every `t = 4` system stopped at the 300 s budget, A0 at partial D = 7 and
A11 at partial D = 6. §1's threshold was derived at `m = 4`. F3 showed that a
3600 s budget completes cells 300 s cannot, and that 13 GB is the limit that
binds next (A0, `m = 3`, `t = 5`).

**G1. `m = 4` from `t = 4`, p₁, arms A0 and A11.**
- F4 budget 3600 s, hard kill 3900 s (as F3).
- **Memory limit `ulimit -v 15000000`** (item 1; raised from 13 GB). That is
  the physical memory of the host that runs G1 (15 GB, no swap, no cgroup
  limit). A system that outgrows it is killed, by the allocator's abort or
  by the kernel's OOM killer, and recorded as aborted with its reason. A
  larger limit needs a larger host and a new amendment.
- Seeds as in Amendment 2. With G2 the `t = 4` systems are the F4 systems
  that timed out, unless G2 redraws one; the rows record `target_redraws`.
- `t` rises from 4 until the stop rule, per arm. Files are
  `runs/G1-{arm}-m4.jsonl`.
- An arm's `m = 4` series is its F4 rows at `t = 2, 3` plus its G1 rows. A
  cell enters the σ fit only if both of its systems complete. F4's `t = 4`
  timeout rows stay committed, and G1 supersedes them in the fit.

**G2. Distinct targets (construction; harness flag `--distinct-targets`).**
- *Rule:* a planted target whose **system** (every equation, term for term)
  equals an earlier target's system in the same cell is redrawn from its own
  stream (item 2). Targets are built in index order, so the rule gives the
  same result in every process.
- *Why the system and not `x_R`:* N's and N11's systems take random
  coefficients from the target stream, and A11's depends on `y(−R)`. So equal
  `x_R` does not mean equal systems for every arm, but equal systems is what
  makes a cell hold one system twice.
- *Scope:* without a collision every draw is the first one. Committed rows
  are therefore reproduced unchanged: checked before registration with
  `regress` (flag off: A0 `m = 3` `t = 3`, A11 `m = 3` `t = 3`, N `m = 2`
  `t = 4`), and with the flag on for A11 `m = 3` `t = 3` and A0 `m = 3`
  `t = 4` (target 1, equal to the committed `x_R`, abscissae and D). The
  flag changes the collided cells, for example A0 `m = 2` `t = 2` target 1:
  one redraw, a new `x_R`.
- G2 applies to G1 and later runs. Committed collided cells are not re-run.

**More targets per cell: not added.** All 97 completed cells so far have
equal D across their two systems, 76 of them with distinct `x_R`. 96 have
equal `max_cols`; the exception is A1g at `m = 3`, `t = 3`.

**Decision rules.**
- *G1:* §1's σ verdict per arm at `m = 4`, over the arm's completed `m = 4`
  cells. With fewer than 3 the verdict stays inconclusive, its reason is
  recorded, and no further budget or memory change is made without a new
  amendment.
- *G1, secondary (descriptive):* Δlog₂ `max_cols`/Δb per arm, and A11's cell
  `max_cols` over A0's at each shared completed `t`.

**Predictions.**
- G1: A0's σ ≥ 0.5 at `m = 4`. This is F4's registered prediction, still
  undecided.
- No prediction for A11's σ or width (item 3). The draft's width prediction
  rested on widths of systems the budget cut off, so it is dropped.
- A0's `t = 4` systems may hit the memory limit (F3).

### Amendment 6, 2026-10-19: the sparse tower engine for A0's memory-bound cells (T1, T2), before any of their rows

Registered by the experiment lane before any T1 or T2 row exists. It adds a
second instrument for one arm, under an admission rule. It changes no
committed row, verdict, threshold, decision rule or prediction.

**Why.** Two of the incumbent's cells are missing for memory under `f4_fp`.
- A0 at `m = 3`, `t = 5` aborted at 13 GB in F3. That is the cell where A11
  shows the largest measured gap: it completes at D = 5 in about 565 s.
- A0 at `m = 4`, `t = 4` aborted at 13 GB in C2 and at 15 GB in G1
  (Amendment 5), so A0's `m = 4` σ prediction (Amendments 2 and 5) is
  undecided.
- The repository's sparse tower engine `f4_fp_tower` (pilot round 2,
  `research/pkm_tower_round2_20260925/`) works in the Kummer quotient ring
  and needs far less memory. Round 2 completed Kummer `m = 3`, `t = 5`
  systems with D = 7, but only for random targets. This study uses planted
  targets, whose certification is heavier, so the engine's reach here is
  untested.

**The instrument.** `f4_fp_tower::f4_tower`, degree bound `n + d_max + 6`,
with no `--max-nnz` and no staircase stop. A0's tower equations become the
ring's square rules (`y_{j+1} = y_j²`, `y_{t−1}² = 1`). The summation
equations, reduced into the ring, are the input. D is the engine's
`solving_degree_max`. Its cost counter (`muladds`) is a different unit from
`f4_fp`'s `field_ops`, so κ is not extended by this engine; `muladds` is
recorded descriptively. Seeds and constructions are A0's own (Amendments
1–2): the systems are identical to the `f4_fp` systems, and only the solver
differs. Every system's planted solution is checked before measuring, and
every tower equation is checked to vanish in the ring.

**T1, admission (p₁).** Every planted A0 system that completed under
`f4_fp` on p₁ is measured again with the tower engine, under the same
per-system limits as its original row (300 s for the first run and F4). That
is 26 systems (13 cells): the first run's `m = 2`, `t = 2–9` and `m = 3`, `t = 2–4`,
and F4's `m = 4`, `t = 2, 3`. The engine is **admitted** if, on every one of
them, it completes, its D equals the committed D, it does not derive 1, and
the planted solution satisfies its basis. If any compared system fails, it
is **not admitted**, T2 is not run, and the failures are reported.

**T2, the missing cells (only if admitted).** A0 at `m = 3`, `t = 5` and A0
at `m = 4`, `t = 4`, on p₁, 2 planted targets each, F4 budget 3600 s, hard
kill 3900 s, `ulimit -v 15000000`. Each is a single cell, not a rising
series. A cell whose two systems both complete joins A0's σ fit for its
`m`, labelled as an admitted-engine cell, and the fit is reported with and
without it. A system that hits a limit is recorded with its reason.

**Decision rules.**
- T1: admitted / not admitted, as above.
- T2: the σ verdicts of §1 for A0 at `m = 3` and `m = 4` over the extended
  series, scored against the standing predictions, including "A0's σ ≥ 0.5 at
  `m = 4`". At `m = 3`, `t = 5`, A0's cell D is reported beside A11's (F3,
  D = 5) as a difference, descriptively.

**Predictions.**
- T1: admitted. Round 2's cross-checks found the engines in agreement.
- T2, `m = 3`: completes with D = 7, so A0's `m = 3` series reads 5, 6, 6, 7
  (σ = 0.6, *no middle ground*).
- T2, `m = 4`: does not complete within the limits. Round 2's random-target
  system at this cell ran out of memory after 29 minutes at 14 GB.

### Amendment 7, 2026-10-19: the `m = 3`, `t = 5` cell on p₂ (T3), before any of its rows

Registered by the experiment lane before any T3 row exists. It replicates
the one cell where A11 and A0 differ most (T2 and F3: A11 D = 5, A0 D = 7)
on the second prime, as §5 requires of a reading before it is described as
more than single-prime. It changes no committed row, verdict, threshold,
decision rule or prediction.

**T3a, admission of the tower engine on p₂.** Amendment 6 admitted the
engine on p₁ only. Every completed planted A0 system on p₂ (F1: `m = 2`,
`t = 2–9` and `m = 3`, `t = 2–4`; 22 systems) is measured again on
`f4_fp_tower`, under its committed limits. The engine is **admitted on p₂**
if all 22 complete with the committed D, do not derive 1, and keep their
planted solution. If any fails, T3b is not run and the failures are
reported.

**T3b, the cell (only if admitted).** A0 on the tower engine and A11 on
`f4_fp`, `m = 3`, `t = 5`, p₂ = 2013265921, 2 planted targets each, F4
budget 3600 s, hard kill 3900 s, `ulimit -v 15000000`, seeds as in
Amendment 2 (p₂ index), no G2 rule (as F1). A system that hits a limit is
recorded with its reason. A cell joins a series only if both of its
systems complete.

**Decision rules.**
- *The gap replicates* if both cells complete and each arm's cell D on p₂
  equals its p₁ cell D (A0: 7, A11: 5). *Fails to replicate* if a completed
  cell's D differs from its p₁ value. *Inconclusive* if a cell does not
  complete.
- Per §1, σ on p₂ for `m = 3` over each arm's completed cells (F1's
  `t = 2–4` plus the T3 cell), reported with and without the cell.

**Predictions.** Both cells complete (A0 about 380 s and A11 about 570 s
per system on p₁), with D = 7 and D = 5. The gap replicates. A0's `m = 3`
σ on p₂ is 0.6000 on 4 cells (*no middle ground*), and A11's is 0.3000 on 4
(*weak*). The basis is that all 44 systems completed on both primes so far
have equal D (Amendment 2 report), not a new argument.

### Amendment 8, 2026-10-20: three tests of what is left (T4, K1, X1), before any of their rows

Registered by the experiment lane before any T4, K1 or X1 row exists. It
changes no committed row, verdict, threshold, decision rule or prediction.
Instrument, limits, seeds and the completed-cell and σ rules are those of
Amendments 1–7, except as stated. A new curve-family index joins the seed at
bit 52 (generic = 0, so every earlier seed is unchanged).

**What is left.** (1) A0 at `m = 4`, `t = 4` has no completed cell: `f4_fp`
runs out of memory (13 and 15 GB), and the tower engine stops at 3600 s at
partial D = 7 (T2). A0's `m = 4` σ prediction is therefore undecided. (2)
Every instance so far is a random short-Weierstrass curve over one of two
primes, on the dense `f4_fp` path or the tower engine. Replicating a cell on
a second prime (T3) shows the prime does not matter, not that the curve or
the code path does not.

**A fact that bounds (1).** A partial D at a timeout is the highest degree F4
has learned something at so far, so a completed cell's D is at least that.
T2's two `t = 4` systems reached partial D = 7. A0's `m = 4` series is D = 6,
7 at `t = 2, 3` (F4). With a completed `t = 4` cell the least-squares slope
over `b = 2, 3, 4` is `(D₄ − 6)/2`, which is at least 0.5 for any `D₄ ≥ 7`.
So *if* A0's `m = 4`, `t = 4` cell completes, its σ is ≥ 0.5 (*no middle
ground*), whatever D₄ is. This is a consequence of the bound, not a
measurement: no verdict is read from it until a cell completes.

**T4, a staircase stop on the tower engine, for A0 at `m = 4`.** The tower
engine's `stopping_below(k)` ends a run once the leading monomials leave at
most `k` standard monomials. A planted system at `m = 4` has at least 24
solutions (every ordering of the decomposition), so the certification that
follows is the slow part (pilot round 2, G2b). Its D is then **stop-bounded**:
the highest degree at which F4 learned something before the stop. It is not
the certified D of the other rows, and every T4 row says so.
- *T4a, admission.* Threshold `k = 64`. Every completed planted A0 system at
  `m = 4` on p₁ (F4, `t = 2, 3`; 4 systems) is measured again with the stop.
  The stop is **admitted** if all 4 complete, with the committed D, without
  deriving 1, and with the planted solution kept. The completed `m = 3`
  (6) and `m = 2` (16) systems are measured too, and reported, not gating.
  If the stop is not admitted, T4b is not run.
- *T4b, the cell (only if admitted).* A0, `m = 4`, `t = 4`, p₁, tower engine,
  `k = 64`, 2 planted targets, budget 3600 s, hard kill 3900 s,
  `ulimit -v 15000000`. A system that does not reach the stop is recorded
  with its reason. If both systems stop with at most 64 standard monomials,
  the cell joins A0's `m = 4` σ fit, labelled stop-bounded, and the fit is
  reported with and without it.

**K1, other curve families (the instance, not the prime).** A0 and A11, p₁,
`f4_fp`, with the target curve drawn from a family: `a0` (`a = 0`, `b`
uniform; `j = 0`) and `am3` (`a = −3`, `b` uniform; the P-256 shape).
`m = 2`, `t = 2–9` and `m = 3`, `t = 2–4` (the largest `t` the generic family
completed), 2 planted targets, 300 s budget, hard kill 420 s,
`ulimit -v 13000000`, as in the first run. Files `runs/K1-{arm}-{family}-m{m}`.
The reference is the first run's committed A0 and A11 rows (generic family).
The seeds differ, so towers and curves differ; D is compared by
`(arm, m, t)`.
- A family is **curve-independent** if, at every shared completed
  `(arm, m, t)`, the cell D equals the generic cell D. It is
  **curve-dependent** if any shared completed cell differs; the cells are
  listed.
- **The gap holds** in a family if, at `m = 3` and every shared completed `t`,
  A11's cell D ≤ A0's cell D − 1.
- `max_cols` against the generic cell is reported (descriptive).

**X1, a third prime on the narrow code path.** p₃ = 40961 = 5·2¹³ + 1 is below
2¹⁶, so `f4_fp` takes its 32-bit narrow path, which holds half the memory per
entry. Amendment 4 (C3) did not admit p₃ because 2 of 194 systems differed on
D, max_cols, max_rows and steps. Both were A1g, `m = 3`, `t = 3` (a set with
no fixed coset shape). Every one of its 26 A0 and 30 A11 systems matched. p₃
is therefore **admitted for A0 and A11 only**, on C3's committed rows. No A1g
row is read from p₃. The cells, p₃, `f4_fp`, 2 planted targets each, budget
3600 s, hard kill 3900 s, `ulimit -v 15000000`, G2 off, files
`runs/X1-{arm}-m{m}-p3`:
- A0 at `m = 3`, `t = 5`;
- A0 at `m = 4`, `t = 4`;
- A11 at `m = 4`, `t = 4`.

(A11 at `m = 3`, `t = 6`, which C3 also listed, is not run here: it is not
needed for either open item, and its cost is large.) A cell that completes
extends the p₁ series for its arm, labelled as an admitted-p₃ reading, and
the fit is reported with and without it.
- *Cross-engine check.* A0 at `m = 3`, `t = 5` is the cell whose D = 7
  rests on the tower engine (T2, T3). If `f4_fp` completes it on p₃, its D
  is compared: **confirms** if D = 7, **contradicts** if not, and
  **inconclusive** if it does not complete.

**Decision rules.** T4a and T4b as above. K1: curve-independent or
curve-dependent per family, and the gap per family. X1: the cross-engine
check, and for `m = 4` the σ rule of §1 with and without the p₃ cell. A0's
`m = 4` series may gain a completed cell from either T4b or X1. It is read
as the bound above says, and each source is labelled.

**Predictions.**
- T4a: admitted (the pilot's G2b stopped at 24–43 standard monomials at `m = 4`,
  `t ≤ 3` and kept every planted solution). T4b: no prediction on whether
  both systems reach the stop in 3600 s. If a cell completes, its D = 7.
- K1: both families are curve-independent, and the gap holds in both. For
  `am3`, whose support equals the generic family's, `max_cols` equals the
  generic cell's at every shared cell. For `a0`, whose summation polynomial
  has fewer terms, `max_cols` is at most the generic cell's at every shared
  cell.
- X1: A0 at `m = 3`, `t = 5` completes with D = 7 (*confirms*). A0 at `m = 4`,
  `t = 4` does not complete in 15 GB and 3600 s. A11 at `m = 4`, `t = 4`
  completes with D = 6, as on p₁.

### Amendment 9, 2026-10-20: what was not yet shown (P4, Z1, M5), before any of their rows

Registered by the experiment lane before any P4, Z1 or M5 row exists. It
changes no committed row, verdict, threshold, decision rule or prediction.
Instrument, limits, seeds and the completed-cell and σ rules are those of
Amendments 1–8. The seed gains a prime index 3 (p₄) and a target-structure
bit at bit 56 (random = 0, so every earlier seed is unchanged).

**What the Amendment 8 report listed as untested:** other curve models
(Edwards, binary-field curves), structured targets, `m ≥ 5`, and primes of a
different 2-adic shape. This amendment takes the three that this harness can
measure honestly, and says why it leaves the other.

**P4, a prime of another 2-adic shape.** p₄ = 10 007 297 = 2⁸·39091 + 1, with
p₄ − 1 = 2⁸·13·31·97: 2-adic valuation 8 (against 18, 27 and 13 for p₁, p₂,
p₃), so `t ≤ 8`, and a smooth odd part. A0 and A11, generic random targets,
`f4_fp`, `m = 2` (`t = 2–8`) and `m = 3` (`t = 2–4`), 2 planted targets, 300 s
budget, hard kill 420 s, `ulimit -v 13000000`, as in the first run. Files
`runs/P4-{arm}-m{m}`. Reference: the first run's committed p₁ rows.
- *Rule:* **prime-independent** if, at every shared completed `(arm, m, t)`,
  the cell D equals the p₁ cell D; **prime-dependent** if any differs. The gap
  at `m = 3` (A11 D ≤ A0 D − 1 at every shared `t`) is reported. `max_cols`
  against p₁ is descriptive.
- *Prediction:* prime-independent, the gap holds, and `max_cols` equals p₁'s at
  every shared cell, as p₂'s did.

**Z1, structured targets.** The only way a target enters a system is through
`x_R`, so a structured target can change the system only by changing
`S₃`'s support. `x_R = 0` does: it removes every term in `x_R` (for `m = 2`,
`S₃(x₁, x₂, 0) = (x₁x₂ − a)² − 4b(x₁ + x₂)`; for `m = 3` the last chain
polynomial loses its `x_R` terms). A random curve rarely has a decomposition
into points of `V` with `x_R = 0`, so each target brings its own curve,
built around its points: for `m = 2`, `x₁ ≠ x₂` in `V` with `x₁ + x₂` a
non-zero square, `y₂ = y₁ + λ(x₂ − x₁)` with `λ² = x₁ + x₂` and random `y₁`,
and `a`, `b` from the two points, so that `x(P₁ + P₂) = λ² − x₁ − x₂ = 0`; for
`m = 3`, `a`, `b` from random `P₁`, `P₂`, then `P₃` on that curve, searched
until `x(P₁ + P₂ + P₃) = 0`. Every target is checked on its curve and by the
group law, and every system at its planted solution, before F4 runs. A0 and
A11, p₁, `f4_fp`, `m = 2` (`t = 2–9`) and `m = 3` (`t = 2–4`), 2 planted
targets, 300 s budget, hard kill 420 s, `ulimit -v 13000000`. Files
`runs/Z1-{arm}-m{m}`. Reference: the first run's generic p₁ rows.
- *Rule:* **target-independent** if the cell D equals the generic cell D at
  every shared completed cell; **target-dependent** if any differs. The gap at
  `m = 3` is reported. `max_cols` against the generic cell is descriptive.
- *Prediction:* target-independent, and the gap holds. No prediction on
  widths (Amendment 8's width prediction for `a = 0` failed on one cell).

**M5, `m = 5`.** A0 at `m = 5`, `t = 3`, the smallest cell (`m` summands need
`|V| = 2^t ≥ m`, so `t = 2` has no cell): p₁, `f4_fp`, 2 planted targets,
300 s, hard kill 420 s, `ulimit -v 13000000`. File `runs/M5-A0-m5`. A11 is not
run: its function-and-chain form is written for `m ≤ 4`. One cell gives no
σ. The reading is whether the cell completes and, if it does, its D.
- *Prediction:* the cell does not complete within 300 s and 13 GB.

**Not in this amendment.**
- *Edwards curves* need their own summation polynomial, group law and planting,
  each verified, in a new construction. This harness is built on the short
  Weierstrass `S₃` over `F_p`.
- *Binary-field curves* are a different field and solver. The repository's
  binary Semaev code (`binary_semaev*.rs`, `gf2_elim.rs`) runs on its own
  engines, so degrees from it would not be measured by this study's
  instrument.
- *Other structured targets* (an `x_R` other than 0, or `R` of small order)
  and primes with a 2-adic valuation below 8 are not run here.

### Amendment 10, 2026-10-20: Z1 at `m = 2` from `t = 3`, before any of those rows

Registered after Amendment 9's runs and before any Z1 row at `m = 2` and
`t ≥ 3` exists. It changes no rule, prediction or committed row.

**What happened.** Z1's `m = 2` series stopped at `t = 2`. All four systems
there (A0 and A11, 2 targets each) aborted in the target construction, not in
the solver: `plant_x0` stopped after 50 331 712 trials with no target. At
`t = 2`, `V` has 4 elements and so 6 pairs `{x₁, x₂}`, and the construction
needs `x₁ + x₂` to be a non-zero square. For the `V` this cell seed draws, no
pair qualifies, and the pair does not depend on the random `y₁`, so the
search cannot succeed. The 4 aborted rows are committed as recorded. Under
Amendment 1 item 5 a failing cell ends its series, so `m = 2` produced no
completed cell. `m = 3` is not affected: its 12 systems (`t = 2–4`) completed.

**The change.** Z1 at `m = 2` is run again for `t = 3` to `t = 9` (the largest
`t` the generic family completed), with every other Amendment 9 setting
unchanged. At `t = 3`, `V` has 8 elements and 28 pairs, so a `V` with no
qualifying pair is a 2⁻²⁸ event. The `t = 2` cell is recorded as *not
constructible for this `V`* and is excluded from the shared cells and the fit.
If a later cell also cannot be constructed, it is reported the same way and
ends the series.

**Rule and predictions** are Amendment 9's, for the cells that exist. The
prediction that every `t ≥ 3` cell constructs and completes is added.

### Amendment 11, 2026-10-20: twisted Edwards curves (E1) and a long tower-engine run for A0 at `m = 4` (T5), before any of their rows

Registered by the experiment lane before any E1 or T5 row exists. It changes
no committed row, verdict, threshold, decision rule or prediction. Instrument,
limits, seeds and the completed-cell and σ rules are those of Amendments 1–10,
except as stated. A curve-model index joins the seed at bit 58 (Weierstrass =
0, so every earlier seed is unchanged).

**What is left.** Amendment 9's report listed other curve models and A0's
`m = 4`, `t = 4` cell as not shown. A bigger host cannot be had here: the
account has one environment (`anthropic_cloud`, the kind this session runs
on). T5 is the substitute. Binary-field curves stay out (a different field and
solver).

**E1, A0 on twisted Edwards curves.** The curve is `a·x² + y² = 1 + d·x²y²`
(`a ≠ d`, `a, d ≠ 0`). The sum of two points is
`((x₁y₂ + y₁x₂)/(1 + d·x₁x₂y₁y₂), (y₁y₂ − a·x₁x₂)/(1 − d·x₁x₂y₁y₂))`, and
`y(−Q) = y(Q)`, so the base lives on the `y`-coordinate and the summation
polynomial is in `y`. *The polynomial was derived and checked before this
registration* (`fb_designs_slope edwards-s3`, rows in the report): the
unique polynomial of degree at most 2 in each variable that vanishes on random
triples `(y(P₁), y(P₂), y(P₁ + P₂))` has a null space of dimension 1 on 5 random
curves at p₁, 3 at p₂ and 3 at p₃, vanishes on 200 fresh triples per curve in
every order, is symmetric, and equals
`S₃ = 1 − Σ yᵢ² + (d/a)·Σᵢ<ⱼ yᵢ²yⱼ² − 2·((d − a)/a)·y₁y₂y₃ − (d/a)·y₁²y₂²y₃²`
(9 terms, total degree 6). The solver uses the closed form. For `m = 3` the
chain is `S₃(y₁, y₂, u₀)`, `S₃(u₀, y₃, y_R)` with `u₀ = y(P₁ + P₂)`.
- *System.* A0's Kummer tower on the `y`-coordinate (the same `V` as A0 for
  the cell seed's `g` and `ζ`), with this `S₃`. A target curve is drawn
  (`a`, `d` uniform) until at least `m` elements of `V` are the `y` of a point
  (`x² = (1 − y²)/(a − d·y²)` a non-zero square). A planted target is `m`
  distinct such `y`, random signs of `x`, and their sum by the Edwards law,
  checked on the curve at every step. The planted solution is checked against
  every equation before F4 runs.
- *Cells.* A0 only (the other arms are written for Weierstrass), p₁,
  `f4_fp`, `m = 2` (`t = 2–9`) and `m = 3` (`t = 2–4`), 2 planted targets,
  300 s budget, hard kill 420 s, `ulimit -v 13000000`. Files
  `runs/E1-A0-m{m}`. Reference: the first run's A0 rows on p₁.
- *Rule.* **Model-independent** if the cell D equals the Weierstrass D at
  every shared completed cell. **Model-dependent** if any differs; the cells
  and the direction are listed. §1's σ verdict is read for the Edwards series
  at each `m`. `max_cols` is descriptive.
- *Predictions, informed by the plumbing test.* The test of the construction
  on p₃ (outside E1's cells) printed D = 4, 6, 6 at `(m, t) = (2, 3), (3, 2),
  (3, 3)`, against Weierstrass's 4, 5, 6. So: **model-dependent** (at least one
  shared cell differs), and **Edwards D ≥ Weierstrass D at every shared
  cell**. An earlier guess, that Edwards D is at least the input degree 6, is
  already contradicted by the first of those three cells and is not
  registered.

**T5, A0 at `m = 4`, `t = 4`, long budget.** The tower engine (admitted for
A0 by Amendment 6) with no stop, p₁, F4 budget 21600 s (6 h), hard kill
22200 s, `ulimit -v 15000000`. T2 reached degree 7 in 3600 s at 6–8 GB, so
neither limit had been reached. Target 0 runs first; target 1 runs only if
target 0 completes. If both complete, the cell joins A0's `m = 4` series
(labelled long-budget tower-engine) and §1's σ rule is read with and without
it. The bound of Amendment 8 applies: a completed cell has D ≥ 7, so
σ ≥ 0.5. If target 0 does not complete, the reason is recorded and the cell
does not join.
- *Prediction.* Target 0 does not complete within 6 h and 15 GB, by timeout
  or memory.

**Not run:** binary-field curves; Edwards arms other than A0; an Edwards
analogue of A11's norm chain.

### Amendment 5, 2026-10-19: the m = 4 longer-budget follow-up (L1, L2), before any of their rows

Registered after Amendment 4's run (`reports/run-20261016T0616Z`) and before
any row of L1 or L2 exists. It adds two cells. It changes no existing row,
verdict, threshold or prediction. Instrument, seeds, the completed-cell rule
and the scores σ (§1) and κ (Amendment 4, C1) are unchanged. The only changes
are the budget and the memory limit stated below.

**Why.** The open question from the Amendment 2 review is whether A11's
constant-factor advantage survives at m = 4 for t ≥ 4. At m = 4, A0 has only 2
completed cells: its t = 4 systems (C2) aborted for memory at about 525 s,
well inside their 3600 s budget, under `ulimit -v 13000000`. A11's t = 4 cell
completed in about 1030 s. A longer budget alone cannot complete A0's cell, so
this amendment raises the memory limit to what the host allows. The
independent lab runner is still unavailable from this container (Amendment 4,
C2).

- **L1, A0 m = 4, t = 4, p₁.** The same two planted systems as C2 (the seeds
  are unchanged, so the systems are identical). Budget 14400 s, grace 600 s,
  `ulimit -v 14500000` (the host has 15 GB; the lane checks `MemTotal` first and
  records it). This is the one memory change.
- **L2, A11 m = 4, t = 5, p₁.** 2 planted systems, seeds by the existing rule.
  Budget 14400 s, grace 600 s, `ulimit -v 14500000`.
- One system at a time, L1 before L2, on an otherwise idle host. Each row
  records `field_ops`, `max_cols`, D, wall time and `ulimit_v_kb`. If the
  harness can read the child's peak resident memory (`getrusage`), it is
  recorded too, as descriptive only.

**Decision rules, fixed now.**
- **R1, does A11's constant factor survive at m = 4, t = 4?** This applies
  only if both L1 systems complete. *Survives* if A11's t = 4 cell mean
  `field_ops` (C2, committed) is below A0's L1 cell mean `field_ops`. *Crossed*
  otherwise. If L1 does not complete, the outcome is *A0 not measurable at
  t = 4 within 14.5 GB and 4 h*, reported with its limit and its partial
  values. No cost comparison is made from a cell that did not complete.
- **R2, A11 at m = 4, t = 5.** If both L2 systems complete, the cell joins
  A11's m = 4 series as a fourth point, and σ and κ are recomputed with their
  verdicts (§1 and Amendment 4: cost-favourable iff κ < m − 1 = 3 on ≥ 3
  cells). If not, it is recorded with its limit, and the 3-cell verdicts
  stand (σ = 1.5, κ = 14.9: cost-unfavourable).
- If L1 completes, A0's m = 4 series gains a third cell, and its σ and κ
  verdicts are computed for the first time.

**Predictions.** L1: A0 aborts for memory again (its matrices at t = 4
already exceed 102 516 columns at 300 s). L2: A11 does not complete within
14.5 GB (its width grows about 6 bits per bit at m = 4). Neither failure
would change a recorded verdict.

### Amendment 6, 2026-10-19: L2 rerun under a limit the host can honour (L2r), before any of its rows

Registered after Amendment 5's run (`reports/run-20261019T1633Z`) and before
any L2r row exists. It adds one cell and changes no existing row, verdict,
threshold or prediction. Instrument, seeds, the completed-cell rule, σ, κ and
rules R1 and R2 are unchanged. L1 and L2 stay as recorded.

**Why.** Amendment 5 registered `ulimit -v 14500000` for L2 and checked only
`MemTotal` (16 479 424 kB). The container also has a memory cgroup on the
shell's processes, with a limit of 14 007 508 kB (14 343 688 192 B). That is
below the registered limit. Both L2 systems were killed by the kernel's
memory-cgroup OOM killer at 97.3 s and 108.0 s, with peak RSS 13 974 324 kB
(`runs/L2-A11-m4-t5.dmesg.txt`). So L2's binding limit was not the one
registered, and its prediction was scored *not decided as registered*. L1 is
not affected: both of its systems hit `ulimit -v` first (malloc failure,
peak RSS 12.8 GB), and the kernel log has no OOM kill for them. This
amendment reruns L2 under an address-space limit set below the cgroup limit.
The registered limit is then the one that binds, and the cell's outcome can
be attributed to it.

**What it can and cannot decide.** L2 already passed 13.97 GB resident
without finishing, and F4's memory path is deterministic up to the small
`field_ops` jitter seen in replays. So L2r is not expected to complete under
any limit below 14.0 GB. The rerun cannot decide whether A11 at m = 4, t = 5
completes within 14.5 GB: no host available to this study gives it 14.5 GB.
It can only record the cell as not measurable within a stated and honoured
limit. The rerun's value is attribution, not a new cost point. A11's 3-cell
m = 4 verdicts (κ = 14.886, cost-unfavourable; σ = 1.5) stand unless L2r
completes.

- **Pre-flight, before the first row.** Read the memory cgroup limit of the
  lane's own shell. Take the path from the `memory:` line of
  `/proc/self/cgroup`, then read
  `/sys/fs/cgroup/memory/<path>/memory.limit_in_bytes`, or `memory.max` under
  cgroup v2. Record it, with `MemTotal` and `nproc`, in the run log and the
  report. If neither file is readable, do not run: report the blocker.
- **Limit.** `V = min(14500000, cgroup_limit_kB − 524288, MemTotal_kB − 524288)`
  kB, where 524 288 kB (0.5 GiB) is headroom for the harness parent, shell,
  kernel memory and page cache that the cgroup also charges. On the current
  host this is 14 007 508 − 524 288 = **13 483 220 kB**. If the pre-flight
  reads a different cgroup limit, use the formula and record the value.
- **L2r, A11 m = 4, t = 5, p₁.** The same two planted systems as L2 (the seeds
  are unchanged, so the systems are identical). Budget 14400 s, grace 600 s,
  `--vmem-kb V`, rows in `runs/L2r-A11-m4-t5.{jsonl,log}`:
  `run --arms A11 --m 4 --t-min 5 --t-max 5 --budget 14400 --grace 600 --vmem-kb V --file 'L2r-{arm}-m4-t5'`.
  One system at a time on an otherwise idle host, detached from the session.
  Each row records `field_ops`, `max_cols`, D, wall time, `ulimit_v_kb` and
  `peak_rss_kb` (descriptive).
- **Attribution check, after each system.** Copy the kernel log's OOM lines
  (`invoked oom-killer`, `memory: usage`, `Memory cgroup out of memory`)
  verbatim to `runs/L2r-A11-m4-t5.dmesg.txt`, or record that there are none.
  A system's limit counts as the registered one only if its row says
  `memory allocation failed under ulimit -v V` (or `timeout`) and the kernel
  log has no OOM kill for its process. Otherwise the row is recorded as
  *limit not attributable*, and the cell is reported with that status.
- **Analysis.** L2r rows form their own group (prefix `L2r-`, not merged with
  `L2-`). The Amendment 5 analysis is extended, or `analyze --amendment6`
  added, to report L2r beside L2: rows, limits, the attribution check, and
  identity with L2's systems (cell seed, target seeds, `n_vars`, `n_eqs`,
  planted x, x_R, curve, set). `analyze --amendment2`, `--amendment4` and
  `--amendment5` must still reproduce their committed outputs byte for byte.

**Decision rule, fixed now.**
- **R2 applied to L2r.** If both L2r systems complete, the cell joins A11's
  m = 4 series as a fourth point, and σ and κ are recomputed with their
  verdicts, exactly as Amendment 5's R2 states. If not, and both systems'
  limits are attributable, the outcome is *A11 not measurable at m = 4, t = 5
  within V kB and 4 h*. It is reported with the limit and the partial values
  (wall time, peak RSS), and the 3-cell verdicts stand. If a limit is not
  attributable, the outcome is *not decided*, with the reason.
- Amendment 5's L2 prediction keeps its recorded outcome (*not decided as
  registered*). It is not re-scored from L2r, whose limit is lower.

**Prediction.** L2r: both systems abort under `ulimit -v V` (malloc failure,
no OOM kill), each before its own L2 kill time (target 0 by 97.3 s, target 1
by 108.0 s). V is below the resident size each L2 system had reached at its
kill, and address space is never smaller than resident size. That would make the outcome *A11 not
measurable at m = 4, t = 5 within 13 483 220 kB and 4 h*.

**Not in this amendment.** A run that could decide the 14.5 GB question
needs a host whose cgroup limit is at least 15 GB (e.g. the independent lab
runner, still unavailable from this container). A memory abort leaves no F4
counters. A per-degree progress record would leave partial `max_cols` and
`field_ops`, but it would change the harness's output and needs its own
amendment.

### Numbering note, 2026-10-20 (merge reconciliation; changes no rule, threshold, prediction or row)

Amendments 7 and 8 below were registered on 2026-10-19 on the branch
`research/factor-base-designs-experiments-2` (PR #2298) as Amendments 5
and 6, each before any of its rows. Amendments 5 and 6 above, from
`research/factor-base-designs-m4-long`, reached `main` first. On merging
`main` into PR #2298 the later pair is renumbered 7 and 8. Their text is
unchanged except the numbers and the cross-references between them
("Amendment 5" → "Amendment 7" and "Amendment 6" → "Amendment 8" where
they mean these two). The E-K1 analysis file is renamed to
`runs/analysis-amendment8.txt`, because `runs/analysis-amendment6.txt` is L2r's.

### Amendment 7, 2026-10-19 (registered as Amendment 5 on a parallel branch): κ becomes the primary score for every later round

Registered at the close-out of the prime-field slope line, before any further
row. It changes no existing row, verdict, threshold or prediction, and no
verdict above is re-read under it.

**Rule.** Every later round of this study scores each (arm, m, prime) series
by **κ = Δlog₂ field_ops / Δb** (Amendment 4, C1), against the break-even
`m − 1`, with Amendment 4's verdict thresholds unchanged: *cost-favourable*
if κ < m − 1 on at least 3 completed cells, *cost-unfavourable* if κ ≥ m − 1
on at least 3, otherwise *inconclusive*, read at the sizes measured and not
extrapolated. **σ (the degree slope of §1) stays a secondary quantity**,
recorded and reported with its §1 labels, but it decides nothing and is never
the headline of a verdict. Any new arm's rows must carry `field_ops`, and a
series whose rows lack it is *inconclusive* for the primary score.

**Reason.** σ misread two arms, as `runs/analysis-amendment4.txt` shows.
- A1 (A1z and A1g): D is 8 in every cell, its input degree, so σ = 0.0000 and
  both read *candidate* at `m = 2` and `m = 3`. Their κ is 6.050 and 5.809 at
  `m = 2` (against 1) and 8.905 and 9.189 at `m = 3` (against 2), above A0's
  3.593 and 7.468. Their `max_cols` at `m = 3` is 39 648 (A1z) and 41 629
  (A1g) at `t = 4`, against A0's 12 314.
- A11: D is flat or nearly so (σ = 0.1786 at `m = 2`; 0.0000 at `m = 3` on
  p₂ and 0.3000 with the `t = 5` cell on p₁), yet κ is 3.995 and 8.769 against
  A0's 3.593 and 7.468, and 14.886 against 3 at `m = 4`. It starts cheaper
  and grows faster per bit.

**Scope.** Amendment 4 already scored κ; this amendment only fixes its role.
The `m = 4` break-even is 3, and A0's `m = 4` series (2 completed cells)
stays *inconclusive* until its `t = 4` cell completes (README §5, "Close-out").

### Amendment 8, 2026-10-19 (registered as Amendment 6 on a parallel branch): E-K1, a Gauss-period window in `s = x + 1/x` on Koblitz analogues, before any of its rows

Registered by the E-K1 lane (`examples/fb_koblitz_ek1.rs`) before any row of
`runs/K1-*` exists. §4 left E-K1 "designed, not run; gets its own amendment
before running"; this is that amendment. **Numbering and the score.**
Amendment 7, appended in another checkout and merged with this one by the
coordinator, makes κ = Δlog₂ field_ops/Δb the primary score for every later
round. This amendment follows that rule: κ is primary here, σ is secondary.
The text of this amendment does not depend on Amendment 7's wording; it
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

### Amendment 8, erratum, 2026-10-19 (after the rows; changes no rule, threshold, prediction or row)

§10 says that no arm has a planted decomposition with `m = 3` distinct
summands at `ℓ = 3`, at either `n`, and expects at most 2 completed cells for
S, X, XR and NS at `m = 3`. That sentence was read from the `check` listing of
the arms S, X, U, NS and PC, and XR was not in it. XR's window is a random
subspace drawn per (`n`, `m`, `ℓ`), and at `ℓ = 3` it holds a planted
decomposition at both `n`. XR at `m = 3` therefore has cells at `ℓ = 3, 4, 5`
(3 completed cells) while S, X and NS have `ℓ = 4, 5`. Prediction P8, as
registered ("S, X, XR and NS each have fewer than 3 completed cells"), is
scored **failed** on XR; the analysis prints it so. Also not stated in §3: the
random windows of XR and U are drawn independently for each `ℓ` (they are
not nested), so their `b` is not monotone in `ℓ` and can repeat
(XR, `n = 23`, `m = 3`: `|F| = 19` at `ℓ = 4` and at `ℓ = 5`). That is how §3
defines them, and it is read as such in the report. After the rows, the
analysis harness gained the printed marks for P6, P7 and P8; no rule R1–R7
changed.
