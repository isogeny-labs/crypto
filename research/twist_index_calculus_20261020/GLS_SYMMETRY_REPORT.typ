#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
#set heading(numbering: "1.")

= Twist-derived Frobenius symmetry in exact index calculus
21 October 2026 · Full research record: `GLS_SYMMETRY_REPORT.md`

== Question and scope
We tested whether a quadratic twist supplies a Frobenius symmetry that can reduce relation-matrix work for a discrete logarithm already on the twist subgroup. The exact signed width-three oracle and physical factor base were identical in the flat and folded arms. Three curves, three base sizes, and three one-target DLPs per cell gave 54 index-calculus solves and 27 matched rho controls. The experiment did not implement a Semaev polynomial-system solver.

== Algebraic result
For the GLS twist over $F_(p^2)$, let $alpha = c^((1-p)/2)$ and $psi(x,y) = (alpha^2 x^p, alpha^3 y^p)$. Since $alpha^(p+1)=-1$, $psi^2(x,y)=(x,-y)$. On the selected cyclic subgroup, $psi(G)=[lambda]G$ and $lambda^2=-1 mod r$. The independent verifier checked all subgroup points.

For a physical pair $P,psi(P)$ with coefficients $a,b$, the folded coefficient is $a+lambda b$. Conjugation changes $(a,b)$ to $(-b,a)$ and the folded coefficient to $-b+lambda a=lambda(a+lambda b)$. The right-hand side changes by the same factor. Hence four conjugate relations yield one folded rank direction. Certificate `IDC1hfa7f1de676984669` seals the polynomial part; the native audit checks the subgroup premise and every signed triple.

== Exact relation census
#table(
  columns: (1fr, .6fr, 1.1fr, 1.1fr, .8fr, .8fr),
  inset: 4pt,
  [Curve $p;r$], [Base $B$], [Covered targets], [Raw hits], [Flat rank], [Fold rank],
  [101;401], [12], [392/400], [1,752], [12/12], [6/6],
  [101;401], [24], [400/400], [16,152], [24/24], [12/12],
  [101;401], [36], [400/400], [56,980], [36/36], [18/18],
  [211;44,221], [12], [1,680/44,220], [1,760], [12/12], [6/6],
  [211;44,221], [24], [13,708/44,220], [16,192], [24/24], [12/12],
  [211;44,221], [36], [32,784/44,220], [57,120], [36/36], [18/18],
  [307;509], [12], [508/508], [1,756], [12/12], [6/6],
  [307;509], [24], [508/508], [16,160], [24/24], [12/12],
  [307;509], [36], [508/508], [57,012], [36/36], [18/18],
)

All raw relation sets are identical across arms. Every witness has a four-element orbit; retaining one representative per orbit preserves folded rank. The complete curve, relation, negative-control, and scalar audits passed for v1, v2, and the v2 A/A repeat.

#pagebreak()
== Count and time diagnostics
#image("gls_fold_cost_v2.png", width: 100%)

The chart shows flat/folded recorded $F_(p^2)$ multiplication-count ratios for the cold one-target v2 solves. There are three target scalars per cell; setup and collection follow one deterministic trajectory per cell. Ratios on $p=211$ are 1.638, 1.317, and 1.117 for $B=12,24,36$. At $p=101$ and $p=307$, base and table construction dominate and the ratio is approximately one. Rho uses fewer recorded $F_(p^2)$ multiplications in every cell.

The v2 wall times are shared-host L0 descriptions, not isolated benchmark verdicts. Counted group additions contain field multiplications and inversions; summing these counters would double-count. Field additions, inner base-field arithmetic, hashing and allocations are not exhaustively priced. The required complete group-addition-equivalent cost model, isolated timing, polynomial-system solver, external fastest-implementation comparison, and scalable cryptanalytic claim remain open. Visual copies remain in `benchmarks/20261021-gls-twist-index-calculus/`, but this study is held out of the admitted benchmark table under the current `VALIDATION.json` gate.

The toy producer finds $lambda$ by a charged linear scan for $x^2=-1 mod r$ and checks the subgroup action. At large prime order, an implementation should compute the modular square roots and test both; that implementation was not benchmarked here.

== Consequences for relation finding
- Folding reduces unknown logs from $B$ to $B/2$ in this GLS subgroup. It does not increase the number of raw hits at fixed physical base.
- Applying $psi$ to one relation yields three more witnesses, but no new folded rank. This was checked on all signed triples.
- A phase-aware query scheduler could avoid repeated target orbits. Among the recorded sequential prefixes, only $p=211,B=12$ has repeats: 15 of 829 flat queries and 2 of 254 folded queries. This is an upper bound on avoidable repeated queries, not a measured rescheduled solve.
- For rational $E_0(F_p)$ targets, the earlier mixed-component projection theorem still blocks independent odd-order twist columns under its assumptions. This experiment does not transport such a target to the GLS twist.
- A Semaev/Gröbner implementation must separately test whether an invariant polynomial presentation improves solve time, degree, success rate, and descent cost. The exact group oracle cannot answer that question.

== Evidence and proof boundary
Frozen protocol: `GLS_SYMMETRY_PROTOCOL.md`. Raw runs: `gls_symmetry_results_v1.ndjson`, `gls_symmetry_results_v2.ndjson`, `gls_symmetry_results_v2_aa.ndjson`. Native replay: `gls_symmetry_verify.rs`; native summary: `gls_symmetry_summarize.rs`. The abstract subgroup and folding lemmas compile in `GLSSymmetry.lean` with printed axioms inspected. The concrete finite-field formula is verified by native exhaustive replay, not formalized in Lean. Full detail and source citations are in the accompanying Markdown report.
