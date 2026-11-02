#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R64: compact-order bridge to the actual field
The retained rank-32 compact PID order is now bound to the actual R61
degree-32 source under $z=1/u$, $V=z²v=v/u²$, and old $lambda=l$.
Both old source equations reduce to zero in the actual quotient. The
source matrix identity $H M = D I$, where M is the saved source lift, holds before and after
coefficient inversion. All 32 compact basis functions map to nonzero
actual functions; their complete coefficient matrix has rank 32 and
a checked two-sided inverse. Altered quadratic and matrix entries fail.

#image("bridge64.svg", width: 100%)

== Full attempt record
#table(columns: (2fr,1fr,1.2fr,1fr), inset: 4pt,
[Attempt], [Status], [Wall seconds], [Error lines],
[bridge64‑01], [rejected], [2], [23],
[bridge64‑02], [rejected], [1], [22],
[bridge64‑03], [rejected], [1], [20],
[bridge64‑04], [accepted], [26], [0],
)
The first attempt cast a source module after switching rings. The second
mapped the module before casting, creating a zero fourth image. The third
tested the original identity on that changed representation. All three
are rejected despite late markers. Attempt 04 uses the original R53
matrix construction order and passes the strict gate. Wall intervals
are receipts, not controlled performance comparisons.

== Exact boundary and next step
The accepted run serializes both source equations, all 32 actual basis
functions, the 32 by 32 image matrix and its inverse. Four frozen raw
attempts and 24 historical byte pins are in results.json. The bridge
identifies the generic field represented by the compact order. It does
not certify that the order is integrally closed.

The next exact calculation starts with its trace dual. The retained
Gram discriminant predicts a finite quotient of F2 dimension 160. At
each descending Frobenius step, construct the complete binary square
map on that quotient, solve its full kernel and reconstruct the PID
lattice. Equality certifies affine normalization under the checked
source hypotheses. Then calculate branch valuations on both proper
charts for the level-77 map and subgroup action.

Primary normalization source:
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
