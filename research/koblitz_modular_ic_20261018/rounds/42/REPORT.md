# R42: larger CM-block recovery and exact denominator lattices

This is an observations report for exact finite calculations. It is not an
independent evidence verdict or an index-calculus performance comparison.
The native runs use the actual retained n=11,83,131 subgroup moduli, not
convenient small replacements. The large n83/n131 moduli have 82/130 bits.
Large primality remains a retained input assumption; the exact integer
unit identities do not need it.

The retained R15 one-prime CM construction, R18 joint systems, and R39/R41
twist and source-action corrections informed the experiment. The primary
Molin–Page endpoint timed out during this round. This round asserts no new
modularity theorem. Historical carrier genera are 309,54781,45589 for
n11,83,131, not newly constructed low-genus covers.

## Explicit recovery

Put Phi=Phi_(2n)(T), u=T^i, v=T^j and
D=2(v-u)(2u²+3uv+2v²), with actual finite-CM exponents from R15.
The native residual computation enumerates all primitive embeddings and
both quadratic-diamond twists, with independently checked modular products.
A generator diamond and diamond-2 must be distinguished: diamond-2 has
order11/131 in those cases, so its nth power is not the order-two twist.

For s²=1, put q1=F²+s*u*F+2u² and q2=F²+s*v*F+2v². Then

    (u+2v-sF)q2 - (v+2u-sF)q1 = D.

The accepted global computation constructs actual polynomial inverses A
of D modulo (r,Phi), with degrees9,81,129 for n11/r991,n83,n131.
It exports exact integer certificates Dred*A=1+Phi*Q+r*H and
D=Dred+Phi*L. Multiplying the displayed identity by A therefore gives
identity recovery on the stated r-torsion primitive-character CM block.
Both integer quadratics and their opposite-annihilator factors are retained.
The three controls n5/trace±1/r11 and n11/trace-1/r23 remain obstructed.
Rational fixed-system separation alone does not settle global separation.

## Integral denominator lattice

The integer run builds every column of multiplication by D in the integral
basis1,T,...,T^(n-2) modulo Phi. Bareiss elimination records every exact
division. An independently evaluated norm formula checks its determinant.
The adjugate column is checked against every original matrix row.

| n | Integral dimension | Least scalar d for this denominator map |
|---|---:|---:|
|5|4|22|
|11|10|506|
|83|82|12302938186694|
|131|130|10095126166020618049226|

All four finite values match the prospective formula2n|U_n| when
 gcd(i-j,2n)=2 and2|S_n| when that gcd is1, where S,U are the retained
alpha²-alpha+2 trace/Lucas sequences. This is a checked finite pattern,
not a universal theorem. The full determinants and primitive polynomials
are in integer42-01/output/integer-recovery.json.

Here is the least-scalar argument for exactly this lattice. From the checked
adjugate solve obtain M*A=d*e0 and a weighted integer sum of d and all
coordinates of A equal to1. The nonzero determinant makes M injective over
Q. If M*z=h*e0 for an integral z, injectivity gives d*z=h*A.
Multiply the primitive weighted sum by h: every term is divisible by d,
so d divides h. Thus d is precisely the order of e0 in coker(M).
This does not prove that arbitrary combinations of geometric transfer
maps require d, or that arbitrary quadratic Bezout choices have this minimum.
The measured d values are units at all three larger target moduli, with
exact integer Bezout certificates; they are nonunits in all three controls.

## Formal scope and failures

The four accepted Lean workflows audit54 declarations: nine universal
recovery/projector implications plus22 literal unit identities; fifteen
actual larger-case polynomial identities/corollaries; the generic primitive
coordinate divisibility implication; and four literal primitive plus three
literal target-unit identities. The polynomial statements directly express
the coefficient congruences over arbitrary commutative rings.

Every accepted workflow uses Lean4.34.1, mathlibd13f23b, -j1 -M4096,
1200s compile/audit limits, and reports zero failing/trusted declarations.
No native_decide is used. Native matrix nonsingularity and the bridge from
the CM construction to geometric objects are not formalized by these lemmas.
Independent semantic review and independent scientific replay remain pending.

Two unsuccessful attempts are preserved. lean42-global-01 stopped before
compilation because the base audit receipt was not yet available; it contains
only the gate log. least-scalar42-01 rejected a missing finite-sum import;
the repair was committed before the fresh accepted attempt. Neither is a
mathematical negative. No cap increased and no accepted producer reran.

## Remaining construction obligations

This round supplies larger-size CM algebra, not an evaluated larger-size
point transfer, whole-Jacobian projector, chosen Koblitz model isogeny,
proper polarized embedding or full quotient saturation. The proper level77
map, actual pole divisors/Riemann–Roch space, node187 simultaneous valuations,
precision615 jets, complete452/1476/3012 spaces and ordinary degree460/367
divisors remain open. Their previous results have not been overwritten.
There is no measured IC relation rank, public-input recovery or speedup.

Reproduce with the committed REPLAY scripts and fresh attempt names, using
the pinned offline dependencies and original caps. results.json indexes all
attempts; the raw artifact contains complete native matrices, both operator
signs, logs and proof sources. Source byte custody is separate from scientific
replay. The figure plots recorded lattice dimensions and inverse degrees;
it is not a cost or attack scaling graph.

The first archive packaging attempt was rejected by the strict restore
checker because Cargo build products were hard-linked. Its digest and
source-pin record are retained under publication-attempts. The corrected
package expands hard links to ordinary files; it reuses identical raw runs.
This operational failure does not change a scientific outcome.
