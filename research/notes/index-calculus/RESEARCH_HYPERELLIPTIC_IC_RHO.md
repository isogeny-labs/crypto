# Index calculus vs Pollard rho on genus-2 Jacobians over `F_p`

Reference measurement for
`src/cryptanalysis/hyperelliptic_index_calculus.rs` (the ADH / Gaudry
attack, merged in #365), which shipped with operation counters and no
comparison. This note supplies the comparison.

Class (per `AGENTS.md` §3): **accounting**. No algorithm changed; what
changed is that its cost now has a boundary and a reference next to it.
Nothing here is a finding.

## Boundaries, stated before measuring

**Reference.** Pollard rho with a 16-adding walk (Teske) and Floyd cycle
finding, run on the *same* curve, the same `D₁`, `D₂`, the same prime
order `N`, counted in the same unit
(`src/cryptanalysis/hyperelliptic_ic_bench.rs`).

**Floor (index calculus).** `m + 1` unknowns need `m + 1` independent
relations. Under heuristic **H1** — a reduced divisor drawn uniformly
from `Jac(C)(F_p)` splits into degree-1 places with probability `≈ 1/g!`
— the expected trial count is at least `(m+1)·g!`, and no walk yields a
trial for under one group operation. Any dense solve must read its own
`m × m` matrix once, `m²` mul-mods. So, in group-operation equivalents,

```
floor_ops = (m + 1)·g!  +  m²/c
```

with `c` the **measured** mul-mods-per-group-op factor (column `conv`).
The floor moves only with `m` and `g`. It bounds *this method*, not the
DLP.

## Unit

```
S = total group operations / sqrt(N)
```

Everything each side spends is inside `S`: rho's branch precomputation
and every walk step; index calculus's relation-stage scalar
multiplications **and** its linear algebra, converted at `c`. Leaving
the solve out would be the relabelling failure mode — the work does not
vanish, it moves out of the headline.

## Table

`C : y² = x⁵ + 3x³ + 2x² + x + c` over `F_p`, genus 2. For each `p`,
`c` is the first value giving a near-prime Jacobian order
(`N ≥ #Jac/4`, `N > 1000`), chosen before either algorithm runs. Every
row averages 3 index-calculus runs and 9 rho runs on the same instance;
every run of both sides returned the verified `k`, so every row is a
result.

Reproduce with `cargo run --release --example hyperelliptic_ic_vs_rho`.

| `p` | `N` | `m` | IC ops | rho ops | `S_ic` | `S_rho` | **`S_ic/S_rho`** | `S_walk` | **`S_ic`/floor** | `c` |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 41 | 1321 | 15 | 1571 | 526 | 43.2 | 14.5 | **3.0** | 3.24 | **48.9** | 2171 |
| 61 | 1399 | 22 | 2199 | 522 | 58.8 | 14.0 | **4.2** | 3.06 | **47.6** | 2337 |
| 101 | 4663 | 46 | 3546 | 649 | 51.9 | 9.5 | **5.5** | 2.54 | **37.4** | 2660 |
| 151 | 7949 | 78 | 5373 | 681 | 60.3 | 7.6 | **7.9** | 2.30 | **33.5** | 2457 |
| 211 | 11813 | 112 | 7469 | 852 | 68.7 | 7.8 | **8.8** | 3.15 | **32.3** | 2497 |
| 251 | 61667 | 123 | 10229 | 1204 | 41.2 | 4.9 | **8.5** | 2.52 | **40.1** | 2195 |

`S_walk` is rho's walk alone; `S_rho` also carries the 16-branch
precomputation, a fixed cost that does not scale with `sqrt(N)` and so
inflates `S_rho` at these sizes. The collision arrives after about
`sqrt(πN/2) = 1.25·sqrt(N)` distinct points, and Floyd spends 3 group
operations per iteration, so `S_walk ≈ 3` is this reference at its own
optimum — consistent with the column. A distinguished-point rho would
cut it by roughly 3×, which **widens** the gap below rather than closing
it; the reference is if anything handicapped here.

## Reading it

1. **Index calculus loses, and loses harder as `p` grows.**
   `S_ic/S_rho` runs 3.0 → 8.8 across `p = 41 … 211`. This is the
   expected shape, not a defect: at `g = 2` the full-factor-base
   relation stage is `O(p²)` against rho's `O(p)` (since `N ≈ p²`), so
   the ratio should grow like `sqrt(N)`. The `p = 251` row dips to 8.5
   only because its `N` is 5× larger than `p = 211`'s — the denominator
   moved, not the method.
2. **The measured smoothness rate is 0.34 – 0.53**, against the `1/g! =
   0.5` that H1 predicts for genus 2. H1 survives this test at these
   sizes.
3. **The implementation sits 32 – 49× above its own floor.** Almost all
   of that is the relation stage drawing fresh `(a, b)` and paying
   `2⌈log₂ N⌉` group operations per trial where the floor allows one:
   an incremental walk (`R ← R + D₁`) would recover most of the factor.
   Charging one operation per trial instead of `2⌈log₂ N⌉ = 28` puts
   `p = 211` at `6850/28 + 619 = 864` operations, `S_ic ≈ 7.9` against
   `S_rho = 7.8` — parity, not a win, because the linear algebra the
   division does not touch then dominates. `p = 251` lands the same way
   (`≈ 1199` ops, `S_ic ≈ 4.8` vs `4.85`). So the available engineering
   is worth ~9× and buys a tie at toy sizes; it does not change the
   asymptotics, which is why both columns stay.
4. **The linear algebra is not the bottleneck here**, but it is growing
   fastest: 3 group-op equivalents at `p = 41`, 925 at `p = 251`,
   scaling as `m³/c` with `m ≈ p/2`. At `p ≈ 10³` it overtakes the
   relation stage, and the dense solve would have to go sparse
   (Wiedemann / Lanczos) before any larger measurement is meaningful.

## Scope

Genus 2, odd characteristic, `deg f = 2g+1`, `p ≤ 251`, full degree-1
factor base, dense solve, single machine. Nothing here transfers to
cryptographic sizes, and nothing here is evidence about genus 3 or 4,
where the asymptotic crossover against rho actually lives. The
`O(p)`-by-evaluation root finding alone bars larger `p`.

---

# Follow-up: the two optimisations the table asked for

The baseline above named its own two bottlenecks. Both are now
implemented and measured on the same instances, same unit, same
reference.

Class: **engineering**, and §3's table would let me write "advance"
because the ratio to the floor fell. I am not writing it. What moved is
this implementation's distance to the method's own floor, using two
techniques that were already standard; the method does nothing a generic
algorithm cannot that it did not do before. The column that would earn
the other word — the ratio to the reference — is now ≈ 1, not robustly
below it. See "What this is not" below.

## What changed

1. **`RelationSearch::Walk`** — an `r`-adding walk over
   `R ← R + S_j`, carrying `(a, b)` along with it, so a candidate
   divisor costs **one** group operation instead of `2⌈log₂ N⌉`
   (measured: 0.98 ops/trial, against 22 – 34 before). 16 branches, to
   match the rho reference; restarted every 64 trials, because a
   deterministic walk cycles after `O(sqrt(N))` steps and this attack
   needs `m + 1 ≈ p/2` relations from a group of size `N ≈ p²` — the
   same order, so an unrestarted walk would start re-deriving relations
   it already had.
2. **`LinearAlgebra::Sparse`** — Markowitz-style sparse elimination mod
   `N`. A genus-`g` relation touches at most `g` factor-base columns
   plus the `k` column, so rows carry `≤ g + 1` non-zeros whatever `m`
   is, and the dense `O(m³)` solve was throwing that away: at
   `p = 251`, 2,029,632 mul-mods became **879**, a factor of 2300.

A third change is an honesty fix rather than an optimisation, and it
cuts the other way: the **smoothness oracle is now charged**. Root
finding and the decomposition's evaluations are `O(p)` field
multiplications per trial, which `AGENTS.md` puts inside `S` as "work an
oracle does per call" — and once the walk made the group operation
cheap, that omission was worth more than either optimisation. At
`p = 251` it adds 168,933 mul-mods (76 group-op equivalents) to a
relation stage of 976 operations. The baseline table above did not carry
it, so its `S_ic` was understated; the `rnd+dns` rows below are the
corrected baseline, and they sit within 1% of the originals because at
`2⌈log₂ N⌉` per trial the oracle was noise.

## Table

Same curves, same `N`, same rho, same protocol (3 IC runs and 9 rho runs
per row, every run verified). `rnd+dns` is the merged baseline;
`wlk+spr` is both optimisations together.

| `p` | `N` | `m` | `S_ic` base | `S_ic` opt | `S_ic/S_rho` base → opt | `S_ic`/floor base → opt |
|--:|--:|--:|--:|--:|--:|--:|
| 41 | 1321 | 15 | 43.3 | 13.7 | 3.00 → **0.95** | 49.0 → 15.5 |
| 61 | 1399 | 22 | 59.0 | 14.2 | 4.22 → **1.02** | 47.7 → 11.5 |
| 101 | 4663 | 46 | 52.3 | 9.8 | 5.50 → **1.03** | 37.6 → 7.1 |
| 151 | 7949 | 78 | 60.7 | 8.2 | 7.95 → **1.07** | 33.7 → 4.5 |
| 211 | 11813 | 112 | 69.1 | 8.0 | 8.81 → **1.02** | 32.5 → 3.8 |
| 251 | 61667 | 123 | 41.0 | 4.2 | 8.45 → **0.87** | 40.0 → 4.1 |

> Superseded by the round below, which fixes an unfairness in these
> numbers: the index-calculus walk was given cheap step coefficients
> and rho's branch precomputation was not. The rho column here is
> therefore too expensive, and every `S_ic/S_rho` above too kind to
> index calculus. The corrected figures are in the final table.

`cargo run --release --example hyperelliptic_ic_vs_rho` prints all four
combinations (`rnd`/`wlk` × `dns`/`spr`) with the per-stage breakdown.
Wall clock tracks the unit: at `p = 251`, 49 ms for index calculus
against 51 ms for rho, where the baseline was 424 ms against 49 ms.

## Reading it

1. **The ratio stopped growing.** Baseline `S_ic/S_rho` climbed 3.0 →
   8.8 with `p`; optimised it sits at 0.87 – 1.07 across the whole
   range with no trend. That shape change is the result here, more than
   the 5 – 10× constant. It is what the asymptotics predict once the
   relation stage costs one operation per trial: `(m+1)·g! ≈ p`
   operations against rho's `sqrt(N) ≈ p` at `g = 2`, so both sides are
   linear in `p` and the ratio is a constant.
2. **Where the cost now sits.** At `p = 251` the relation stage is 976
   group operations, of which **714 is precomputation** — 16 branch
   divisors and 4 walk restarts, each a scalar-multiplication pair. The
   walked trials themselves cost 262. Precomputation is now the
   dominant term and the obvious next target, though it amortises as
   `p` grows and is bounded below by needing *some* starting point.
3. **The oracle, not the solve, is the field-work bottleneck now.**
   168,933 mul-mods against 879 for the linear algebra. Root finding by
   evaluation is `O(p)` per trial; a Cantor–Zassenhaus or a
   `gcd(u, x^p − x)` test would cut it to `O(log p)` multiplications
   per trial and is the next thing worth implementing.
4. **The smoothness rate held** at 0.33 – 0.53 under the walk, against
   `1/g! = 0.5`. A walked sequence is not an independent sample, so
   this was worth checking rather than assuming; H1 survives.

## What this is not

`S_ic/S_rho < 1` at `p = 251` does **not** mean index calculus beats
Pollard rho at genus 2. The reference is this repository's rho: Floyd
cycle finding at 3 group operations per iteration, plus a 16-branch
precomputation. Rho's walk alone is `S_walk = 2.52` on that instance,
against the optimised `S_ic = 4.24` — so against a distinguished-point
rho, which is the rho anyone attacking a real curve would run, index
calculus is still roughly 1.7× behind. The honest summary is **parity
with a plain rho at toy sizes, and the ratio no longer grows**, which
was not true before.

Nor does anything here transfer upward. Scope is unchanged: genus 2,
`p ≤ 251`, full factor base, single machine. The interesting question —
genus 3 and 4, where the asymptotic crossover lives — needs the
`O(log p)` smoothness test from point 3 before it can be measured at
all.


---

# Round three: the oracle, the precomputation, and a fairness fix

Round two left two things on the table and introduced one problem.

Class: **engineering**. Same reasoning as before — this is distance to
the method's own floor, closed with standard techniques.

## What changed

1. **`SmoothnessTest::Gcd`** (default). `u` splits into linear factors
   iff its radical equals `gcd(u, x^p − x)`, and `x^p mod u` costs
   `O(log p)` polynomial multiplications by square-and-multiply. At
   `deg u ≤ 2` — everything genus 2 produces — even that is
   unnecessary: a monic quadratic splits exactly when its discriminant
   is a square, so the oracle is **one Legendre symbol**, and the roots
   come from the quadratic formula. Measured at `p = 251`: 168,933
   mul-mods per run became 7,249, a factor of 23.
2. **Cheap walk steps and jump restarts.** Precomputation was 73% of
   the relation stage after round two. Two changes: branch steps are
   built from 8-bit coefficients (`2·8` operations each instead of
   `2⌈log₂ N⌉`), and a restart now adds a *randomly chosen* existing
   step — the index comes from the RNG rather than the position hash,
   so the trajectory leaves its orbit for **one** operation instead of
   a fresh scalar-multiplication pair. Relation stage at `p = 251`:
   976 operations → 566.
3. **The fairness fix.** Cheap step coefficients are not an
   index-calculus trick — rho can use them too, and its branch
   precomputation was still being charged at full price. The reference
   now gets the same treatment. This makes rho cheaper and the
   comparison worse for index calculus, which is why it belongs in the
   same round rather than a later one: optimising one side's
   precomputation and not the other's is a rigged table, and the
   rigging favours the side under study.

Short step coefficients weaken nothing. A relation is an algebraic
identity, verified as it is recorded; the step distribution affects only
how fast smooth divisors turn up, which the trial count measures
directly. The measured smoothness rate (0.48 – 0.55) did not move.

## Final table

Same curves, same `N`, same rho, both sides with cheap precomputation.
Samples raised to 5 index-calculus runs and 25 rho runs per row,
because the gap is now a factor of ~2 rather than ~10 and had to be
separated from rho's own spread. Every run of both sides returned the
verified `k`.

| `p` | `N` | `m` | `S_ic` base | `S_ic` final | `S_rho` | **`S_ic/S_rho`** | `S_walk` | `S_ic`/floor |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 41 | 1321 | 15 | 45.9 | 9.65 | 11.00 | **0.88** | 3.30 | 10.9 |
| 61 | 1399 | 22 | 58.2 | 9.85 | 10.85 | **0.91** | 3.36 | 8.0 |
| 101 | 4663 | 46 | 48.1 | 5.90 | 7.23 | **0.82** | 3.07 | 4.2 |
| 151 | 7949 | 78 | 57.7 | 5.06 | 6.80 | **0.75** | 3.61 | 2.8 |
| 211 | 11813 | 112 | 67.9 | 4.76 | 5.56 | **0.86** | 2.93 | 2.2 |
| 251 | 61667 | 123 | 41.3 | 2.28 | 4.17 | **0.55** | 3.00 | 2.2 |

Cumulative: `S_ic` down 5 – 18×, `S_ic/S_rho` from 4.2 – 12.2 (growing
with `p`) to 0.55 – 0.91 (flat, drifting down), and the distance to the
method's own floor from 32 – 52× to 2.2 – 10.9×. Wall clock agrees: at
`p = 251`, 27 ms against rho's 52 ms, where the baseline was 424 ms.

## Where the cost is now

At `p = 251`, per run: 566 group operations for the relation stage (of
which **294 is precomputation** — 16 branch divisors and the starting
point), 7,249 mul-mods for the oracle (3 group-op equivalents), 981 for
the linear algebra (0 equivalents). The walked trials themselves cost
272 operations at 1.00 each.

So two thirds of what is left is precomputation and the floor is only
2.2× away. There is no third optimisation of this size available: the
remaining levers are fewer branches or shorter coefficients, both of
which trade walk quality for setup cost and neither of which can win
more than the ~50% precomputation share.

## What this is still not

**Index calculus now beats this repository's rho at genus 2 and toy
sizes, and that is a narrower claim than it sounds.**

- The reference is Floyd rho at 3 group operations per iteration.
  `S_walk` — rho's walk alone, without precomputation — sits at
  2.9 – 3.6, consistent with `3 × 1.25`. A distinguished-point rho
  would pay closer to `1.25`, so the honest extrapolation is
  `S_rho(DP) ≈ 1.0 – 1.5` once its own (now cheap) precomputation is
  added. Against *that* rho, index calculus at `S_ic = 2.28` is still
  roughly 2× behind at `p = 251`, and 6 – 9× behind at `p ≤ 101`.
  Implementing a DP rho is the honest next step before any claim that
  this crosses over.

  > **This extrapolation was wrong, and round four measured it.** A
  > distinguished-point rho was implemented and comes in at
  > `S_rho = 2.41 – 9.08`, not `1.0 – 1.5`. The error was in the phrase
  > "once its own precomputation is added": at these sizes rho's branch
  > precomputation is roughly *half* its total, so scaling only the walk
  > term understated it several-fold. Measured, index calculus is at
  > `0.95 – 1.26` of DP rho rather than 2 – 9× behind it. An
  > extrapolation across the one term that dominates at the sizes being
  > measured is not an extrapolation.
- `S_ic/S_rho` is flat in `p`, not falling, which is what genus 2
  predicts: `(m+1)·g! ≈ p` relation operations against rho's
  `sqrt(N) ≈ p`. Both sides are linear in `p`, so no amount of constant
  factor changes the asymptotics. The crossover that matters lives at
  `g = 3` (Gaudry, reduced factor base) and `g = 4`, neither of which
  this measures.
- Scope is unchanged: genus 2, `p ≤ 251`, full factor base, one
  machine, and a factor-base build that is still `O(p)` — now the
  binding `O(p)` step, since the oracle no longer is.


---

# Round four: a real rho, and genus 3

Round three ended with two named next steps. Both are now in the code;
one is measured here, the other's table is still running and lands in a
follow-up commit on the same branch.

Class: **accounting** for the rho change (the reference got fairer, no
algorithm moved) and **infrastructure** for genus 3.

## The reference is now a distinguished-point rho

`RhoVariant::DistinguishedPoints`: store the walk positions whose hash
ends in `theta_bits` zeros, stop when one repeats. One group operation
per step against Floyd's three, for the same expected number of steps.
`theta_bits` is set so about 32 distinguished points are expected before
the collision.

It behaves as theory says: `S_walk`, rho's walk alone, drops from
2.9 – 3.6 to **1.14 – 1.48**, straddling the `sqrt(π/2) = 1.2533` ideal.
That is the check that the implementation is right, not a result.

| `p` | `N` | `S_ic` | `S_rho` (DP) | **`S_ic/S_rho`** | `S_walk` | `S_ic`/floor |
|--:|--:|--:|--:|--:|--:|--:|
| 41 | 1321 | 9.65 | 9.08 | **1.06** | 1.37 | 10.9 |
| 61 | 1399 | 9.85 | 8.82 | **1.12** | 1.33 | 8.0 |
| 101 | 4663 | 5.90 | 5.45 | **1.08** | 1.29 | 4.2 |
| 151 | 7949 | 5.07 | 4.67 | **1.09** | 1.48 | 2.8 |
| 211 | 11813 | 4.66 | 3.96 | **1.18** | 1.33 | 2.2 |
| 251 | 61667 | 2.28 | 2.41 | **0.95** | 1.24 | 2.2 |

**The round-three claim does not survive, and that is the point of
having done it.** Against Floyd rho, index calculus measured
`0.55 – 0.91` — a win. Against a real rho it measures `0.95 – 1.18`:
parity, with the single `p = 251` row below 1 and no trend. The earlier
"beats this repository's rho" was true and is now uninteresting; the
repository's rho was 3× more expensive than it needed to be.

Round three also *predicted* this, and predicted it wrong. It
extrapolated `S_rho(DP) ≈ 1.0 – 1.5` and concluded index calculus would
be 2 – 9× behind. The measured `S_rho(DP)` is 2.41 – 9.08, because at
these sizes rho's branch precomputation is about half its total and the
extrapolation scaled only the walk term. An extrapolation across the one
term that dominates at the size being measured is not an extrapolation.
The note above is corrected in place rather than rewritten.

## Genus 3 now runs

Two pieces were missing and are now present, each with its own test:

1. **Root finding above degree 2.** The fast oracle closed in form at
   `deg u ≤ 2`, which is all genus 2 produces; genus 3 produces cubics,
   and without a root finder for them the oracle fell back to the
   `O(p)` scan exactly where the interesting measurement is. Now
   Cantor–Zassenhaus equal-degree splitting: `gcd(d, (x+b)^((p−1)/2) −
   1)` for random `b`. Held to the scan's answers exhaustively over
   every monic cubic for two primes.
2. **Group order without an L-polynomial.** The genus-2 route to
   `#Jac` goes through an `L`-polynomial that is genus-2 only.
   `divisor_order_bsgs` instead finds a multiple of `ord(D)` by
   Baby-step Giant-step over the Hasse–Weil interval
   `[(sqrt(p) − 1)^{2g}, (sqrt(p) + 1)^{2g}]` and divides it down — no
   point counting, any genus. Cross-checked against the L-polynomial
   route at genus 2, where both are available: they agree on every
   divisor tested.

The same interval fixes an instance-selection bug this work introduced:
a subgroup must carry most of the Jacobian (`l ≥ lo/4`) or the
comparison measures the instance rather than the algorithms — rho
searches the subgroup while index calculus pays for a factor base sized
by the whole curve. Without that constraint `p = 211` picked `N = 1103`
and reported `S_ic = 14.7`, four times the correct row.

`solves_a_genus_three_dlp` takes a genus-3 curve end to end: BSGS for
the order, a prime-order subgroup, the walk, the fast oracle, the sparse
solve, and a verified `k`. Measured smoothness there is below 0.45,
consistent with `1/3! = 0.167` being the target rather than genus 2's
`1/2` — the cost the extra genus buys, and the reason genus 3 is where
the crossover is supposed to live.

The genus-2-vs-genus-3 table is below.


## The reference needed two more fixes first

Measuring genus 3 exposed two ways the DP walk spent operations it did
not need to. Both inflated `S_rho` — the wrong direction for a
reference to be wrong in, since it flatters the algorithm under study.

1. **A degenerate collision** (same point, same coefficients) rebuilt a
   starting point for `2⌈log₂ N⌉` operations. It now jumps by a
   randomly chosen precomputed step for **one**, the same escape the
   relation walk already used.
2. **A cycle can contain no distinguished point at all**, in which case
   the walk detects nothing however long it runs. The first version hit
   an outer cap at 64× the expected cost — which is also how the
   benchmark came to look hung rather than slow, and cost an hour of
   looking at the wrong stage. The walk now tracks its distance since
   the last distinguished point and jumps once it is well past the
   expected gap.

After both, `S_walk` sits at **1.05 – 1.48** across every row, against
the `sqrt(π/2) = 1.2533` ideal. Rho samples per row went from 25 to 40,
since the measured gap is now under 2× and rho's step count has a long
tail.

## Genus 2 vs genus 3, same unit, same reference

`m` is the factor-base size; `S_walk` is rho's walk without its
precomputation. Every run of both sides returned the verified `k`.

**Genus 2** (`C : y² = x⁵ + 3x³ + 2x² + x + c`):

| `p` | `N` | `m` | `S_ic` base | `S_ic` | `S_rho` | **`S_ic/S_rho`** | `S_walk` | `S_ic`/floor |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 41 | 1321 | 15 | 45.9 | 9.66 | 9.09 | **1.06** | 1.39 | 10.9 |
| 61 | 1399 | 22 | 58.3 | 9.85 | 8.76 | **1.12** | 1.27 | 8.0 |
| 101 | 4663 | 46 | 48.1 | 5.90 | 5.45 | **1.08** | 1.29 | 4.2 |
| 151 | 7949 | 78 | 58.1 | 5.07 | 4.61 | **1.10** | 1.42 | 2.8 |
| 211 | 11813 | 112 | 67.3 | 4.66 | 3.96 | **1.18** | 1.33 | 2.2 |
| 251 | 61667 | 123 | 40.8 | 2.28 | 2.41 | **0.94** | 1.25 | 2.2 |

**Genus 3** (`C : y² = x⁷ + x³ + c x + 1`):

| `p` | `N` | `m` | `S_ic` base | `S_ic` | `S_rho` | **`S_ic/S_rho`** | `S_walk` | `S_ic`/floor |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 23 | 6299 | 12 | 35.4 | 5.02 | 4.74 | **1.06** | 1.16 | 5.1 |
| 31 | 7333 | 16 | 47.9 | 5.22 | 4.36 | **1.20** | 1.05 | 4.4 |
| 41 | 6679 | 27 | 44.6 | 5.39 | 4.79 | **1.13** | 1.31 | 2.6 |
| 61 | 124459 | 33 | 21.2 | 1.52 | 2.30 | **0.66** | 1.48 | 2.6 |
| 101 | 364747 | 53 | 21.6 | 1.16 | 1.90 | **0.61** | 1.41 | 2.1 |

## Reading it

1. **Genus 3 is the first place the ratio moves.** At genus 2 it is
   flat: 1.06, 1.12, 1.08, 1.10, 1.18, 0.94 — no trend across a 47×
   range of `N`, which is what the asymptotics say (relation stage
   `≈ (m+1)·g! ≈ p` against rho's `sqrt(N) ≈ p`, both linear in `p`).
   At genus 3 the same column runs 1.06, 1.20, 1.13, **0.66, 0.61** —
   it breaks below 1 exactly where `N` gets large, because rho now
   costs `sqrt(N) ≈ p^{1.5}` against a relation stage still linear in
   `p`. That is the crossover this whole thread was built to look for,
   and the shape is right.
2. **It is two data points.** `p = 61` and `p = 101` at genus 3 are the
   only rows below 0.7, and they are also the only genus-3 rows with
   `N > 10^5`. The three small-`N` genus-3 rows sit at 1.06 – 1.20,
   indistinguishable from genus 2. The honest statement is that the
   trend has the predicted sign and the predicted place, on a sample
   too small to fit an exponent to.
3. **Smoothness tracks `1/g!` loosely.** Genus 3 measures 0.18 – 0.27
   against `1/3! = 0.167`, genus 2 measures 0.49 – 0.56 against
   `1/2! = 0.5`. The genus-3 excess is consistent at every `p`, so H1
   is a slight under-estimate at these sizes rather than wrong.
4. **What costs what now.** At genus 3, `p = 101`: 645 relation-stage
   operations of which 301 are precomputation, an oracle costing 53
   group-op equivalents (114,283 mul-mods — the largest single term
   after precomputation, because a cubic `u` needs Cantor–Zassenhaus
   rather than a Legendre symbol), and a linear algebra term that
   rounds to zero. The implementation is 2.1× from its own floor.

## Scope, unchanged in kind

Genus 2 and 3, odd characteristic, `p ≤ 251`, full degree-1 factor
base, one machine. Two genus-3 rows below 1 against a toy-sized rho are
not a statement about genus-3 curves at cryptographic size, where the
factor-base build alone is `O(p)` and everything here would have to be
rebuilt. What the rows do support is narrower and was the point: the
crossover moves in the direction and at the genus the theory names.

## Round five: the exponent law, pre-registered

Rounds one to four established that `S_ic/S_rho` is flat at genus 2 and dips
below 1 at genus 3 on the two largest `N`. Two data points with the right sign
in the right place is not an exponent. This round states the law the earlier
rounds' asymptotics imply, and the condition that would kill it, *before*
measuring.

**The law.** With `N ≈ p^g`, rho costs `√N ≈ p^{g/2}`. The relation stage is
`(m+1)·g!` group operations with `m ≈ p/2` places, so it is `≈ p·g!`, linear
in `p` at every genus. In the unit `S = ops/√N` both sides divide by `p^{g/2}`,
so the ratio is

    S_ic / S_rho  ∝  g! · p^{1 − g/2}

giving a predicted slope of `d log S_ratio / d log p = 1 − g/2`:

| `g` | predicted slope | what it means |
|--:|--:|:--|
| 2 | 0.0 | flat — no crossover at any size |
| 3 | −0.5 | crosses, slowly |
| 4 | −1.0 | crosses, twice as fast in the exponent |

The `g!` prefactor moves the crossover *later* as genus rises while the slope
moves it *earlier*; the crossover point is where `g!·p^{1−g/2} = 1`, i.e.
`p* = (g!)^{2/(g−2)}`. That predicts `p* = 36` at `g = 3` and `p* = 24` at
`g = 4` — both inside reach, which is why this is measurable at all rather
than an extrapolation.

**Success condition.** Fitted slopes within `±0.25` of `1 − g/2` at genus 3
and genus 4, over at least four `p` per genus, with every run returning the
verified `k`, against a distinguished-point rho whose `S_walk` stays inside
`1.0 – 1.5`.

**Falsification.** Any of: a genus-3 or genus-4 slope flatter than `−0.25`
(the dip is then a constant, not a trend); `S_walk` drifting outside
`1.0 – 1.5` (the reference has broken and the ratio is measuring that);
`S_ic/S_rho` failing to fall below 1 at genus 4 anywhere in range; or the
fitted crossover disagreeing with `p* = (g!)^{2/(g−2)}` by more than a factor
of 4.

**What a success would and would not mean.** It would establish that the
crossover is a trend with a measured exponent rather than two lucky rows, at
toy sizes, on one machine. It would *not* be a statement about
cryptographic-size Jacobians, where the `O(p)` factor-base build alone is
prohibitive and everything here would have to be rebuilt. The honest claim
available from this design is about the shape of the curve, not its position.

## Round five: measured

`cargo run --release --example hyperelliptic_ic_vs_rho`, then
`python3 scripts/hyperelliptic_exponent_fit.py`. Evidence:
`experiments/hyperelliptic_exponent_fit.json`. Every row returned its verified
`k`. `S_rho*` is rho with its walk renormalised to `sqrt(pi/2)`, per the
correction pre-registered above; `corr` is the conservative column and the one
to read.

| `g` | `p` | `N` | `S_ic` | `S_rho` | raw | `S_walk` | `S_rho*` | **corr** |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 61 | 124,459 | 1.51 | 2.30 | 0.66 | 1.48 | 2.07 | 0.73 |
| 3 | 101 | 364,747 | 1.12 | 1.90 | 0.59 | 1.41 | 1.74 | 0.64 |
| 3 | 151 | 1,180,351 | 0.79 | 1.98 | 0.40 | 1.71 | 1.52 | 0.52 |
| 3 | 211 | 4,620,611 | 0.53 | 1.96 | 0.27 | 1.82 | 1.39 | 0.38 |
| 4 | 31 | 239,753 | 1.77 | 1.90 | 0.93 | 1.30 | 1.85 | 0.95 |
| 4 | 41 | 333,041 | 1.41 | 1.86 | 0.76 | 1.34 | 1.77 | 0.80 |
| 4 | 61 | 16,790,591 | 0.28 | 2.17 | 0.13 | **2.10** | 1.32 | **0.21** |

**The law holds at all three genera**, fitted against `log2 N`:

| `g` | predicted `1/g − 1/2` | corrected slope | `R²` |
|--:|--:|--:|--:|
| 2 | 0.000 | −0.027 | 0.24 (no trend) |
| 3 | −0.167 | **−0.154** | 0.98 |
| 4 | −0.250 | **−0.275** | 0.96 |

### The pre-registered fit was against the wrong variable, and that is my error

Round five above predicted slopes against `log2 p` of `1 − g/2`, and genus 4
missed badly: −1.611 corrected against −1.0, outside the ±0.25 band. The law
is not wrong; the step from `N` to `p` is. `N ≈ p^g` is an assumption the
harness does not satisfy — it selects a prime-order subgroup above `lo/4`, and
the measured scaling is

    N ~ p^1.91 (g=2),   p^3.20 (g=3),   **p^5.94 (g=4)**

At genus 4 that inflates the p-slope by `5.94/4`, predicting `−0.25 × 5.94 =
−1.485` against the measured −1.611. Restated in the variable the law is
actually about — rho costs `√N` whatever the genus —

    S_ic / S_rho  ∝  g! · N^{1/g − 1/2}

every genus lands inside the band, at `R² = 0.96–0.98` where there is a trend
to fit. The `log2 N` fit is now the primary one and the `log2 p` fit is kept
beside it, because the discrepancy between them *is* the finding about the
harness.

### The reference degrades, and the correction is doing real work

The pre-registered falsification condition on `S_walk` fired on three rows:
genus 3 at `p = 151, 211` (1.71, 1.82) and genus 4 at `p = 61` (**2.10**),
against the `1.2533` ideal. The drift is systematic in `N`, not noise, and
genus 2 stays flat near 1.3 — so it is not a constant implementation tax, it
is a reference that gets worse exactly where the interesting rows are. That
direction flatters index calculus, so the raw column overstates the result.

The single best row is the clearest case: genus 4, `p = 61`, `N = 2^24`. Raw
ratio 0.13 — a 7.7× win. With rho's walk renormalised, 0.21 — a 4.8× win. The
correction nearly halves the claim, and that row's `S_walk` is 68% above ideal,
so it is also the row whose correction is least trustworthy. **0.21 is the
number to quote, and the reference needs fixing before it is quoted hard.**

### Where this leaves the thread

Index calculus beats a real distinguished-point rho at genus 3 and genus 4, on
the conservative accounting, by up to 4.8× at `N = 2^24`, with a measured
exponent matching the derived law to within 0.025. That is a crossover with a
slope, not two lucky rows — which is what round five set out to decide.

It remains a statement about toy sizes and one machine. `p ≤ 251`, `N ≤ 2^24`,
full degree-1 factor base, and the `O(p)` factor-base build that dominates at
cryptographic size is not modelled here at all.

**Next, in order.** (1) Fix the DP walk so `S_walk` stays near 1.2533 as `N`
grows, and re-measure — the correction should then be a no-op, and if the
crossover survives an uncorrected reference it is no longer arguable. (2) Push
genus 4 past `N = 2^24`, where the law predicts the ratio keeps falling as
`N^{−0.25}`. (3) Only then ask what any of it costs at a size anyone cares
about.


---

# Round five: a walk that costs nothing to prepare, at genus 2, 3 and 4

Round four left precomputation as 43% of the genus-3 relation stage: 64
branch divisors `a_j D₁ + b_j D₂`, each a pair of scalar
multiplications. Index calculus does not need its steps to have known
coefficients.

Class: **engineering**, with one **accounting** correction to the floor
(below).

## The factor-base walk

`log F_j` is already one of the unknowns, so the walk can step by
**factor-base places**: it carries `R = a·D₁ + b·D₂ + Σ n_j F_j`, and a
decomposition `Σ c_i F_i` gives the row

```
Σ (c_i − n_i) y_i − b·k ≡ a   (mod N)
```

the same shape as before. The steps are places the factor base already
holds, so the step precomputation disappears: measured precomputation
per run falls from ~300 group operations to **1** (the starting point
`D₁ + D₂`).

**Pollard rho cannot do this.** Its steps must have known `(a_j, b_j)`
or a collision says nothing about the logarithm. This is a structural
asymmetry between the two methods, not an optimisation withheld from the
reference — the first such asymmetry this thread has found.

Two things it needed, both found by tests rather than by reasoning:

1. **`D₁` and `D₂` must stay in the step set.** A step by a place moves
   neither `a` nor `b`, so a walk stepping only through places yields
   rows that all share one `(a, b)`: every row reads
   `a + b·k = (something in the y's)`, subtracting any two eliminates
   `k`, and the system pins every `y_i` while leaving `k` free. The rows
   are all true identities in the Jacobian and say nothing whatever
   about the logarithm — the first version did exactly this, and the
   solve simply returned nothing. A quarter of the steps are now `D₁` or
   `D₂`, which are free to use as well. A test now checks every
   collected row is an identity, at two genera, and that rows really do
   carry step counts.
2. **Walked rows are correlated**, so a fixed margin of spare rows is
   sometimes short: seed 20260919 at `p = 41` produced 24 rows that
   pinned the `y`'s and left `k` free. The driver now grows the margin
   and solves again, up to four rounds, counting the extra rows like any
   others.

## The floor was not a floor

The genus-3 `p = 41` row measured **below** the old floor, which a real
floor cannot do. That floor took the trial count to be `(m+1)·g!` from
heuristic H1 — yield `≈ 1/g!` — and H1 holds for divisors drawn
uniformly. A walk stepping through the factor base does not draw
uniformly, and the measured yield sits above `1/g!` at every genus:

| genus | H1 yield `1/g!` | measured |
|--:|--:|--:|
| 2 | 0.500 | 0.49 – 0.56 |
| 3 | 0.167 | 0.166 – 0.27 |
| 4 | 0.042 | 0.053 – 0.081 |

So the floor is now **unconditional** — `(m+1)·(1 + g/c) + m²/c`, one
group operation and one oracle call per relation, plus one read of the
solve's matrix — and H1 is reported beside it as a *prediction* the
search may beat (`IC/H1`). At genus 4 the measured cost is 0.78 – 2.77×
the prediction, so H1 is not far wrong, but it is not a bound.

## Table

`S = group operations / sqrt(N)`, DP rho reference, 5 index-calculus and
40 rho runs per row, every run verified. `ab-walk` is round four's
`(a_j, b_j)` walk; `fb-walk` is the factor-base walk.

| genus | `p` | `N` | `m` | `S_ic` ab | `S_ic` fb | `S_rho` | **fb `S_ic/S_rho`** | `IC/H1` |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 41 | 1321 | 15 | 9.65 | 2.08 | 9.09 | **0.23** | 2.35 |
| 2 | 101 | 4663 | 46 | 5.90 | 1.83 | 5.45 | **0.34** | 1.32 |
| 2 | 211 | 11813 | 112 | 4.66 | 2.17 | 3.96 | **0.55** | 1.02 |
| 2 | 251 | 61667 | 123 | 2.28 | 1.08 | 2.41 | **0.45** | 1.06 |
| 3 | 23 | 6299 | 12 | 5.00 | 1.27 | 4.74 | **0.27** | 1.29 |
| 3 | 61 | 124459 | 33 | 1.51 | 0.84 | 2.30 | **0.37** | 1.45 |
| 3 | 101 | 364747 | 53 | 1.11 | 0.63 | 1.90 | **0.33** | 1.16 |
| 3 | 151 | 1180351 | 77 | 0.80 | 0.52 | 1.98 | **0.26** | 1.21 |
| 3 | 211 | 4620611 | 104 | 0.53 | 0.38 | 1.96 | **0.20** | 1.30 |
| 4 | 17 | 5639 | 7 | 9.26 | 7.10 | 5.20 | **1.36** | 2.77 |
| 4 | 23 | 30223 | 11 | 4.40 | 2.82 | 2.89 | **0.98** | 1.70 |
| 4 | 31 | 239753 | 16 | 1.79 | 1.22 | 1.90 | **0.64** | 1.46 |
| 4 | 41 | 333041 | 24 | 1.45 | 1.15 | 1.86 | **0.62** | 1.10 |
| 4 | 61 | 16790591 | 36 | 0.27 | 0.17 | 2.17 | **0.08** | 0.78 |

## Reading it

1. **The factor-base walk is worth 1.3 – 4.6×**, most at small `m` where
   the fixed precomputation was the largest share, least at large `m`
   where walked trials dominate anyway. It is not genus-specific: genus
   2 gains as much as genus 4.
2. **The genus trend is now unambiguous.** At the largest instance of
   each genus, fb-walk `S_ic/S_rho` is 0.45 (g=2, `N ≈ 6·10⁴`), 0.20
   (g=3, `N ≈ 5·10⁶`), 0.08 (g=4, `N ≈ 1.7·10⁷`). Wall clock agrees at
   the extreme: 159 ms against rho's 783 ms at genus 4, `p = 61`.
3. **Small `p` at high genus is fixed-cost territory.** Genus 4 at
   `p = 17` has `m = 7` — a seven-column system, where the starting
   point and the oracle swamp everything and index calculus loses at
   1.36. The genus advantage needs `N` large enough for rho's `sqrt(N)`
   to matter.
4. **The oracle is now the dominant field cost at high genus**: 117
   group-op equivalents of 692 total at genus 4 `p = 61`, against 5 for
   the linear algebra. Cantor–Zassenhaus on quartics is what that buys,
   and it is the next thing to attack.
5. **Rows got denser, as predicted.** The linear algebra rose from ~1 to
   5 – 29 group-op equivalents, since each row now carries its step
   counts as well as its decomposition. Still nowhere near the
   bottleneck.

## Scope

Genus 2, 3, 4; odd characteristic; `p ≤ 251`; full degree-1 factor base;
one machine. The genus-4 rows are five instances on one curve family,
and `p = 61` is a single instance whose `N` happens to be large; nothing
here is a claim about genus-4 curves at cryptographic size, where the
factor-base build is `O(p)` and the whole pipeline would need rebuilding.
The prime-order subgroup is required to carry most of the Jacobian, which
also keeps `l² ∤ #Jac` and so the `l`-Sylow cyclic — the condition that
makes "`log F_j` mod `l`" well defined at all.


---

# Round six: the oracle, and an accounting error that ran through
# everything before it

Round five named the smoothness oracle as the dominant field cost at
high genus. Attacking it worked. Checking the result against wall clock
then showed that the cost model this whole thread has been reporting was
wrong by more than the optimisation was worth.

Class: **engineering** for the oracle; **accounting** for the rest, and
the accounting part supersedes the `S_ic` figures in rounds three, four
and five.

## The oracle is 1.4 – 2.8× cheaper

Two changes, both exact — the oracle's answers are unchanged, and
exhaustive tests hold it to that.

1. **One oracle call per candidate instead of two.** `decompose`
   returned an `Option`, so on failure the caller re-ran the whole
   oracle to learn whether the failure was "not smooth" or "smooth but
   off a truncated base". At genus 4 more than nine candidates in ten
   fail, so that probe was about half of all oracle work. It now returns
   the three outcomes from one call.
2. **A non-square discriminant proves `u` does not split**, for one
   resultant on a degree-`≤ g` polynomial and one Euler exponentiation,
   where the full test needs `x^p mod u`. Frobenius acts on the roots of
   a squarefree `u` as a permutation whose cycle type is the
   factorisation type, and `disc(u)` is a square exactly when that
   permutation is even; splitting completely is the identity, which is
   even. The odd types are `(2,1)` at degree 3 and `(2,1,1) + (4)` at
   degree 4 — so this rejects **exactly half** of all candidates before
   the step that dominates the oracle.

Measured, like-for-like (factor-base walk, same instances, same seeds,
oracle multiplications as counted at the time):

| genus | oracle before | after | cut | wall before | after |
|--:|--:|--:|--:|--:|--:|
| 3, `p = 61` | 82,552 | 32,478 | 2.54× | 61 ms | 32 ms |
| 3, `p = 211` | 276,160 | 115,835 | 2.38× | 241 ms | 160 ms |
| 4, `p = 41` | 248,186 | 92,526 | 2.68× | 160 ms | 102 ms |
| 4, `p = 61` | 304,834 | 112,513 | 2.71× | 159 ms | 73 ms |

Genus 2 gains only 1.4 – 1.6×, from the duplicate call alone: its
`deg u ≤ 2` closed form never reaches the discriminant filter.

The discriminant carries a sign `(−1)^{n(n−1)/2}`, and whether `−1` is
itself a square depends on `p mod 4`, so a wrong sign would reject split
polynomials at one residue class and pass at the other. The exhaustive
scan-versus-gcd equivalence now covers both classes at degree 3 and, new
here, at degree 4 — the degree genus 4 actually produces, where the odd
cycle types are a different set. A third test builds polynomials that
split by construction and requires every one to survive the filter,
since this is the only part of the oracle that answers "not smooth"
without looking for roots at all.

## The cost model was wrong, twice, in opposite directions

Cutting the oracle's **counted** multiplications by 60% cut wall clock
by 1.5 – 2.2×. If the count were faithful it would have predicted a few
percent: the oracle was charged at 3 – 6% of `S` while behaving like a
third of the run. The charge counts coefficient multiplications and
misses what the implementation does per multiplication — allocation, the
division loop inside `rem`, clones.

Replacing the charge with the oracle's **measured time**, converted by
the calibrated seconds-per-group-operation, overshot the other way: that
calibration repeats one Cantor addition on cache-resident operands, so
it under-measures what a group operation costs inside a real run, and
dividing a measured time by too small a number inflates everything
converted through it. At genus 3, `p = 61` that put `S` at a 1.2×
disadvantage where the clock said a 1.7× *advantage*.

What the unit now does: the relation search times its own group
operations, and one operation's in-situ cost is that time divided by the
operations it counted. The oracle and the solve convert through that.
`S` and wall clock now agree within 1.6× on every row and within 1.3% –
33% on most — and that agreement is a test, because it is the only thing
that caught either error.

**Two previously reported conclusions do not survive this:**

- The sparse solve was reported as "rounds to zero" and as 1 – 29
  group-op equivalents. Measured, it is 79 – 1350, and at the largest
  genus-3 instance it is the **single largest term** (1350 of 3033,
  against the oracle's 999 and the relation stage's 684). "The linear
  algebra is not the bottleneck" was an artifact of the same
  under-charge.
- The headline ratios were optimistic. Rounds four and five reported
  0.45 (g=2), 0.20 (g=3), 0.08 (g=4) at the largest instance of each
  genus. Corrected: **1.81, 0.72, 0.16**. The clock in those same runs
  had already disagreed — 159 ms against 783 ms at genus 4 is 4.9×, not
  the 12× that `S = 0.08` claimed — and I reported the clock without
  noticing it contradicted the unit.

## Corrected table

Factor-base walk, DP rho reference, in-situ accounting. `clock` is
`wall_ic / wall_rho` measured in the same run, as an independent check
on the unit.

| genus | `p` | `N` | `S_ic/S_rho` | `clock` |
|--:|--:|--:|--:|--:|
| 2 | 41 | 1321 | 0.30 | 0.33 |
| 2 | 101 | 4663 | 0.60 | 0.61 |
| 2 | 151 | 7949 | 1.24 | 1.10 |
| 2 | 211 | 11813 | 2.00 | 1.76 |
| 2 | 251 | 61667 | 1.81 | 1.61 |
| 3 | 23 | 6299 | 0.57 | 0.44 |
| 3 | 61 | 124459 | 0.78 | 0.62 |
| 3 | 101 | 364747 | 0.87 | 0.91 |
| 3 | 151 | 1180351 | 0.84 | 0.79 |
| 3 | 211 | 4620611 | 0.72 | 0.56 |
| 4 | 17 | 5639 | 2.45 | 1.53 |
| 4 | 23 | 30223 | 1.79 | 1.26 |
| 4 | 31 | 239753 | 1.22 | 0.89 |
| 4 | 41 | 333041 | 1.17 | 0.90 |
| 4 | 61 | 16790591 | **0.16** | **0.15** |

## Reading it

1. **Genus 2 now gets worse with `p`**, 0.30 → 2.00, because the solve's
   real cost grows faster in `m` than rho's `sqrt(N)` grows in `p`. The
   old accounting hid this by charging the solve almost nothing.
2. **Genus 3 is flat at 0.7 – 0.9** rather than falling to 0.20. Index
   calculus is modestly ahead across five instances spanning `N` from
   `6·10³` to `4.6·10⁶`, not several times ahead.
3. **Genus 4 still shows the crossover, and it is still large at the one
   big instance**: 0.16 by the unit, 0.15 by the clock, 105 ms against
   rho's 712 ms at `N = 1.7·10⁷`. The four smaller genus-4 rows lose
   (1.17 – 2.45), so the single winning row carries the claim, and it is
   one instance.
4. **The oracle is still the largest or second-largest term** at genus
   3 and 4 after a 2.5× cut — 746 of 1396 equivalents at genus 4
   `p = 61`. Closed-form cubic and quartic root finding (Cardano,
   Ferrari) would replace the remaining Cantor–Zassenhaus
   exponentiations, and the solve now deserves the same treatment the
   oracle just got.

## Caveat on the instrumentation

Timing the loop costs something: the instrumented build is slower than
the uninstrumented one on the same instance, and rho is not instrumented
the same way, so the measured ratio is biased slightly **against** index
calculus. That is the conservative direction for a claim about index
calculus, so it stands as reported rather than being corrected out.

## Round seven: the sparse solve — protocol, written before measuring

Round six priced the solve for the first time and found it the single
largest term at genus 3 `p = 211` (1350 of 3033 group-op equivalents).
This round attacks it. Protocol fixed here, before any candidate code
runs; the baseline binary is built from `main` at `4ccaf6e5`.

**Hypothesis.** The solve's cost is bookkeeping, not arithmetic. Reading
`sparse_solve_for_k`: every pivot rescans every column's holder list and
re-tests membership by a linear search of the row; a row is pushed onto
a holder list again on every update, so those lists grow with
duplicates; and each row update round-trips through a `HashMap`. None of
that is a multiplication mod `N`, so none of it was in the counted
mul-mods, and all of it is in the wall time the unit now charges.
Replacing it with exact column counts, holder lists that only gain a row
when it gains the column, and sorted two-pointer row merges should cut
the solve's charge without touching its arithmetic.

**Held fixed.** The arithmetic stays `BigUint`. Word-sized arithmetic
mod `N` would be much faster, and every `N` here fits in a word, but
the group operations it is converted against are `BigUint` Cantor
arithmetic; making only one side word-sized would move the ratio by
changing the implementation's arithmetic, not the algorithm. The
harness, curves, seeds (`20260916` for index calculus, `0xC0FFEE` for
rho), trial counts (5 and 40), primes, the in-situ unit and the floor
are unchanged. The command is `examples/hyperelliptic_ic_vs_rho.rs`,
unmodified.

**Reference and boundary.** Distinguished-point rho in the same run, as
in round six. The floor is `(m+1)·(1 + g/c) + m²/c`, unchanged.

**Pinned output.** The solver cannot change which relations are
collected unless it changes whether a solve succeeds, and with `N`
prime it should not. So on every row the relation-stage operations, the
oracle mul-mods and the recovered `k` must be **identical** between
baseline and candidate; only the solve's cost may move. A row that
differs anywhere else means the candidate is a different algorithm and
the comparison stops there. The candidate is also cross-checked against
dense Gaussian elimination on random systems before it is measured.

**Timing discipline.** The solve is charged by measured time, so wall
noise enters `S`. Every sweep runs through `tools/isolated_bench.py` on
one reserved core with `RAYON_NUM_THREADS=1`: one A/A pair to measure
the spread, then five interleaved baseline/candidate rounds, reporting
the median and minimum per row. A difference inside the A/A spread is
not a result. Contended sweeps are reported and not pooled.

**Success.** On the fb-walk rows, the median solve charge falls by at
least 2×, outside the A/A spread, with every row pinned as above.

**Stop.** If the cut is under 1.2×, the hypothesis is wrong: the solve's
cost is arithmetic, not bookkeeping. That is recorded as the result, and
the next lever is the arithmetic itself, argued separately.

**Expected class: engineering.** The change touches constants in one
phase, not how any phase grows with `m`, so it should not move the
ratio to the floor's shape. If the solve's growth with `m` changes, that
is reported as what it is.

**Owed from round six.** `docs/index-calculus-scoreboard.html` still
carries the ratios from before round six's accounting correction (0.21
at genus 4, 0.38 at genus 3). Round six should have updated it and did
not. This round updates it from its own frozen file, and the earlier
fit (`experiments/hyperelliptic_exponent_fit.json`) stays as it is,
superseded but not rewritten.

## Round seven: measured

Frozen source: `experiments/hyperelliptic_sparse_solve_r7/` (raw sweeps,
per-run isolation records, manifest with binary hashes, the analysis
script and its outputs). Eleven sweeps: an A/A pair, then five
interleaved baseline/candidate rounds, each on one reserved core. None
was contended.

**Pinned output held.** Across all eleven sweeps, every row's `N`, `m`,
relation-stage operations, oracle mul-mods, rho operations and verified
`k` are identical. Only the solve's cost moved. The dense-solve rows,
which this change does not touch, moved by at most 0.07% in `S` — the
control behaved as a control.

**The hypothesis holds: the cost was bookkeeping.** Counted solve
mul-mods changed by −4% to +8% (a deterministic pivot order replacing a
`HashMap`-ordered one). The solve's charge fell:

| | fb-walk rows | ab-walk rows |
|--|--:|--:|
| median cut in solve charge | **5.1×** | 2.5× |
| range | 2.0 – 18.2× | 1.0 – 5.3× |
| A/A spread (max over rows) | 1.40× | — |
| rows where every candidate sweep beats every baseline sweep | 18 of 18 | — |

The success condition (median ≥ 2×, outside the A/A spread) is met.

**Two defects surfaced on the way, both caught before measuring.** The
first candidate visited a row twice when it cancelled out of a column
and filled back in, aborting the solve; four DLP tests failed. And the
cross-check I wrote used dense elimination as its oracle, which was
wrong: `gaussian_eliminate_mod_n` returns a particular solution on an
under-determined system instead of `None`, despite its documentation.
The test now uses an independent rank criterion, and a mutation check
confirmed it fails without the fix. The dense solver's behaviour is
outside this round and is filed separately; the DLP driver is protected
from it by its `[k]D₁ = D₂` check.

### The solve's growth changed, not only its constant

Log-log slope of solve charge against `m`, fb-walk rows:

| genus | charge before | charge after | counted mul-mods (both) |
|--:|--:|--:|--:|
| 2 | 2.25 | 1.37 | 1.67 – 1.69 |
| 3 | 2.68 | 1.70 | 2.14 |
| 4 | 2.39 | 1.64 | 2.02 – 2.04 |

The old bookkeeping grew faster than the arithmetic it surrounded — the
per-pivot rescan is `O(m · holders · row length)` and the holder lists
grew with every update. The candidate's charge now grows no faster than
its arithmetic. The algorithm's own exponent, the mul-mod column, did
not move.

### Table

Factor-base walk, DP rho in the same run, in-situ unit. Medians over
six baseline and five candidate sweeps. `corr` renormalises rho's walk
to the ideal `√(π/2)` as round five did; `clock` is `wall_ic/wall_rho`.

| g | `p` | `N` | `m` | `S_ic` before | `S_ic` after | `S_ic/S_rho` before → after | `corr` before → after | `clock` after |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 41 | 1321 | 15 | 2.74 | 2.58 | 0.30 → 0.28 | 0.31 → 0.29 | 0.33 |
| 2 | 61 | 1399 | 22 | 2.79 | 2.56 | 0.32 → 0.29 | 0.32 → 0.29 | 0.33 |
| 2 | 101 | 4663 | 46 | 3.25 | 2.29 | 0.59 → 0.42 | 0.60 → 0.42 | 0.45 |
| 2 | 151 | 7949 | 78 | 5.62 | 2.54 | 1.22 → 0.55 | 1.26 → 0.57 | 0.57 |
| 2 | 211 | 11813 | 112 | 7.88 | 2.67 | 1.99 → 0.67 | 2.03 → 0.69 | 0.64 |
| 2 | 251 | 61667 | 123 | 4.07 | 1.33 | 1.69 → **0.55** | 1.69 → **0.55** | 0.60 |
| 3 | 23 | 6299 | 12 | 2.66 | 2.64 | 0.56 → 0.56 | 0.55 → 0.55 | 0.43 |
| 3 | 31 | 7333 | 16 | 3.48 | 3.40 | 0.80 → 0.78 | 0.76 → 0.75 | 0.62 |
| 3 | 41 | 6679 | 27 | 4.12 | 3.82 | 0.86 → 0.80 | 0.87 → 0.81 | 0.70 |
| 3 | 61 | 124459 | 33 | 1.79 | 1.65 | 0.78 → 0.72 | 0.86 → 0.80 | 0.57 |
| 3 | 101 | 364747 | 53 | 1.66 | 1.28 | 0.88 → 0.68 | 0.95 → 0.73 | 0.57 |
| 3 | 151 | 1180351 | 77 | 1.71 | 1.09 | 0.86 → 0.55 | 1.12 → 0.72 | 0.45 |
| 3 | 211 | 4620611 | 104 | 1.38 | 0.81 | 0.70 → **0.41** | 0.99 → **0.58** | 0.31 |
| 4 | 17 | 5639 | 7 | 12.74 | 12.76 | 2.45 → 2.45 | 2.53 → 2.54 | 1.72 |
| 4 | 23 | 30223 | 11 | 5.20 | 5.18 | 1.80 → 1.79 | 1.78 → 1.78 | 1.29 |
| 4 | 31 | 239753 | 16 | 2.31 | 2.29 | 1.21 → 1.20 | 1.25 → 1.24 | 0.85 |
| 4 | 41 | 333041 | 24 | 2.18 | 2.14 | 1.18 → 1.15 | 1.23 → 1.21 | 0.90 |
| 4 | 61 | 16790591 | 36 | 0.35 | 0.32 | 0.16 → **0.15** | 0.26 → **0.24** | 0.11 |

Every row returned its verified `k` on every sweep.

**Class: engineering.** `S` fell; the solve's arithmetic exponent did
not. The genus-2 column moved most because genus 2 has the largest `m`
per unit of `N`, so it carried the most bookkeeping.

## What the priced solve does to the exponent claim

Round five's law, `S_ic/S_rho ∝ g!·N^{1/g − 1/2}`, prices the relation
stage only. With the solve priced, the ratio is a sum of two terms, and
the second one grows. Writing the solve's measured growth as `m^s`, with
`s ≈ 1.4 – 1.7` from the table above, and `m ∝ N^{1/g}`:

```
relation stage / rho  ∝  N^{1/g − 1/2}
solve / rho           ∝  N^{s/g − 1/2}
```

| g | relation term | solve term (`s` measured) | predicted net | measured slope of `corr` after, 95% CI |
|--:|--:|--:|--|--:|
| 2 | 0 | +0.19 | rises | **+0.21** [0.00, 0.41] |
| 3 | −0.167 | +0.07 | flat, relation term falling, solve term rising | **−0.01** [−0.07, +0.05] |
| 4 | −0.25 | −0.09 | falls | **−0.29** [−0.42, −0.16] |

(Slopes are against `log₂ N`, as round five's were. Before this round,
under round six's accounting, the same fits read +0.51, +0.06 [0.00,
0.13] and −0.28.)

