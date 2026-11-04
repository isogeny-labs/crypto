#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Round 5: solver side — requirements, barriers, and three
pre-registered solver/coupling tests
<round-5-solver-side--requirements-barriers-and-three-pre-registered-solvercoupling-tests>
2026-11-04 · follow-up to
#link("../round4/README.md")[`../round4/README.md`] (\#3387) ·
predictions in `.resume/RESUME.md` (commits `72dde4fd1e`, H; then L and
Y), written before their runs · analysis
#link("SOLVERS.md")[`SOLVERS.md`] · generator
#link("../gen.sing")[`../gen.sing`] (new arms L and Y) · input hashes
#link("SYSTEMS.sha256")[`SYSTEMS.sha256`] · logs #link("logs/")[`logs/`]
· records #link("rows.tsv")[`rows.tsv`] · graph
#link("round5.svg")[`round5.svg`] · PDF
#link("report.pdf")[`report.pdf`]

#strong[Class: derived bounds plus stage diagnostics.] No pipeline is
priced and no leaderboard row changes. Engine: msolve 0.6.5, p \=
786433, target k \= 0.

== Result
<result>
The question was whether a better #emph[solver], rather than a better
encoding, could close the gap. None of the options does at 256 bits.
`SOLVERS.md` has the derivations.

+ #strong[Faster Macaulay is not enough (derived).]
  - Beating rho at 2^256 needs ω ≲ 1.3 #strong[and] a solving degree
    flat in the tower depth.
  - With the measured depth slope, even ω \= 1 gives a ratio of 1.35.
  - The recorded matrices have 50–1,464 nonzeros per row, so sparse
    (block Wiedemann) elimination has an effective exponent of
    2.41–2.72. Sparsity does not supply the missing factor.
+ #strong[Enumeration-type solvers cannot beat √p (proved counting
  bound).] This covers SAT/CDCL branching on separators,
  meet-in-the-middle, dynamic programming over the tree, and exact
  tensor networks.
  - The root\'s two partial-sum sets satisfy |A|·|B| ≳ p, so one of them
    has at least √p elements.
  - Batching across targets does not help.
  - The coupling\'s incidence matrix has rank at least |A|/2, so exact
    tensor contraction cannot compress the bond.
+ #strong[Geometric resolution (Kronecker) has intermediate degree ≳
  p^{1−1/m}] (heuristic count).
+ #strong[Test H, branching on separator characters: a measured no-op.]
  - At m3 t4 both branches reproduce T\'s matrices exactly (D \= 6,
    widest 7,077, in 3.6–3.7 s), each with quotient dimension 6.
  - Over F̄\_p every X is c·z², so characters, power residues and
    \"bits\" carry no algebraic information. Making F\_p-rationality
    algebraic takes degree p.
+ #strong[Test L, fewer couplings with a fat leaf (one symmetric triple
  plus one leaf, 1 separator): prediction falsified.]
  - D\_L ≥ 8 at t \= 4 (cut off at 1,800 s in degree 8, widest ≥ 79,300)
    and at t \= 5 (memory cap at 782 s in degree 8), against T\'s D \=
    7.
  - The triple\'s node equation S₄(e; X₁) has total degree 8. The
    biquadratic S₃ node (degree 4) is the minimum, and the pair tree
    already uses it everywhere.
+ #strong[Test Y, a coupling that is a function rather than a 2-to-2
  correspondence (y-coordinates and the chord slope): no gain, and worse
  in t.]
  - Unsaturated, it picks up 2,060 solutions over F̄\_p at t \= 4 (D \=
    7, 30 s), against T\'s 3. Almost all come from the degenerate chart
    x₁ \= x₂, where λ·0 \= 0 leaves the slope unconstrained.
  - With the chart removed by v·(w₂ − w₁) \= 1 (arm Yv), t \= 4 gives D
    \= 6, the same as T. It is 5.4× wider (38,133 columns), 20× slower
    (76 s), and has quotient dimension 12, which is 3 × 2 orderings × 2
    signs.
  - At t \= 5, Yv selects degree 7 and is cut off at 1,800 s, while T
    has D \= 6 in 512 s. The function coupling therefore has the
    #emph[steeper] t-slope.

