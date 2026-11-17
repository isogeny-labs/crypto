#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Exact two-sided generator compression
2026-11-02 · Derived construction and syntax-checked source · Zero numerical trials

*Purpose.* Remove redundant columns introduced by pivot and Schur boundary
corrections while preserving the same displacement matrix and prime field.
The construction uses no source-operator query and never forms the full
displacement matrix. It is a classical compression method with explicit
certificates and accounting for the current companion pipeline.

#image("pipeline.svg",width:100%)

*Preservation and minimality.* From G=UR and H=KRᵀ=VS, the returned
pair satisfies (USᵀ)Vᵀ=URKᵀ=GKᵀ. Since U has full column rank,
rank(GKᵀ)=rank(H)=t. Every width-w factor pair has product rank at most w;
therefore t is minimal. This includes cancellations between nonzero factors.
No estimate that t stays small after repeated pivots is assumed.

*Certificate.* Deterministic column elimination retains basis columns,
coordinates, pivot rows and original pivot columns. The selected basis-row
minor is unit lower triangular; the coordinate-column minor is upper
triangular with nonzero diagonal. The independent checker validates both
factor identities, both minor pairs, the intermediate product and outputs.
It imports no producer code. The field primality assumption is caller supplied.

*Derived arithmetic.* Let g=rank(G), t=rank(H), and qG,qH be sums of ranks
before each processed column in the two factorizations. With the existing
Meter convention (including initial-zero dot additions), the producer uses:

```text
products:   n (qG + g + qH + t + s g + g t)
add/sub:    n (qG + qH + s g + g t)
inversions: g + t
queries:    0
```

These are derived schedules, not measured counts. Pivot entry tests are
separate. Setup is classical O(n s²); certificates and copies are additional
storage. A smaller returned width saves work only if later query savings
exceed setup and any required verification cost, in the same counted unit.

*Zero rank and scope.* Width-zero factors are mathematically valid. An
explicit adapter supplies one zero column to legacy positive-width kernels;
encoding width one does not change certified rank zero. No frozen caller is
modified. Compression preserves displacement, not a pivot success guarantee.

*Validation next.* Full rank range, width greater than dimension, zero and
cancelling factors, nonmonotone pivot rows, reconstruction in both directions,
minor and identity mutations, exact counters and unchanged-width controls.
No execution under the currently exhausted daily budget.

*Sources.* compress.py, certificate.py and DERIVATION.md. Bostan et al.
(2017), DOI 10.1137/16M1062855, introduction, printed p. 756 and Section 6,
establishes the structured-matrix context and prior generator compression.
No novelty claim or fast-matrix-multiplication complexity is imported.
