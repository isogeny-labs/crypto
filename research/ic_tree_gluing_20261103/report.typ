#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)

= Tree-glued, cherry-symmetric PDP systems: F4 solving degree against
the caterpillar and full-symmetric arms
<tree-glued-cherry-symmetric-pdp-systems-f4-solving-degree-against-the-caterpillar-and-full-symmetric-arms>
2026-11-03 · theory and proposal in #link("THEORY.md")[`THEORY.md`] ·
generator #link("gen.sing")[`gen.sing`] (Singular 4.3.2, native) with
#link("instances.txt")[`instances.txt`] · equivalence check
#link("compare.sing")[`compare.sing`] · runner #link("run.sh")[`run.sh`]
(msolve 0.6.5) · parser #link("parse.sh")[`parse.sh`] · records
#link("rows.tsv")[`rows.tsv`], #link("logs/")[`logs/`] (sha256 in
`logs/SHA256SUMS`) · graph #link("d_vs_t.svg")[`d_vs_t.svg`]
(#link("plot.py")[`plot.py`], recorded values only) · PDF
#link("report.pdf")[`report.pdf`]

#strong[Class: stage diagnostic of the solver axis (AGENTS.md §2). No
pipeline is priced, and there is no `S` value, rho comparison or
leaderboard row.] It measures one new encoding of the prime-field PDP,
motivated by phylogenetic tree models (THEORY.md §3), against the lab\'s
two existing encodings, on identical targets.

== Result
<result>
The tree-glued cherry-symmetric encoding #strong[T] is new to the lab.
Every summand pair (\"cherry\") is encoded by its symmetric functions
(s, q) with a degree-2 remainder tower, and cherries are glued by S₃.
All inputs have degree ≤ 4 for every arity m.

- #strong[Against the plain chain C] (same variable count, same
  targets), T is better in every completed cell:
  - #strong[m \= 2:] D stays at #strong[4 for t \= 3..7], while C rises
    to 5 at t \= 6, 7. T\'s matrices are 1.8–3.6× narrower.
  - #strong[m \= 3, t \= 5:] T finishes at #strong[D \= 6]. C selects
    degree 7 and is cut off at 600 s with a matrix ≥ 3.4× wider.
  - #strong[m \= 4, t \= 4:] T finishes at #strong[D \= 7] (697 s). C is
    cut off at 600 s in degree 7 with a matrix ≥ 3.9× wider than T\'s
    widest.
  - The pre-registered kill criterion (T\'s D ≥ C\'s D in every
    comparable cell) is #strong[not met].
- #strong[Against the full-symmetric arm F] (S\_{m+1} in e₁..e\_m, the
  lab\'s symmetric PKM):
  - #strong[m \= 3:] F is better: D 5 vs 6 at t \= 4, and 2–6× narrower
    matrices. Both arms reach D \= 6 at t \= 5.
  - #strong[m \= 4, t \= 4:] T\'s D \= 7 is #strong[one below] the D \=
    8 the lab recorded for F (`research/pkm_symmetric_lead1_20261026`, N
    \= 16). But F was 59× faster there (11.8 s vs 697 s) and its matrix
    was narrower (13,926 vs 25,742 columns). That F row comes from a
    different engine (`f4_fp`), so only its D is comparable here, and
    even that loosely.
- #strong[The t-growth is delayed, not removed.] At m \= 3, t \= 6, T
  selects degree 7 before the 600 s cut-off. Lead 1\'s question (is D
  bounded in t?) stays open for T, as it does for F.

#strong[Reading.] T dominates the plain chain at every measured cell and
gives the first measured cell with a lower solving degree than full
symmetrisation (m \= 4). It does not beat F\'s cost at the sizes
measured. Its case is asymptotic: F\'s input degree is 2^{m−1}, while
T\'s is bounded. THEORY.md §3 shows that an L(1/2) cost needs D \= O(m)
and D nearly flat in t, and only a bounded-input-degree encoding can
meet the first condition. Nothing here shows that T meets the second.

#image("d_vs_t.svg")