#strong[Reading.] Of the solver families, only Gröbner bases compress,
because their D stays far below the separators\' domain sizes. Their
cost is set by two slopes: about +1.5 per summand and about +0.5 per
tower level, both coming from the couplings. Encoding the couplings
differently (lifting, eliminating, fattening, function form), branching
on cheap separator properties, and speeding up the linear algebra each
leave the slopes unchanged or make them worse. An improvement on this
line has to come from a node relation of degree below 4, which x-only
coordinates cannot give, or from outside the summation-polynomial
framework.

#image("round5.svg")

== Table (recorded; `rows.tsv`)
<table-recorded-rowstsv>
#align(center)[#table(
  columns: 8,
  align: (col, row) => (auto,auto,right,right,right,right,right,auto,).at(col),
  inset: 6pt,
  [cell], [arm], [N], [D (≥: cut off)], [widest], [quotient dim], [wall
  s], [exit],
  [m3 t4],
  [T (round 1)],
  [13],
  [6],
  [7,077],
  [3],
  [3.7],
  [0],
  [m3 t4],
  [H true],
  [14],
  [6],
  [7,077],
  [6],
  [3.70],
  [0],
  [m3 t4],
  [H false],
  [14],
  [6],
  [7,077],
  [6],
  [3.58],
  [0],
  [m3 t5],
  [H true / false],
  [17],
  [6 (stopped)],
  [19,852],
  [—],
  [42],
  [143 stopped (redundant)],
  [m3 t4],
  [Y (unsaturated)],
  [16],
  [7],
  [57,153],
  [2,060],
  [29.8],
  [0],
  [m3 t5],
  [Y (unsaturated)],
  [19],
  [≥ 7],
  [≥ 204,643],
  [—],
  [1,800],
  [124],
  [m3 t4],
  [Yv],
  [17],
  [6],
  [38,133],
  [12],
  [75.8],
  [0],
  [m3 t5],
  [Yv],
  [20],
  [≥ 7],
  [≥ 78,989],
  [—],
  [1,800],
  [124],
  [m3 t5],
  [T (round 1)],
  [16],
  [6],
  [23,720],
  [3],
  [512–544],
  [0],
  [m4 t4],
  [L],
  [14],
  [≥ 8],
  [≥ 79,300],
  [—],
  [1,800],
  [124],
  [m4 t5],
  [L],
  [18],
  [≥ 8],
  [≥ 278,857],
  [—],
  [782],
  [139 memory (6 GiB)],
  [m4 t4],
  [T (round 1)],
  [18],
  [7],
  [25,742],
  [6],
  [697],
  [0],
)
]

Two of the arms were built by text edits on generated systems:

- #strong[H] appends z and X\_c − n·z².
- #strong[Yv] appends v and v·(w₂ − w₁) − 1.

Arms L and Y are generated by `gen.sing` with the planted check. The
appended H and Yv equations hold at the planted point by construction: n
was chosen from the planted X\'s character, and w₁ ≠ w₂.

== Caveats
<caveats>
- The H runs at m3 t5 and m4 t4 were stopped or not run once the t4
  result showed the construction adds no algebraic constraint.
- Wall times come from runs with up to 2 msolve jobs on 4 cores, and
  should be read only as an ordering.
- The cut-off cells are lower bounds.
#pagebreak()
= Round 5: the solver side — what a faster solver would have to do, and
why the obvious ones cannot
<round-5-the-solver-side--what-a-faster-solver-would-have-to-do-and-why-the-obvious-ones-cannot>
2026-11-04 · follow-up to
#link("../round4/README.md")[`../round4/README.md`] and
#link("../round4/CROSSOVER.md")[`../round4/CROSSOVER.md`]. Question:
rounds 1–4 show that no encoding change lowers the tree\'s cost. Is
there a solver whose cost is not the dense Macaulay size: SAT, faster or
sparse Macaulay linear algebra, tensor networks, geometric resolution,
hybrids? Tests H and L were pre-registered in `.resume/RESUME.md`
(commits `72dde4fd1e` and the next one) before their runs.
#strong[Class: derived bounds plus two stage diagnostics.] No pipeline
is priced.

