# Frozen degree-seven rational-square denominator screen

16 November 2026 UTC. This is a second **partial** degree-seven search for the fixed `Y/F3` and the ten trace-`+26` targets over `K=F243`. It covers exactly `B=(x-c)^m` for `m=2,4,6` and `B=Qi*(x-c)^m` for `m=2,4`, where `c∈K`, `c!=-1`, and `Qi` is one of the five irreducible quadratic factors of `f`. The first three shapes have 242 denominators each; the other two have 1,210 each. No other denominator shape is implied by the result.

The rational branch factor `L=x+1` remains unpoled and is assigned to the unique rational target 2-torsion x-coordinate. The other unpoled quadratic branch factors receive all three Galois-compatible target 2-torsion values. The CRT modulus `M` has degree 11 when no `Qi` poles and degree 9 when one does. For each branch assignment h and each B, `A=(B*h) mod M` is the unique possible numerator of degree at most seven. To avoid scanning all 242 c values for every assignment, let `R_i=(x^i h) mod M` or `R_i=(Qi*x^i h) mod M`, respectively. The coefficient of x^d in A is the polynomial in c

`F_d(c)=sum_(i=0)^m binom(m,i) (-c)^(m-i) coeff_(x^d)(R_i)`.

Solve the common K-roots of all `F_d` for d>7. If every polynomial vanishes identically, scan all c; this fallback is part of completeness for these five shapes. Reject c=-1, reconstruct B and A, require `deg A=7`, `gcd(A,B)=1`, and exact `f*C²=B*(A³+2A²B+bB³)` with C polynomial. A hit must retain A, B, C and branch choices. Each target mask runs separately with source-bound stdout and timing. The previously frozen `DEGREE7_SPARSE_PROTOCOL.md` covers four other shapes and does not make this screen complete for degree seven.
