# Ordinary elliptic Prime31 graph relation gate

## Frozen source group

The exact registered instance is
`docs/ic/research/fp_p31_subexp_calibration_20261121/CURVE.json`:
`p=2147483647`, `E: y²=x³-3x+21`, prime group order
`n=2147487637`, and generator
`G=(1391443431,177979044)`. The public smoke target in that file is
`Q=(291202666,1848813168)`, with validation scalar 987654321 kept outside
any solver input. This is an elliptic-curve group, unlike the genus-four
Picard Jacobian in PR #5090.

## First mechanism and falsifier

For a factor base of `B` distinct points, draw a uniform public-group point
`R=aG+bQ`. For each factor-base point `F_i`, compute `L_i=R-F_i` and retain
the partial equation only when `L_i` belongs to a fixed external pool of
size `S`. A repeated `L_i` cancels exactly and yields a full relation;
every candidate must be replayed as a point equation before rank insertion.
This is the simplest sound one-large-prime graph producer. It has no
special curve-decomposition oracle: `L_i` is computed by a group addition.

Before a native implementation, screen its optimistic uniform-occupancy
model. Residuals from the same draw `R` are distinct because the `F_i` are
distinct. For two independent draws and two fixed factor-base indices, the
probability of a common residual in the pool is exactly `S/n²`.
For `T` random `R` draws, the expected number of cross-draw collision pairs
is therefore `T(T-1)B²S/(2n²)`. Giving **every** collision one independent
equation, choose the least integer `T` whose expected count reaches `B+1`.
Testing all `B` points costs `TB` group additions. This optimistic count
excludes target point generation, factor-base
construction, storage, graph processing, row rank, linear algebra and
failures. The best permitted `S` is `n`; smaller pools increase this count.
At `S=n`, this is approximately `sqrt(2n(B+1))` group additions, versus
`sqrt(pi*n/2)` expected generic collision steps for the same subgroup.

The pair count deliberately overcredits rank. For a realized one-LP stream
with `E` retained partials and `U` occupied LP columns, the large-prime
coefficient block has rank exactly `U` when every coefficient is nonzero:
its left kernel has dimension `E-U`. Therefore **at most `E-U`** independent
LP-free combinations can reach the factor-base matrix, even if one occupied
LP has many collision pairs. The actual factor-base rank can be lower still.
This exact rank bound is the first check for any native stream.

`generic_lp_gate.py` computes the discrete collision target and this
optimistic cost for `B=1,16,64,256,1024` and `S=n,n/16,n/256`. The gate
rejects **this producer** if even the optimistic source work exceeds a
matched rho run; that is not a lower bound on algebraic point-decomposition
or other graph-assisted producers. A successor needs a source-verified
partial producer with non-generic yield before applying the repository's
gain-aware DSU, two-core, low-stretch, MST, matching/DM or block-Wiedemann
machinery.

The calculated best-case pool `S=n` gives the following source-addition
counts. They are derived occupancy expectations, not measured timings or
certified useful rank. The rho column uses the *unfolded* birthday
expectation `sqrt(pi*n/2)` (~58,075 steps), which is generous to this
graph source compared with optimized negation-map rho.

| Base size B | Random R draws | Optimistic source additions | Ratio to unfolded rho steps |
| ---: | ---: | ---: | ---: |
| 1 | 92,683 | 92,683 | 1.596 |
| 16 | 16,889 | 270,224 | 4.653 |
| 64 | 8,257 | 528,448 | 9.099 |
| 256 | 4,105 | 1,050,880 | 18.094 |
| 1,024 | 2,050 | 2,099,200 | 36.143 |

The row count requirement and collision-rank optimism make the larger-B
ratios a screen against this *uniform residual* producer. An algebraic
producer with a measurably different conditional LP distribution could
change the conclusion and needs its own frozen experiment.

## Pair-table extension of the same screen

The existing S₄ meet-in-the-middle oracle builds a table of two-factor-base
point sums and probes it with `R−F_k`. In an idealized point-keyed version,
let `H` be the number of distinct table points, at most `B(B+1)/2` before
sign variants. Replacing the third factor-base point by an external LP from
a fixed pool changes the query from `R−F_k` to `R−L`. For uniform `R`, each
fixed probe hits the same pair table with probability `H/n`, regardless of
whether the queried point is a factor-base point or an external LP.
After `M` probes, expected accepted partials are `MH/n`. To obtain `B+1`
full rows, exact LP elimination needs `E−U≥B+1`, so the partial producer
needs at least `B+1+U` accepted rows; direct full probes need only `B+1`.
This rules out a relation-acquisition advantage **under this uniform
point-keyed model** at equal table construction and probe cost. The actual
implementation uses x-keyed sign variants and must be measured separately.
An improvement requires a source with conditional yield, per-probe cost,
or projected-rank behavior outside this model, with exact group replay.

## Algebraic partial producer to qualify next

