#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= R70: affine order reaches the integral closure
The complete descending Frobenius chain on the rank-32 level-77 source
order has binary quotient lengths $160, 80, 32, 8, 4, 2, 0$. The last
two-class square map has 24 binary rows and rank two. Its first two
rows are $(1,0)$ and $(1,1)$, a determinant-one witness that the kernel
is zero. The sixth lattice is checked as the old order $O$ by a full
32-by-32 field-coordinate identity and all 32 old-order basis squares.

#image("frobenius70.svg", width: 100%)

== Why this proves affine normality
The retained rank-32 order has all 528 source multiplication columns
and an actual degree-32 field identification. Its nondegenerate trace
Gram gives a trace-dual over-lattice $V_0$ containing every element
integral over $F_2[z]$. The integral closure is square-closed, so it
lies in each $V_i$ by induction. As $V_6=O$, it lies in $O$; conversely
$O$ lies in its integral closure. The corresponding conditional
six-step implication is kernel-checked in Lean. The native source
identities and trace-dual containment are the stated external premises.

== Full attempt record and boundaries
The first native attempt is rejected despite a late marker: zero-column
Singular errors occur after rank two. Fresh attempt 02 passes exit 0,
the exact marker, zero error lines, and an explicit two-row witness.
The first Lean compile failed on a field instance and set inclusion;
the repaired second run compiled and audited its four authored claims.
The third run added the direct six-step implication and audited 12
declarations with zero failing and zero trusted. All five attempts,
including failures, are bound to historical source bytes in results.json.

This is the affine $F_2[z]$ chart. Proper branches, valuations of
$b,c,lambda,nu$ and denominator functions at the omitted finite fibers
and infinity, the level-77 map, and its order-11 action remain next.
The general descending method is described by
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