== Tables (recorded values, `rows.tsv`)
<tables-recorded-values-rowstsv>
D is the largest selected F4 degree. \"≥\" marks a run stopped by the
600 s timeout (exit 124); for those runs, D is the degree being reduced
when it was cut off, and the width is the widest matrix reached so far.
N is the number of unknowns. Both targets gave identical D and widths in
every cell where both ran. The two m \= 2 C targets differ in sum
rows·cols by \< 0.1 %.

#align(center)[#table(
  columns: 10,
  align: (col, row) => (right,right,right,right,right,right,right,right,right,auto,).at(col),
  inset: 6pt,
  [m], [t], [N (C, T / F)], [D: C], [D: T], [D: F], [widest: C],
  [widest: T], [widest: F], [quotient dim C / T / F],
  [2],
  [3],
  [6],
  [4],
  [4],
  [\= T],
  [126],
  [47],
  [],
  [2 / 1],
  [2],
  [4],
  [8],
  [4],
  [4],
  [],
  [276],
  [155],
  [],
  [2 / 1],
  [2],
  [5],
  [10],
  [4],
  [4],
  [],
  [597],
  [327],
  [],
  [2 / 1],
  [2],
  [6],
  [12],
  [#strong[5]],
  [#strong[4]],
  [],
  [2,056],
  [627],
  [],
  [2 / 1],
  [2],
  [7],
  [14],
  [#strong[5]],
  [#strong[4]],
  [],
  [4,317],
  [1,193],
  [],
  [2 / 1],
  [3],
  [4],
  [13 / 9],
  [6],
  [6],
  [#strong[5]],
  [12,279],
  [7,077],
  [1,224],
  [6 / 3 / 1],
  [3],
  [5],
  [16 / 12],
  [#strong[≥ 7]],
  [#strong[6]],
  [6],
  [≥ 81,920],
  [23,720],
  [10,493],
  [— / 3 / 1],
  [3],
  [6],
  [19 / 15],
  [not run],
  [≥ 7],
  [≥ 6],
  [],
  [≥ 57,152],
  [≥ 32,678],
  [],
  [4],
  [4],
  [18],
  [≥ 7],
  [#strong[7] (697 s run)],
  [lab f4\_fp: 8],
  [≥ 102,212],
  [25,742],
  [lab: 13,926],
  [— / 6 / —],
)
]

The quotient dimensions equal the orbit sizes of the planted relation:
2! \= 2, 3! \= 6 and 4!/4 \= 6 copies, against 1 for T at m \= 2 and for
F. This shows the systems have exactly the planted solution and its
relabellings.

Times are contended: up to 5 single-thread jobs shared 4 cores, plus a
helper agent. Use them as an ordering only (AGENTS.md: wall time is
evidence only from uncontended runs). The 600 s cut-offs are wall-clock,
so a contended C or T run could have been cut off earlier than it would
have been uncontended. This affects which cells finished, not the D of
any completed run. The m \= 4 T run with a 2,400 s budget ran with at
most two other jobs.

== Checks and controls
<checks-and-controls>
- #strong[Planted check.] Every system is evaluated inside Singular on a
  planted decomposition before it is written, and the generator aborts
  on any nonzero equation. The decomposition uses m distinct points of
  E(F\_p) with x ∈ V, random signs, x\_R computed by the group law, and
  partial sums from the actual points. The curve is y² \= x³ + ax + b
  over F\_786433 with (a, b, g) in `instances.txt`.
- #strong[Cross-instrument positive control.] msolve on this generator
  reproduces the lab\'s recorded `f4_fp` values from
  `research/pkm_symmetric_tower_20261026`:
  - plain m \= 2: D 4, 5, 5 at N \= 10, 12, 14;
  - symmetric m \= 3: D 5 at t \= 4 and 6 at t \= 5;
  - symmetric m \= 3, t \= 4: widest matrix #strong[1,224 columns,
    identical].
- #strong[Generator equivalence.] `gen.sing` was ported from a Python
  pilot that was kept outside the repository (AGENTS.md: no Python
  instruments). Its 60 outputs are byte-identical to the pilot\'s.
  `compare.sing` checked every polynomial as well: 60 files, 0
  mismatches; a deliberately perturbed copy was caught.
- #strong[Parser.] `parse.sh` (awk) agreed with the pilot parser on all
  30 logs it was compared on.

== What did not run, and why
<what-did-not-run-and-why>
- #strong[C at m \= 3, t \= 6–7, and T and C at m \= 3, t \= 7:] after C
  was cut off at t \= 5 these were not started. The main queue was
  stopped to free cores for m \= 4.
- #strong[m \= 4 second target:] C target 1 was killed by the operator
  at about 300 s, while it was reducing degree 7, to free a core. The
  partial log is kept as `logs/run600/m4_t4_k1_C.killed.log` and is not
  used in the table. T target 1 at m \= 4 was not run.
- #strong[m \= 4, t ≥ 5], and F at m \= 4 with this generator (S₅(e) is
  not implemented in `gen.sing`). The m \= 4 F reference is the lab\'s
  recorded row.
- #strong[The S3 resume checkpoint] (`scripts/checkpoint.sh push`) could
  not upload: the container has no AWS profile and no upload token.
  `.resume/RESUME.md` was written locally before the runs and holds the
  pre-registered prediction and kill criterion.
- #strong[Isolab-EKS.] This is a deterministic algebraic diagnostic
  (degrees, matrix widths, quotient dimensions), and it ran locally. No
  timing claim is made.

== Next measurements
<next-measurements>
+ #strong[F at m \= 4 and 5 against T on one engine] (msolve), so the m
  \= 4 D comparison is not cross-instrument. This is the decisive test
  of the asymptotic argument: does D\_F − D\_T grow with m?
+ #strong[T on a larger host at m \= 3, t \= 6–7 and m \= 4, t \= 5], to
  measure T\'s t-slope directly.
+ #strong[Separator test of Conjecture B] (THEORY.md §2): in T, replace
  one internal S₃ by a planted known X and measure how much D drops.
+ #strong[Wreath-complete T:] also quotient the root swap at m \= 4 (S₂
  ≀ S₂ \= D₄, leaving 3 copies instead of 6).
#pagebreak()
= Two cross-field connections for prime-field index calculus
<two-cross-field-connections-for-prime-field-index-calculus>
2026-11-03 · companion to #link("README.md")[`README.md`] (the
measurement). #strong[Class: theory note and proposal. No algorithm is
priced, and no `S` value or leaderboard row is produced.] The proofs
here are sketches unless they are marked \"proved\". Statements marked
\"heuristic\" are not established.

The lab\'s prime-field IC work has tried every factor-base symmetry
group available to a single instance:

#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [Symmetry], [Lab result],
  [S\_m on the summands],
  [symmetric Semaev, symmetric PKM, Graeffe and remainder towers],
  [Negation],
  [used throughout],
  [2-torsion translations],
  [FGHR D\_m, Montgomery T₀],
  [μ₃ / μ₄ automorphisms],
  [GLV/j\=0 orbit bases],
  [Frobenius],
  [binary and extension fields only],
  [Isogeny kernels],
  [generic-group null theorem],
)
]

