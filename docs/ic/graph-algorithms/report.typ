// Pandoc template; report.typ is the generated, directly compilable source.
// Some definitions presupposed by pandoc's typst output.
#let horizontalrule = [
  #line(start: (25%,0%), end: (75%,0%))
]

#let endnote(num, contents) = [
  #stack(dir: ltr, spacing: 3pt, super[#num], contents)
]
#set document(title: "ECDLP graph algorithms and asymptotic costs")
#set page(paper: "a4", margin: (x: 18mm, y: 18mm), numbering: "1", number-align: center)
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("16283d"))
#set par(justify: false, leading: 0.68em)
#set heading(numbering: none)
#set math.equation(numbering: none)
#show math.equation: set text(font: "DejaVu Math TeX Gyre")
#show link: set text(fill: rgb("007c83"))
#show heading.where(level: 1): set text(size: 20pt, weight: "bold")
#show heading.where(level: 2): set text(size: 14pt, weight: "bold", fill: rgb("007c83"))
#show heading.where(level: 3): set text(size: 10.5pt, weight: "bold")
#show table: set text(size: 8pt)
#set table(stroke: (x: none, y: 0.3pt + rgb("d5e0e5")), fill: (x, y) => if y == 0 {rgb("16283d")} else if calc.odd(y) {rgb("f0f5f7")} else {white})
#show table.cell.where(y: 0): set text(fill: white, weight: "bold")
#show raw: set text(font: "DejaVu Sans Mono", size: 7.8pt)
#show figure.where(kind: image): set block(breakable: false)

= Graph algorithms and asymptotic costs for ECDLP index calculus
<graph-algorithms-and-asymptotic-costs-for-ecdlp-index-calculus>
Research reference, 22 November 2026.

#strong[Graph bookkeeping is often nearly linear in the relation graph.]
The larger costs can be producing enough independent relations,
expanding their algebraic payloads, controlling fill, and solving the
remaining matrix. This catalog identifies algorithms for those stages,
states their asymptotic runtimes and assumptions, and proposes an
evaluation order for the Rust pipeline.

Companion artifacts: #link("./report.pdf")[PDF report];,
#link("./pipeline.svg")[editable pipeline diagram];, and
#link("./SOURCES.md")[repository grounding and integration ownership];.
The PDF is rendered from this Markdown catalog together with
`SOURCES.md`.

#figure([#box(width: 85%, image("pipeline.svg"));],
  caption: [
    Relation processing, sparse linear algebra, and complete ECDLP cost
    accounting.
  ]
)

== Scope and evidence status
<scope-and-evidence-status>
This is a documentation and mathematical complexity audit for the
ordinary prime-field ECDLP research program. It includes techniques
established in related index-calculus systems while preserving their
original applicability conditions. Complete-solver performance remains
an empirical question.

#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [Status], [Meaning in this catalog],
  [#strong[Literature];],
  [A bound attributed to a primary paper or official software
  documentation, under the stated model and hypotheses. A citation is
  not an independent formal verification.],
  [#strong[Derived \[pcc:unverified\]];],
  [An operation count, algebraic argument, or combination of published
  results explained here. It requires the repository’s applicable review
  and proof process before being treated as certified.],
  [#strong[Proposed];],
  [An experiment or integration candidate. The catalog supplies neither
  a new Rust implementation nor measurements of its performance.],
)]
)

The proposed queue must reuse existing arithmetic, relation, filtering,
and solver implementations where available. The repository already
contains `large_prime_filter_mst`, the
`gaudry_cubic_graph::GraphEliminator` wrapper around
`pencil_symbolic_graph::SymbolicGraph`, and the conservative unweighted
`jv_large_prime_rank_bound::StructuralRankBound`. The latter is not an
exact gain-aware DSU and does not use path compression.
#link("https://github.com/isogeny-labs/crypto/pull/4905")[PR \#4905]
owns gain-forest/MITM integration; this catalog does not duplicate or
supersede that implementation work. See
#link("./SOURCES.md")[SOURCES.md] for repository-specific grounding and
source links.

== Notation and runtime units
<notation-and-runtime-units>
#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [Symbol], [Meaning],
  [$v , e$],
  [Vertices and edges of an ordinary graph. In a double-large-prime
  graph, edges represent partial relations.],
  [$R , C , I$],
  [Relation rows, large-prime columns, and nonzero row-column
  incidences. The bipartite support graph has $R + C$ vertices and $I$
  edges.],
  [$N , Z$],
  [Dimension of a square residual matrix and its number of nonzeros.
  Rectangular results use $R + C$ explicitly.],
  [$L$],
  [Total edge appearances in all explicitly emitted cycles.],
  [$k , S$],
  [Rows participating in one merge, and their total sparse support
  before the merge.],
  [$b$],
  [Krylov block width.],
  [$t$],
  [Width of a supplied decomposition of the relevant graph; use at least
  $1$ in runtime formulas for degenerate edgeless cases.],
  [$ell$],
  [Prime subgroup order. Relation linear algebra takes place over
  $bb(F)_ell$, regardless of the curve’s base-field characteristic.],
  [$alpha$],
  [Inverse Ackermann function.],
  [$omega$],
  [Any admissible dense matrix multiplication exponent greater than
  $2$.],
  [$tilde(O)$],
  [Suppresses logarithmic factors.],
  [$O^(\*)$],
  [Suppresses factors polynomial in input size, as in parameterized
  SAT.],
)]
)

Graph bounds count graph or pointer operations. Algebra bounds count
scalar field operations unless explicitly labeled as machine-word
operations. An inversion is a field operation in this model; its bit
cost must be charged separately. Input normalization, witness expansion,
relation payload arithmetic, randomization failures, reconstruction,
communication, and output size must be included in a complete solver
estimate.

The existing LP-filter API checks for an odd modulus in the range
$3 lt.eq m < 2^63$. That check alone does not establish primality. The
prime-$ell$ field hypothesis used in this catalog remains a caller
precondition; composite odd moduli do not justify the field-rank and
division arguments below. Repository contracts are documented in
#link("./SOURCES.md")[SOURCES.md];.

These are bounds for named algorithms, rather than a claim that every
entry is the latest theoretical record or the fastest practical
implementation.

