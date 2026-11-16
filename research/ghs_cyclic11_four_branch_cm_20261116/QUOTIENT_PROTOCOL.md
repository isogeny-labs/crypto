# Frozen exact quotient identity check

16 November 2026. Extend the opposite-pair lead only for the exact F3 curve

`C: y^11 = x(x-1)/(x+1)`.

Check the involution `tau(x,y)=(2/x,1/y)`, invariant coordinates
`u=x+2/x`, `v=y+1/y`, and the rational identity

`f(x)+1/f(x) = u+1+1/u`, with `f=x(x-1)/(x+1)`.

The checker uses native Rust polynomial arithmetic over F3 to verify the
cross-multiplied identity coefficient by coefficient. It also computes the
degree-11 Dickson polynomial via `D0=2,D1=v,Dn=v D(n-1)-D(n-2)` and prints
its coefficients. The report will derive the fixed-point count and genus-five
quotient by Riemann–Hurwitz on paper. It will distinguish the verified
degree-two Jacobian pullback from an elliptic-source transfer, which is not
constructed by this check.

Freeze this protocol and source before execution. Retain compiler, source
hash, stdout, stderr, and any failure. No experiment parameters are to be
changed after execution; corrections require an amendment.
