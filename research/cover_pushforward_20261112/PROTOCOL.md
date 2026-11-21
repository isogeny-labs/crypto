# Cover push-forward: does a bielliptic cover give a logarithm on the embedded elliptic subgroup? Frozen protocol (2026-11-12)

Frozen before any data. Native Rust only: `src/cryptanalysis/chained_transfer/pushforward.rs`,
binary `cover_pushforward`. Builds on `research/chained_prime_transfer_20261111/`.
Every number in this file is a prediction or a definition, `[pcc:unverified]`.

## Question (owner, 2026-11-12)

For a cover `c : C -> E` over `F_p`, `J_C ~ E x P` with `P = ker(c_*)^0` the Prym, and
`c_* c^* = [deg c]`. Pull-back preserves the DLP on the order-`r` subgroup (`r` does not divide
`deg c`). The owner's rule: a proposed shortcut must explain how it computes logarithms on the
embedded elliptic subgroup; decomposing the Jacobian does not supply that step, and a small Prym
dimension is not evidence. This protocol turns the rule into a measurement for the lab's
bielliptic covers `phi1 : C -> E1`, `phi2 : C -> E2`.

## The mechanism under test

`C : y^2 = F(x^2)`, `phi1(x, y) = (x^2, y)`. Let `B` be the set of images under `phi1` of the
degree-1 places of `C` (affine). A weight-2 relation `P1 + P2 ~ [alpha]G + [beta]T` in `J(C)`,
`G = phi1^*(G_E)`, `T = phi1^*(T_E)`, pushes forward to
`phi1(P1) + phi1(P2) = [2 alpha] G_E + [2 beta] T_E` on `E1`: a two-term decomposition over `B`.
`B` is the set of points of `E1` whose `X` is a square, about half of `#E1`. Through `phi2`,
`phi2_* phi1^* = 0`, so pushed relations carry no information about the target on `E2`.

A reading of `Bielliptic::pull` (before any run) sharpens this. `phi1^*(Q)` is *by construction*
the fibre `P + iota(P)` of `Q`, `iota : x -> -x`, and the reduced Mumford representative of a
degree-2 class is unique. So every reduced class
`R = [alpha]G + [beta]T = phi1^*([alpha]G_E + [beta]T_E)` is the fibre of `Q = [alpha]G_E + [beta]T_E`,
and it splits into degree-1 places exactly when `X(Q)` is a square. Both places have the same
image, `phi1(P1) = phi1(P2) = Q`: the pushed two-term decomposition is the degenerate
`[2]Q = Q + Q`.

## Arms (per prime p, same curve, instance and seeds as `chained_transfer_bench`)

Curves: first admissible candidate of the bench's own search loop (same coefficient formula,
candidate seeds `0xC0FFEE + 101 i + s`), `p` in {211, 419, 821, 1619} (the four rows of
`results_v3.json`) and, as affordable, {3203, 6421, 12841}. Instance: `G_E`, `T_E = [x] G_E` of
prime order `r` on `E1` (`ec_instance`), `G = phi1^*(G_E)`, `T = phi1^*(T_E)`. Three seeds per cell
(the bench seed, and two derived by xor with 0x1000 and 0x2000); seed 0 reproduces `results_v3.json`.

- **A0 rho on E1**: `rho_on_ec`, matched reference.
- **A1 IC on J(C)**: a copy of `measure::collect_relations` (comment in code; `measure.rs` is not
  changed), full degree-1 base (`fb_size = p`), stop at `nfb + 8` relations, the library's
  oracle-free elimination. The copy must reproduce `genus2_index_calculus` (group ops, trials,
  relations, recovered log) exactly; if not, the copy is wrong and nothing is reported.
- **A2 pushed relations**: every row of A1, pushed through `phi1_*` and `phi2_*` by
  `Bielliptic::push` and, independently, by point arithmetic on the places
  (`sum e_i phi(P_i)` with the explicit formulas, `T0` terms cancelling in even weight).
- **A3 structure of B**: `|B|` from the places of `C` and, independently, from counting the points
  of `E1` with square `X` (equality is asserted), `|B|/#E1`, `|B|/r`, `|B cap <G_E>|/r`,
  fibre histogram (places per image point), ramified images (`X = 0`), the places at `x = infinity`.