The tower presentations tried are the Kummer (μ\_{2^t}), Dickson/torus
and isogeny (Lattès) towers. This note asks two questions:

+ Why does no factor-base structure lower the elliptic degree?
+ Which symmetry is left that keeps every input degree bounded as the
  arity m grows?

The connections come from complex dynamics and incidence combinatorics
(question 1) and from phylogenetic algebraic statistics (question 2). A
grep of both repositories for Elekes, Ritt, \"toric fiber product\" and
phylogenetics finds no hits.

== 1. The Ritt dichotomy: the three PKM tower families are Ritt\'s three
families
<1-the-ritt-dichotomy-the-three-pkm-tower-families-are-ritts-three-families>
#strong[Ritt\'s theorem (1923) on permutable maps.] Let f and g be
polynomials (or rational maps) of degree ≥ 2 that commute and share no
common iterate. Then, up to a common conjugacy, each falls in one of
three families:

- power maps z ↦ z^n;
- Chebyshev/Dickson maps T\_n;
- Lattès maps, the maps induced on P¹ \= E/±1 by an endomorphism of an
  elliptic curve E.

#strong[Observation (proved; elementary).] The lab\'s three tower
families are exactly these three families, each attached to a
one-dimensional algebraic group:

#align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [lab tower], [self-map], [algebraic group it is an endomorphism of],
  [Kummer, V \= g·μ\_{2^t}],
  [w ↦ w²],
  [G\_m (the smooth locus of the #strong[nodal] cubic)],
  [Dickson / norm-one torus],
  [x ↦ x² − 2],
  [G\_m/±1, i.e. the Kummer line of the nodal cubic],
  [isogeny / Lattès tower],
  [x ↦ x(\[2\]P) or x(φ(P))],
  [E/±1 itself],
)
]

