#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
#set heading(numbering: "1.")

= Twist-derived Frobenius symmetry in exact index calculus
21 October 2026 · Full research record: `GLS_SYMMETRY_REPORT.md`

== Question and scope
We tested whether a quadratic twist supplies a Frobenius symmetry that can reduce relation-matrix work for a discrete logarithm already on the twist subgroup. The exact signed width-three oracle and physical factor base were identical in the flat and folded arms. Three curves, three base sizes, and three one-target DLPs per cell gave 54 index-calculus solves and 27 matched rho controls. This panel did not implement a Semaev polynomial-system solver; the repository's earlier E2b line oracle did implement a Weil-descent $S_3$ resultant. The subsequent quartic twist24 study tests an order-24 combined fold and a two-line relation-yield comparison.

== Algebraic result
For the GLS twist over $F_(p^2)$, let $alpha = c^((1-p)/2)$ and $psi(x,y) = (alpha^2 x^p, alpha^3 y^p)$. Since $alpha^(p+1)=-1$, $psi^2(x,y)=(x,-y)$. On the selected cyclic subgroup, $psi(G)=[lambda]G$ and $lambda^2=-1 mod r$. The independent verifier checked all subgroup points.

For a physical pair $P,psi(P)$ with coefficients $a,b$, the folded coefficient is $a+lambda b$. Conjugation changes $(a,b)$ to $(-b,a)$ and the folded coefficient to $-b+lambda a=lambda(a+lambda b)$. The right-hand side changes by the same factor. Hence four conjugate relations yield one folded rank direction. Certificate `IDC1hfa7f1de676984669` seals the polynomial part; the native audit checks the subgroup premise and every signed triple.

For scaled abscissa $X=x/c$, the action is $X -> X^p$. The explicit third Semaev polynomial satisfies $S_3(a c^2,b c^3; c X_1,c X_2,c X_3)=c^4 S_3(a,b; X_1,X_2,X_3)$. Thus simultaneous Frobenius of all relation variables preserves zeroes, while independently conjugating one variable need not. On the actual $p=211$ twist relation $G+[2]G-[3]G=O$, a one-factor conjugation changes $S_3$ from zero to $(136,208)$ in $F_(211^2)$. This is an algebraic symmetry, not a measured Gröbner cost reduction.

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

== Count and time diagnostics
#image("gls_fold_cost_v2.png", width: 100%)

The chart shows flat/folded recorded $F_(p^2)$ multiplication-count ratios for the cold one-target v2 solves. There are three target scalars per cell; setup and collection follow one deterministic trajectory per cell. Ratios on $p=211$ are 1.638, 1.317, and 1.117 for $B=12,24,36$. At $p=101$ and $p=307$, base and table construction dominate and the ratio is approximately one. Rho uses fewer recorded $F_(p^2)$ multiplications in every cell.

== Equal-column post hoc comparison
Flat $B=12$ and folded $B=24$ each have 12 log unknowns and use the same three targets per curve. A new orbit-distinct flat control takes one representative from each of the same 12 orbits used by the folded 24-point base. It charges the same orbit scan and geometry checks; folding additionally charges eigenvalue identification. Separate native arithmetic replay verifies the complete signed-triple census, collection stop, descent query counts, and all nine control scalars.

#table(
  columns: (.5fr, 1.2fr, 1.2fr, 1.2fr, .8fr),
  inset: 4pt,
  [$p$], [Covered flat → fold], [Queries to rank flat → fold], [Cold $F_(p^2)$ mul flat → fold], [Flat/fold],
  [101], [398 → 400], [7 → 1], [14,259 → 16,565], [0.861],
  [211], [1,658 → 13,708], [860 → 37], [115,865 → 56,704], [2.043],
  [307], [504 → 508], [5 → 1], [98,842 → 101,253], [0.976],
)

#image("gls_equal_columns_v2.png", width: 85%)

The $p=211$ gain is a sparse-relation case: coverage increases from 3.75% to 31.00%; median group additions fall from 23,347 to 3,615. At $p=101,307$, near-saturated coverage cannot pay for the larger base. Across all targets, raw witness counts rise about 9.2-fold because $binom(24,3)/binom(12,3)=9.2$; this is more candidates, not a higher hit probability per candidate. The comparison was chosen after the original run and does not benchmark a polynomial solver. Same-target rho remains cheaper in counted multiplications.

The v2 wall times are shared-host L0 descriptions, not isolated benchmark verdicts. Counted group additions contain field multiplications and inversions; summing these counters would double-count. Field additions, inner base-field arithmetic, hashing and allocations are not exhaustively priced. The required complete group-addition-equivalent cost model, isolated timing, polynomial-system solver, external fastest-implementation comparison, and scalable cryptanalytic claim remain open. Visual copies remain in `benchmarks/20261021-gls-twist-index-calculus/`, but this study is held out of the admitted benchmark table under the current `VALIDATION.json` gate.

The toy producer finds $lambda$ by a charged linear scan for $x^2=-1 mod r$ and checks the subgroup action. At large prime order, an implementation should compute the modular square roots and test both; that implementation was not benchmarked here.

== Consequences for relation finding
- Folding reduces unknown logs from $B$ to $B/2$ in this GLS subgroup. It does not increase the number of raw hits at fixed physical base.
- At a fixed 12-column budget, the doubled orbit-complete physical base improves coverage and collection on the sparse $p=211$ cell; cold counted cost improves there but worsens in the two saturated cells.
- Applying $psi$ to one relation yields three more witnesses, but no new folded rank. This was checked on all signed triples.
- An orbit-aware folded scheduler could remove only 2 of 254 redundant queries in the sparse $p=211,B=12$ prefix (0.79% of queries); the exact oracle returns all witnesses and conjugate folded rows are scalar multiples. The 15 of 829 flat repeats may add rank and are not safely removable. No rescheduled solver was timed.
- For rational $E_0(F_p)$ targets, the earlier mixed-component projection theorem still blocks independent odd-order twist columns under its assumptions. This experiment does not transport such a target to the GLS twist.
- A Semaev/Gröbner implementation must separately test whether an invariant polynomial presentation improves solve time, degree, success rate, and descent cost. The exact group oracle cannot answer that question.

== Evidence and proof boundary
Frozen protocol: `GLS_SYMMETRY_PROTOCOL.md`. Raw runs: `gls_symmetry_results_v1.ndjson`, `gls_symmetry_results_v2.ndjson`, `gls_symmetry_results_v2_aa.ndjson`. Native replay: `gls_symmetry_verify.rs`; native summary: `gls_symmetry_summarize.rs`. The abstract subgroup and folding lemmas compile in `GLSSymmetry.lean` with printed axioms inspected. The concrete finite-field formula is verified by native exhaustive replay, not formalized in Lean. Full detail and source citations are in the accompanying Markdown report.