- **A4 E1-fibre arm (native 1-term)**: on `E1`, same stream of `(alpha, beta)` as A1; trial
  `Q = [alpha]G_E + [beta]T_E` (scalar multiplications counted as in `scalar_counted`, plus 1 for the
  sum), accepted iff `X(Q)` is a square (so `Q` is in `B`); row = one column, the class of `Q` up to
  sign. Same stop rule as A1 (number of rows equal to A1's), same elimination. Reported also: the
  first row whose column was already seen (a collision), the group operations spent up to it, and
  the log it yields.
- **A5 E1-2T arm (generic two-term IC over B_r)**: factor base `B_r = B cap <G_E>` (the points of
  `B` in the order-`r` subgroup, columns up to sign), built by `r - 1` counted additions (a cost
  every arm using the target-side base must pay and A1/A4 hide). Trial `R = [alpha]G_E + [beta]T_E`;
  candidates `Q1` from a seeded permutation of `B_r`, each tested by one counted subtraction
  `R - Q1` and a hash lookup; first hit gives the row `R = e1 Q1 + e2 Q2`. Stop when the number of
  rows is `|columns| + 8`; library elimination.
  Group-operation total = base build + scalar multiplications + combine + search subtractions.
- Every recovered log is judged by an independent double-and-add `[k]G_E = T_E` (and `[k]G = T`
  for A1) after the solve, never inside it.
- The transfer (cover construction, `phi^*`, instance building) is a one-time cost, reported
  once per curve as wall time and never added to any per-instance total.

Units: one counted group operation (an elliptic or a Cantor addition or doubling) is 1 GAE, as in
`research/chained_prime_transfer_20261111/amortized/PROTOCOL.md`. Linear-algebra multiplications
over `Z/r` (A1, A4, A5) and the hash lookups are unpriced native work, priced at 0, 0.1 and 1 GAE
per operation (AGENTS.md section 5a, lookups unpriced as a stated limitation); two arms are ranked
only where the order holds at all three prices. A local wall-clock ratio of a Cantor addition to an
elliptic addition is measured in process and reported as unisolated (no isolation tier, no timing
claim).

## Measures

M1 `|B|/#E1`, `|B|/r`, `|B cap <G_E>|/r`. M2 fibre histogram. M3 pushed relations: number valid and
invalid on E1; number valid and invalid on E2; number whose E2 image of the target
(`phi2_*(T)`, `phi2_*(G)`) is not the identity (those are the only rows that could carry the target
to `E2`). M4 fraction of A1 relations that are fibre type (`phi1(P1) = phi1(P2)`). M5 trials per
relation, relations, group ops, LA multiplications, for A1, A4, A5; for A1 and A4, equality of the
accepted-trial index sets. M6 gap to matched rho at each price: `total / rho` (distance remaining,
not a pass/fail gate). M7 first-collision cost in A1 and A4.

## Frozen predictions (probabilities that the stated outcome holds in every measured cell)

- P1 `|B|/#E1` within [0.45, 0.55] at every `p`: 0.90. (`|B cap <G_E>|/r` within [0.40, 0.60] at every `p`: 0.85.)
- P2 every image point has exactly two places except `X = 0` (one place), no other fibre size: 0.97.
- P3 every pushed relation holds on E1 (0 invalid): 0.98. Every pushed relation holds on E2 (0 invalid): 0.97.
- P4 `phi2_*` of the generator and of the target are the identity in every cell, so 0 pushed E2
  relations involve the target: 0.98.
- P5 (sharpened, derived from the code before any run) every A1 relation is fibre type (M4 = 100%): 0.95.
  Consequently the pushed relation is `[2]Q = [2] phi1(P1)`, not an independent two-term decomposition.
- P6 (the owner's statement, stated as a test) A1 and A5 have the same trials per relation and the
  same number of relations up to the group-operation cost ratio: probability 0.15 (trials per
  relation within 20% in every cell). My expectation is that A5 needs about one trial per
  relation (a random point has about `r/4` representations as a sum of two points of `B_r`),
  A1 about two (the splitting probability), and that A5 needs about `|B_r|/2` columns against
  A1's `nfb`.
- P7 A1 and A4 accept exactly the same trials (identical index sets): 0.93; same relation count.
- P8 A1 and A5 total group operations exceed matched rho at every `p` and at all three prices
  (gap ratio above 1 in every cell): 0.92. A4 (with its first-collision stop) is the arm with
  the best chance to approach rho; probability that A4 first-collision cost is below rho at some
  cell: 0.25.
- P9 A5 total includes a base-build cost of `r - 1` that alone exceeds rho's cost in at least one cell: 0.60.

## Decision rule (frozen)

Let the **shortcut hypothesis** be: a cover gives the target-side logarithm cheaper than the
matched rho. The verdict is **no help from the cover** if all of the following hold in the measured
cells:

1. M3: zero pushed E2 relations involve the target and zero pushed relations are invalid on E1 or E2
   (the mechanism is as stated);
2. M1: `|B cap <G_E>|/r >= 0.4` at every `p` (the target-side factor base is not smaller than the
   group by more than a factor 2.5);
3. M6: A1, A4 (full stop rule) and A5 each exceed matched rho at every `p` at all three prices.

If (3) fails in a cell, that cell is reported as a measured improvement [pcc:unverified] with its remaining distance
to rho (success criterion of the lab), after checking it against the rho band and the certificates
and not otherwise. If (1) or (2) fails, the verdict is "mechanism differs from the prediction" and
the table says how. The equivalence statement P6 is judged separately and does not enter the
verdict. A cover would have to supply, to be of use: an explicit map `E1 -> (something)` together
with a target-side factor base of size well below `r` (for example `r^(1/2)` times a polylog)
on which two-term or few-term relations are found at a rate that makes the relation phase cost
below `sqrt(r)` group operations; the measured `|B|` bounds that factor base from below by about
`r/2` for bielliptic covers of degree 2.

## Amendment 1 (2026-11-12, after a two-prime smoke run, before the full run)

Nothing above this line is edited; the sha256 of the text above is the first line of
`frozen.sha256`, the sha256 of this file with the amendment is the second line.

A smoke run at p = 211 and 419 (two seeds, no record) showed the first-collision stop of the J(C)
and E1-fibre arms costing less than the matched rho at those two small primes, which prediction P8
had given 0.25. The cause is visible in the arms: the accepted classes are fibres of single points
`Q`, so the elimination is a collision search on the `X` of `Q`, and rho's fixed start-up (sixteen
jump points, each two scalar multiplications) dominates its count at `r` of about 100. To separate
what the cover contributes from what plain birthday search contributes, one control arm is added,
declared here after that smoke run (post hoc, a control and not a hypothesis test):

- **A6 birthday on E1 (control)**: the same stream of `(alpha, beta)`, every nonzero
  `Q = [alpha]G_E + [beta]T_E` kept (no square-`X` filter, no cover, no factor base), keyed by `+-Q`,
  stopped at the first usable collision, log certified by an independent double-and-add. Counted
  like A4 (scalar multiplications plus one combine per trial).

Reading rule added: a cell where a first-collision arm is below the matched rho is reported as a
measured gap with its trend in `p`, together with A6; it is attributed to the cover only if A4/A1
first-collision cost is below A6's at the same cell (A6 uses no cover at all). Prediction for A6
(frozen now): A6 first-collision group operations are below A4's first-collision group operations
at every cell where both exist (the filter keeps about half of the trials): 0.85.

## Amendment 2 (2026-11-12, after the primary run, before the supplementary run)

The primary run (three seeds per cell, as frozen) showed first-collision costs that vary by a factor
of 20 or more between seeds in one cell (a birthday count is geometric-like), so three seeds cannot
place a first-collision arm against rho. A supplementary run of the same binary with `--seeds 15`
(seeds `seed0 xor (k * 0x1000)`, k = 0..14, the first three are the primary run's) is added. The
verdict rules above are judged on the primary run; the supplementary run is used only for the
distance-to-rho statement of the first-collision arms and of A6, reported as the median and the
range over 15 seeds and the fraction of seeds below the matched rho on the same seed. No other
change.
