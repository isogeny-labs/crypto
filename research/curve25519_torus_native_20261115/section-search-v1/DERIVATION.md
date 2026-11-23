# Implicit section queries and the coefficient-search gap

Derived construction awaiting independent/formal review. Classical ingredients
are identified in SOURCES.md; no literature novelty is asserted. [pcc:unverified]

## 1. Move the cofactor into the public target

Original columns have signed images [8]P(z) for indexed raw parameters z. A
six-term relation to Q is equivalent to sum P_i=R for some rational R with
[8]R=Q. Work through the complete rational fibre, not only one chosen inverse.
For a nonidentity prime-subgroup Q this is a finite nonramified constant-degree
group-map problem. Enumerating all rational kernel translates, their cost and
the subgroup/order hypotheses remain explicit obligations. No generic group
logarithm is used. This is classical homomorphism transport, not a new isogeny.

Write C(X)=X^3+A X^2+X and R=(r,s), s!=0. For coefficients c of the frozen
L(5O+R) basis set

    U=(X-r)(c0+c1 X+c3 X^2)+c5 s,
    V=(X-r)(c2+c4 X)+c5,
    S_c=(U^2-C V^2)/(X-r).

At X=r the numerator is c5^2(s^2-C(r))=0. Its leading term is -c4^2 X^7,
so S_c has degree6 when c4!=0; normalize c4=1 and make S_c monic. For the
line-bundle section f, g=(x-r)f has zero divisor Z+[-R], where deg Z=6 and
sum Z=R. The forced -R factor is removed by division by X-r. Thus S_c is the
x-pushforward of Z with multiplicities, including R/-R charts. This uses
the classical divisor/section argument, not merely evaluation of the norm at
r. If U,V share a factor, retain both sheets and determine their local orders;
do not infer the sign from an x root. [pcc:unverified]

## 2. Three support tests, retaining multiplicities

For a known coefficient vector work in A_c=Fp[X]/(S_c), dimension6. Require
that 2X+A is a unit; the missing chart x=-A/2 is outside the original map.
Put lambda=A/(2X+A) and u=T_q(lambda). Let omega=h^q=(a1,b1) and
omega^j=(a_j,b_j), with exact odd order a and 1<=j<=k, 2k<a. The original
parameter group must have the stated norm/order, and q is odd. Define

    H_J(X)= product_(j in J) (u^2-a_j^2).

The section support is in the rational admissible preimage domain exactly
when the following remainders vanish, subject to the stated map contracts:

    (X^p-X)^6=0,
    (C(X)^((p-1)/2)-1)^6=0,
    H_[1,k](X)^6=0                 in A_c.

The first makes every support abscissa rational. The second makes its curve
ordinate nonzero and rational. The sixth powers retain arbitrary root
multiplicities up to6: a polynomial F vanishes on the support of a degree6
polynomial S iff S divides F^6. Using only S|X^p-X silently deletes repeated
roots. These tests still require local section recovery and original raw-row
checks; they are not a target root finder. [pcc:unverified]

For each rational support x, lambda0=A/(2x+A). Since q is odd, the membership
test chooses lambda=lambda0 or -lambda0 with T_q(lambda)=a_j. Odd order a
and 2k<a make a_j^2 distinct and b_j nonzero. The old trace adapter gives

    b=(lambda^2-1) U_(q-1)(lambda)/(2 b_j),  z=lambda+b sqrt(2),

with norm z=1 and z^q=omega^j by the Chebyshev/Pell identities. Recover t
from z/h^j in <h^a> with the original bounded index decoder; charge its
setup and query work. Curve branch/sign, raw uniqueness, exclusion of
identity eightfold images, and public-target replay remain mandatory.
Geometric aliases after [8] are not discarded. The older guarded AD adapter
additionally requires q nonzero in Fp. No new native map proof is claimed.

## 3. Replace the coset product by geometric products

Adjoin v formally over A_c tensor Fp2 with

    v^2-(4u^2-2)v+1=0.

This is a monic extension, free of rank2 even if S_c has repeated roots.
In particular v is a unit, with inverse 4u^2-2-v. Let rho=omega^2. Then

    (v-rho^j)(v-rho^(-j))=4v(u^2-a_j^2),
    P_J(v;rho) P_J(v;rho^-1)=(4v)^|J| H_J,

where P_J(v;rho)=product_(j in J)(v-rho^j). Because 4v is a unit and the
extension is free, the zero and sixth-power zero tests are equivalent.
This avoids an invalid cancellation of a possible zero divisor. [pcc:unverified]

