#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Weighted-symmetry pivot primitives
2026-11-02 · Derived algebra / syntax checks · Numerical trials: zero

*Actual compiler.* Its weighted Gram matrix B is symmetric and
A=D⁻¹JBJ, D=diag(1,γ,…). Thus DA is symmetric. The source constructor
already returns the inverse diagonal. Finite-field weights need not be positive.

#image("pipeline.svg",width:100%)

*Complete local pivot rule.* Find the first nonzero column j. Use its diagonal
if nonzero. Otherwise choose i with Aᵢⱼ≠0 and query column i. The metric gives
dᵢAᵢⱼ=dⱼAⱼᵢ, so the reverse entry is nonzero. Use diagonal i if nonzero;
otherwise the principal pair is (0,u;v,0), with determinant −uv≠0 and inverse
(0,1/v;1/u,0). This costs at most one extra column query. All-zero columns
certify a zero source, with RHS compatibility still to be decided.

*Closure.* Simultaneous permutations give A′=PAPᵀ and D′=PDPᵀ. Partition
A′=(E,F;C,H), D′=diag(D₁,D₂). For S=H−CE⁻¹F,
D₂S=D₂H−FᵀD₁E⁻¹F is symmetric. The child metric is the trailing portion
of D′. It need not remain a geometric progression. Arbitrary separate row
and column swaps do not supply this invariant.

*Border reuse.* A cached column cⱼ gives row entry
A′ⱼᵢ=dⱼ⁻¹ dᵢ cⱼ(i). The helper reconstructs leading rows with zero source
queries. It explicitly allocates rows; no complete memory reduction is claimed.

#table(columns:(1.7fr,2.5fr),inset:4pt,
 [Primitive],[Derived accounting],
 [Build metric / inverse],[1 inversion + 2(n−1) products],
 [Selection metric validation],[n products],
 [Selection queries],[j+1 or j+2; n on zero source],
 [Reciprocal-entry guard],[2 products on offdiagonal branch],
 [Each simultaneous swap],[4 source queries; width +4; 4 products; 2(n+1) additions],
 [Small inverse setup / apply],[cut inversions / cut products],
 [Reconstruct cut rows],[n+2n·cut products including validation; no query])

*Evidence scope.* Syntax and frozen dependency hashes checked; no numerical,
corruption-control or performance trial. Metric inverse checks and the local
reciprocal guard do not certify global symmetry. The invariant must come from
the public compiler or independent source certification. Cached selection has
internal one-use provenance. Existing frozen callers are unchanged.

*Next implementation.* Complete the symmetry-preserving solve/witness driver,
shared weighted block borders and compact block lifts. Preserve zero/singular
outcomes and separate feature-exclusion checks. Charge transport queries,
changing widths, metric storage and indices. Validate both scalar branches,
2×2 pivots, all permutations, inherited Schur metrics and all upstream cohorts
when cost admission permits. No numerical run was launched.

*Sources.* metric.py, selection.py, transport.py, check\_preparation.py and
DERIVATION.md. Dumas–Pernet (2018), arXiv:1802.10453, introduction/Section 2,
establishes exact-field 1×1/2×2 factorization context. Bostan et al. (2017),
DOI 10.1137/16M1062855, provides companion/Schur context. No rank-profile or
fast recursive complexity claim is inherited.