Only the third family is compatible with the elliptic group law. Each
Kummer or Dickson tower is a doubling tower for a different group: the
multiplicative group of the nodal degeneration y² \= x²(x+c) of E. The
summation polynomials of that degenerate group are linear in log
coordinates and interact with the tower. The summation polynomials of E
do not.

#strong[Lemma A (proved; rigidity).] Let Φ : E → E be a morphism of
varieties such that Φ(P)+Φ(Q)+Φ(R) \= O whenever P+Q+R \= O. Then Φ \= φ
+ T with φ ∈ End(E) and 3T \= O.

Proof: every morphism E → E is a translate of a homomorphism. Then 3T \=
Φ(O)+Φ(O)+Φ(O) \= O. ∎

So a tower that preserves the summation variety level by level is a
Lattès tower. Its factor base F \= Φ\_t^{-1}(S) is a union of cosets of
ker Φ\_t. A relation over F is then a relation over E/ker Φ\_t on the
image of S, which is the lab\'s generic-group null theorem
(RESEARCH\_ISOGENY\_FIBERS\_ECDLP.md Thm 15.1). A tower that does not
preserve it is, by Ritt, a power or Chebyshev tower, which belongs to
the nodal group.

#strong[Dichotomy (heuristic as stated; Lemma A is its rigorous half).]
A factor-base tower is either:

- compatible with the group law, in which case the base degenerates to
  IC on an isogenous curve; or
- incompatible, in which case it is a tower for the nodal degeneration
  and gives no elliptic cancellation in S\_{m+1}.

This predicts what the lab measured:

- the solving degree D of every Kummer/Dickson tower rises with the
  depth t (staircase 4, 5, 5, 6, 6 at m \= 3);
- relation counts on structured bases equal those of random bases
  (lever-2 null, 5,760 instances).

== 2. Elekes–Szabó: the quantitative version
<2-elekesszabó-the-quantitative-version>
#strong[Elekes–Szabó (2012), and the finite-field variants of Tao and of
Bays–Breuillard.] Let F(x,y,z) be an irreducible surface. Either |Z(F) ∩
A×B×C| ≤ O(n^{2−η}) for all n-sets A, B, C, or F is locally the graph of
addition in a one-dimensional algebraic group: G\_a, G\_m or an elliptic
curve. In the special case, the large intersections come from
approximate subgroups (progressions) of that group.

The S₃ surface is special: it #emph[is] the elliptic group law. So a
factor base A with many more relations than random must be close to a
coset progression of E(F\_p). Writing down such a set needs discrete
logarithms. A Kummer base is a progression in G\_m, the wrong group. For
a set that is multiplicatively but not elliptically structured, the
theorem gives an upper bound on the relation count over the random
expectation; it does not give an enhancement. That is the lever-2 null.
The finite-field statements hold only for |A| below a power of p, so
this is the heuristic reading at IC sizes, not a proof.

