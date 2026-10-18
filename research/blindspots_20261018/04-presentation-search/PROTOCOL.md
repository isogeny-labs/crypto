# Item 04 protocol: presentation search for the m-sum variety in the binary descent

Frozen 2026-10-18, before any measured run. Source design: section "04" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere that the budget does not force; every deviation is
listed at the end.

Class: **stage diagnostic** (a solving degree). It computes no `S`, no rho
ratio and no GAE, so the section 5a three-point pricing band and the
leaderboard do not apply to this item's own rows. The unit is the counted
operation: the Macaulay degree `D`, the matrix size and the word XORs of the
elimination ("column touches"). Wall time is descriptive.

## Question

Does the descended solving degree `D` depend on the presentation of the
summation variety, at a fixed `F_2`-subspace factor base and a fixed target?
The incumbent presents `P_1 + .. + P_m = R` as Semaev's `S_{m+1}(x_1, .., x_m,
x_R) = 0` with no intermediates. Five other presentations are measured against
it.

## What already exists, and what is reused

* `examples/blindspots_07_blocked_closure.rs` and its protocol measure the
  closure degree of the **chained `S_3`** system (arm e here) for a Koblitz-type
  `b = 1` curve and for `m = 4`, `l = 4, 5`, `n = 11 .. 17`. Committed
  evidence for arm e, not re-measured as a cell: `D_F4 = 4` on all four
  unsatisfiable draws of (11,4,4) and a 3 GB crash at (13,4,4), item 06
  (`research/ic_next10_20261014/06-equivariant-f4-m4/README.md`); closure
  degree 4 at (13,4,4), (11,5,4), (12,5,4) and `m = 3`, `l = 6`
  (`research/dreg_closure_scale_20261016/RESULTS.md`,
  `research/dreg_closure_driver_20261016/RESULTS.md`). None of the five cells
  here is one of those (different `n`, `l`, curve and subspace), so **no cell
  is reused**; arm e is measured on all five. That `D_e = 4` there is a
  consistency expectation for arm e, not a threshold of this protocol.
* The closure engine is a port of item 07's blocked MutantXL engine (same
  column order, same fall/product scheme), generalised to three-word monomials
  (up to 190 variables) so that arm b (98 to 136 variables) fits. Item 07's
  file is not edited.
* `examples/degree_reduction_presentation.rs` is about saturation "doses" of one
  system, not about presenting the variety; no committed result compares
  presentations of the summation variety. `RESEARCH_ECC2K130_DESCENT_DEGREE.md`
  (under `research/notes/ecc2k130/`) fits `D` against `l` for the chained
  presentation only.
* msolve, Magma and Sage are not installed; no external arm is run (the
  closure is the instrument, as in item 07). The F4 step degree equals the
  closure degree on every pair item 07 inspected; that is a property of the
  instrument inherited from item 07, not re-established here.

## The blocker that shaped the instrument

`cargo build --release --lib` fails at the branch base (25 errors in `ecbench`
and `gaudry_quartic`, as items 01, 02 and 07 recorded), so no `cargo build
--example` target can be built. The instrument
`examples/blindspots_04_presentation_search.rs` is `std` only; each card builds
it with `rustc` into its own directory under `target/blindspots-04/` and
removes it at the end. Nothing in `src/` is touched.

## Objects

For a cell `(n, l, m)` ("field degree, factor-base dimension, summands"):

* `F = F_{2^n}`, `n` odd, with the repository's sparse irreducible polynomial.
* A curve `E_{a,b}: y^2 + xy = x^3 + a x^2 + b` derived from binary Edwards
  parameters `(d1, d2)` (Bernstein, Lange, Farashahi): `Tr(d2) = 1` (complete
  addition law, neutral element `(0,0)`), `a = d1^2 + d2`,
  `b = d1^2 (d1^2 + d1 + d2)`. Every cell asserts that the Edwards affine point
  count equals the Weierstrass point count before any arm runs. `(d1, d2)` are
  drawn from a splitmix stream seeded by `cell_seed(n, l, m)`
  (`20261018 ^ n<<40 ^ l<<32 ^ m<<24`, second output).
* The factor base `V = <1, beta_2, .., beta_l>`, an `F_2`-subspace of dimension
  `l` whose first basis vector is `1`; the next vectors are drawn in the seeded
  order, and the first basis is taken for which at least `m + 1` non-zero
  elements carry rational points in every frame below (so a planted
  decomposition with distinct summands exists). **The same `V`, as the same
  field elements, is the factor base of every arm.**
* Three frames, in which an arm's unknown `P_i` has its frame coordinate
  `z_i` in `V`:
  * X: `z = x` (Weierstrass);
  * U: `z = u = c/(x + c)`, `c = b^(1/4)`, which turns translation by the
    rational 2-torsion point `T = (0, sqrt b)` into `u -> u + 1` (verified);
  * W: `z = w = x + y` on the Edwards curve, invariant under `P -> -P` and
    `P -> P + (1,1)`.
* Six presentations of `P_1 + .. + P_m = R`:

| arm | frame | presentation | unknown bits `N` |
|---|---|---|---|
| a | X | Semaev `S_{m+1}(x_1..x_m, x_R) = 0`, no intermediates; `n` Boolean equations in the `m l` subspace bits (incumbent) | `m l` |
| b | X | unprojected chain: `y_i` (n bits each) with the curve equation, chord slopes `lambda_j` with `lambda_j (X_Q + x_{j+1}) = Y_Q + y_{j+1}`, the affine addition law, `P_1+..+P_m = +-R` through one sign bit | `m l + (2m-1) n + 1` |
| c | W | binary Edwards `w`-frame `S_{m+1}`, built by nested resultants from `S_3^Edw`, which is **interpolated** from group-law triples (a one-dimensional solution space is asserted) | `m l` |
| d0 | U | Moebius-frame `S_{m+1}` (diagnostic: the frame without the symmetrisation) | `m l` |
| d | U | d0 with the 2-torsion symmetrisation: the first-coordinate bits of the `m` summands are replaced by their parity (valid because `1 in V` and the zero set is invariant under even-weight shifts) | `1 + m(l-1)` |
| e | X | chained `S_3` with `(m-2)` intermediates of `n` bits (Semaev 2015), reference | `m l + (m-2) n` |

`S_4` and `S_5` of a frame are evaluated at every point of `V^m` as nested
resultants of the frame's `S_3` (quadratic-quadratic Sylvester formula; for
`m = 4` the degree-4 factor is interpolated at five nodes), and the `n`
coordinate functions are converted to algebraic normal form by the Moebius
transform of the truth table. That ANF is the exact descended polynomial
system (an `F_2`-valued function has a unique squarefree normal form).

### The measured quantity `D`

Every system is first reduced by an **affine elimination** (variables fixed by
affine equations are substituted; an invertible affine change of variables
preserves the degree filtration and the closure, so `D` is unchanged; it only
shrinks the column set; applied to every arm; tested). The equations are then
echelonised (pivot on the greatest monomial in a degree-refining order) so that
the rows of degree `<= d` span exactly the degree-`<= d` part of their span.

`V_d` is the MutantXL fixed-degree closure: the least subspace of
polynomials of degree `<= d` containing the echelon rows of degree `<= d` and
closed under multiplication by a variable of every element of degree `< d`
(Boolean ring, squarefree monomials). **`D` is the least `d` at which `V_d`
equals the whole space of polynomials of degree `<= d` vanishing on the
system's exact solution set.** For an unsatisfiable system that is the
refutation degree (`1 in V_d`), the quantity item 07 reads; for a satisfiable
one it is the degree at which every polynomial of degree `<= d` that vanishes
on the solutions is generated (checked by `rank V_d = columns - rank of the
evaluation matrix of the solutions`). By the Boolean Nullstellensatz the
equality always holds at `d = N`; the self-test checks this on tiny systems of
every unchained and chained arm, which is the engine's completeness control.

If the closure at degree `d` saturates without reaching the vanishing ideal,
`D >= d + 1` is a **certified lower bound** (`D_lb`); a run that stops on its
time limit or memory cap reports the lower bound of the last saturated degree
and is recorded BLOCKED(time) or BLOCKED(memory), never as evidence of `D`.

### Exact solution sets and "identical solution sets"

For every target the instrument enumerates the arm's solution set exactly
(truth table for a, c, d0, d; depth-first search over the chain for e; a
search over rational points and signs for b) and independently enumerates the
true decompositions by the group law (`G`: tuples in `V^m` for which some
choice of points in the fibres sums to `R`; `G_gen`: those with a chain of
distinct `x` at every step). The gate for "the arm solves the same problem" is,
per run: (i) every `G` tuple (arms a, c, d0, d) or `G_gen` tuple (arms b, e) is
in the arm's projected solution set; (ii) every enumerated solution satisfies
the arm's polynomial equations; (iii) a planted run contains its planted
tuple. Raw set sizes and the spurious counts (projected solutions that are
not true decompositions: non-rational intermediates in a, d0, d, c, e) are
recorded, so a raw difference between arms is visible and never silently
absorbed. Arm d's lifted zero set must equal arm d0's exactly (checked in the
self-test).

## Controls

1. **Arithmetic self-test** (`b04 selftest`): field axioms, quadratic roots
   against brute force, both group laws (closure, associativity), `x(P+T) =
   sqrt(b)/x`, the Moebius frame, `S_3` vanishing on `P+Q` and `P-Q` in all
   three frames, the invariances of `w`, the Edwards/Weierstrass point-count
   equality, `S_4` and `S_5` vanishing on true decompositions and not
   identically zero off the variety, the symbolic residuals of arms b and e
   against field arithmetic at random assignments, exact vanishing of the ANF
   systems on their enumerated solutions, `d` against `d0`, and closure
   completeness at degree `N` on tiny systems of every arm.
2. **Positive control, planted decompositions** (design): 4 planted targets
   per cell (separate from the 8 random targets, built in each frame from that
   frame's own factor base). Every arm must contain the planted tuple in its
   enumerated solution set and satisfy the gate; where the closure reaches the
   vanishing ideal, the satisfiable `D` is reported next to the refutation
   `D`.
3. **A/A control** (negative, the right null): arm a on an independent second
   draw (a different `V` and different targets, same curve) compared with arm a
   on the primary draw under the same rule. It must not lower `D` on more than
   one of the measured cells, else the rule is not discriminating and the
   verdict is INCONCLUSIVE.
4. **Presentation-invariance null**: arm a with the unknowns replaced by a
   random invertible `F_2`-linear recoding of the same bits (`a-linear`);
   `D` must equal arm a's exactly (an invertible linear change of variables
   cannot change `D`).
5. **Sensitivity control**: arm a with a triangular quadratic recoding of the
   unknowns (`b_k <- b_k + b_{k+1} b_{k+2}`, a bijection) (`a-nonlinear`),
   which raises Boolean degrees; `D` is expected to change, showing the
   instrument sees a change of presentation.

## Cells, draws, targets (frozen)

Cells `(n, l, m)` = (17,4,3), (19,4,3), (13,3,4), (15,3,4), (17,4,4) with
unknowns `N`: a, c, d0 = 12, 12, 12, 12, 16; d = 10, 10, 10, 10, 13; e = 29,
31, 38, 42, 50 (28 to 49 after elimination); b = 98, 108, 104, 118, 136 (81,
89, 91, 103, 119 after elimination).

* Draw 0: 8 random targets (a random rational point per frame family: one
  Weierstrass point for the X and U frames, an independent Edwards point for
  the W frame; the Edwards `R` is not the image of the Weierstrass `R`, see
  Deviation 3) and 4 planted targets.
* Draw 1: arm a only (the A/A control), 8 random targets.
* Controls 4 and 5: arm a, draw 0, 4 random and 2 planted targets, cells 1 to
  4 (cell 5's arm a has `D >= 13` and is not run beyond that bound).
* Seeds and every parameter are derived deterministically from the cell; the
  cards fix every flag, and each run prints its curve, basis and counts.

## Decision rule (frozen)

For a candidate arm `X` in {b, c, d} (d0 is a diagnostic, e the reference
that cannot by itself advance), on a cell, with `D_X(t)` exact for the 8 random
targets `t` and `D_a(t)` exact **or** certified `D_a(t) >= D_lb(t)`:

* `X` **lowers D on the cell** if `median_t D_X <= median_t D_a(lb) - 1` and
  `max_t D_X <= max_t D_a(lb) - 1` and the gate holds on every random and
  planted run of `X` on the cell. (Using a lower bound for the incumbent is a
  tightening: a certified bound can only understate `D_a`.)
* Verdict **ADVANCES (primitive)** if some candidate lowers `D` on at least 4
  of the 5 cells and the A/A control is valid (at least 4 measured cells, at
  most 1 passing); **INCONCLUSIVE** if a candidate satisfies the first part and
  the A/A control is invalid; **MATCHES** otherwise. The per-arm lines say on
  which cells a candidate is unmeasured (BLOCKED) so that a MATCHES verdict
  cannot hide an unmeasured arm. Whether a lowering is also below arm e is
  reported separately (a primitive that beats only the unchained incumbent is
  not new against the known chained route).

`b04 summarize raw/*.jsonl` applies this rule mechanically; it interprets
nothing.

## Budget and stop rules

One thread per job, address space capped at 4 GiB (`prlimit --as=4294967296`),
basis capped at 3000 MiB; at most 2 CPU-hours of measured runs in all; every
card finishes in under 9 minutes. The instrument stops itself at `--time-limit`
(whole invocation) and `--job-time` (one arm on one target). Arms e and b stop
at the first target of a cell that ends on the time limit or memory cap; the
remaining targets of that invocation are recorded `not-attempted-stop-rule`,
and the later cards are written so that a blocked cell is not retried.
Estimated costs, from the smallest-cell smoke test scaled by the column count
(arm b: time ~ columns^3, 30 s at 88,642 columns): arm b degree-3 closure
30, 70, 85, 260 s per target at cells 1 to 4; at cell 5 (280,960 columns) the
reduced basis alone needs about 4.9 GB, which is beyond the 4 GiB cap, so the
cell is **BLOCKED(memory) by projection** (`b04 plan` records it) and is not
run. Arm e: about 1.3, 2, 22, 62 and 360 s per target at cells 1 to 5. Arms a,
c, d0, d: seconds, except a and c at cell 5 where the equations have Boolean
degree 12 on 16 unknowns, so `D >= 13` is certified by a degree-12 closure
that saturates and the exact `D` (degree 13 to 16, minutes per target) is
**not** attempted (BLOCKED(time) for the exact value; the certified bound is
what the rule uses).

Expected total about 105 CPU-minutes, command ceilings summing to under 2 CPU-
hours; the allocation of targets to cards is in the cards themselves.

## Deviations from the frozen design

1. **Instrument and build.** `std`-only Rust compiled with `rustc` per card
   (the library does not build), not `cargo build --example`. D is the MutantXL
   closure degree, not an external F4 step degree (msolve is absent); the
   justification is item 07's.
2. **Closure engine details.** Processing order lowest degree first (changes
   the time to find a refutation, not the closure); affine elimination before
   the closure (invariance argued above, tested).
3. **"Same base and target" across frames.** The same `F_2`-subspace `V` (the
   same field elements) is the factor base in every arm's own coordinate. For
   arms a, b, e this is the same point set and the same target. For d0, d (the
   Moebius coordinate) and c (the Edwards `w`) the point sets differ from the
   X frame's; the target `R` is the same point for d0 and d and an
   independent point of the Edwards curve for c (the birational map between
   the two models is not available, only the equality of point counts, which
   every cell asserts). `D` is therefore compared across presentations of the
   same shape of problem on the same curve family, not on one point set.
4. **Interpretation of "identical solution sets".** The raw solution sets of
   the presentations legitimately differ off the rational generic locus, so the
   gate is completeness on the true decompositions plus soundness and planted
   containment, with raw and spurious counts reported (see above). Arm d0 and d
   are required to agree exactly.
5. **Arm b's solution set** lists the generic solutions (distinct `x` at every
   chord step). A step with equal `x` and `y` leaves its slope free in the
   polynomial system; those solutions are not enumerated (their expected number
   is below 0.05 per target) and would show up as a closure that never refutes.
   Arm b's planted runs at cells 2 and 3 use degree cap 2, so they verify
   enumeration, soundness and containment only, not the satisfiable `D`.
6. **Added arms and controls**: arm d0, the A/A control, the linear and
   nonlinear recodings, the closure-completeness self-test. None loosens the
   rule.
7. **Reductions forced by the 2 CPU-hour ceiling** (recorded here and printed in
   the raw rows): arm b at cell 5 is BLOCKED by memory projection, not run;
   arm e at cell 5 has 1 random target and no planted target; arm e at cell 4
   has 8 random and 2 planted targets; arm b has 4 planted targets at cell 1
   and 2 at cells 2 to 4 (none at cell 5). A cell with fewer than 8 measured
   random targets for an arm does not count for that arm under the rule.
8. **Smoke test and pilots disclosed.** Before this freeze, the instrument ran
   the smallest cell (17,4,3) for arms a, c, d0, d, e, b (one to twelve
   targets) and a timing pilot at the unmeasured shape (11,4,4) for arms a, c,
   d0, d. Their outputs were used to size the cards (and to choose lowest-
   degree-first processing and the degree cap of arms a and c at cell 5), are not
   raw data of this item, and are not committed; the cards re-run the cell
   from scratch. The values seen at (17,4,3) were `D = 7` (a, c), `5` (d0, d),
   `4` (e) and `3` (b), one target for b; they do not enter any threshold.
9. **Conductor.** `conductor` is not installed on this host; the hooks warned
   and let edits through. Every commit uses an explicit pathspec because
   another lane commits in the same working tree.

## What a card cannot decide

A card never interprets. The assess stage reads `raw/`, applies the rule above
with `b04 summarize`, and writes `README.md`, the graph and `REPORT.pdf`.

## Amendment 1 (before any measured run): the card plan and the operational stop rule

The measured runs are twenty frozen cards, `card-01.yaml` to `card-20.yaml`,
each building the instrument with `rustc` (7 s) into
`target/blindspots-04/card-NN/` and removing it, each with command timeouts
summing to at most 530 s:

| card | content | expected |
|---|---|---|
| 01 | self-test; arms a, c, d0, d at cells 1 to 4 (8 random + 4 planted); cell 5 arms d0, d; cell 5 arms a, c capped at degree 12 (certifies `D >= 13`) | 2 min |
| 02 | A/A control (arm a, draw 1) at cells 1 to 5; linear and nonlinear recodings of arm a at cells 1 to 4 | 1 min |
| 03 | arm e: cells 1 and 2 (8 random + 4 planted), cell 3 random 0 to 3 + planted 0 to 3 | 4 min |
| 04, 05 | arm e: cell 3 random 4 to 7; cell 4 random 0 to 3 and 4 to 7; cell 4 planted 0, 1 | 6, 7 min |
| 06 | arm e: cell 5, one random target | 6 min |
| 07, 08 | arm b: cell 1, random 0 to 7 and planted 0 to 3 (degree cap 3) | 5, 1 min plus card 08's cell-2 share |
| 08, 09 | arm b: cell 2 random 0 to 3, 4 to 7; cell 2 planted 0, 1 (degree cap 2) | 5 min each |
| 10, 11 | arm b: cell 3 random 0 to 3, 4 to 7; cell 3 planted 0, 1 (degree cap 2) | 6 min each |
| 12 to 19 | arm b: cell 4, one random target per card (`--first t`); card 12 also cell 4 planted 0, 1 (degree cap 2) | 4.5 min each |
| 20 | `b04 plan --build` for the five cells (records the cell-5 arm-b projection), `b04 summarize` of `raw/` into `raw/summary.txt` and `raw/summary.tsv` | 1 min |

Expected total about 92 CPU-minutes; the instrument time limits (which bound
what a card can spend) sum to about 117 CPU-minutes, inside the 2 CPU-hour
ceiling. Each cell invocation prints its curve, basis and per-run counts to the
`.log` next to its `.jsonl`.

**Operational stop rule.** Arms e and b are attempted cell by cell in the order
above. If the first card of a cell ends a run on `timelimit` or `memcap` (a
`.jsonl` row whose status is `timelimit` or `memcap`), that arm's remaining
cards for the same cell are not relayed (cards 05, 06 for arm e; cards 13 to 19
for arm b at cell 4) and the cell is recorded BLOCKED with the exact reason. A
card whose instrument rows show `skipped-timebudget` leaves those targets
unmeasured (BLOCKED(time)); they are not retried. Card 20 is relayed last in
every case.