For the first identity the unconditional polynomial residual is

    (v-w)(v-wbar)-4v(u^2-c^2)
    = [v^2-(4u^2-2)v+1]
      -v[w+wbar-(4c^2-2)] + [w*wbar-1].

The new native evaluated certificate covers this residual identity only.
Substitution of rho^j, the extension's faithfulness and the whole algorithm
are separate steps. The unit-cancellation Lean endpoint covers the named
existing law, not this complete chain.

## 4. Exact construction and interval restriction recurrence

Use Bostan--Yurkevich Algorithm2, not a linear Horner evaluation hidden in
the query. For n=s^2, compute A(Z)=product_(j=0..s-1)(t-rho^j Z), evaluate
it at 1,rho^s,...,rho^((s-1)s) by the chirp convolution, then multiply those
s values. The result is product_(j=0..n-1)(t-rho^j). Non-square tails have
O(sqrt(n)) factors. Construct A by its homothety recurrence, not all n roots.
An interval starting L uses t=rho^-L v and a scalar rho^(Ln).

All polynomial work occurs in a fixed-dimensional Fp algebra (dimension24
including Fp2 and v). Chirp inversion is only of known nonzero scalar powers
of rho. There is no inversion of t, of a factor t-rho^j, or of an unknown
coefficient. For a supplied c, this has O(M(sqrt(n))+sqrt(n)) algebra work,
including scalar multiplications and construction. A quasi-linear native
polynomial backend gives the stated soft-O(sqrt(n)) field bound; a naive
convolution would only give O(n), and is not an implementation of that bound.
The transfer from the cited field algorithm to this finite free algebra is
by its explicit addition/multiplication/scalar-unit operations, not by
pretending the nonreduced algebra is a field. [pcc:unverified]

Recover H_J by multiplication by (4v)^(-|J|) and projection onto the base
algebra; verify other free-basis components are zero. Define
G_J=gcd(S_c,H_J^6). Splitting J into two intervals partitions support primary
factors, because labels have distinct a_j^2. Descend only nonconstant G_J.
There are at most6 occupied label paths per level; the sum of their square-root
costs is soft-O(6 sqrt(k)). Factor only degree<=6 polynomials and determine
local ordinate/multiplicity charts. This extracts witnesses for a supplied
section and preserves original checks. It does not select c. [pcc:unverified]

## 5. Cold cost and an exact rejection of one seed distribution

Let C_seed count every attempted coefficient vector, including failed tests,
and R_out count usable checked rows. With fixed section degree the derived
cost contract, conditional on a native fast-polynomial implementation, is

    W_setup + C_seed * soft-O(sqrt(k)+log p+log q)
      + R_out * soft-O(sqrt(k)+sqrt(q)+log p)
      + W_private_selection + W_kernel + W_recovery.

W_setup includes public fibre construction and decoder tables. Factoring
degree<=6 polynomials uses its stated expected finite-field cost; no worst-case
rootfinding bound is inferred. Memory includes polynomial scratch of
soft-O(sqrt(k)) field words, BSGS tables and all retained relation rows. In a
declared growing family B~qk, k=B^theta and C_seed=B^chi, the seed-testing
phase has exponent chi+theta/2 in B. It needs chi+theta/2<3, plus all other
cold phases and useful yield. No such seed-generation bound has been found.

Uniform c in the normalized chart c4=1 has p^5 choices. A useful finite
degree6 zero divisor uniquely determines the section up to scale: two such
sections have constant ratio on the projective integral curve. For M signed
preimage points, there are at most binomial(M+5,6) zero divisors, with
repetitions allowed. Hence for any fixed R

    Pr[uniform c is useful] <= binomial(M+5,6)/p^5.

At M=O(p^(1/6)), this is O(p^-4), before charging even one query. This
mathematically rejects uniform chart sampling as a route to the stated cold
bound under this model. It does not reject biased coefficient generation,
constraints solved algebraically, other bases or index calculus. Averaging
over uniform public targets in a prime image group of size Theta(p) adds
another inverse-p factor to the success probability, when every divisor is
charged to its unique image target. No distributional experiment or typical-
target theorem is claimed. [pcc:unverified]

The next exponent-moving obligation is therefore a target-conditioned,
strongly biased coefficient constructor with original witness extraction,
not uniform seeds plus a faster predicate. This round specifies its inputs,
query/restriction cost and falsifiable seed budget; it does not implement it.