The repository's RR5 public four-summand extractor is a specific candidate
outside the pair-table screen. For a public query `R=aG+bQ=(r,s)` on a curve
`y²=x³+Ax+B`, it constructs a monic quartic `U_R(X)` whose four distinct
roots lift to signed points summing to `R`. Its complete-relation form
requires all four roots in the factor base. A one-LP form would use disjoint
abscissa selectors `h_F` and `h_L` and accept only a recovered quartic with
exactly three roots in `h_F`, one root in `h_L`, and a verified signed point
sum. It would emit the actual equation

`aG+bQ = ε₁F_i+ε₂F_j+ε₃F_k+ε₄L_t`.

The external point `L_t` must carry a stable identity across queries; it
cannot be relabelled by abscissa alone because the sign is part of the row.
For a fixed disjoint base of `B` abscissae and LP pool of `S` abscissae, a
uniform `R` has exactly `16*C(B,3)*S/n` expected **signed witness tuples**
per draw in the three-base/one-LP family, compared with
`16*C(B,4)/n` in the four-base family. This follows by linearity of
expectation over signed tuples, even when several tuples have the same sum.
Their ratio is `4S/(B-3)` for `B≥4`. It is a witness-count ratio, not a
probability of successful extraction or independent-row yield. In particular,
it does not include the cost of building and solving the enlarged selector
system, the occupancy loss `E-U`, or the factor-base rank of its projection.

The published RR5 Prime31 controls use
`E:y²=x³+847514846x+2087444724`, order `2147446831`, whereas this study's
registered curve is `E:y²=x³-3x+21`, order `2147487637`. Its completed B8
controls and public-query recovery on a smaller field establish neither
extractor completion nor useful graph rank on this registered curve. The
existing B677 input-construction check is likewise not an extraction result.

The first falsifier is a fixed, source-replayed panel on the registered curve:
use a disjoint public base and LP pool, retain `Complete`, `Empty`, and every
incomplete extractor state separately, replay every signed equation, then
measure accepted `E`, occupied `U`, projected factor-base rank, wall time,
CPU time, and memory against the same queries with four-base extraction.
Count selector construction, Groebner work, roots, signs, group replays,
graph elimination, linear algebra, target recovery, and failed queries.
If the larger signed witness family does not produce useful rank at a lower
cold cost than direct rows and matched negation-map rho, it does not advance
the end-to-end boundary. Only after this gate should gain-aware DSU,
two-core filtering, low-stretch forest, MST, matching/DM, and block
Wiedemann be evaluated on the actual partial stream.

### First registered-curve control, 2026-11-23

`native/main.rs` calls the inherited RR5 `extract` on a five-point combined
selector: four publicly enumerated base points and one disjoint external
point. The positive query is formed test-side from three base points and the
external point; the separate smoke query is the registered public `Q`. The
native run completed both charts: the positive returned one signed one-LP
row in 1 ms, and the smoke query returned `Empty` in under 1 ms. The native
producer never receives the positive witness indices. The separate
`verify_rr5_control.py` uses its own integer group law and exhaustive 64-tuple
oracle **after** extraction; it found exactly one positive witness and zero
smoke witnesses, matching both native outputs. The raw JSON and build stderr
are in `evidence/rr5-control-01/`.

The build uses the existing RR5 source closure. Its two whole-source pins
were refreshed after reviewing current-main changes: `ic_boundary.rs` gained
test-only map views and binary-group provenance fields; `semaev_prime_poly.rs`
gained support diagnostics and factored `poly_mulmod` through `poly_mul`.
The selected source ranges and current hashes are checked at build time.
The native diagnostics report incomplete phase accounting. This is a
planted correctness control at `B=4,S=1`, not evidence of random-query
yield, useful graph rank, cold solve cost, or a speed advantage.

## Prior work and claim boundary

- Semaev, [*Summation polynomials and the discrete logarithm problem on
  elliptic curves*](https://eprint.iacr.org/2004/031.pdf), defines the
  prime-field relation problem but conditions its advantage on solving the
  bounded polynomial systems efficiently.
- Amadori, Pintore and Sala, [*On the discrete logarithm problem for
  prime-field elliptic curves*](https://eprint.iacr.org/2017/609.pdf),
  distinguish the prime-field decomposition problem from the extension-field
  subspace construction.
- Shoup, [*Lower bounds for discrete logarithms and related
  problems*](https://www.shoup.net/papers/dlbounds1.pdf), gives generic-group
  bounds. The occupancy screen here is a specific model calculation, not a
  reproof of that theorem.
- The repository already implements graph filtering and diagnostics in
  `src/cryptanalysis/large_prime_graph.rs`,
  `src/cryptanalysis/large_prime_filter.rs`, and
  `docs/ic/graph-algorithms/README.md`. Graph processing alone cannot
  substitute for the missing ordinary-elliptic partial-relation source.

No same-target scalar or performance claim is admitted by this screen.