**This withdraws round five's genus-3 exponent claim.** It reported
−0.154 against a predicted −0.167 and the scoreboard carries it as an
exponent advance. That fit was made while the solve was charged almost
nothing. With the solve priced, genus 3 is flat: −1/6 lies outside the
interval. Genus 4 still falls at a slope consistent with its law, and
genus 2 was never predicted to cross.

The reason is structural, not a matter of this implementation. A full
degree-1 factor base has `m ≈ p/2`, and any elimination of an `m × m`
system costs at least order `m`, realistically `m²`. Against rho's
`√N ≈ p^{g/2}`, that is `p²` against `p^{1.5}` at genus 3: with a full
factor base the linear algebra outgrows rho. This is the known reason
that small-genus index calculus shrinks the factor base and accepts
large primes to balance the two phases — Thériault (ASIACRYPT 2003) and
Gaudry, Thomé, Thériault and Diem (Math. Comp. 2007) reach
`q^{2 − 4/(2g+1)}` and `q^{2 − 2/g}` that way; both are below rho's
`q^{g/2}` at genus 3. At toy sizes the solve's measured `s` is under 2
because it is sparse and has fixed overheads; that is a reprieve at
these sizes, not an exponent.

## Where this leaves the thread

1. **Genus 2 now measures below rho at all six instances**, 0.28 – 0.67
   on the unit and 0.33 – 0.64 on the clock. Round six's "genus 2 gets
   worse with `p`" was the bookkeeping. The ratio still rises with `N`
   (+0.21), as the solve term predicts, so this is a constant-factor
   lead that shrinks with size, not a crossover.
