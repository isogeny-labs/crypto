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

### Exact selector-separator prerequisite

`selector_split_gate.py` reconstructs the public degree-1024 Dickson
selector `D(X)` and degree-168 Y7 selector `Y(X)` from the frozen fixture.
FLINT computes `gcd(D,Y)=1` over this field. For the grader-side positive
quartic `U`, it independently checks `U=U_D*U_Y`, where `U_D=gcd(U,D)` and
`U_Y=gcd(U,Y)` have degrees two and two. This is a structural certificate,
not an extraction of an unknown public-query relation.

The separate `verify_selector_admission.py` checks the finite-field
hypotheses used by the four-leaf anti-Frobenius argument. It enumerates
all 1,024 Dickson and 168 Y7 roots with FLINT, checks monicity,
squarefreeness, complete splitting and coprimality, and independently
tests curve liftability. Exactly 509 Dickson roots and all 168 Y7 roots
lift; their union is exactly the frozen 677-point base, with 515
nonliftable Dickson roots and no zero-ordinate roots. It also verifies
the full odd curve order from the prime generator order and Hasse bound,
and checks that both the positive target and public `Q` lie outside the
selector. The deterministic result and three-file source manifest are
`selector-admission-20261124.json` and
`selector-admission-sources.sha256` in the same evidence directory.
These are admission checks, not a solver or cost result.

The coprime factorization gives an exact branch decomposition for any
squarefree quartic with `U | D*Y`: its Dickson and Y7 factors have degrees
`(4,0)`, `(3,1)`, `(2,2)`, `(1,3)`, or `(0,4)`. The first three can feed the
existing direct/one-LP/two-LP graph consumer. The current 122-variable
selector circuit multiplies the two branches in one final modular gate;
the branch split is a sound way to test a smaller algebraic system before
increasing the combined F4 matrix cap again. The next implementation gate
must construct and solve each relevant branch from the public query alone,
without receiving the grader's factorization, then check every signed row
and charge branch setup, algebraic solving, graph work and failures. If the
split systems still exceed the resource cap or their fully charged cost
cannot approach matched rho, the branch split is rejected. The certificate
and source hash are in `evidence/rr5-b677-prerequisite-01/selector-split-gate.json`.

### Bounded algebraic 2+2 split admission

`split_selector.rs` implements the exact two-Dickson/two-Y7 branch using
unknown monic quadratic factors `U_D` and `U_Y`. Four equations enforce
`U=U_D U_Y` for the public RR5 quartic; separate modular circuits enforce
`D mod U_D=0` and `Y mod U_Y=0`. The system has 61 variables, 62 input
equations, and 250 input terms. A grader-side assignment for the frozen
positive relation satisfies every equation, including all 18 quotient and
remainder gates. The candidate API receives only the public query and
fixture; signed roots and point sums remain checked after algebraic solving.

`native/split_gate.rs` ran a frozen positive query and the separate public
`Q` under a 30-second deadline. At two million matrix entries, 8,192
symbolic monomials and one critical pair per step, both returned
`ResourceLimit` at `basis_update` (observed 8,202 monomials), with zero
visited solutions and zero extracted relations. Four pairs per step instead
hit `symbolic_workspace` (2,006,253 versus two million matrix entries).
Increasing the monomial cap to 16,384 and 32,768 with one pair advanced
the positive run but stopped at observed 16,430 and 32,819 monomials.
At 65,536 monomials, the positive arm hit the two-million matrix cap after
5,402 ms; raising only that cap to eight million then hit 65,540 monomials
after 13,986 ms. These are incomplete, source-bound attempts, not empty
charts. `verify_split22_gate.py` independently replays the grader-side
positive point equation and checks that every promoted arm retained its
exact curve, fixture, limits, and incomplete status. The summary and source
hashes are in `evidence/rr5-b677-prerequisite-01/`. Earlier cap-only runs
in that directory predate the final limit CLI and remain development
receipts, outside the promoted comparison.

Post-merge source qualification (2026-11-24): mainline formatting and
Clippy annotations changed the whole-file hashes of `ic_boundary.rs`
and `prime_fast.rs`, so the isolated F4 source build correctly refused
its previous pins. After reviewing those diffs, `f4-source/build.rs`
pins their current hashes. The copied `f4_fp.rs` also has an equivalent
`rows % 64 != 0` to `!rows.is_multiple_of(64)` rewrite. With Rust 1.98,
the focused planted 2+2 gate test and all 33 isolated library tests pass
against the merged sources. This qualifies the current positive gate
construction; the seven timed split receipts retain their original
`5096df8283bd1a6991d1119ab23b58f2d535d98c` source identity and
have not been rerun on the new closure.

