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
the end-to-end boundary. Only after this gate should graph filters and
linear algebra be evaluated on the actual partial stream. Start with the
existing exact gain-aware DSU, singleton peeling, BFS witness paths, and
pivot filter. The repository's `research/lp_graph_algorithms_20261123/README.md`
found its low-stretch heuristic longer than BFS on its measured LP streams
and MST slightly sparser but much slower; neither result transfers
automatically to Prime31. Compare those choices, matching/DM, and block
Wiedemann on the same source rows only when their graph and matrix sizes
warrant them.

### First registered-curve control, 2026-11-23

`native/main.rs` calls the inherited RR5 `extract` on two combined selectors:
four publicly enumerated base points plus one disjoint external point, and
seven base points plus one external point. The positive queries are formed
test-side from three base points and the external point; the separate smoke
query is the registered public `Q` on the five-point selector. The native
run completed all three charts: the five-point positive returned one signed
one-LP row in 9 ms, the smoke query returned `Empty` in 1 ms, and the
eight-point positive returned one signed one-LP row in 166 ms. The native
producer never receives the positive witness indices. The separate
`verify_rr5_control.py` uses its own integer group law and exhaustive
64- and 560-tuple one-LP oracles **after** extraction; it found exactly one
witness for each positive and zero for the smoke query, matching all native
outputs. A second eight-point positive query shares the same external point;
the native extractor completed it with one one-LP row in 204 ms. The
independent checker subtracts the two extracted point equations, cancels the
external point, and replays the resulting six-term factor-base-only equation.
The raw JSON and build stderr are in `evidence/rr5-control-01/`.

The build uses the existing RR5 source closure. Its two whole-source pins
were refreshed after reviewing current-main changes: `ic_boundary.rs` gained
test-only map views and binary-group provenance fields; `semaev_prime_poly.rs`
gained support diagnostics and factored `poly_mulmod` through `poly_mul`.
The selected source ranges and current hashes are checked at build time.
The native diagnostics report incomplete phase accounting. These are
planted correctness controls at `B=4,S=1` and `B=7,S=1`, not evidence of
random-query yield, useful graph rank, cold solve cost, or a speed advantage.
The cycle control only has public point queries; their IC scalars `a,b` are
not supplied, so it validates graph cancellation without becoming a target
discrete-log solve.

### Unplanted public-query panel and fixed-selector gate

The same native executable also ran 16 fixed SplitMix-labelled public queries
`R=aG+bQ` on the `B=7,S=1` selector. It reported 16 `Empty` charts, zero
accepted one-LP rows, 2,013 ms total extractor wall time, and 119 ms median
per query on this host. The independent checker recomputed every `aG+bQ`
with its own integer group law and enumerated all 560 signed one-LP tuples
after extraction; each had zero witnesses. The frozen labels, per-query
statuses, and diagnostics are in `evidence/rr5-control-01/public-panel.json`.
These 16 fixed labels are a correctness and timing panel, not a statistical
yield estimate or a cold solve cost.

For independent uniform public queries on this fixed selector, the exact
expected **witness tuple** count is `560/n = 2.607698365e-7` per draw. The
16-draw expectation is `4.172317384e-6`, consistent with the observed zero.
Even if each witness produced a distinct accepted partial, and the single LP
column were occupied, a full `B+1=8` factor-base-rank route would require at
least `E=9` accepted partials. The first integer number of draws whose
**expected witness count** reaches nine is 34,513,195, about 594 times the
unfolded birthday-rho step expectation before algebraic extraction, setup,
graph filtering, and linear algebra. This is a derived gate on this
`B=7,S=1` producer size, not a lower bound on every target-recovery path or
on larger structured selectors. It makes increasing algebraic support while
controlling extractor cost the next necessary experiment.

### Nine-point cofactor boundary

The next combined selector, `B=8,S=1`, has 896 signed one-LP tuples. The
fixed positive query has one independently enumerated witness. The existing
cofactor presentation was run under a 30-second deadline, 20,000 construction
terms, 4,096 assignment limit, 100,000-pair cap, two-million symbolic-term
cap, and successively 2, 4, and 8 million matrix-entry caps. All three
attempts returned `ResourceLimit`, specifically matrix-entry admission just
above each cap, and zero extracted rows. Their raw outputs are retained as
`wide9-cap{2,4,8}m.json`; each represents an incomplete extraction, not an
empty chart. The independent verifier confirms that every attempt used the
same curve, selector, and target with one actual one-LP witness. The 2m and
4m arms are development cap probes; the final 8m arm is the source-pinned
boundary for this native control. This shows a resource bottleneck at nine
combined points under these limits. It gives no cost or completeness result
for a different structured selector or a higher admitted resource cap.

### Separate same-field curve: structured B677 graph partition