== 1. What \"faster Macaulay\" would have to achieve (derived)
<1-what-faster-macaulay-would-have-to-achieve-derived>
This is the round-4 cost model, with the linear-algebra exponent ω swept
from the information floor (1, linear in matrix size) up. Cost is log₂
cost / log₂ √p at the best m; below 1 beats rho.

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,auto,auto,).at(col),
  inset: 6pt,
  [ω], [β \= 0 (D flat in t)], [β \= 0.5 (measured t-slope)],
  [1.0],
  [2^128: 1.14 · #strong[2^256: 0.86] · 2^512: 0.64],
  [2^256: 1.35 · 2^512: 1.06 · 2^1024: 0.82],
  [1.2],
  [2^256: 0.95],
  [2^256: 1.54],
  [1.5],
  [2^256: 1.04],
  [2^256: 1.84],
  [2.0],
  [2^256: 1.22],
  [2^256: 2.33],
)
]

At 256 bits, beating rho needs #strong[both] near-linear linear algebra
(ω ≲ 1.3) #strong[and] a solving degree flat in the tower depth (β ≈ 0).
With the measured depth slope, even ω \= 1, linear in the size of the
matrix, does not get there. Then the size of the matrix itself is the
obstacle, not the elimination.

#strong[Sparsity does not supply ω ≈ 1.] The recorded msolve matrices
are not sparse enough. Block Wiedemann costs about cols² · (nonzeros per
row), which gives an effective exponent log(cols²·w)/log(cols) between
2.41 and 2.72:

#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,right,right,right,).at(col),
  inset: 6pt,
  [matrix], [widest], [nonzeros per row], [exponent],
  [T at m4 t4],
  [12,782 × 25,742],
  [713],
  [2.65],
  [F at m4 t4],
  [15,086 × 13,503],
  [50],
  [2.41],
  [T at m3 t5],
  [26,344 × 23,720],
  [1,464],
  [2.72],
  [T at m5 t4],
  [372,489 × 654,028],
  [654],
  [2.48],
)
]

(These come from the logs\' density column, by awk.)

== 2. Three barriers for solvers that avoid Macaulay matrices
<2-three-barriers-for-solvers-that-avoid-macaulay-matrices>
#strong[(a) Enumeration-type solvers: SAT/CDCL branching,
meet-in-the-middle, dynamic programming over the tree, exact
tensor-network contraction.] #emph[Counting bound (proved).] In any
decomposition tree, the root\'s two children carry partial sums from
sets A and B. Covering a random target R requires R ∈ A + B, hence
|A|·|B| ≳ p up to the symmetry factor, and therefore max(|A|, |B|) ≳ √p.

A solver that decides a separator by its value has to range over at
least one of these sets. Branching on its bits, tabulating it, or
contracting a bond indexed by it all count. So it pays at least √p, the
same as rho, and batching over targets does not escape this. Here is the
batching calculation. A table of k-fold sums costs |V|^k. Each target
scans |V|^{m−k}. There are about |V|·p/|V|^m targets per |F| relations.
Keeping both terms below √p would require |V|^k ≤ √p and |V|^{k−1} ≥ √p,
which contradict each other. These solvers are generic in effect.

#emph[Rank version, for tensor networks.] The root coupling S₃(X₁, X₂,
x\_R) \= 0 is a 2-to-2 correspondence, a curve isomorphic to E. Its
incidence matrix on A × B has at most 2 ones in each row and column, so
its rank is at least |A|/2. No change of basis compresses the bond,
which settles exact contraction. Approximate low-rank contraction would
need a different argument.