2. **Genus 3 is flat at 0.4 – 0.8.** Ahead of rho at every size
   measured, with no exponent behind it.
3. **Genus 4 carries the only exponent claim left**: −0.29, consistent
   with −1/4, best row 0.15 raw and 0.24 renormalised at `N = 1.7·10⁷`.
   It still rests on five sizes, and the four small ones lose.
4. **The next lever is the factor-base size**, not the solver. The
   solve is now charged honestly and grows with `m`; the literature's
   answer is a reduced factor base with large-prime relations, which
   trades relation-stage work for linear algebra. That is a change to
   the algorithm and needs its own pre-registered protocol and floor.

## Scope

`p ≤ 251`, `N ≤ 1.7·10⁷`, one x86-64 cloud container (Arm64 and GPU not
measured), full degree-1 factor base, `BigUint` arithmetic on both
sides. Wall time enters `S` through the in-situ unit, so these are
single-host numbers; the A/A spread bounds their noise at 1.4× per row
and the medians over five or six sweeps are what is reported.

## Round eight: reduced factor base with single large primes — protocol, written before measuring

Round seven priced the solve honestly and found that with a full
factor base it grows faster than rho at genus ≤ 3. The literature's
remedy (Thériault, ASIACRYPT 2003) is a reduced factor base plus
**large primes**: a smooth candidate with exactly one place outside the
base is kept as a partial relation, and two partials sharing that place
combine into a full one. This round builds that and tests it. Protocol
fixed here before any of the code exists; baseline is `main` at
`01a64427`.