The branch split reduces the algebraic input size but has not produced a
public-query row. Critical-pair batching moves the admission bottleneck;
it does not yet reduce extraction cost enough to support an index-calculus
advantage. The next falsifier is a source-exact pair-selection or symbolic
closure strategy that completes the positive branch and at least one
unplanted public query under a frozen resource budget, with every candidate
relation independently replayed. Only then can graph rank and a cold
same-target cost be measured for this algebraic source.

A conditional cold-cost screen sharpens that gate. The matched rho run
below took 25.411 ms for this same public `Q`; an eleven-query algebraic
campaign with equal-cost calls would have at most 2.310 ms per query even
if selector setup, graph work and linear algebra cost zero. The promoted
public-`Q` split arm at the two-million matrix and 8,192-monomial caps
already spent 241 ms before returning `ResourceLimit` with no row, about
9.48 times that rho wall time. Its completed extraction cost is unknown
and can only exceed this recorded stop time for the exact execution.
These are unisolated, separate-run wall observations and a conditional
eleven-query budget, not a measured algebraic-source speed ratio or a
bound on a different solver, source yield, or query count.

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

The source-bound `forty-cap-full-cost.json` run stopped after 11 queries when it recovered
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
the solver counter reports 10,987,529 group additions and 703,155,831 field
multiplications. Frozen support construction and validation added 222 group
additions, 136,795 field multiplications and 116 field inversions, for
10,987,751 total counted group additions. Cold wall time was 3,367 ms on
this development host. The earlier `forty-cap.json` is a pre-accounting
development receipt with the same relation rows and rank. The 10-query arm produced 645 equations but no
target-only row and remains a censored arm. The one-query receipt predates
the final source refactor and is a development record, not a source-bound
benchmark. The algebraic RR5 B677 arms above remain incomplete; this generic-source solve does not establish
that their high witness count is extractable at useful cost.

The same full-cost receipt records 336 ms for pair-table construction,
3,006 ms summed over the 11 relation-source calls, and 13 ms for their
combined graph elimination and replay stage. These sum to 3,355 ms;
12 ms of the 3,367 ms cold wall interval is unallocated by those
millisecond-resolution fields. Table plus source occupy 99.26% of the
reported interval. Holding the observed source, setup and stopping point
fixed, even replacing the recorded graph-and-replay stage with zero work
would change this interval by at most `3367/(3367-13)=1.00388` times.
This is a conditional phase budget for the generic pair-table source,
not an upper bound on a different algebraic relation source or a
separately instrumented graph implementation. It makes faster relation
production the first cost gate for this architecture.

`native/rho_control.rs` supplies an **independent unfolded** 16-jump Brent
rho control on exactly the same curve, generator, public `Q`, and native
`Arithmetic` backend. It uses no solver-side validation scalar and charges
jump-table setup, the walk, failed attempts, and in-solver scalar replay.
The first fixed seed recovered `987654321` after 98,366 steps and 99,970
charged group additions, with 25.411 ms total cold wall time; the repeat
was 24.187 ms. The repeat graph solve produced the same 707 relations and
rank, took 3,215 ms cold wall time, and charged the same 10,987,529 group
additions. On the same development host, `/usr/bin/time -l` measured 3.25 s
real, 3.15 s user and 119,259,136 bytes maximum RSS for the graph binary,
versus 0.03 s real, 0.02 s user and 2,129,920 bytes maximum RSS for the rho
binary. The charged group-addition ratio including fixture validation is 109.9, and the paired native
cold-wall ratio is 132.9. These are two deterministic same-target runs on
an unisolated host, not a cross-seed distribution or a comparison with the
repository's tuned WZ negation walk. Their large separation is a measured
boundary for this generic four-sum source. The frozen Y7 point CSV was
generated by an earlier study; regenerating that CSV is outside the cold
solver timer. Since this source already loses with that precomputation
credited for free, charging its creation cannot reverse this comparison.