== 1. Connectivity and cycle handling
<connectivity-and-cycle-handling>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Algorithm], [Asymptotic runtime], [Role and conditions],
  [BFS / DFS],
  [$O (v + e)$],
  [Components, spanning forests, and unweighted paths. $O (v + e)$
  storage including adjacency. Literature:
  #link("https://www.cs.cmu.edu/~cdm/resources/Tarjan1972-sccs.pdf")[1];.],
  [Union-find],
  [$O ((v + e) alpha (v))$ amortized],
  [Incremental connectivity with $O (v)$ metadata. Detects topological
  cycle closure, not coefficient cancellation. Literature:
  #link("https://doi.org/10.1145/321879.321884")[2];.],
  [Gain-aware union-find],
  [$O ((v + R) alpha (v))$ scalar and pointer operations],
  [Exact rank accounting for rows with at most two nonzero LP
  coefficients, using potentials and a pinning flag. $O (v)$ metadata.
  Witness expansion is additional. #strong[Derived \[pcc:unverified\]]
  in Section 6, using #link("https://doi.org/10.1145/321879.321884")[2]
  and
  #link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3];.],
  [Fundamental cycle basis],
  [$O (v + e + L)$],
  [A forest and its chords give an implicit linear-size representation.
  Explicit cycles can have total length $Theta (e v)$; retaining them
  requires $O (L)$ output storage. Literature structure and
  output-sensitive count:
  #link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3];,
  #link("https://people.mpi-inf.mpg.de/~mehlhorn/ftp/BarrierCycleBasisJournal.pdf")[7];.],
  [Link-cut trees],
  [$O (log v)$ amortized per link, cut, or constant-size path
  aggregate],
  [Dynamic forests and scalar path costs; $O (v)$ metadata. Listing a
  path of length $a$ costs $Omega (a)$. Vector-valued payload
  aggregation has nonconstant cost. Literature:
  #link("https://www.cs.cmu.edu/~sleator/papers/dynamic-trees.pdf")[4];.],
  [Bridges / articulation points / biconnected components],
  [$O (v + e)$],
  [Topology diagnostics. Bridge deletion preserves ordinary
  incidence-cycle spaces but can destroy dependencies of gain matrices.
  Literature:
  #link("https://www.cs.cmu.edu/~cdm/resources/Tarjan1972-sccs.pdf")[1];,
  #link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3];;
  counterexample in Section 5.],
  [All graph core numbers],
  [$O (v + e)$],
  [Batagelj–Zaveršnik bucket algorithm. A 2-core locates potential
  cyclic structure. Pruning a higher core is not a general algebraic
  preservation rule. Literature:
  #link("https://arxiv.org/abs/cs/0310049")[5];.],
  [Low-stretch spanning tree],
  [$O (e log v log log v)$, plus isolated-vertex handling],
  [Abraham–Neiman construction. In an unweighted graph, its fundamental
  cycles have total length $O (e log v log log v)$. Algebraic usefulness
  still depends on the coefficient model. Literature:
  #link("https://doi.org/10.1137/17M1115575")[6];; output consequence
  discussed in Section 7.],
  [Minimum cycle basis],
  [$O (e^omega + kappa e^2 log v)$, Monte Carlo],
  [Amaldi et al.~Suitable amplification gives $O (e^omega)$ in the
  paper’s model. Applies to ordinary incidence-cycle spaces, not
  arbitrary gain-matrix kernels. Dense algebra makes this a later
  candidate. Literature:
  #link("https://people.mpi-inf.mpg.de/~mehlhorn/ftp/BarrierCycleBasisJournal.pdf")[7];.],
)]
)

For the minimum-cycle-basis result, $kappa$ controls the failure
probability. The connected $"GF" (q)$ formulation bounds it by

$ (e - v + 1) log (v e) q^(- kappa) . $

The cited proof uses a prime $q = O (e log e)$ and constant-time field
arithmetic. Its time model must not be silently imported as a
bit-complexity result for an arbitrary subgroup modulus.
#link("https://people.mpi-inf.mpg.de/~mehlhorn/ftp/BarrierCycleBasisJournal.pdf")[7]

== 2. Hypergraphs, matching, and sparse filtering
<hypergraphs-matching-and-sparse-filtering>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Algorithm], [Asymptotic runtime], [Role and conditions],
  [Singleton / hypergraph 2-core peeling],
  [$O (R + C + I)$],
  [Queue and incidence lists, after repeated coefficients are combined
  and zeros removed. Exact for cancellation of designated LP columns. A
  nonempty core may still have full coefficient rank. Literature:
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8];; #strong[derived
  \[pcc:unverified\]] preservation argument in Section 5.],
  [MST-guided $k$-way merge],
  [$O (k S + k^2)$ plus materialization],
  [Pairwise costs of normalized sparse rows with total support $S$,
  followed by dense Prim in $O (k^2)$. The tree selects $k - 1$
  independent coefficient combinations; resulting rows may still be
  dependent. A local optimum under the chosen edge costs, not globally
  minimum fill. Field-general adaptation of
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8];, #strong[derived
  \[pcc:unverified\]];.],
  [Structured Gaussian elimination],
  [$O (sum_i r_i c_i)$ arithmetic, plus pivot search and data-structure
  updates],
  [$r_i , c_i$ are current pivot row and column sizes. Fill can produce
  $O (N^3)$ classical worst-case work. Preserve the transformation for
  recovery. Literature framework:
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8];,
  #link("https://doi.org/10.1137/0205021")[9];; explicit operation count
  #strong[derived \[pcc:unverified\]];.],
  [Markowitz pivot selection],
  [Naive $O (sum_i Z_i + sum_i r_i c_i)$],
  [The score $(r_i - 1) (c_i - 1)$ estimates potential fill; $Z_i$ is
  the active nonzero count. The full-scan implementation can cost
  $O (N^3)$. Literature framework
  #link("https://doi.org/10.1137/0205021")[9];; implementation count
  #strong[derived \[pcc:unverified\]];.],
  [Minimum-degree ordering],
  [$O (sum_i d_i^2)$ explicit clique-update work, plus selection and
  data-structure costs],
  [$d_i$ is degree in the filled graph. Count adjacency operations under
  the chosen representation. Global minimum fill is NP-hard. Literature:
  #link("https://doi.org/10.1137/0205021")[9];,
  #link("https://doi.org/10.1137/0602010")[10];; clique enumeration
  count #strong[derived \[pcc:unverified\]];.],
  [Hopcroft–Karp matching],
  [$O ((R + C + I) sqrt(R + C))$],
  [Maximum matching of the row-column support graph; $O (R + C + I)$
  storage. Computes structural rank, an upper bound on actual field
  rank. Literature:
  #link("https://web.eecs.umich.edu/~pettie/matching/Hopcroft-Karp-bipartite-matching.pdf")[11];,
  #link("https://www.hsl.rl.ac.uk/catalogue/hsl_mc79.html")[12];.],
  [Dulmage–Mendelsohn decomposition],
  [$O (R + C + I)$ after maximum matching],
  [Reachability and SCCs expose underdetermined, well-determined, and
  overdetermined blocks and block triangular structure. Matching cost is
  additional. Coefficient singularities may remain. Literature:
  #link("https://www.cs.cmu.edu/~cdm/resources/Tarjan1972-sccs.pdf")[1];,
  #link("https://www.hsl.rl.ac.uk/catalogue/hsl_mc79.html")[12];.],
)]
)

=== Why an MST is relevant
<why-an-mst-is-relevant>
#strong[Derived \[pcc:unverified\].] Suppose $k$ rows $A_i$ contain a
common pivot column with nonzero coefficients $a_i$. Normalize each row
to

$ B_i = a_i^(- 1) A_i . $

#strong[Derived \[pcc:unverified\].] An edge $(i , j)$ represents
$B_i - B_j$, which cancels the pivot. The coefficient vectors of the
edges of any spanning tree form a basis of the $(k - 1)$-dimensional
subspace of combinations whose normalized coefficients sum to zero.
Therefore a tree supplies all required pivot-canceling coefficient
combinations. This does #strong[not] prove that their images as
materialized rows are independent: if the inputs are identical, all
differences are zero.

Weight each candidate edge by actual modular support or a specified
merge-cost model. With sorted sparse inputs of total support $S$,
scanning every pair takes $O (k S)$ work, and dense Prim takes
$O (k^2)$. Producing the selected rows and their provenance is
additional. This is an explicit field-general adaptation of Cavallar’s
local merge strategy. XOR support calculations are not valid for
arbitrary odd-modulus coefficients.
#link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8]

Track both $N$ and $Z$ after every filter. Reducing matrix dimension
while increasing density can raise the cost of a sparse solve. Retain
enough transformation data to reconstruct original relation and target
coefficients.

== 3. Exact finite-field linear algebra
<exact-finite-field-linear-algebra>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Algorithm], [Arithmetic runtime], [Memory and qualifications],
  [Sparse matrix-vector product],
  [$O (Z + N)$],
  [$O (Z + N)$ including the matrix. Reordering affects locality and
  communication, not this arithmetic count. Literature:
  #link("https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf")[13];.],
  [Scalar Wiedemann],
  [$O (N Z + N^2)$ for a standard successful solve; general variants
  $tilde(O) (N (Z + N))$],
  [$O (Z + N)$ storage. Projection, preconditioning, verification, and
  retry assumptions matter. For $Z = Theta (N)$, the work is
  quadratic-scale. Literature:
  #link("https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf")[13];.],
  [Block Wiedemann],
  [$O (N Z + N^2 b) + tilde(O) (N b^(omega - 1))$ with dense
  projections],
  [Typical $O (Z + N b)$ storage. There are $O (N \/ b)$ sequential
  block rounds. Charge preconditioner application, reconstruction, and
  retries separately. Literature and stage count:
  #link("https://members.loria.fr/EThome/files/jsc.pdf")[14];,
  #link("https://perso.ens-lyon.fr/claude-pierre.jeannerod/papers/issac03.pdf")[15];;
  assembled expression #strong[derived \[pcc:unverified\]];.],
  [Finite-field Lanczos variants],
  [$O (N Z + N^2)$ for $O (N)$ iterations],
  [$O (Z + N)$ storage. Requires appropriate symmetry, preconditioning,
  and breakdown handling. Real numerical convergence arguments do not
  transfer. Literature:
  #link("https://users.cs.duke.edu/~elk27/bibliography/97/EbKa97.pdf")[16];.],
  [Montgomery block Lanczos over $bb(F)_2$],
  [$O (N Z \/ w + N^2)$ machine-word operations in the cited model],
  [$w$ binary vectors packed in one word. This is not an odd-prime-field
  arithmetic bound. Literature:
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8];,
  #link("https://doi.org/10.1007/3-540-49264-X_9")[17];.],
  [Dense elimination / fast dense algebra],
  [$O (N^3)$ classically; $O (N^omega)$ with fast algorithms],
  [Usually quadratic storage. Suitable for small or dense residual
  cores; compare with the applicable sparse baseline. Literature:
  #link("https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf")[13];,
  #link("https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2022.33")[18];.],
  [Sparse inverse, rank, and full nullspace],
  [Published expected $O (N^2.2131)$ under near-linear matrix access],
  [Casacuberta–Kyng, 2022. This is a stronger output task than finding
  one dependency. The decimal is the paper’s published exponent, not a
  claim of the current record. Literature:
  #link("https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2022.33")[18];.],
)]
)

=== Block size changes scheduling, not total sparse-product work
<block-size-changes-scheduling-not-total-sparse-product-work>
#strong[Derived \[pcc:unverified\], from
#link("https://members.loria.fr/EThome/files/jsc.pdf")[14] and
#link("https://perso.ens-lyon.fr/claude-pierre.jeannerod/papers/issac03.pdf")[15];.]
A balanced block implementation performs

$ O (N \/ b) upright(" rounds") times O (Z b) upright(" work per block product") = O (N Z) . $

Dense left projection costs $O (N b^2)$ per round, hence $O (N^2 b)$
overall. Fast polynomial-matrix generator computation contributes
$tilde(O) (N b^(omega - 1))$ under the usual balanced-block setup.
Sparse coordinate projections may reduce projection overhead but require
a separate admissibility and success argument. If preconditioning
changes the matrix-vector cost to $tau$, charge that cost rather than
the original $Z$.

Thomé’s subquadratic generator algorithm accelerates one stage. It does
not alone make the complete sparse solve subquadratic. A full inverse or
nullspace basis may also require quadratic output space even when the
application needs only one dependency.
#link("https://members.loria.fr/EThome/files/jsc.pdf")[14];,
#link("https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2022.33")[18]

== 4. Structural and parallel algorithms
<structural-and-parallel-algorithms>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Algorithm], [Asymptotic runtime], [When the bound applies],
  [Exact low-treewidth elimination],
  [$O ((R + C) t^3)$; or $O ((R + C) t^2 log^2 (R + C))$ through
  pathwidth],
  [Any field, given a valid suitably sized width-$t$ decomposition of
  the bipartite nonzero graph. Use the smaller bound. Discovery is
  additional. Literature
  #link("https://fedorvf.github.io/articles/2018/2018e.pdf")[19];;
  combined route #strong[derived \[pcc:unverified\]] in Section 7.],
  [Exact path / tree-partition elimination],
  [$O ((R + C) t^2)$ factorization; $O ((R + C) t)$ per subsequent RHS],
  [Requires the corresponding supplied decomposition. Tree-partition
  width and treewidth are distinct parameters. Literature:
  #link("https://fedorvf.github.io/articles/2018/2018e.pdf")[19];.],
  [Nested dissection],
  [Classical planar setting: $O (N^(3 \/ 2))$ work and $O (N log N)$
  storage],
  [Requires suitable separators and elimination assumptions.
  Positive-definite Cholesky bounds do not automatically cover modular
  pivoting. Literature:
  #link("https://www.cs.cmu.edu/~15859n/RelatedWork/LiptonRoseTarjan.pdf")[20];.],
  [Fiduccia–Mattheyses partition refinement],
  [$O (I)$ per pass in the original net model],
  [$p$ passes cost $O (p I)$. The pass count and multilevel
  preprocessing are additional. Relevant to repeated distributed sparse
  products. Literature:
  #link("https://users.ece.utexas.edu/~dpan/EE382V_PDA/papers/fm.pdf")[21];,
  #link("https://www.cs.bilkent.edu.tr/~aykanat/papersAbs/99IEEETPDSabs.html")[22];.],
  [Greedy maximal independent set],
  [$O (v_c + e_c)$ for a supplied conflict graph],
  [Schedules compatible jobs. Charge construction, changing fill, and
  subsequent rounds. Maximum independent set is a different optimization
  problem. Standard greedy count; parallel algorithm background
  #link("https://www.cs.cmu.edu/~guyb/paralg/papers/Luby86.pdf")[23];.],
  [2-SAT through implication SCCs],
  [$O (s + c)$ for $s$ variables and $c$ two-literal clauses],
  [Requires a genuine 2-CNF subproblem. A general relation-generation
  encoding need not satisfy that condition. Literature:
  #link("https://doi.org/10.1016/0020-0190(79)90002-4")[24];.],
  [SAT / \#SAT with incidence treewidth $t$],
  [$O^(\*) (2^t)$ with a supplied decomposition],
  [Count every encoded variable and clause, including auxiliary
  variables. Sparse encodings need not have small width. Literature:
  #link("https://ac.tuwien.ac.at/files/tr/ac-tr-20-006.pdf")[25];.],
  [Exact max-flow / min-cost-flow],
  [$e^(1 + o (1))$ deterministic],
  [Polynomially bounded integral capacities, demands, and costs. A
  unit-capacity reduction gives an almost-linear theoretical
  bipartite-matching route, not general modular matrix rank. Literature:
  #link("https://arxiv.org/abs/2309.16629")[26];.],
)]
)

=== Directed shortest paths: a 2026 result
<directed-shortest-paths-a-2026-result>
Duan, Mao, Shu, and Yin give the following deterministic bound for
directed single-source shortest paths with nonnegative real weights:

$ O (e sqrt(log v) + sqrt(e v log v log log v)) . $

The paper compares this bound with the 2025 $O (e log^(2 \/ 3) v)$
result. On sparse graphs in the relevant connected/reachable input
regime, the displayed expression becomes $O (e sqrt(log v log log v))$.
These are literature results; this catalog supplies no implementation or
timing measurements for them.
#link("https://arxiv.org/abs/2602.07868")[27]

It is relevant to the ECDLP pipeline only if weighted path searches
dominate measured cost. Unweighted paths already admit linear-time BFS.
Actual merged support is generally non-additive because modular
coefficients may cancel; a shortest path under fixed scalar edge weights
does not automatically minimize the output relation.

== 5. What a graph dependency must certify
<what-a-graph-dependency-must-certify>
=== Relation model
<relation-model>
#strong[Derived \[pcc:unverified\].] Work inside a prime-order subgroup.
Let $G$ generate it and let the target be $Q = x G$. Write factor-base
and large-prime logs as $x_F$ and $x_L$. Partial relations have the form

$ A_F x_F + A_L x_L = a + b x #h(2em) upright("over ") bb(F)_ell . $

A row combination $y$ eliminates all large primes exactly when

$ y^T A_L = 0 . $

It then produces

$ y^T A_F x_F = y^T a + (y^T b) x . $

The projected relation may still be zero, redundant, or unable to
contribute to target recovery. The relevant objective is independent
usable equations at lower total cost. Neither the number of topological
cycles nor the LP-block nullity alone establishes that objective.

=== A triangle can have no cancellation
<a-triangle-can-have-no-cancellation>
#strong[Derived \[pcc:unverified\].] Consider LP parts $x + y$, $y + z$,
and $z + x$:

$ A_L = mat(delim: "(", 1, 1, 0; 0, 1, 1; 1, 0, 1) , #h(2em) det (A_L) = 2 . $

The support has one cycle and a nonempty 2-core. Over any odd prime
modulus, the matrix has full rank and zero left kernel. In
characteristic two it has a dependency. Topology therefore cannot
replace coefficient algebra.

=== A bridge can be essential for LP elimination
<a-bridge-can-be-essential-for-lp-elimination>
#strong[Derived \[pcc:unverified\].] Take triangles $a b c$ and $d e f$
joined by edge $c d$. Put coefficient $+ 1$ at both endpoints of every
edge row. In edge order

$ (a b , b c , c a , d e , e f , f d , c d) , $

the coefficients

$ (1 , - 1 , - 1 , - 1 , 1 , - 1 , 2) $

#strong[Derived \[pcc:unverified\].] These coefficients sum to zero at
every vertex over the integers. The bridge coefficient is nonzero over
every odd field. Deleting the bridge destroys this dependency. This
illustrates the loose-handcuff behavior in signed/gain graph matroids.
It proves LP cancellation, not usefulness of an unspecified factor-base
payload.
#link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3];,
#link("https://people.math.binghamton.edu/zaslav/Tpapers/mananthavady-lecture-notes.20110729.pdf")[28]

=== Why singleton peeling is safe
<why-singleton-peeling-is-safe>
#strong[Derived \[pcc:unverified\].] If an LP column contains just one
nonzero entry $a$ in row $i$, its coordinate in $y^T A_L = 0$ is
$a y_i = 0$. Since $a eq.not 0$ in the field, every such dependency has
$y_i = 0$. Delete that row, update all affected incidences, and repeat.
A queue and incidence lists touch each incidence a bounded number of
times, giving $O (R + C + I)$ after canonicalization.
#link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8]

#strong[Derived \[pcc:unverified\].] This proves preservation of the
dependency space for cancellation of designated LP columns, with the
deleted coordinates restored as zeros. A different solving formulation
may require a recovery transcript. Repeated endpoints must first be
combined and resulting zero coefficients removed; raw multiplicity is
not algebraic degree.

=== Structural rank is an upper bound
<structural-rank-is-an-upper-bound>
The support graph of

$ mat(delim: "(", 1, 1; 1, 1) $

has a matching of size two, although its field rank is one. The example
is #strong[derived \[pcc:unverified\]];; the structural-rank distinction
is documented in
#link("https://www.hsl.rl.ac.uk/catalogue/hsl_mc79.html")[12];.
Random-hypergraph core and rank predictions also depend on their
distributional hypotheses. Algebraically generated ECDLP relations need
evidence that those hypotheses fit before the predictions are used.
#link("https://arxiv.org/abs/1906.05757")[29]

== 6. Derived gain-aware DSU for two-entry LP rows
<derived-gain-aware-dsu-for-two-entry-lp-rows>
#strong[Status: derived \[pcc:unverified\].] This algorithmic derivation
applies when every row of the large-prime block has at most two nonzero
coefficients. Here $v = C$ counts the LP variables, including isolated
columns. It uses gain-graph structure and does not solve the
unrestricted factor-base matrix in near-linear time. The supporting
framework is weighted disjoint sets and frame representations.
#link("https://doi.org/10.1145/321879.321884")[2];,
#link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3]

=== Invariant
<invariant>
Maintain weighted union-find with a spanning-tree potential

$ x_u = p_u t_("root" (u)) , #h(2em) p_u eq.not 0 . $

For each component, store a Boolean flag $U$ indicating that its
homogeneous root parameter has been forced to zero. Initially all
components are unflagged. Combine repeated endpoints and discard zero
coefficients before inserting a row.

For a two-entry row $a x_u + b x_v = 0$, both $a$ and $b$ below are
nonzero after normalization.

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Case], [Update], [Rank increment],
  [Different roots],
  [Union using $t_(r_u) = - (b p_v) \/ (a p_u) thin t_(r_v)$. The new
  flag is $U_1 or U_2$. Invert the potential ratio if the union
  heuristic chooses the other root orientation.],
  [$1$ unless both components were already flagged; then $0$.],
  [Same root],
  [Compute $gamma = a p_u + b p_v$. If $gamma eq.not 0$ and the
  component is unflagged, flag it.],
  [$1$ precisely for that first pinning event; otherwise $0$.],
  [One nonzero],
  [Pin the corresponding component.],
  [$1$ if it was unflagged; otherwise $0$.],
  [Zero row],
  [No update.],
  [$0$; the row is dependent.],
)]
)

The resulting rank and left-nullity counts are

$ "rank" (A_L) = v - \# { upright("unflagged components") } , $

$ dim ker (A_L^T) = R - "rank" (A_L) . $

Tree equations leave one homogeneous degree of freedom in each
component. A cycle with inconsistent gain pins that degree of freedom to
zero. A singleton row does the same. Joining two already pinned
components introduces a graph connection without increasing rank. This
is the frame-rank principle $v - b$, where $b$ counts balanced
components in the associated representation.
#link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[3]

=== Cost and output boundary
<cost-and-output-boundary>
Here $R$ counts #strong[every processed row];, including singleton and
zero rows. Weighted path compression with union by rank or size uses

$ O ((v + R) alpha (v)) $

pointer operations and scalar field operations, with $O (v)$ metadata.
This count treats division as one field operation; bit-complexity
comparisons must price inversions appropriately.
#link("https://doi.org/10.1145/321879.321884")[2]

Producing a kernel witness requires a retained algebraic explanation.
Materializing the projected factor-base relation requires its sparse
payload. Those costs are not included in the metadata bound. A proposed
implementation can maintain cheap rank metadata while expanding
witnesses only for candidates selected for evaluation in the residual
system, but it must retain and charge the required provenance.
Coordination with
#link("https://github.com/isogeny-labs/crypto/pull/4905")[PR \#4905] is
required for the existing gain-forest/MITM integration work.

== 7. When a graph improvement changes complete ECDLP cost
<when-a-graph-improvement-changes-complete-ecdlp-cost>
Use a complete phase accounting:

$ T_(upright(t o t a l)) =  & T_(upright(s e t u p)) + T_(upright(r e l a t i o n s)) + T_(upright(g r a p h)) + T_(upright(m e r g e))\
 & + T_(upright(f i l t e r)) + T_(upright(L A)) + T_(upright(t a r g e t)) + T_(upright(v e r i f y)) . $

#strong[Derived \[pcc:unverified\].] Suppose $R$ scales as $ell^r$, $N$
as $ell^a$, and $Z$ as $ell^z$. A near-linear graph bookkeeping method
costs roughly $tilde(O) (ell^r)$ only when its graph input satisfies
$v + I = tilde(O) (R)$, for example with bounded LP arity and no
asymptotically larger isolated-column inventory.

The scalar sparse-LA expression $O (N Z + N^2)$ instead gives

$ tilde(O) (ell^(max { a + z , 2 a })) . $

It simplifies to $tilde(O) (ell^(a + z))$ when $z gt.eq a$, such as a
retained square matrix with every row nonzero and therefore $Z gt.eq N$.
Relation-generation cost depends on attempts, acceptance probability,
and independence yield, not just the stored row count. If $A$ attempts
each cost approximately $ell^d$, charge approximately $A ell^d$,
together with all acceptance and verification work; do not substitute
$R$ for $A$ without a justified yield model.

#strong[Illustrative derivation \[pcc:unverified\].] Near-linear graph
processing changes one component cost; the complete exponent depends on
the dominant phases. A smaller residual matrix also need not be cheaper:
a hypothetical $20 %$ reduction in $N$ and $50 %$ increase in $Z$
multiplies the $N Z$ term by $0.8 times 1.5 = 1.2$. This is an
illustrative calculation, not a measurement.

=== A conditional exponent opportunity from treewidth
<a-conditional-exponent-opportunity-from-treewidth>
#strong[Derived \[pcc:unverified\], combining
#link("https://fedorvf.github.io/articles/2018/2018e.pdf")[19];.] For a
square residual matrix and a supplied width-$t$ tree decomposition of
its bipartite nonzero graph,

$ T_(upright(L A)) = O (N min { t^3 , t^2 log^2 N }) . $

The direct route uses the general tree-decomposition theorem. The second
route converts the supplied decomposition into a path decomposition of
width $O (t log N)$ and applies the stronger path-decomposition theorem.
With $O (N)$ input bags, centroid recursion constructs the path
decomposition in $O (t N log N)$, which is absorbed in the displayed
bound.

#strong[Derived \[pcc:unverified\].] Constant $t$ gives a linear solve
bound; polylogarithmic $t$ gives a near-linear one. If $t = N^delta$ for
fixed $delta < 1 \/ 2$, the path route costs
$O (N^(1 + 2 delta) log^2 N)$, which is subquadratic. Equivalently,
$t = o (sqrt(N) \/ log N)$ is a sufficient regime for that route. These
are sufficient conditions, not necessary conditions for faster solving.

Discovery remains separate. The approximation algorithm in
#link("https://fedorvf.github.io/articles/2018/2018e.pdf")[19] costs
$O (t^7 N log N)$ and returns width $O (t^2)$ or a justified treewidth
lower-bound conclusion. Its inflated output width cannot be inserted
into the solve formula as though it were $t$. A heuristic decomposition
can be useful if validated and timed. An unsuccessful heuristic does not
establish that treewidth is large.

=== Improving an explicit-output bound
<improving-an-explicit-output-bound>
#strong[Derived \[pcc:unverified\], from
#link("https://doi.org/10.1137/17M1115575")[6];.] An arbitrary
fundamental cycle basis can require $Theta (e v)$ edge appearances. For
an unweighted ordinary incidence graph, the low-stretch-tree guarantee
limits total fundamental-cycle length to $O (e log v log log v)$. This
improves a worst-case certificate-expansion bound when ordinary cycles
are algebraically admissible. It does not guarantee small projected
factor-base support or supply a basis of an arbitrary gain kernel.

=== Preserve the scope of related DLP results
<preserve-the-scope-of-related-dlp-results>
Double-large-prime graphs support index-calculus improvements for
suitable fixed-genus hyperelliptic Jacobians. The familiar
$tilde(O) (q^(2 - 2 \/ g))$ bound belongs to that setting and its
relation-generation analysis. Substituting $g = 1$ does not establish an
ordinary elliptic-curve algorithm. Efficient graph recombination cannot
supply an unproved decomposition oracle.
#link("https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf")[30]

=== Full-cost acceptance criterion
<full-cost-acceptance-criterion>
For the ordinary prime-field benchmark, require blind target recovery
and verify $[x] G = Q$. Include setup, rejected relations, filtering,
linear algebra, reconstruction, target work, and verification. Compare
with the applicable index-calculus frontier and optimized Pollard rho
using the same arithmetic pricing and hardware. Assess component results
against complete solver cost.

== 8. Proposed Rust evaluation queue
<proposed-rust-evaluation-queue>
#strong[Status: proposed.] Reuse existing implementations and coordinate
with their owners. In particular,
#link("https://github.com/isogeny-labs/crypto/pull/4905")[\#4905] owns
gain-forest/MITM integration. The work below describes evaluations and
possible integrations; this catalog contains no new implementation or
timing measurements.

First freeze identical relation streams for comparisons. Then evaluate
any promising policy inside complete cold solver runs, including its
setup and reconstruction costs.

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [Priority and candidate], [Proposed Rust work and measurements],
  [Acceptance criterion],
  [#strong[1. Gain-aware DSU and singleton peeling];],
  [Reuse existing gain-forest work and canonical relation
  representation; audit against the existing conservative
  `StructuralRankBound` without relabeling it as exact gain rank. Record
  $R , C , I$, ordinary cycles, balanced/pinned components, LP rank
  against exact elimination at audit sizes, and projected relation
  rank.],
  [Correct coefficient cancellation and lower complete cost per
  independent usable equation. Metadata timing alone does not pass.],
  [#strong[2. MST merges versus lightest-row pivot];],
  [Reuse and audit `large_prime_filter_mst` and modular sparse-row
  operations. Preserve `MstAttempt` atomic-refusal and source-witness
  semantics. Measure $k , S$, pair-weight construction, selected row
  materialization, fill, $N , Z$, and provenance size under an identical
  input stream.],
  [Lower filtering-plus-solving cost after charging weight construction
  and recovery. A smaller local MST score alone does not pass.],
  [#strong[3. BFS trees versus low-stretch trees];],
  [Reuse the graph representation and witness-expansion machinery.
  Measure tree construction, total and tail cycle lengths $L$, payload
  arithmetic, and final $Z$. Apply only where the coefficient model
  permits the selected cycles.],
  [Reduced merge and complete solve cost after paying for the
  alternative tree.],
  [#strong[4. Matching, DM, and width measurement];],
  [Reuse sparse support export. Record structural matching, actual field
  rank on audit cases, block sizes, validated decomposition width,
  discovery time, and recovery cost.],
  [Evidence of useful separability or sufficiently small width, with
  discovery and reconstruction charged.],
  [#strong[5. Hypergraph partitioning for SpMV];],
  [Reuse the sparse product interface. Compare FM/multilevel candidates
  using partition time, imbalance, communicated residues, observed
  bytes, memory traffic, and every product’s wall time.],
  [Lower complete solve wall time at the same thread/node count; a lower
  partition score alone does not pass.],
  [#strong[6. Krylov block and generator tuning];],
  [Reuse the existing finite-field solver. Record $b$, exact product
  count, projection, generator, reconstruction, retries, and peak
  memory.],
  [Throughput and complete-cost improvement with modular checking of
  every recovered result.],
)]
)

=== Communication model
<communication-model>
In a column-net distribution of sparse products, let $lambda_j$ be the
number of partitions touching column $j$. Under the corresponding
ownership scheme,

$ sum_j (lambda_j - 1) $

models communicated column data. Block width $b$ scales the amount of
vector data. Measure observed communication in addition to the partition
score, and account for the actual ownership and reduction scheme.
#link("https://www.cs.bilkent.edu.tr/~aykanat/papersAbs/99IEEETPDSabs.html")[22]

=== Minimum experiment record
<minimum-experiment-record>
Record curve and subgroup parameters, factor-base policy, relation seed
and digest, canonicalization rules, all phase times, operation counts,
raw and retained relation counts, independent usable rank, $N$ and $Z$
after each stage, maximum row weight, fill created, merge transcript
bytes, peak memory, thread/node count, failures and retries, and target
verification. Keep cold and amortized costs separate when precomputation
is reused.

== Rendering the report
<rendering-the-report>
From this directory, run:

```bash
bash build-report.sh
```

The document build requires Graphviz 2.43 or later, Pandoc 3.1 or later,
and a Typst 0.15 renderer. This build used the Python package
`typst==0.15.2`, whose PDF metadata identifies the embedded engine as
Typst 0.15.1. The build renders the authored catalog, diagram, and
repository grounding into `report.pdf`; it does not run a research
algorithm or benchmark. Edit #link("./pipeline.dot")[pipeline.dot] to
update the diagram, and keep the Markdown and diagram sources
synchronized with the rendered artifact.

== References
<references>
The links below identify primary papers or official software
documentation. Publication dates refer to the work itself. Derived
arguments and proposed integrations are labeled separately above.

+ #strong[Tarjan.]
  #link("https://www.cs.cmu.edu/~cdm/resources/Tarjan1972-sccs.pdf")[Depth-First Search and Linear Graph Algorithms];,
  1972. Linear DFS, SCCs, and biconnected decomposition.

+ #strong[Tarjan.]
  #link("https://doi.org/10.1145/321879.321884")[Efficiency of a Good But Not Linear Set Union Algorithm];,
  1975. Inverse-Ackermann disjoint-set analysis.

+ #strong[Zaslavsky.]
  #link("https://people.math.binghamton.edu/zaslav/Tpapers/bg4.pdf")[Biased Graphs IV: Geometrical Realizations];,
  2003. Multiplicative gain representation and frame rank $v - b$.

+ #strong[Sleator and Tarjan.]
  #link("https://www.cs.cmu.edu/~sleator/papers/dynamic-trees.pdf")[A Data Structure for Dynamic Trees];,
  1983. Logarithmic dynamic-tree operations.

+ #strong[Batagelj and Zaveršnik.]
  #link("https://arxiv.org/abs/cs/0310049")[An O\(m) Algorithm for Cores Decomposition of Networks];,
  2003 / 2011. Linear core decomposition.

+ #strong[Abraham and Neiman.]
  #link("https://doi.org/10.1137/17M1115575")[Using Petal-Decompositions to Build a Low Stretch Spanning Tree];,
  2012 / 2019. $O (e log v log log v)$ construction and total stretch.

+ #strong[Amaldi, Iuliano, Jurkiewicz, Mehlhorn, and Rizzi.] ESA 2009 /
  journal version.

  #quote(block: true)[
  #link("https://people.mpi-inf.mpg.de/~mehlhorn/ftp/BarrierCycleBasisJournal.pdf")[Improved Minimum Cycle Bases Algorithms by Restriction to Isometric Cycles]
  ]

  Theorem 3.2 and Corollary 3.3 provide the randomized cycle-basis
  bounds.

+ #strong[Cavallar.]
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[Strategies in Filtering in the Number Field Sieve];,
  2000. Singletons, controlled merging, local MST optimization, and
  binary block-Lanczos pricing.

+ #strong[Rose, Tarjan, and Lueker.]
  #link("https://doi.org/10.1137/0205021")[Algorithmic Aspects of Vertex Elimination on Graphs];,
  1976. Graph model of elimination and fill. The catalog’s explicit
  implementation counts are labeled as derivations.

+ #strong[Yannakakis.]
  #link("https://doi.org/10.1137/0602010")[Computing the Minimum Fill-In is NP-Complete];,
  1981. Hardness of globally minimum symbolic fill.

+ #strong[Hopcroft and Karp.]
  #link("https://web.eecs.umich.edu/~pettie/matching/Hopcroft-Karp-bipartite-matching.pdf")[An n^\(5/2) Algorithm for Maximum Matchings in Bipartite Graphs];,
  1973. $O ((v + e) sqrt(v))$ maximum matching.

+ #strong[HSL.]
  #link("https://www.hsl.rl.ac.uk/catalogue/hsl_mc79.html")[HSL\_MC79: Maximum Matching and Dulmage–Mendelsohn Decomposition];,
  official documentation. Structural rank and sparse block structure.

+ #strong[Wiedemann.]
  #link("https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf")[Solving Sparse Linear Equations Over Finite Fields];,
  1986. Sparse exact solving and memory bounds.

+ #strong[Thomé.] 2002.

  #quote(block: true)[
  #link("https://members.loria.fr/EThome/files/jsc.pdf")[Subquadratic Computation of Vector Generating Polynomials and Improvement of the Block Wiedemann Algorithm]
  ]

  Sequence, generator, reconstruction, and projection assumptions.

+ #strong[Giorgi, Jeannerod, and Villard.]
  #link("https://perso.ens-lyon.fr/claude-pierre.jeannerod/papers/issac03.pdf")[On the Complexity of Polynomial Matrix Computations];,
  2003. Fast polynomial-matrix and approximant computations.

+ #strong[Eberly and Kaltofen.]
  #link("https://users.cs.duke.edu/~elk27/bibliography/97/EbKa97.pdf")[On Randomized Lanczos Algorithms];,
  1997. Randomized finite-field Lanczos variants and reliability
  conditions.

+ #strong[Montgomery.]
  #link("https://doi.org/10.1007/3-540-49264-X_9")[A Block Lanczos Algorithm for Finding Dependencies over GF\(2)];,
  1995. Binary block method;
  #link("https://ir.cwi.nl/pub/2078/2078D.pdf")[8] also discusses the
  cited cost model.

+ #strong[Casacuberta and Kyng.] 2022.

  #quote(block: true)[
  #link("https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2022.33")[Faster Sparse Matrix Inversion and Rank Computation in Finite Fields]
  ]

  Published expected $O (N^2.2131)$ sparse inverse, rank, and
  full-nullspace result.

+ #strong[Fomin, Lokshtanov, Saurabh, Pilipczuk, and Wrochna.]
  #link("https://fedorvf.github.io/articles/2018/2018e.pdf")[Fully Polynomial-Time Parameterized Computations for Graphs and Matrices of Low Treewidth];,
  2018. Theorems 1.2–1.3, exact any-field elimination, and the pathwidth
  tradeoff.

+ #strong[Lipton, Rose, and Tarjan.]
  #link("https://www.cs.cmu.edu/~15859n/RelatedWork/LiptonRoseTarjan.pdf")[Generalized Nested Dissection];,
  1979. Separator-based time and fill results; the original numerical
  assumptions matter.

+ #strong[Fiduccia and Mattheyses.]
  #link("https://users.ece.utexas.edu/~dpan/EE382V_PDA/papers/fm.pdf")[A Linear-Time Heuristic for Improving Network Partitions];,
  1982. Linear per-pass partition refinement.

+ #strong[Çatalyürek and Aykanat.]
  #link("https://www.cs.bilkent.edu.tr/~aykanat/papersAbs/99IEEETPDSabs.html")[Hypergraph-Partitioning-Based Decomposition for Parallel Sparse-Matrix Vector Multiplication];,
  1999. Hypergraph communication models for sparse products.

+ #strong[Luby.]
  #link("https://www.cs.cmu.edu/~guyb/paralg/papers/Luby86.pdf")[A Simple Parallel Algorithm for the Maximal Independent Set Problem];,
  1986. Maximal independent sets and parallel scheduling background.

+ #strong[Aspvall, Plass, and Tarjan.]
  #link("https://doi.org/10.1016/0020-0190(79)90002-4")[A Linear-Time Algorithm for Testing the Truth of Certain Quantified Boolean Formulas];,
  1979. 2-SAT through implication graph structure.

+ #strong[Slivovsky and Szeider.] 2020.

  #quote(block: true)[
  #link("https://ac.tuwien.ac.at/files/tr/ac-tr-20-006.pdf")[A Faster Algorithm for Propositional Model Counting Parameterized by Incidence Treewidth]
  ]

  $O^(\*) (2^t)$ with a supplied incidence decomposition.

+ #strong[van den Brand et al.]
  #link("https://arxiv.org/abs/2309.16629")[A Deterministic Almost-Linear Time Algorithm for Minimum-Cost Flow];,
  2023. Exact integral-capacity/cost flow in $e^(1 + o (1))$ time.

+ #strong[Duan, Mao, Shu, and Yin.] 2026.

  #quote(block: true)[
  #link("https://arxiv.org/abs/2602.07868")[A Faster Directed Single-Source Shortest Path Algorithm]
  ]

  Deterministic nonnegative real-weight shortest-path bound.

+ #strong[Zaslavsky.]
  #link("https://people.math.binghamton.edu/zaslav/Tpapers/mananthavady-lecture-notes.20110729.pdf")[Signed Graphs and Geometry];,
  2011. Signed incidence rank, frame circuits, and handcuffs.

+ #strong[Coja-Oghlan, Ergür, Gao, Hetterich, and Rolvien.]
  #link("https://arxiv.org/abs/1906.05757")[The Rank of Sparse Random Matrices];,
  2019 / 2024 revision. Rank with prescribed degree distributions over
  arbitrary fields.

+ #strong[Gaudry, Thériault, and Thomé.]
  #link("https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf")[A Double Large Prime Variation for Small Genus Hyperelliptic Index Calculus];,
  February 2005 preprint. LP-graph construction; this early version
  precedes the 2007 paper with Diem. Preserve its stated hypotheses.

= Repository grounding and audit receipt
<repository-grounding-and-audit-receipt>
Audit date: 2026-11-22. The six Rust source files below were read at
`isogeny-labs/crypto` commit `ce396efc7129c8fc90409fbdb2720461272a366b`.
This is a bounded source inspection: no Rust tests, benchmarks, complete
ECDLP solves, or signed PCC verification were performed by this audit.
Existing test definitions are evidence of intended checks, not a passing
execution receipt. The catalog’s literature bounds do not automatically
describe these implementations. Source observations and derived caveats
below remain `[pcc:unverified]`.

== Existing implementation to reuse
<existing-implementation-to-reuse>
=== Large-prime filtering and provenance
<large-prime-filtering-and-provenance>
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/large_prime_filter.rs")[`large_prime_filter.rs`]
exports `PartialRelation`, `CompleteRelation`,
`LargePrimeFilterOptions`, `filter_large_primes`, and
`replay_complete_relation`. Input rows separate factor-base columns
\(`u32`) from large-prime identities \(`u64`), carry an RHS, and require
distinct source IDs. Normalization combines repeated keys modulo the
supplied modulus and drops cancellations before incidence construction.
Output rows retain sparse coefficients of original source IDs. The
replay helper reconstructs that combination; callers still compare the
reconstruction and check the original group relations.

The implementation already performs singleton peeling, light-row
ordering, and sparse elimination with a Markowitz-style score. Its
column degrees come from the initial core, so it is not the catalog’s
generic dynamically maintained Markowitz algorithm. Fill/source limits
can discard a candidate row. Normalization, ordered deduplication,
sorting, payload arithmetic, and provenance growth must be charged
beyond the abstract linear peeling bound. The API accepts odd moduli
with `3 <= modulus < 2^63`; it does not establish primality. Prime
subgroup order is a caller obligation, and noninvertible pivots return
an error.

=== MST merging already exists
<mst-merging-already-exists>
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/large_prime_filter_mst.rs")[`large_prime_filter_mst.rs`]
is exported through the preceding module as `filter_large_primes_mst`,
`MstAttempt`, `MstLimits`, and `MstStats`. It peels, chooses a
minimum-degree large-prime column, normalizes its incident rows, and
uses Prim selection over actual modular row differences. The primary
edge weight is remaining small-plus-large support; source support and
endpoint indices break ties. This is a local tree objective, not global
minimum fill.

Limits cover row count, column degree, edge evaluations, persistent
coefficient count, and row/source support. Infeasible edges are omitted;
failure to complete the projection returns `relations: None` atomically.
That is a refusal requiring a charged fallback, not a partial relation
set. The same modulus obligation applies. Whole-filter cost also
includes repeated column-map reconstruction and retained witness
materialization. The next experiment should compare this existing
implementation with the existing pivot filter on identical streams,
rather than reimplement MST merging.

=== Structural DSU bound and symbolic graph adapter
<structural-dsu-bound-and-symbolic-graph-adapter>
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/jv_large_prime_rank_bound.rs")[`jv_large_prime_rank_bound.rs`]
provides `StructuralRankBound::{new, add_row, snapshot}`. It accepts
bounded, canonical homogeneous rows with zero, one, or two distinct
nonzero LP coefficients. Its forest and singleton anchors give a lower
bound on LP rank and an upper bound on possible small-only rank.
Weighted-cycle contributions are intentionally ignored. Union-by-size is
present; path compression and coefficient potentials are not. Therefore
this module is neither the exact gain-rank algorithm nor an
implementation of the catalog’s inverse-Ackermann bound. It is a reuse
point for instrumentation and conservative screening.

#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/gaudry_cubic_graph.rs")[`gaudry_cubic_graph.rs`]
already contains the internal `GraphEliminator` adapter to
`pencil_symbolic_graph::SymbolicGraph`. It protects target column
`small` and a constant-one column, translates the RHS with the
appropriate sign, and returns a relation on `Outcome::Constraint`;
joins/anchors return none. It records graph work and rejects
out-of-range columns. The imported `SymbolicGraph` implementation was
not among these six files, so this audit does not establish its complete
complexity or supported input contract.

=== Two different Krylov uses
<two-different-krylov-uses>
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/pq_wiedemann.rs")[`pq_wiedemann.rs`]
exports scalar `berlekamp_massey` and `wiedemann_solve` over `BigUint`
residues. It uses direct sparse products for square systems and implicit
`M^T M` products for rectangular systems, then checks `Mx = b` on the
original matrix before returning a solution. Block Wiedemann appears in
commentary, not as an implementation in this file. Modular inversions
can fail. The full-column-rank justification in its normal-equations
comment needs qualification: over `F_5`, the nonzero column `(1,2)^T`
has zero Gram matrix. This direct calculation is unformalized; changing
a projection seed does not remove that structural degeneracy. Preserve
original-system checks and audit preconditioning before reusing this
branch.

#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/src/cryptanalysis/gaudry_cubic_krylov.rs")[`gaudry_cubic_krylov.rs`]
implements internal sparse-FGLM shape extraction from a multiplication
matrix, with recurrence/Hankel checks and a failure signal for dense
fallback. Here coefficients use the polynomial base field `p`, not
automatically the subgroup-order field of relation linear algebra. It
constructs sparse edges from a dense matrix and stores Krylov frames.
Its delayed-reduction fast path requires normalized coefficients,
dimension at most 64, and `p < 2^20`. These costs and restrictions must
not be replaced by a generic linear-memory Wiedemann claim.

== Related work and retrieval limits
<related-work-and-retrieval-limits>
The pinned
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/docs/ACTIVE.md")[`docs/ACTIVE.md`]
and
#link("https://github.com/isogeny-labs/crypto/blob/ce396efc7129c8fc90409fbdb2720461272a366b/docs/session-logs/2026-11-21.md")[2026-11-21 session log]
already track graph-IC and hyperelliptic large-prime/sparse-algebra
work. They establish lane context here; their timing headlines are not
imported as audited results.

As inspected on 2026-11-22,
#link("https://github.com/isogeny-labs/crypto/pull/4905")[PR \#4905] was
draft, on `claude/ecbench-graph-theory-ic`, head
`0b92b96c572d0f8dade30bb7319c94ba9a63a0b6`, implementing a gain-forest
harvester and coset-filtered MITM integration. This catalog, tracked by
#link("https://github.com/isogeny-labs/crypto/pull/4917")[PR \#4917];,
does not replace that implementation work. Low-stretch, matching/DM,
width, partitioning, and block-solver candidates require further
targeted repository discovery; these six files cannot establish
repository-wide absence.

#strong[Sources checked \(KB):] KB unavailable. No research-KB MCP tools
were exposed; `scripts/research-kb status` returned 127 because the
executable was unavailable. `conductor check` also returned 127. The
fallback was the pinned source-file, active-item, session-log, and
related-PR review above, plus the catalog’s primary literature. This is
not an exhaustive KB review or an absence proof.
