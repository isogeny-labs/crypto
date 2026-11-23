# Exact simple-high-root lift, conditional k=1

Write u=1/Xand T=X^(d-1)Z. Normalize F(Z,u)=u^(5(d-1))H(u^(-(d-1))Z).
The k1leading stratum gives F(Z,0)=Z^4(Z+1). At Z=1the derivative is1,
so a unique formal root r=1+sum a_n*u^nexists. Once coefficients below n
are fixed, adding a_n*u^n changes coefficient n of Fby exactly a_n. Thus
a_n=[u^n]F(r_<n,u), with no independent root selector introduced.

Compute only n<d. It corresponds to one candidate x=X^(d-1)r(1/X)in the
upper prefix half. This candidate is a genuine polynomial root iff the FULL
F(r,u)vanishes. Its degree is at most5(d-1); low coefficients already vanish,
so requiring the remaining tail coefficients is necessary and sufficient.
The result is exact high-root existence/recovery, not allfive-root support.
Repeated low roots are allowed; the high root is simple by its leading count.

Independent literal high-half enumeration verifies existence and uniqueness;
generic polynomial evaluation is independent of the Boolean Frobenius-based
circuit. Exhaustive small Boolean families compare every recovered root/tail.
Actual coefficient normalization uses raw HC0+M*qand the same original q.
No reduction moduloM is performed before reversing coefficient bits.

Onecore4GiB90souter,78/84sfinalization. No hard search or full339-variable
root completeness claim. Hensel/power-series lifting is known prior art;
its use on an unknown affine coefficient family does not inherit a solved
parameter-search complexity bound.