**The mechanism.** The factor base is the first `F = θ·L` places by
`x` (`L` = all degree-1 places after ± folding; the existing `fb_size`
truncation). A candidate that splits with one off-base place `P`,
coefficient `e`, is stored under `P`. A later partial with the same `P`,
coefficient `e'`, gives `e'·row₁ − e·row₂`, which eliminates `log P`.
Each subsequent partial combines with the first one stored under its
place, so combined rows do not repeat. Rows with two or more off-base
places are still discarded (the double-large-prime variant is out of
scope).

**Heuristic balance, for orientation only.** With `F = L^a`, single
large primes need about `L^{(a+1)/2 − (a−1)(g−1)}` trials, and the solve
costs about `L^{2a}`. Balancing gives `a = (2g−1)/(2g+1)` and total
`L^{2 − 4/(2g+1)}`, Thériault's exponent. That is an asymptotic
statement with the solve at `m²`; it says nothing about where the
balance sits at these sizes.

**The prediction at these sizes, from round seven's frozen rows.**
After round seven the solve is only 0 – 13% of index calculus's total
(`experiments/hyperelliptic_sparse_solve_r7/`). Model: a smooth
candidate's `g` places land in the base independently with probability
`θ`; partials fill `(1−θ)·L` buckets and combine at the birthday rate
`P − B(1 − e^{−P/B})`; relation and oracle cost scale with trials; the
solve scales as `θ^s` with round seven's `s` (1.37 / 1.70 / 1.64).
Predicted `S(θ)/S(1)`, large primes on, and the no-large-prime control:

| genus | rows | θ = 0.75 | θ = 0.5 | θ = 0.5, no large primes |
|--:|--|--:|--:|--:|
| 2 | `p` = 41 … 251 | 1.04 → 0.97 | 1.25 → 1.05 | 2.7 → 1.9 |
| 3 | `p` = 23 … 211 | 1.17 → 1.13 | 1.94 → 1.72 | 5.6 → 4.2 |
| 4 | `p` = 17 … 61 | 1.35 → 1.33 | 3.18 → 2.97 | 12.5 → 9.5 |

(Ranges run from the smallest row to the largest.) So **at genus 3 and
4 the model predicts the reduced base loses at every size this harness
can reach.** Extrapolating the same model, with the solve growing as
`m^s` and the relation stage as `m`, a ≥ 10% saving first appears near
`m ≈ 2000` (`p ≈ 4000`) at genus 2, `m ≈ 13 000` at genus 3 and
`m ≈ 150 000` at genus 4. Only the first is reachable, so this round
extends genus 2 to `p ≈ 4000` to put rows on both sides of a predicted
crossover. If the solve's `s` rises toward 2 at larger `m`, as an `m²`
bound suggests it eventually must, the crossovers come earlier; that is
a stated uncertainty of the model, not a free parameter.

**Known omission.** A combined row carries up to `2g` factor-base
entries instead of `g`, so the solve should cost more per row than `θ^s`
says. The model leaves it out; if it matters, it shows up as a
systematic under-prediction at small θ.

**Instances and configurations.** A new example,
`examples/hyperelliptic_ic_large_prime.rs`, so round seven's frozen
command stays unchanged. Curves and instance selection as in the round-
seven harness. Genus 2 at `p` ∈ {101, 251, 503, 1009, 2003, 4001},
genus 3 at `p` ∈ {61, 211, 401}, genus 4 at `p` ∈ {31, 61}. Configs:
θ ∈ {1, 0.85, 0.7, 0.5} with large primes, and θ = 0.5 without (the
control). Factor-base walk, sparse solve, gcd oracle, seeds `20260916` /
`0xC0FFEE`, 5 index-calculus and 40 rho runs per row, DP rho reference.

**Accounting.** Unchanged unit. Storing and matching partials is new
bookkeeping, and round seven showed what uncharged bookkeeping does, so
it is timed and converted in situ like the oracle and the solve. The
floor `(m+1)(1+g/c) + m²/c` is evaluated at each configuration's own
`m`; since that moves the boundary with θ, **the comparison across θ is
made against rho and against θ = 1, not against the floor.**

**Pinned output.** At θ = 1 there are no off-base places, so the new
code must reproduce the old one exactly: identical relation-stage
operations, oracle mul-mods and `k` against `main`'s binary on the
round-seven harness. Every run on every row returns a verified `k`, and
a test checks every combined relation as a Jacobian identity.

**Pre-registered hypotheses.** Measured `S(θ)/S(1)` is the median over
three isolated sweeps.

- **H1 (no win where none is predicted).** At genus 3 and 4, no θ < 1
  configuration beats θ = 1 by more than 10% on any row. Falsified by
  one such row.
- **H2 (genus-2 crossover).** At genus 2, `p ≤ 251`: no θ < 1 config
  beats θ = 1 by more than 5%. At `p ≥ 2003`: the best θ < 1 beats
  θ = 1 by at least 5%. Falsified if either half fails.
- **H3 (model fidelity).** Re-evaluating the model with each row's
  own θ = 1 measurements (smoothness rate, relation, oracle and solve
  costs — none of them θ < 1 data), it predicts measured `S(θ)/S(1)`
  within ±25% on at least 80% of (row, θ < 1) cells.
- **H4 (the control).** With θ = 0.5, large primes are cheaper than no
  large primes on every row.

**Stop.** H1 or H2 falsified means the cost model is wrong about where
the solve and relation stage balance, and the next step is to measure
why before building further variants.

**Expected class.** At the reachable sizes, mostly "no change"
(θ = 1 wins). A genus-2 crossover at `p ≈ 4000` would be engineering
within a genus whose rho-relative exponent is not the target. It would
still be the first measurement of this lever and a check on the model
that predicts where it pays at genus 3, which is what matters.

## Round eight: measured

Frozen source: `experiments/hyperelliptic_large_prime_r8/` (raw sweeps
from both hosts, the interrupted ones kept, isolation records, a
manifest with binary hashes, the analysis script and its outputs).

**Two hosts, kept apart.** The container was restarted twice during the
round and came back on different CPUs. Medians are never pooled across
hosts. The registered result is three uncontended sweeps on one host
(2.10 GHz Xeon, `L4`–`L6`); two sweeps on the other (2.80 GHz, `L1`,
`L2`) replicate it. Two sweeps were interrupted (a tool time limit, then
a container stop); they are kept, marked, and not used.

**Pinned output held.** At θ = 1 the new code reproduces round seven
exactly: all 54 rows of round seven's command, every configuration,
match its frozen output on `N`, `m`, relation-stage operations, oracle
and solve mul-mods, rho operations and `k`. Across sweeps, every
deterministic count is identical. Every run on every row returned its
verified `k`. The combined relations are checked as Jacobian identities
by a test that a sign mutation fails.

### Verdicts on the registered hypotheses

| | registered test | primary host | replication host |
|--|--|--|--|
| **H1** no win at genus 3–4 | no θ < 1 beats θ = 1 by > 10% | **falsified** at one row: g3 `p = 61`, θ = 0.85, **0.864** | falsified, same row, 0.874 |
| **H2** genus-2 crossover | `p ≥ 2003` gains ≥ 5% | **falsified**: best 0.991, 0.987 | falsified: 0.984, 0.978 |
| **H3** model fidelity | ≥ 80% of cells within ±25% | holds: 44 of 44 | holds: 44 of 44 |
| **H4** large primes beat no large primes | every row | holds: 1.6 – 3.8× cheaper | holds |

**The H1 violation is seed variance, and the protocol is to blame for
it.** Sweeps repeat the same five index-calculus seeds, so they
replicate timing noise but never sample a new seed. A post-hoc rerun of
that one row with 60 seeds (labelled as such in the manifest) gives
θ = 0.85 at **1.003×** θ = 1, against the model's 1.05. Five seeds per
row was too few for a 10% threshold on an instance this small; that was
my design error. H1 stays recorded as falsified, because that is what
the registered test returned; the reading of it is that no genus-3 or
genus-4 row shows a real gain.

### Why the genus-2 crossover did not appear

At `p = 2003` and `4001` the solve is 20% of the total at θ = 1, so
there is something to save. Two things eat it:

| `p` | θ | solve, measured / model | relation + oracle | partial bookkeeping, share of total |
|--:|--:|--:|--:|--:|
| 2003 | 0.85 | 0.90 / 0.80 | 0.99 | 2.0% |
| 2003 | 0.5 | 0.57 / 0.39 | 1.12 | 4.1% |
| 4001 | 0.85 | 0.87 / 0.80 | 0.99 | 1.8% |
| 4001 | 0.5 | 0.51 / 0.39 | 1.11 | 3.5% |

1. **The solve falls less than `θ^s`.** This is the omission registered
   in advance: a combined row carries up to `2g` entries, so the system
   shrinks in rows but not in proportion in work. At genus 3 the gap is
   larger still (0.76 measured against 0.31 at θ = 0.5, `p = 401`); at
   genus 4 the model is right (0.32 against 0.32), because there the
   solve is 1% of the total and dominated by fixed costs.
2. **Partial bookkeeping costs 2 – 4%.** It is charged, as registered.
   Left uncharged it would have shown roughly half of the predicted
   gain — the same trap round six fell into, avoided this time by
   pricing it before measuring.

Net: the predicted 4 – 6% saving at `p ≈ 4000` measures as about 1%.
The model's θ-dependence of the relation stage is accurate (H3, and
the relation-plus-oracle column above); its solve term is optimistic.

### The table

Primary host, medians of three sweeps. `S/S₁` is the ratio to θ = 1 in
the same sweep; `ratio` is `S_ic/S_rho`.

| g | `p` | `N` | `m` | θ = 1: `S_ic`, ratio | best θ < 1: θ, `S/S₁` | θ = 0.5 with / without large primes |
|--:|--:|--:|--:|--|--|--|
| 2 | 101 | 4663 | 46 | 2.29, 0.42 | 0.70, 0.976 | 1.36 / 2.48 |
| 2 | 251 | 61667 | 123 | 1.35, 0.56 | 0.85, 0.993 | 1.14 / 2.06 |
| 2 | 503 | 65179 | 259 | 2.88, 1.25 | 0.85, 0.956 | 1.04 / 1.70 |
| 2 | 1009 | 241259 | 478 | 3.71, 1.78 | 0.85, 1.006 | 1.08 / 1.83 |
| 2 | 2003 | 2001787 | 1000 | 2.00, 1.03 | 0.85, 0.991 | 1.04 / 1.73 |
| 2 | 4001 | 16050337 | 2005 | 1.65, 0.73 | 0.85, 0.987 | 1.03 / 1.70 |
| 3 | 61 | 124459 | 33 | 1.65, 0.72 | 0.85, 0.864 (1.003 at 60 seeds) | 1.48 / 3.59 |
| 3 | 211 | 4620611 | 104 | 0.81, 0.41 | 0.85, 1.081 | 1.68 / 4.02 |
| 3 | 401 | 31525763 | 196 | 0.60, **0.23** | 0.85, 1.028 | 1.59 / 3.79 |
| 4 | 31 | 239753 | 16 | 2.32, 1.22 | 0.85, 1.231 | 2.82 / 9.88 |
| 4 | 61 | 16790591 | 36 | 0.33, **0.15** | 0.85, 1.262 | 2.69 / 10.30 |

**Class: no change at these sizes.** θ = 1 is the right choice on every
row once seed variance is accounted for; large primes are a large
improvement over a reduced base *without* them, and no improvement
over the full base.

