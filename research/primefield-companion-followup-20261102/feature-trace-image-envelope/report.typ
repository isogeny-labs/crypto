#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Power images from quotient traces
T-1819 · 2026-11-07 · native producer evidence

Compute the stable envelope from the smaller candidate polynomial G. In the quotient algebra, let $B=X^m mod G$. Its power traces give the image moments. Reconstruct the monic image-multiset polynomial C, remove repeated factors with C/gcd(C,C'), and apply zero-corrected inflation. Candidate roots are never enumerated at runtime.

#image("flow.svg", width: 100%)

*Blocked projection.* Group quotient powers into short and giant powers. Trace pairings are Hankel convolutions using the first2n-1 power sums of G. Scalar dot products combine each block. Projection work is O(sqrt(n) M(n)+n²), with O(n^(3/2)) short-table coefficients. Initial X^m mod G, conversion, squarefree reduction, final envelope output and parent verification remain charged separately.

*Exact checks.* Sequential and blocked construction agree on6,522 subsets;5,842 include colliding images and3,258 contain zero. Whole polynomials and moments match independent schoolbook references. Six degree-r cases reject; $n<r$ is required by moment conversion. Three larger cases at r65,537 use(n,m)=(8,128),(16,64),(32,32). Repeated-root and degree/workspace caps reject.

*Public integration.* All63 inherited public recoveries match lcm-envelope recovery in scalar, route and query fields. Source calls remain511. The existing stable-envelope and ordinary-replay controls remain active. Source domain and original4,563 cohort are unchanged.

*Scope.* This is a checked alternative envelope compiler, with sequential and blocked implementations. No total-work ranking or timing result is asserted. Keep input degree n, output degree N, orbit width and workspace separate. Producer statements are unsigned; no full formalization or independent signed verdict.

*Evidence:* validation-01 source snapshot; final validation-02 receipts/raw output; generation.json; DERIVATION.md; trace.rs and block-projection.rs.

*Next:* derive a degree/workspace-aware selection rule and freeze complete accounting before comparing construction paths. Faster power projection remains an independent backend opportunity.

Sources: FLINT nmod_poly power sums/conversion; Brent–Kung1978 section2.1 (maths-people.anu.edu.au/brent/pd/rpb045.pdf); local fast moment and envelope components. KB unavailable; primary/local fallback recorded. Trace reconstruction and power blocking are established algebraic methods.
