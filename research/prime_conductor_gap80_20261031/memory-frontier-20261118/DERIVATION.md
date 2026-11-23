# Conditional source-model derivation

This concerns the pinned complete_pipeline.cpp traversal and its independent
coefficient arrays. It is not a lower bound for all isogeny algorithms. Let
n=(ell-1)/2>=2, 1<=B<=floor(n/2), G=floor(n/(2B)), L=n-2BG. Then G>=1
and 0<=L<2B. Let z be the positive block size; b=min(B,z), g=min(G,z),
i=ceil(B/z), j=ceil(G/z), and T=i*j.

## The two mandatory phases

In baby_product, RootVector owns b fmpz slots while the returned monic h
owns at least b+1 coefficient slots. Their logical source lifetimes overlap:

    R = 2b+1.

In the giant loop the infinity channel is always constructed. Its zeroth
jet is product_k (Z-x_k)^2. Each factor is monic quadratic, so the product
is monic of degree 2g in every field. At norm evaluation it coexists with
h, which remains owned through all giant blocks. Therefore this phase has
at least

    H = (b+1)+(2g+1) = b+2g+2

separate coefficient slots. PreparedNorm's data, other jet coefficients,
bins, multiplication scratch, roots' big-integer payloads, points and all
allocator overhead are excluded. Neither R nor H is measured RSS. The
necessary modeled peak is max(R,H); a bound that fits is not sufficient.

## A cap and all windows

Let C=floor(cap_bytes/sizeof(fmpz)) and M=floor((C-1)/2). If a complete
execution fits, R<=C gives b<=M; H<=C and b>=1 give g<=M. Thus B<=iM
and G<=jM. A cap too small to allow a positive chunk has no candidate.

The current source processes B baby coordinates, iG giant coordinates
(regenerated for each baby block), and L direct leftovers. Put W=B+iG+L.
From iM>=B and n=2BG+L,

    2MW >= 2MB + 2BG + 2ML
         = n + 2MB + (2M-1)L >= n+2M.

Consequently W>=1+ceil(n/(2M)). These are loop-context counts, not distinct
points, group operations, field-operation totals or elapsed times. Caching
all giant contexts or changing the traversal invalidates this source pin.

For rectangles, n<2B(G+1)<=2iM(jM+1)<=2TM(M+1), hence

    T >= floor(n/(2M(M+1)))+1.

This includes every allowed B, all direct leftovers and every block size
whose necessary phase bounds fit. It is stronger in scope than optimizing
one balanced window, but remains conditional on these source loops/objects.

## Exact optimum of the single-rectangle necessary bound

For one rectangle b=B and g=G. Define P(B)=max(2B+1,B+2G+2). To test
whether some B has P(B)<=C, impose B<=floor((C-1)/2), B<=floor(n/2).
The second phase is equivalent to

    floor(n/(2B)) <= floor((C-B-2)/2),
    n < F(B) := 2B floor((C-B)/2).

On either parity let epsilon=(C-B) mod 2. Then F(B)=B(C-B-epsilon) and
F(B+2)-F(B)=2(C-2B-epsilon-2)>0 whenever B+2 remains in the allowed
interval. So checking its two rightmost B values covers both parity maxima.
The feasibility predicate is monotone in C. Binary search between 0 and
2n+3 finds the exact minimum of P; that upper endpoint is feasible using
B=1. The reported witness is rechecked with actual G and P.

This optimizes only a necessary logical slot bound. It neither minimizes
elapsed work nor establishes enough physical RAM. Its L may differ from
the original explicit zero-leftover choice; those workloads remain distinct.

## Validation standing

The accompanying native checker compares against exhaustive enumeration of
all B for bounded synthetic n and checks all-window bounds in explicit
blocked loops. A second native checker constructs existing h and zeroth jet
objects and checks their monic degrees and coexistence. These are development
checks of the derivation/source binding, not an independent signed theorem
verdict. No Lean/lake executable is exposed. Producer claims remain
unverified/not_covered until an independent applicable verdict exists.
