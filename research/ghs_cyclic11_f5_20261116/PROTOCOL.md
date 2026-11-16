# Cyclic-11 genus-five trace screen over F5

16 November 2026. Literature was checked before execution: cyclic
three-branch covers, their automorphisms, and Jacobi-sum point counts are
established methods (Kallel--Sjerve 2005; Álvarez 2011). Iijima--Momose--Chao
2015 classifies a specified `(2,...,2)` GHS cover construction in odd
characteristic; its stated completeness is within that construction. A
bounded exact-equation web search did not identify this proposed F5 trace
table, but absence of a match does not establish priority.

Screen the 90 rational Kummer equations
`C_(a,b)/F5: y^11=x^a(x-1)^b` with `1<=a,b<=10` and
`a+b != 0 (mod 11)`. Each has three geometrically branched rational
points and genus five by tame Riemann--Hurwitz. For `1<=r<=4`,
`gcd(11,5^r-1)=1` and `#C(F_(5^r))=5^r+1`. The L-polynomial is therefore
`1-t*T^5+3125*T^10`, with `t=(3126-N5)/5`. Count `N5` exactly over
F3125 using an order-11 multiplicative-residue histogram, including
the three rational branch fibers. Require integrality and Hasse bound
`t^2<=4*3125` for every row.

Mark `5∤t` as ordinary elliptic-eligible over F3125. For eligible rows,
test descent from F5 using `s5=s^5-25*s^3+125*s` for every elliptic trace
`s` with `|s|<=4`. A non-descended factor is an isogeny-class lead only:
it does not give an evaluated subgroup map, a complete attack, or a
vulnerability. Directly recount one representative of every distinct
trace and seek prior exact equations before promoting any candidate.

The source and this protocol are committed before the screen. Run it only
after the isolated s=7 same-target rho benchmark ends.

Primary sources:
- Kallel--Sjerve, *On the group of automorphisms of cyclic covers of the Riemann sphere* (2005): https://math.univ-lille.fr/~kallel/Papers/pcyclic.pdf
- Álvarez, *The p-rank of the reduction mod p of Jacobians and Jacobi sums* (2011): https://arxiv.org/abs/1111.0545
- Iijima--Momose--Chao, *A classification of elliptic curves with respect to the GHS attack in odd characteristic* (2015): https://eprint.iacr.org/2015/805.pdf