The measured pair-table setup alone took 336 ms, or 13.22 times the
25.411 ms same-target rho wall interval. Holding that setup fixed and
optimistically assigning zero time to every later relation, graph and
replay step still cannot cross the observed rho interval. This is a
floor for this recorded pair-table setup on this host and target, not
for a different relation source or a redesigned setup. A candidate
that retains the table must price setup changes as well as faster
relation calls before claiming a same-target gain.

The first local negation-map attempt used an x-only jump partition and fell
into short fruitless cycles on all three fixed seeds (33, 5 and 33 steps).
The failed receipt and its exact source snapshot are retained as
`negation-fruitless.json` and `.rs`; these are failed attempts, not a rho
baseline. The root-library tuned WZ example could not be built in this
checkout because unrelated `prime_root_trace` name errors and unstable
`atomic_try_update` calls fail compilation. The independent unfolded control
does not replace a tuned same-target baseline. All source, raw run and
`time -l` records are in `evidence/mitm-b677-control-01/`.

### Same-stream graph ordering selection

`compare_graph_orders.py` replays the same 669 point-checked eligible rows
from `forty-cap-full-cost.json` and changes only the LP pivot admission
order. The graph has 168 LP vertices plus a root, 448 one- and two-LP
edges, and an exact LP rank of 162. It compares stream order, BFS forest,
a weighted minimum spanning forest, and a weighted shortest-path forest.
The last is equivalent to independent uncapacitated unit-demand min-cost
flows from each component root, with edge cost equal to the number of
factor-base leaves in that partial row. Each ordering is followed by exact
modular LP elimination and independent FLINT RREF of the projected rows.

| Pivot order | Projected nonzeros | Modular updates | Largest projected row |
| --- | ---: | ---: | ---: |
| Stream | 4,276 | 3,996 | 22 |
| BFS forest | 3,924 | 2,734 | 14 |
| Weighted minimum spanning forest | 7,194 | 14,800 | 39 |
| Unit min-cost paths | **3,919** | **2,722** | **14** |

Every arm has 507 projected rows, augmented rank 506, and the same
target-only row `d=987654321`. The selected min-cost-path order reduced
projected nonzeros by 8.35% and modular updates by 31.88% relative to stream
order. BFS is within five nonzeros of the selected order. These are
deterministic component metrics on this fixed relation stream; the script's
one-run Python wall timings are not comparative native solver timings.
The relation source, which accounts for millions of additions, is unchanged.
Maximum independent set and spectral partitioning have no measured benefit
on this stream yet, so neither is promoted into the native pipeline.
`graph-order-comparison.json` records the source receipt hash and all arms.

For this exact eleven-query source-bound solve, pair construction and
complement probing alone charge `915,304*(1+11)=10,983,648`
group-addition-scale operations. That is 109.87 times the matched rho
control's 99,970 charged group additions before any graph or final linear
algebra work. Even a zero-cost graph consumer cannot reverse this
group-operation comparison while retaining this recorded relation source
and query stream. This is a stage-cost floor for the fixed run, not a
wall-time theorem, an early-stop bound for other streams, or a bound on
an algebraic relation producer.

### All-base necessity control for the generic source

`all_base_control.py` independently replays the same frozen signed equations
and treats all 677 support points as factor-base columns, with no LP graph.
At ten queries, its 645 rows have augmented rank 645 and no target-only row.
At eleven queries, its 707 rows have augmented rank 672 and the target-only
row `d=987654321`, verified as `dG=Q`. The 509-base/168-LP graph consumer
uses only 669 of those rows and has augmented rank 668. Thus the LP graph
was **not necessary** for this generic-source target solve. The graph-order
comparison above measures a valid projection component, but cannot be used
as evidence that graph processing enabled the recovered scalar. The exact
source hash and both all-base ranks are in `evidence/mitm-b677-control-01/
all-base-control.json`.

For every fixed support of 677 distinct abscissae, each signed four-point
tuple sums to a fixed group element. A uniform `R=aG+bQ` equals that sum
with probability `1/n`, so the expected signed witness count is exactly
`16*C(677,4)/n = 64.6376197055` per query on this curve, independent of
the support geometry or any graph partition. A graph algorithm can change
duplicate handling, extraction cost, and useful rank on a finite stream;
it cannot increase this generic source's expected witness count solely by
rearranging the same 677 points. Consequently, MIS or spectral support
selection requires a measured rank or cost gain on a held-out public-query
panel, with selection work charged, before it becomes a candidate pipeline
component. The algebraic RR5 source remains the route to test for a
non-generic relation advantage.

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