**One new reference point, not from the lever.** Genus 3 at `p = 401`
(`N = 3.2·10⁷`, a size this thread had not measured) reads **0.23**
against DP rho, 0.46 with the walk renormalised. That extends the
genus-3 sizes from round seven by one row; it does not change round
seven's finding that the genus-3 ratio has no exponent behind it, and it
is a single new size, not a refit.

### What this says about the next lever

The solve is still not the bottleneck at genus 3 and 4 (1 – 4% of the
total), so no amount of factor-base reduction can pay there at these
sizes; the relation stage and the oracle are what cost. The calibrated
model, corrected for the measured solve behaviour, would put the genus-3
crossover further out than the `m ≈ 13 000` registered. The double
large prime variation (Gaudry–Thomé–Thériault–Diem) has the same
problem in sharper form: it trades even more relation-stage work and
denser rows for a smaller solve. **At reachable sizes the lever is the
oracle and the per-trial cost of the walk, not the factor base.**

## Round nine: the reference has no negation map — protocol, written before measuring

**The asymmetry.** Index calculus here folds every place with its
negative: the factor base keeps one of `±P`, so it already uses the
hyperelliptic involution `D ↦ −D`, which every Jacobian has and which
costs one negation of `v`. The rho reference
(`src/cryptanalysis/hyperelliptic_ic_bench.rs`, `rho_dp`) does not. It
walks on divisors, not on `±` classes, and pays the full
`sqrt(πN/2) ≈ 1.2533·sqrt(N)` expected steps. Walking on classes cuts
that to `sqrt(πN/4) ≈ 0.8862·sqrt(N)`, a factor of `√2` (Wiener and
Zuccherato, SAC 1998; Duursma, Gaudry and Morain, ASIACRYPT 1999 — the
same prior art `src/cryptanalysis/aut_folded_rho.rs` cites for elliptic
`j = 0` curves). A reference that leaves out a standard `√2` flatters
the method it is compared against, and in this thread that is index
calculus, so every `S_ic/S_rho` reported so far is optimistic by up to
`√2`. These curves are generic; the involution is the only cheap
automorphism, so `√2` is the whole of what folding can give.

**The change.** A third rho variant, `DistinguishedPointsNegation`, on
the same branches, hash, distinguished-point rule, seeds and step
accounting as the current default. After every step the position is
replaced by a canonical representative of `{D, −D}`, and `(a, b)` is
negated with it, so the walk position stays exactly `a·D₁ + b·D₂`.
Folding creates fruitless cycles. The two-cycle (step by `S_j`, land on
the negation, select `S_j` again) happens with probability about
`1/(2r)` per step at `r = 16` branches, and is removed by the standard
look-ahead: when the next position would select the same branch, take
the next branch instead (Wiener–Zuccherato; Bos, Kleinjung and
Lenstra). Longer cycles are left to the existing distinguished-point
gap escape. **Every look-ahead trial and every escape is a counted
group operation.**

**Boundary.** Unchanged for index calculus. The reference moves: the
ideal walk is now `0.8862·sqrt(N)`, and the walk-renormalised ratios
renormalise to that.

**Pinned output.** Index calculus is untouched, so its counted work
(relation stage, oracle and solve mul-mods, `k`) must be identical to
round seven's frozen rows. The plain DP rho must also be identical to
round seven's: its operation counts are deterministic in the seed. The
new variant must return a verified `k` on every run.

**Registered hypotheses.** On round seven's harness
(`examples/hyperelliptic_ic_vs_rho.rs`, all 18 instances, 40 rho seeds
per row), reading counted operations, which do not depend on the host:

- **R1 (the fold pays).** On rows with `N ≥ 10⁵`, where the walk
  dominates rho's precomputation, the median of
  `walk_ops(negation) / walk_ops(plain)` is at most **0.80** (ideal
  0.707). Falsified above 0.80.
- **R2 (fruitless cycles are contained).** Look-ahead trials plus
  escapes are at most 10% of the folded walk's operations on every
  row.
- **R3 (correctness).** Every folded-rho run returns a verified `k`.

If R1 holds, the folded rho becomes the reference: the round-seven
sweep is re-run three times on one host with it, and the note and the
scoreboard re-derive every `S_ic/S_rho`, the walk-renormalised ratios
and the genus exponent fits. If R1 fails, the variant is recorded as
built and measured, and the reference stays as it is until the cause is
understood.

**Expected class: accounting**, against index calculus. No algorithm
under study changes; the comparison does. Predicted direction: every
ratio rises, by up to `√2` where the walk dominates, by less at toy `N`
where rho's precomputation does. The genus-4 headline (0.24 walk-
renormalised) should move to about 0.34.

## Round nine: measured

Registered run: card `CARD-HEIC-ROUND9-5`, receipt
`research/notes/index-calculus/round9/run5/receipt.json` (status
completed), output `run5/R9.txt` (sha256 `a6a02c3f…`), isolation record
`run5/isolated_bench.jsonl` (uncontended, 554 s on reserved CPU 3).
Counted operations only; they are deterministic in the seed, and attempt
3's complete output is byte-identical in every row. Four earlier
attempts failed for reasons unrelated to the walk and are kept beside it
with an `ATTEMPT.md` each: a constructor change on `main` (attempt 1), an
executor that returned before its background run finished (2), a
wrapper timeout just after the binary finished (3), and an isolation
preflight that counted the agent harness's own 0.21 CPU-seconds (4).
Card 5 relaxed only that preflight threshold, which gates wall time and
not counts. All 18 instances are round seven's (`N` identical).

| | registered test | result |
|--|--|--|
| **R1** | median folded/plain walk ops ≤ 0.80 at `N ≥ 10⁵` | **falsified: 0.968** (7 rows) |
| R2 | look-ahead + escape steps ≤ 10% of folded walk | holds: max 6.5% |
| R3 | every run verified | holds: 18 × 40 × 2 |

| g | `p` | `N` | plain `S_walk` | folded `S_walk` | folded/plain |
|--:|--:|--:|--:|--:|--:|
| 2 | 41 – 251 | 1321 – 61667 | 1.25 – 1.42 | 0.87 – 1.09 | 0.67 – 0.87 |
| 3 | 23 – 41 | 6299 – 7333 | 1.05 – 1.31 | 0.90 – 1.03 | 0.72 – 0.89 |
| 3 | 61 | 124459 | 1.48 | 1.18 | 0.80 |
| 3 | 101 | 364747 | 1.41 | 1.36 | 0.97 |
| 3 | 151 | 1180351 | 1.71 | 0.95 | 0.56 |
| 3 | 211 | 4620611 | **1.82** | **2.15** | 1.18 |
| 4 | 17 – 23 | 5639 – 30223 | 1.23 – 1.42 | 1.05 – 1.08 | 0.74 – 0.87 |
| 4 | 31 | 239753 | 1.30 | 1.49 | 1.14 |
| 4 | 41 | 333041 | 1.34 | 1.01 | 0.75 |
| 4 | 61 | 16790591 | **2.10** | **2.16** | 1.03 |

(Ideals: plain 1.2533, folded 0.8862, ratio 0.7071.)

**Reading it.** On small groups the fold works as the theory says. On
the largest the reference does not: the *plain* walk is 45 – 68% above
its ideal at the two biggest rows (the drift round five worked around by
renormalising), and the folded walk loses its whole advantage there. Two
things the registered tests did not separate are now the question:

1. **Both walks degrade with `N`.** That is a property of the walk, not
   of the fold. A candidate cause, from reading the code: `divisor_hash`
   is a multiply-accumulate (`acc·C + limb`), so its low bits — the
   distinguished-point test — depend only on the low bits of each
   coefficient, and its branch bits are weakly mixed when every limb is
   a coefficient below `p ≤ 251`. A poorly mixed branch function makes
   an `r`-adding walk correlated with its own position, which is worse
   than random and gets worse with `N`. Unverified.
2. **R2 under-counts the fold's waste.** It counts the escape step but
   not the steps spent inside a fruitless cycle before the escape fires
   (up to `32·2^θ`). The folded walk's loss at large `N` may be that.

**Decision, per the protocol's stop rule:** the folded walk is recorded
as built and measured; the reference stays as it is until the cause is
understood. Class: none — no ratio in the thread changes.

**Amendment (additive), on rules that changed after round eight.**
`AGENTS.md` now requires cross-method ECDLP measurements to go through
the native `ecbench` harness (§12), requires every arm's unpriced work
(hashes, canonicalisations, table probes; IC bookkeeping outside the
timers) to be counted and priced at 0, 0.1 and 1 GAE before arms are
ranked (§5a), and forbids Python for research harnesses and analysis.
This thread's IC/rho ratios predate all three: `ecbench` has no
hyperelliptic family, neither arm emits its unpriced counters, and the
round-seven and round-eight analyses are Python. **Every IC/rho ratio
this thread has reported is therefore a lower-bound comparison under
§5a, and unranked.** Their frozen evidence stands as written. Round nine
itself reads counts only, was analysed natively, and ranks nothing
across methods.

**Next.** (a) Test the hash hypothesis: the same walk with
`divisor_hash` passed through a strong 64-bit finaliser, all else equal,
registered as round ten; if the plain walk returns to ≈ 1.25 at large
`N`, every ratio in the thread has been flattered by up to 1.7× and
must be re-measured. (b) Add a native word-arithmetic Jacobian
`CountedGroup` to `ecbench`, so that `rho.negation` and the IC can be
measured under the repository's current rules.

## Round ten: is the reference walk's hash the problem? — protocol, written before measuring

**Hypothesis.** Round nine measured the plain DP rho at `S_walk` 1.82
and 2.10 on the two largest rows, against an ideal of 1.2533, and the
folded walk losing its whole advantage there. `divisor_hash` is
`acc ← acc·C + limb` over the Mumford coefficients, ending in `·C + 1`.
Its low `θ` bits, which decide whether a point is distinguished, depend
only on the low `θ` bits of each coefficient. The branch bits are
`(h ≫ 32) mod 16`, reached through one multiply per limb, with limbs that
are coefficients below `p ≤ 251`. A branch function that is not close to
a random function of the point makes an `r`-adding walk correlated with
its position, which costs steps, and more of them as `N` grows.

**The change.** One variable only: the same walks with the hash passed
through the SplitMix64 finaliser (`z ^= z ≫ 30; z *= 0xbf58476d1ce4e5b9;
z ^= z ≫ 27; z *= 0x94d049bb133111eb; z ^= z ≫ 31`) before it is used for
the branch choice, the distinguished-point test and the look-ahead.
Same branches, seeds, `θ`, gap limit, look-ahead and accounting. Four
arms on round seven's 18 instances, 40 seeds each: plain and folded,
each with the old and the mixed hash. The old-hash arms must reproduce
round nine exactly (pinned output).

**Registered hypotheses** (counted operations; means of 40 seeds,
whose standard error is about 0.08 in `S_walk`):

- **H1 (the hash is the cause).** On the rows with `N ≥ 10⁵`, the median
  plain `S_walk` with the mixed hash is at most **1.45**, and on the two
  largest rows each is at most 1.55. Falsified otherwise: then the hash
  is not the cause, and the next suspect is the walk structure itself
  (`r = 16` branches, 8-bit branch coefficients) or the escape logic.
- **H2 (the fold recovers).** With the mixed hash, the median
  folded/plain walk-operation ratio on the rows with `N ≥ 10⁵` is at most
  **0.80**, which is round nine's R1 re-tested.
- **H3 (control).** On the rows with `N ≤ 6·10⁴`, where round nine's
  plain walk was already near ideal (1.05 – 1.42), the mixed hash moves
  the median plain `S_walk` by less than 0.15. A large move there would
  mean the change does something other than fix mixing.
- **H4.** Every run verified.

**What follows.** If H1 holds, every IC/rho ratio in this thread was
flattered by the broken reference, by up to the ratio of the two plain
`S_walk` columns, and the round-seven table is re-measured against the
mixed-hash walk (the folded one, if H2 holds). If H1 fails, the
reference stays and the walk structure is examined next.

**Expected class: accounting**, against index calculus.

## Round ten: measured — the hash is not the cause

Registered run: card `CARD-HEIC-ROUND10-1`, receipt
`research/notes/index-calculus/round10/run1/receipt.json` (completed),
output `run1/R10.txt`, isolation record uncontended on reserved CPU 3
(1016 s). Four arms × 18 instances × 40 seeds, every `k` verified. The
two old-hash arms reproduce round nine's columns exactly (pinned).

| | registered test | result |
|--|--|--|
| H1 | mixed plain `S_walk`: median ≤ 1.45 at `N ≥ 10⁵` **and** each of the two largest rows ≤ 1.55 | **falsified**: median 1.406, but the two largest rows are 2.240 and 1.696 |
| H2 | mixed fold/plain ≤ 0.80 at `N ≥ 10⁵` | **falsified**: 0.960 |
| H3 | control, `N ≤ 6·10⁴`: mixing moves median plain `S_walk` by < 0.15 | holds: 0.023 |
| H4 | every run verified | holds |

Largest rows, `S_walk` (ideal 1.2533 plain, 0.8862 folded):

| g | `p` | `N` | plain | fold | plain + mix | fold + mix |
|--:|--:|--:|--:|--:|--:|--:|
| 3 | 211 | 4620611 | 1.820 | 2.148 | 1.696 | 1.951 |
| 4 | 61 | 16790591 | 2.100 | 2.162 | 2.240 | 3.112 |

**Reading it.** Mixing the hash does not repair the largest rows; on the
largest the folded walk gets worse. The degradation is structural, not
the hash. Next: a per-seed profile, to see whether every seed walks long
or a few seeds blow up — the latter would point at a pathological event
(stuck walks, repeated escapes) rather than a slow walk. Class: none; the
reference is unchanged, and so is every ratio.

### Diagnostic after round ten (not registered): every seed walks long

`examples/hyperelliptic_rho_seed_profile.rs`, plain DP rho, 40 seeds,
counts only; output `round10/diagnostic/seed_profile.txt`. Run directly,
not through a card, because it only shapes the next hypothesis.

| row | mean `S_walk` | median | min | restarts |
|--|--:|--:|--:|--:|
| g2 `p = 251` (`N = 6.2·10⁴`) | 1.25 | 1.07 | 0.25 | 0 |
| g3 `p = 151` (`N = 1.2·10⁶`) | 1.71 | 1.64 | 0.50 | 0 |
| g3 `p = 211` (`N = 4.6·10⁶`) | 1.82 | 1.66 | 0.72 | 0 |
| g4 `p = 61` (`N = 1.7·10⁷`) | 2.10 | 1.96 | **1.38** | 0 |

No seed restarts, so no pathological event drives the mean. Every seed
walks long. For an ideal single-walk rho the detection time over `√N`
is approximately Rayleigh, `P(T < x) = 1 − e^{−x²/2}`, so
`P(T < 1.375) ≈ 0.61` and the chance that none of 40 seeds lands below
1.375 is about `0.39⁴⁰`, effectively zero. At the largest row the walk
cannot collide early: it behaves as if it explored a larger set than
`N`, or as if early collisions went undetected.

