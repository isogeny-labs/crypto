# Amendment: staged resultant certificate backend

Date: 2026-11-25. Parent protocol: `PROTOCOL.md`. This amendment precedes
any qualifying run of the new backend. The earlier F97 in-memory eliminations
were exploratory diagnostics and do not enter the frozen panel.

The map, three-summand witness contract, independent-curve selection,
controls, target list, cold setup accounting, rho comparison, and resource
limits remain those in the parent protocol. The generic four-variable
Groebner root call reached the 60-second cap at `p=97,t=3` before any prefix
split, so this backend replaces only the inconsistency test.

For a prefix class polynomial `A_i(x_i)=x_i^(2^(t-k_i))-c_i`, compute

1. `H(u)=Res_x3(A_3(x3), S3(u,x3,x(R)))` over `F_p`.
2. `J(x1,x2)=Res_u(H(u), S3(x1,x2,u))` over `F_p`.
3. `K(x1)=Res_x2(A_2(x2), J(x1,x2))` over `F_p`.
4. `G(x1)=gcd(A_1(x1), K(x1))` over `F_p`.

`G=1` is a checkable algebraic-closure inconsistency certificate for that
prefix. If `G` is nonconstant, factor it over `F_p` to extract candidate
`x1`. For each such root, use `gcd(A_2,J(x1,x2))` to obtain `x2`; then
`gcd(H,S3(x1,x2,u))` for `u`; then
`gcd(A_3,S3(u,x3,x(R)))` for `x3`. Factor only those gcds. Lift point signs
and accept a result only after the exact curve group-law sum equals `R`.
Every resultant, reduction, gcd, factorization, zero polynomial, repeated
root, failed lift, and spent timeout is recorded and charged. A nonconstant
`G` with no verified point triple remains a failed query, not a relation.

The mathematical resultants are reduced over `F_p`. The initial SymPy
diagnostic implementation computes expression resultants over the integers
and reduces their coefficients modulo `p`; this may incur substantial integer
coefficient growth. It uses a single 60-second per-target wall cap after
module import and initial setup. It must report process CPU, peak resident
memory and polynomial degree/term counts before a qualifying performance panel. The
2-GiB memory cap is imposed by the eventual external runner; until that
runner exists, outputs are explicitly diagnostic.

The expanded intermediate `J` can have quadratically many coefficients in
`B=2^t`. If that expansion dominates, this implementation fails the frozen
subquadratic prediction even if it returns valid witnesses at small `B`.
When `x(R)` belongs to the factor base, the intermediate identity-point
branch `(P,-P,R)` is handled separately and charged because `x(O)` is not
defined in the Semaev chain.

The falsifier for this backend is a censored `t=3` root call, an extracted
witness set that differs from independent enumeration at `t<=3`, or a
nondecreasing complete charged-cost ratio to the `B^2` control at `t=3,4,5`.
An algebraic witness extractor that passes these checks still needs the
parent protocol's independent-curve and cold same-target rho gates.