#strong[(b) Geometric resolution, i.e. the Kronecker solver (cost ≈ δ²,
with δ the largest intermediate degree).] If the membership equations
come before the coupling, the variety cut out by all m membership
constraints already has |V|^m ≈ p points. In the other order, the
coupling variety with k leaves restricted has degree ≈ |V|^k · c, so δ ≳
p^{1−1/m} at the last step. Either way the cost is above p.
#emph[Heuristic count, with no ordering found that avoids it.]

#strong[(c) Character and rationality branching (test H, measured).] The
idea is to branch on a property of a separator cheaper than its value:
its quadratic character, imposed as X₁ \= n·z² with n ∈ {1, a
non-square}, one bit per branch.

- At m3 t4 both branches reproduce T exactly: D \= 6, widest 7,077, the
  same sum of rows×cols, in 3.6–3.7 s.
- Each branch has quotient dimension 6, against T\'s 3, because z ↦ −z.
- Gröbner bases count solutions over F̄\_p, where every X is c·z². So \"X
  ∈ F\_p is a square\" carries no algebraic information. Making
  F\_p-rationality algebraic takes degree p (z^p \= z).
- The same holds for every multiplicative character, power-residue
  class, or \"bit\" of a separator: algebraic solvers cannot use them.
- The m3 t5 branches tracked T\'s matrices exactly until they were
  stopped at 42 s. m4 t4 was not run.

== 3. The one structural lever left, and test L
<3-the-one-structural-lever-left-and-test-l>
Fixing the separators gives D \= 4 in every cell (rounds 2–3). So the
excess degree is the cost of the couplings. A natural solver-structure
question is whether #emph[fewer] couplings give a smaller excess.

Test L, the fat leaf, takes m \= 4 as one triple plus one leaf: the
triple is encoded symmetrically (S₄ in e₁, e₂, e₃ with output X₁),
giving 1 separator instead of the pair tree\'s 2.

- #strong[Prediction falsified:] D\_L ≥ 8 at t \= 4 and 5, against D\_T
  \= 7.
- #strong[Cause:] the triple\'s node equation has total degree 8,
  against 4 for the pair tree\'s S₃-type nodes, so the input degree
  alone gives D ≥ 8.
- #strong[Lesson:] the excess comes from the number of couplings times
  the degree of each node relation. S₃\'s biquadratic, degree 4, is the
  floor for an x-only addition relation, and the pair tree already uses
  it everywhere.
- The final L numbers are in `README.md`.

== 4. Where that leaves the solver side
<4-where-that-leaves-the-solver-side>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [solver family], [cost driver], [verdict at these laws],
  [Gröbner/F4, dense or sparse],
  [Macaulay size C(N + D, D), with N ≈ log₂ p],
  [asymptotically subexponential if D \= O(m), D flat in t and ω ≲ 1.3.
  Measured: the sparse exponent is 2.4–2.7 and the t-slope is about
  0.5],
  [SAT, MITM, DP, exact tensor networks],
  [separator domain size],
  [≥ √p (proved counting bound)],
  [geometric resolution (Kronecker)],
  [intermediate degree δ],
  [≳ p^{1−1/m} (heuristic)],
  [hybrid on separator characters],
  [none: invisible over F̄\_p],
  [measured no-op],
)
]

#strong[Reading.] At 256 bits every solver family is held at or above
rho by a different mechanism. Compression is possible only in Gröbner,
where it shows up as D being far below the separators\' domain sizes.
Its price is the Macaulay size, which grows because each added summand
adds about 1.5 to D and each tower level about 0.5.

So a real improvement has to change those two slopes. The candidates
are:

- a node relation of degree \< 4, which x-only coordinates cannot give;
- a coupling that is a function rather than a 2-to-2 correspondence
  (y-coordinate or slope encodings). Test Y measured it afterwards: same
  D at t \= 4, a steeper t-slope, and 5–20× the cost (`README.md`);
- leaving the summation-polynomial framework altogether.

No idea tried in rounds 1–5 changes either slope.