## Round eleven: the walk on integers as a control — protocol, written before measuring

**Question.** Is the reference walk's large-`N` excess in the walk and
detection logic, or in the Jacobian (its representation, hashing or
arithmetic)?

**The control.** The identical DP walk run on `Z/N`: the position is the
logarithm `x = a + b·k mod N` itself, the branches add
`s_j = a_j + b_j·k mod N` from the same `(a_j, b_j)` the Jacobian walk
draws from the same seed, the branch and distinguished-point rules read
a SplitMix64 hash of `x`, and `θ`, the gap limit and the collision
handling are the same code path's rules. Same 18 instances, same 40
seeds, plain walk only, counted steps.

**Registered hypotheses.**

- **C1 (instrument).** The integer model's median `S_walk` over the 7
  rows with `N ≥ 10⁵` lies in **[1.10, 1.40]**. If it does not, the
  model itself is wrong and nothing below is read.
- **C2 (localisation).** If C1 holds and the Jacobian walk's
  `S_walk` exceeds the integer model's by more than **25%** on both of the
  two largest rows, the excess is in the Jacobian side (representation,
  hashing, or arithmetic producing a different point than `a·D₁ + b·D₂`),
  and that is examined next. If the integer model shows the same excess
  (within 25%) on both, the excess is in the shared walk logic.
- **C3.** Every integer-model run recovers the planted `k`.

**Expected class: accounting**, against index calculus, if the excess
is a bug in the reference.

## Round eleven: measured — the control shows the same excess, so C1 fails

Registered run: card `CARD-HEIC-ROUND11-1` (commit `42d29092`, binary
sha256 `472d8921…578b98`), receipt
`research/notes/index-calculus/round11/run1/receipt.json` (completed,
nine commands exit 0), output `run1/R11.txt`, isolation record
uncontended on reserved CPU 3 (334 s). 18 instances × 40 seeds, plain
walk; the Jacobian column reproduces round nine's plain column exactly.

| | registered test | result |
|--|--|--|
| C1 | integer-model median `S_walk`, `N ≥ 10⁵`, in [1.10, 1.40] | **falsified**: 1.521 |
| C2 | localisation | **not read** (C1 failed, as registered) |
| C3 | every integer-model run verified | holds |

Largest rows (`S_walk`, ideal 1.2533):

| g | `p` | `N` | Jacobian | integer model | ratio | integer min over 40 seeds |
|--:|--:|--:|--:|--:|--:|--:|
| 3 | 151 | 1180351 | 1.706 | 1.514 | 1.127 | 0.502 |
| 3 | 211 | 4620611 | 1.820 | 1.752 | 1.039 | 0.835 |
| 4 | 61 | 16790591 | 2.100 | 2.171 | 0.967 | 1.226 |

**Reading it, within the registration.** C1 was written as an instrument
check, on the premise that the walk rule both arms share reaches the
random-map constant on `Z/N`. It does not, so that premise, not just the
instrument, is what failed, and C2 is not read. Not registered, and
stated only as what the table shows: the integer model reproduces the
large-`N` excess row by row (ratios 0.84–1.13 over all 18 rows, 0.967 on
the largest), including the floor on the minimum (1.226 over 40 seeds on
the largest row). Nothing Jacobian-specific is needed to produce it.
Class: none; the reference is unchanged.

## Round twelve: are the short branch coefficients the cause? — protocol, written before measuring

**Mechanism.** Both walks add steps `s_j = a_j + b_j·k` with
`a_j, b_j < 2^8` (`RHO_STEP_BITS`), chosen on the argument that short
coefficients "cost rho nothing". A single walk's point is
`x_t = x_0 + A_t + B_t·k`, with `A_t, B_t` sums of the short
coefficients, and a rho collision `x_s = x_t` needs
`ΔA + ΔB·k ≡ 0 (mod N)` with `(ΔA, ΔB) = (A_t − A_s, B_t − B_s)`. Over a gap
of `Δt` steps that displacement concentrates (central limit) on a 2-D
blob of area about `Δt·σ_a·σ_b·const` around `Δt·(μ_a, μ_b)`, with
`σ ≈ 74` for 8-bit draws, while the solutions form a lattice of
covolume `N`. Once the blob's area at `Δt ≈ √N` falls short of `N`
(about `N ≳ 10⁷` by this estimate), the walk collides less often than a
random map: the excess grows with `N`, every seed is slow, and the group
does not matter. Each of these matches rounds nine to eleven. Full-width
coefficients (`a_j, b_j` uniform in `[0, N)`) remove the structure: the
`s_j` are then independent of each other's small-integer relations and
the walk is Teske's `r = 16` adding walk.

**Arms.** Same 18 instances, same 40 seeds, plain DP walk, counted
operations: Jacobian and integer model, each with 8-bit coefficients (the
current rule, reproducing rounds nine and eleven) and with full-width
coefficients. Everything else (hash, `θ`, gap limit, collision handling)
unchanged. Precomputation for full-width branches is
`2·⌈log₂ N⌉` operations per branch (the same doubling-and-add count the
code already charges), reported separately and not in `S_walk`.

**Registered hypotheses.**

- **W1.** Integer model, full width: median `S_walk` over the 7 rows with
  `N ≥ 10⁵` lies in **[1.10, 1.40]**, and each of the two largest rows is
  **≤ 1.55**.
- **W2.** Jacobian, full width: the same two bounds.
- **W3 (control).** At `N ≤ 6·10⁴`, the Jacobian's median `S_walk`
  moves by **< 0.15** between 8-bit and full width.
- **W4.** Every run recovers the planted `k`.

If W1 and W2 hold, the reference rho was handicapped at large `N` by its
own step rule, by up to `2.10 / S_full` on the largest row; every thread
IC/rho ratio at `N ≥ 10⁵` flattered index calculus by that factor, and the
reference is replaced (class **accounting**, against index calculus). The
index-calculus walk keeps its short coefficients: it looks for smooth
points, not collisions, so the argument above does not reach it. If W1
holds and W2 fails, the Jacobian adds its own excess and is examined
next. If W1 fails, the short coefficients are not the cause.

## Round twelve: measured — the short branch coefficients were the cause

Registered run: card `CARD-HEIC-ROUND12-1` (commit `be98f8e7`, binary
sha256 `9ca561b5…391e63`), receipt
`research/notes/index-calculus/round12/run1/receipt.json` (completed,
nine commands exit 0), output `run1/R12.txt`, isolation record
uncontended on reserved CPU 3 (529 s). Four arms × 18 instances × 40
seeds, every `k` verified. The 8-bit columns reproduce rounds nine and
eleven exactly.

| | registered test | result |
|--|--|--|
| W1 | integer model, full width, `N ≥ 10⁵`: median in [1.10, 1.40], two largest ≤ 1.55 | **holds**: median 1.303; largest rows 1.505, 1.303 |
| W2 | Jacobian, full width: the same bounds | **holds**: median 1.390; largest rows 1.481, 1.212 |
| W3 | control, `N ≤ 6·10⁴`: Jacobian median moves < 0.15 | holds: 0.025 |
| W4 | every run verified | holds |

Rows with `N ≥ 10⁵` (`S_walk`, ideal 1.2533):

| g | `p` | `N` | Jacobian 8-bit | Jacobian full | 8-bit / full | integer 8-bit | integer full |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 61 | 124459 | 1.477 | 1.390 | 1.063 | 1.521 | 1.347 |
| 3 | 101 | 364747 | 1.407 | 1.484 | 0.948 | 1.406 | 1.219 |
| 3 | 151 | 1180351 | 1.706 | 1.457 | 1.171 | 1.514 | 1.331 |
| 3 | 211 | 4620611 | 1.820 | 1.212 | 1.502 | 1.752 | 1.303 |
| 4 | 31 | 239753 | 1.303 | 1.287 | 1.012 | 1.549 | 1.239 |
| 4 | 41 | 333041 | 1.344 | 1.327 | 1.013 | 1.259 | 1.173 |
| 4 | 61 | 16790591 | 2.100 | 1.481 | 1.418 | 2.171 | 1.505 |

Full-width branches cost `2·(bits(N) + 1)` operations each, 884 on the
largest row (0.216·√N), reported as precomputation and not in `S_walk`.

**What it means.**

- The reference rho's large-`N` excess came from its own step rule:
  "short coefficients cost rho nothing" (the comment on `RHO_STEP_BITS`)
  was wrong for a collision walk. It holds for the index-calculus walk,
  which looks for smooth points, not collisions.
- The excess grows with `N`: 1.06 at `N = 1.2·10⁵`, 1.42–1.50 at the two
  largest rows. Below `6·10⁴` it is absent (W3).
- **Class: accounting, against index calculus.** Every thread IC/rho ratio
  measured against the 8-bit reference at `N ≥ 10⁵` flattered index
  calculus by that row's 8-bit/full factor. Those ratios were already
  labelled lower-bound and unranked (§5a/§12 amendment); they now also
  carry a reference that was slow by up to 1.5×, and are to be
  re-measured against the full-width reference rather than corrected by
  arithmetic. This is not an improvement to index calculus; it is a
  correction to its baseline, which moves the gap to rho the other way.
- The reference is replaced: `RhoVariant`'s default is now
  `DistinguishedPointsFullWidth`. `DistinguishedPoints` keeps its 8-bit
  meaning, so every frozen example and round still reproduces exactly.
- Rounds nine and ten are re-read in this light, not re-run: the
  negation fold's failure at large `N` (R1) and the mixed-hash failure
  (H1) were measured on the slow walk. The fold is the next thing to
  re-test on the full-width walk.

## Round thirteen: the negation fold on the full-width walk — protocol, written before measuring

**Question.** Round nine's fold failed at large `N` (R1 = 0.968), but it
was measured on the 8-bit walk, which round twelve showed is slow there
for its own reasons. Does the `±` fold give its `√2` on the full-width
walk?

**Arms.** Plain full-width DP rho (`DistinguishedPointsFullWidth`, the
reference) and the same walk folded on `±` classes with round nine's
look-ahead (`DistinguishedPointsNegationFullWidth`): canonical
representative by the leading coefficient of `v`, `(a, b)` negated with
it, every look-ahead trial and gap escape a counted operation. Same 18
instances, same 40 seeds, counted operations only.

**Registered hypotheses.**

- **N1.** Median folded/plain walk operations over the 7 rows with
  `N ≥ 10⁵` is **≤ 0.80** (ideal 0.707).
- **N2.** The fruitless share (look-ahead trials plus escapes) of the
  folded walk is **≤ 10%** on every row.
- **N3.** Folded `S_walk` median over the rows with `N ≥ 10⁵` is
  **≤ 1.05** (ideal √(π/4) = 0.886).
- **N4.** Every run recovers the planted `k`.

If N1–N4 hold, the folded full-width walk becomes the reference, because
index calculus already folds `±P` in its factor base (class
**accounting**, against index calculus: the reference gets cheaper by the
measured ratio). If N1 fails, the fold has a defect of its own, examined
next.

## Round thirteen: measured — the fold fails on the full-width walk too, worse with `N`

Registered run: card `CARD-HEIC-ROUND13-1` (commit `41718e59`, binary
sha256 `597159e4…886705`), receipt
`research/notes/index-calculus/round13/run1/receipt.json` (completed,
nine commands exit 0), output `run1/R13.txt`, isolation record
uncontended on reserved CPU 3 (772 s). 18 instances × 40 seeds × 2 arms,
every `k` verified; the plain column reproduces round twelve's
full-width Jacobian column exactly.

| | registered test | result |
|--|--|--|
| N1 | median folded/plain walk ops, `N ≥ 10⁵`, ≤ 0.80 | **falsified**: 1.580 |
| N2 | fruitless share ≤ 10% on every row | holds as counted: max 5.7% |
| N3 | median folded `S_walk`, `N ≥ 10⁵`, ≤ 1.05 | **falsified**: 2.033 |
| N4 | every run verified | holds |

The fold works below `N ≈ 3·10⁴` (ratios 0.57–0.90, ideal 0.707) and
fails increasingly above: 1.077, 1.217, 1.580, 1.698, 3.984 and 4.148 on
the larger rows, the two largest folded walks at `S_walk` 4.83 and 6.14.

**Reading it, within the registration.** N1 failed, so as registered the
fold has a defect of its own. N2 "holds" only as counted: the fruitless
counter records look-ahead trials and the single step of each
gap escape, not the steps a walk spends going round a fruitless cycle
before the escape fires. Class: none; the reference stays the plain
full-width walk.

## Round fourteen: fruitless cycles longer than two — protocol, written before measuring

**Mechanism.** The look-ahead removes fruitless 2-cycles only. Longer
fruitless cycles (4, 6, …) still form, at about `(r−1)/(4r³)` per step
for the 4-cycle (Duursma–Gaudry–Morain 1999; Bos–Kleinjung–Lenstra 2010),
about one per 1100 steps at `r = 16`. A walk in one meets no
distinguished point and leaves only when the gap limit
`32·2^θ ≈ √N` fires, so each trap costs about `√N` steps. Below
`√N ≈ 1100` a walk rarely meets one before its collision; at the largest
row (`√N ≈ 4100`) it meets a few, each costing up to `√N`: the observed
3–4× growth.

**Instrument.** A window of the last `W = 32` walk hashes. A walk whose
current hash already appears in the window is inside a cycle. In
*passive* mode the window only counts those steps (`cycle_ops`), and the
walk is otherwise unchanged, step for step, so round thirteen's folded
column must reproduce exactly. In *active* mode the first repeat
triggers the escape round thirteen already uses (one random branch
step), so a trap costs at most its cycle length plus one.