The inherited RR5 study also fixes a **different ordinary elliptic curve**
over the same `p=2^31-1`:
`y²=x³+847514846x+2087444724`, prime order `2147446831`, generator
`(625037715,2100627251)`. Its public hybrid selector has 677 distinct
abscissae. `b677_partition_gate.py` independently recomputes the source
partition: 509 Dickson points as factor-base vertices and 168 Y7 points as
external large-prime vertices. It replays the test-side two-base/two-LP
positive equation with its own integer group law. This second curve cannot
be mixed into the registered `-3x+21` workload or its same-target comparison.

For a uniform query on this second curve, the exact expected signed **witness
tuple** counts per draw are:

| Factor-base leaves | LP leaves | Expected witnesses per query |
| ---: | ---: | ---: |
| 4 | 0 | 20.593316 |
| 3 | 1 | 27.349227 |
| 2 | 2 | 13.512784 |
| 1 | 3 | 2.943730 |
| 0 | 4 | 0.238564 |

Each entry is `16*C(509,k)*C(168,4-k)/2147446831`; the exact integer
numerators and common denominator are in
`evidence/rr5-b677-prerequisite-01/partition-gate.json`. Four-base rows are
direct. One-LP rows attach an external vertex to the graph root; two-LP rows
are signed edges between external vertices. The existing gain-aware DSU and
rank machinery can eliminate LP columns after exact group replay. The three-
and four-LP families need a hypergraph or other reduction and are excluded
from this graph cost case.

The source-bound `rr5-b677-gate --case positive --presentation structured`
was executed with its frozen 30-second and two-million matrix-entry limits.
It returned `ResourceLimit`: the next matrix admission needed 2,000,016
entries. A follow-up with the same curve, fixture, target, and symbolic caps
raised only matrix admission to eight million; it again returned
`ResourceLimit`, now requiring 8,000,881 entries. A final bounded 32-million
entry arm reached the same `symbolic_workspace` admission stop at 32,017,996
entries. Both larger-cap traces record one F4 run, zero visited solutions,
and incomplete traversal. All three runs extracted zero rows. The original
gate had zero adapter rank because no row reached it,
although the independent grader supplies a valid two-LP witness.
`structured-positive.stdout`, `structured-cap{8,32}m.json`, their stderr, and
source hashes preserve these incomplete attempts. The high tuple counts
therefore establish a **potential relation
population**, not an achieved relation stream or a cost advantage. The next
falsifier is a completed source-exact B677 extraction on public `aG+bQ`
queries with signed-row replay, occupied-LP rank, factor-base projection,
target scalar recovery, and a cold same-target rho comparison. Graph
algorithm choices become measurable only after that stream exists.

### Generic signed four-sum end-to-end control on the B677 curve

`native/mitm_four.rs` supplies a separate, point-keyed meet-in-the-middle
relation source for the same B677 ordinary elliptic curve. It enumerates
915,304 signed pairs from the fixed 677-point public support, then probes
the complement of each pair for every public `R=aG+bQ`. The public target is
`Q=(976979610,1228979761)`; the checker-side validation scalar is absent
from the native solver. Every emitted four-point equation is checked by group
arithmetic before entering the graph consumer. The consumer retains the
4FB, 3FB+1LP and 2FB+2LP families, uses exact modular LP pivot elimination,
then solves the projected factor-base and target equations. This is a generic
source control, not RR5 algebraic extraction.

The frozen 40-query cap run stopped after 11 queries when it recovered
`d=987654321` and verified `dG=Q`. It emitted 707 distinct equations, of
which 669 have at least two factor-base leaves. The five class counts by
factor-base leaves 0 through 4 are `[3,35,154,294,221]`. LP rank is 162;
507 projected rows have factor-base-plus-target rank 506. An independent
Python group-law checker replayed all 11 public query labels and all 707
signed equations from the stored receipt. Its separate FLINT RREF of the
669 graph-eligible rows found augmented rank 668, zero contradictions and
the target-only row `d=987654321`. The native LP plus projection ranks sum
to the same 668. Source, receipt and verifier are in
`evidence/mitm-b677-control-01/` and `verify_mitm_graph.py`.

The run charged pair-table construction and 10,068,344 complement probes;
the native operation counter reports 10,987,529 group additions and
703,155,831 field multiplications overall. Cold wall time was 3,279 ms on
this development host. The 10-query arm produced 645 equations but no
target-only row and remains a censored arm. The one-query receipt predates
the final source refactor and is a development record, not a source-bound
benchmark. A same-target tuned negation-rho measurement and repeat timing
are required for a comparative performance result. The algebraic RR5 B677
arms above remain incomplete; this generic-source solve does not establish
that their high witness count is extractable at useful cost.

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

The uniform-residual screen and the separate generic signed four-sum control
have distinct source and cost claims. Neither establishes an algebraic
index-calculus advantage over matched rho.