#strong[What it says about the degree (heuristic).] In any tree-shaped
PDP system, the separators are the partial-sum abscissas X \= x(P\_i +
P\_j). Their elimination ideal is the pair-sum set Σ₂(V) \= {x(P+P\') :
P, P\' ∈ F}. By Elekes–Szabó this set is \"generic\": it has size about
|V|² and no low-complexity presentation. F4 has to reach this degree, or
the analogous degree for larger partial sums. #strong[Conjecture B:] the
t-growth of D comes from the separators, not from the membership towers.
Every lab measurement agrees with it, but none tests it directly. The
direct test is to plant a separator constraint and compare D.

== 3. Phylogenetic gluing: the symmetry that is left
<3-phylogenetic-gluing-the-symmetry-that-is-left>
#strong[Algebraic statistics.] In a group-based phylogenetic model on a
tree, each internal vertex carries the group sum of its children\'s
states. Its ideal is glued from the star ideals of the tree\'s vertices
along the internal edges. These are Sturmfels–Sullivant (2005) and
Sullivant\'s toric fiber products (2007). For codimension-zero toric
fiber products, a Gröbner basis of the glued ideal is assembled from
those of the pieces, in degree at most the pieces\' maximum. The
model\'s automorphism group is the automorphism group of the tree; for a
balanced rooted binary tree with m \= 2^k leaves it is the iterated
wreath product S₂ ≀ … ≀ S₂, which is the Sylow 2-subgroup of S\_m.

#strong[The ECDLP relation is a group-based tree model.] A decomposition
R \= P₁ + … + P\_m is a rooted tree: the leaves are factor-base points,
the internal vertices are partial sums, and each vertex carries the
elliptic group law of its children, which is S₃. The lab\'s two families
are two extremes:

- #strong[plain chain (caterpillar).] The tree is a path, its
  automorphism group is trivial, and its inputs have degree ≤ 4 for
  every m. Each relation is seen m! times.
- #strong[full symmetric (S\_{m+1}(e)).] It quotients all of S\_m but
  has one vertex of degree 2^{m−1}, so its input degree, and with it D,
  is exponential in m. The lab measured D \= 8 at m \= 4.

#strong[The tree-glued cherry-symmetric system (this proposal).] Pair
the summands into cherries of a balanced tree. Encode each cherry
(P\_{2c−1}, P\_{2c}) by the symmetric pair (s\_c, q\_c) \= (x+x\',
xx\'). Its membership is the degree-2 remainder tower T^{2^t} ≡ 1 mod
(T² − s\_cT + q\_c). Its vertex is S₃(s\_c, q\_c, X\_c), the symmetrized
S₃ of degree 4. Internal vertices glue the X\'s by S₃.

- This quotients (S₂)^{m/2} of the wreath symmetry, i.e. 2^{m/2} copies.
- It keeps #strong[every input at total degree ≤ 4 for every m].
- It has the same variable count as the caterpillar.

As far as the lab\'s records show, it is the only one of the three
encodings that both removes a growing part of the symmetry and has
bounded input degree.

#strong[Why bounded input degree is the condition that matters for a
subexponential cost.] Take |F| ≈ p^{1/m}, N ≈ m·t + O(m) unknowns and t
\= log₂ p / m. One PDP costs about binom(N+D, D)^ω, there are about m!
targets per relation, and the relations number about |F|. The total is
L\_p(1/2)-subexponential only if log binom(N+D, D) \= O(√(log p · log
log p)) at m ≈ √(log p / log log p). Since N ≈ log p, that needs:

+ #strong[D \= O(m) up to log factors, which rules out any encoding with
  a degree-2^{m−1} input]; and
+ #strong[D nearly independent of the depth t], because t ≫ m at the
  optimum.

Condition 1 rules out the full symmetric arm asymptotically, whatever it
gains at small m. Condition 2 is what the measurement tests: the
#emph[t-slope] of D at fixed m. The full gluing theorem would give D
bounded by the cherry D. It is not expected to hold, because the S₃
gluing is not toric and Conjecture B predicts separator growth. The
measurement asks whether the tree arm\'s t-slope is below the
caterpillar\'s.

#strong[Kill criterion (pre-registered in `.resume/RESUME.md` before the
solver runs).] The tree\'s D is at least the caterpillar\'s in every
comparable cell.