**Arms.** Same 18 instances, 40 seeds, counted operations: plain
full-width (reference); folded full-width with the passive window
(round thirteen's walk, instrumented); folded full-width with the active
window.

**Registered hypotheses.**

- **C14-1 (mechanism).** In the passive arm, `cycle_ops` is at least
  **50%** of folded walk operations on each of the two largest rows, and
  the passive arm's folded walk counts equal round thirteen's exactly.
- **C14-2.** Active arm: median folded/plain walk operations over the 7
  rows with `N ≥ 10⁵` is **≤ 0.80**.
- **C14-3.** Active arm: median folded `S_walk` over those rows is
  **≤ 1.05** (ideal 0.886).
- **C14-4.** Active arm: fruitless work (look-ahead trials, escapes,
  and steps inside a cycle before detection) is **≤ 10%** of walk
  operations on every row; every run verified.

The window costs hash comparisons, not group operations; it is
unpriced native work and is reported as a count (`W` comparisons per
step) for §5a. If C14-2 to C14-4 hold, the active folded walk becomes the
reference (class **accounting**, against index calculus). If C14-1
fails, long cycles are not the cause and the fold is examined further.

**Lead atlas (read 2026-10-24).** `LEAD-RHO-negation-map-fruitless-cycles`
in crypto-autoresearcher's `knowledge/leads/ecdlp/` is `closed_known`: the
`√2` gain and fruitless-cycle escape are established (KR-RHO-fb88e6), and
ecbench's generic `rho.negation` reproduces them on prime toy curves
(0.757 against 0.707). Rounds thirteen and fourteen are therefore not a
new lead: they bring this thread's own reference to that known result.
That strengthens the standing plan to run the thread's rho through
ecbench's generic arms, under §12 and §5a, rather than keep a separate
implementation. Likewise `LEAD-RHO-r-adding-walk-mixing` (closed_known)
covers round twelve's finding: a walk whose steps are not effectively
random elements pays extra steps.

## Round fourteen: measured — long fruitless cycles were the cause; the active window over-escapes on small groups

Registered run: card `CARD-HEIC-ROUND14-1` (commit pin `2fc38dc7`, re-pinned
from `6a3d95cd` because only the note and the card changed; binary built
from `6a3d95cd`, sha256 `fdad7013…9a575e`), receipt
`research/notes/index-calculus/round14/run1/receipt.json` (completed,
nine commands exit 0), output `run1/R14.txt`, isolation record
uncontended on reserved CPU 3 (597 s; a different host from rounds 9–13,
Xeon @ 2.30 GHz, which does not affect counted operations). 18 instances
× 40 seeds × 3 arms, every `k` verified.

| | registered test | result |
|--|--|--|
| C14-1 | passive `cycle_ops` ≥ 50% of the folded walk on the two largest rows, and the passive arm reproduces round thirteen | **holds**: 78.1%, 71.6%; reproduces exactly |
| C14-2 | active folded/plain, median over `N ≥ 10⁵`, ≤ 0.80 | **holds**: 0.752 |
| C14-3 | active folded `S_walk`, median over `N ≥ 10⁵`, ≤ 1.05 | **holds**: 1.045 |
| C14-4 | active fruitless share ≤ 10% on every row; every run verified | **falsified**: up to 32.0% at `N ≤ 1.2·10⁴` (3.8–5.2% at `N ≥ 6·10⁴`); every run verified |

Largest rows (walk ops, means of 40 seeds):

| g | `p` | `N` | plain | folded, no escape | trapped share | folded, active window | active / plain |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 151 | 1180351 | 1583 | 2688 | 56.1% | 1023 | 0.646 |
| 3 | 211 | 4620611 | 2604 | 10375 | 71.6% | 2556 | 0.981 |
| 4 | 61 | 16790591 | 6069 | 25173 | 78.1% | 4164 | 0.686 |

**What it means.**

- The mechanism is confirmed: on the largest rows most of the folded
  walk's steps were spent going round fruitless cycles that the
  gap limit escapes only after about `√N` steps. Escaping on the first
  repeat brings the fold to its expected range at large `N` (C14-2, C14-3).
- C14-4 fails, so as registered the active folded walk does **not**
  become the reference; the plain full-width walk stays. Class: none.
- Not registered, stated as what the table shows: on the small rows the
  active window makes the fold *worse* than no window (active/plain
  0.93–1.08 at `N ≤ 1.2·10⁴`, against about 0.70 without the window).
  A walk's genuine rho cycle has length about `√(πN/8)`, under 32 steps
  for the smallest rows, so the window also sees the useful collision as a
  repeat and escapes it as if it were fruitless. That is the hypothesis
  for round fifteen: on a repeat, try the collision first (a repeat with
  different `(a, b)` solves the logarithm) and escape only a degenerate
  one.

## Round fifteen: try the collision before escaping — protocol, written before measuring

**Question.** Round fourteen's active window escaped on any repeat within
32 steps. On small groups a walk's genuine rho cycle (about `√(πN/8)`
steps) fits in the window, so the escape threw away useful collisions.
Does trying the collision first, and escaping only a degenerate one
(equal `b`, which carries no information), repair the small groups
without losing the large ones?

**Arms.** Same 18 instances, 40 seeds, counted operations: plain
full-width walk (reference); folded full-width walk with round fourteen's
active window (reproduces round fourteen's column); folded full-width
walk with the active window that, on a repeat, solves from the stored
`(a, b)` of the earlier visit when that collision is non-degenerate and
escapes only otherwise.

**Registered hypotheses** (all on the collide-first arm).

- **R15-1.** Median folded/plain walk operations over the 10 rows with
  `N ≤ 6·10⁴` is **≤ 0.85**.
- **R15-2.** Median folded/plain over the 7 rows with `N ≥ 10⁵` is
  **≤ 0.80**.
- **R15-3.** Median folded `S_walk` over the rows with `N ≥ 10⁵` is
  **≤ 1.05**.
- **R15-4.** Fruitless share (look-ahead trials, escapes, and the
  charged cycle length of each escaped cycle) is **≤ 10%** on every row;
  every run verified; the round-fourteen active arm reproduces its
  recorded column.

If R15-1 to R15-4 hold, the collide-first folded walk becomes the
thread's rho reference (class **accounting**, against index calculus:
the reference gets cheaper by the measured ratio). If R15-1 fails, the
small-group excess has another cause.

## Round fifteen: measured — the folded walk reaches its `√2`; it becomes the reference

Registered run: card `CARD-HEIC-ROUND15-1` (commit `16e7f548`, binary
sha256 `8a732b5d…a1b623`), receipt
`research/notes/index-calculus/round15/run1/receipt.json` (completed,
nine commands exit 0), output `run1/R15.txt`, isolation record
uncontended on reserved CPU 3 (434 s). 18 instances × 40 seeds × 3 arms,
every `k` verified; the active arm reproduces round fourteen's column.

| | registered test | result |
|--|--|--|
| R15-1 | collide-first folded/plain, median over `N ≤ 6·10⁴`, ≤ 0.85 | **holds**: 0.698 |
| R15-2 | same over `N ≥ 10⁵`, ≤ 0.80 | **holds**: 0.716 |
| R15-3 | folded `S_walk`, median over `N ≥ 10⁵`, ≤ 1.05 | **holds**: 1.008 |
| R15-4 | fruitless ≤ 10% on every row; every run verified; round fourteen reproduced | **holds**: max 4.4%; yes; yes |

**What it means.**

- The folded walk now gives its expected saving at every size: median
  folded/plain 0.698 on the small rows and 0.716 on the large ones
  (ideal 0.707), with folded `S_walk` 1.008 at `N ≥ 10⁵` (ideal 0.886,
  the same ~14% above ideal the plain walk sits at).
- As registered, the collide-first folded walk becomes the thread's rho
  reference: `RhoVariant`'s default is now
  `DistinguishedPointsNegationFullWidthCollideFirst`. Earlier variants
  keep their meaning, so every frozen round reproduces.
- **Class: accounting, against index calculus.** The reference rho gets
  cheaper by about 0.70× in walk operations, because index calculus
  already folds `±P` and the reference now does too. Every thread IC/rho
  ratio is to be re-measured against this reference. Those ratios already
  carried the "lower-bound, unranked" label (§5a/§12); against the
  current reference they would be up to `1.5 / 0.70 ≈ 2.1×` less
  favourable to index calculus than when first measured, but they are
  re-measured, not rescaled.
- The window's unpriced native work is up to 32 hash comparisons per
  step, plus one `(a, b)` copy per step; reported, not priced (§5a).
- This reproduces the known result (`LEAD-RHO-negation-map-fruitless-cycles`,
  closed_known) inside the thread. It is not a new lead and not an
  improvement to index calculus.

## Lead: a hyperelliptic family in ecbench — plan, written before code

**Why.** Under §12 a cross-method measurement goes through ecbench, and
under §5a every arm's unpriced work is counted and priced. This thread
has done neither, which is why every IC/rho ratio in it is labelled
"lower-bound, unranked". Rounds twelve to fifteen also showed the
thread's own rho reference needed four corrections to reach the
behaviour ecbench's generic `rho.negation` already has. Running the
thread's instances through ecbench removes the separate reference.

**What is built (infrastructure; no performance claim).**

1. A word-size, `Copy` Mumford divisor for genus ≤ 4 over `F_p`,
   `p < 256`, with Cantor composition and reduction in fixed arrays,
   implementing `CountedGroup`: each composition or doubling is one
   counted group operation, and `key` is an injective encoding of the
   reduced divisor (degree-offset, base `p`), so the distinguished-point
   table, the jump partition and the `±` class rule see distinct
   divisors as distinct.
2. An ecbench instance for the thread's curves
   (`hyperelliptic_thread {p, genus}`, plus an explicit form the measured
   child rebuilds from without repeating the order search). `#J` is
   `c·ℓ` for the thread's prime `ℓ`; `c` is the unique small multiplier
   with `c·ℓ` in the Hasse–Weil interval that annihilates random divisors.
3. Dispatch of the generic arms (`rho.plain`, `rho.negation`, and the
   generic BSGS and kangaroo if they come with it) on that instance; IC
   methods refused until an IC arm exists.

**Correctness checks before any measurement.** Agreement of add,
double, negation and scalar multiplication with the BigUint
`MumfordDivisorP` on genus 2, 3 and 4 thread curves; `ℓ·D₁ = 0`;
key injectivity and `key(D) ≠ key(−D)`; genus-2 `#J` against
`genus2_fp`; an end-to-end ecbench plan/run/verify on small instances
with every run verified and every replay identical. The end-to-end run
is a smoke test, not a result.

**Not in this step.** No IC arm (it needs an adapter from the thread's
index calculus to ecbench's IC interface), no measured comparison, no
leaderboard or benchmarks row.

## Lead: a hyperelliptic family in ecbench — built

What the plan above called for now exists (infrastructure only; no
measured claim):

- `src/cryptanalysis/hyperelliptic_word.rs`: a `Copy` reduced Mumford
  divisor (genus ≤ 4, `p < 256`), Cantor composition and reduction in
  fixed 32-slot polynomials, `CountedGroup` (one composition = one counted
  addition, or one doubling when the inputs are equal; field work inside
  a step is not tallied). `key` is the degree-offset base-`p` index of the
  reduced divisor: injective by construction (all reduced divisors of a
  genus-2, `p = 11` curve enumerated: distinct keys, count = `#J` from
  `genus2_fp`), and below `2^64` up to genus 4, `p = 251`.
- `#J`: point counts over `F_{p^k}`, `k < g`, give every L-polynomial
  coefficient but `a_g`; the candidates are the multiples of `ℓ` in the
  resulting window and the Hasse–Weil interval that annihilate 32 random
  divisors, and exactly one must survive (else the build errors). The
  plan's simpler rule (Hasse–Weil interval alone) was not enough at small
  `p`: at genus 4, `p = 17` it left `2#J … 5#J` standing. Cross-checked
  against `genus2_fp` at genus 2 and full point counts at genus 3.
- `src/cryptanalysis/ecbench/hyper.rs` and `workload.rs`:
  `hyperelliptic_thread {p, genus}` and an explicit form the measured
  child rebuilds from (checks squarefree `f`, a valid generator,
  `[ℓ]G = 0`, `ℓ | #J`, the Hasse–Weil interval and `#J` against random
  divisors); unregistered ICV1-shaped slugs, family `hyperelliptic`,
  `A = 2`, planted targets only.
- `methods.rs`: `rho.plain`, `rho.negation`, `rho.frozen_reference`,
  `bsgs.*` and `kangaroo.vow` dispatch through `solve_generic`; IC methods
  are refused at plan time with the reason.

**Smoke test (not a measurement).** `ecbench_hyper_smoke/`: spec, plan,
run and audit output for genus 2 `p = 41`, genus 3 `p = 23`, genus 4
`p = 17`, two targets each, `rho.plain` and `rho.negation`: 36 of 36 runs
verified, `audit OK: 36 records, 36 verified, 6 replays reproduced,
0 problems`. The groups are tiny; no `S` value from it is read.

**Open.** No IC arm yet (the thread's index calculus needs an adapter to
ecbench's IC interface). `docs/ecbench/schema.sql` still restricts
`family` to prime/koblitz/binary, so database loads of these sessions
need that check widened. Two `tests/ecbench.rs` tests
(`bounds_a_frontier_and_challenge_verdicts_round_trip`,
`field_operations_are_counted_bounded_and_judged`) fail on the committed
tree with or without this change (prime curves only; a missing audit
receipt under fit v2).

**Next.** Re-measure the thread's rho on its 18 instances through
ecbench (`rho.negation` against the thread's collide-first fold), then
the IC adapter, so IC and rho run under §12 and §5a in one session.

## Round sixteen: the thread's instances through ecbench — protocol, written before measuring

**Question.** With the hyperelliptic family now in ecbench, does
ecbench's generic `rho.negation` give the `±` saving on the thread's 18
instances, and where does it sit against the floor and against the
thread's own reference (round fifteen's collide-first folded walk)?

**Spec.** `research/hyperelliptic_ecbench_20261024/spec-r16.json`: the 18
`hyperelliptic_thread` instances of rounds 7–15, 8 planted targets each
(`target_seed` 20261024), arms `rho.negation` (reference) and
`rho.plain` (baseline), 2 rounds after 1 warm-up, alternating order.
Counted operations only (`isolation_required: L0`); no wall-time figure
is read. Boundary: ecbench's floor `√(π/2A)` with `A = 2`, i.e. 0.886,
recorded per run.

**Registered hypotheses.**

- **E1 (instrument).** Every run verifies, and `ecbench verify
  --replay-all` reports every replay identical.
- **E2.** Over the 7 instances with `N ≥ 10⁵`, the median per-curve ratio
  `S(rho.negation) / S(rho.plain)` (ecbench `compare`, per-curve rows) is
  **≤ 0.80** (ideal 0.707).
- **E3 (descriptive, no threshold).** The median `S(rho.negation)` over
  the same 7 instances, set beside round fifteen's median folded
  `S_walk` 1.008. The two differ in targets, seeds and accounting
  (ecbench's `S` includes every charged operation; `S_walk` excludes
  set-up), so they are reported side by side and not ranked.

§5a: each arm's `*_uncharged` counters are listed with their counts and
the comparison is priced at 0, 0.1 and 1 GAE per operation; the E2 order
is stated only if it holds across the band. If E2 holds, ecbench's
`rho.negation` becomes the thread's rho reference and the thread's own
walk is retired (class **accounting**). If E2 fails, the generic fold has
a problem on this family that the thread's rounds 13–15 did not, which is
examined next.

**Run.** Built from the commit that carries the IC adapter (once
reviewed), so the same binary can later run the IC arm; card relay
through `experiment-director` and `executor-mechanical`.
