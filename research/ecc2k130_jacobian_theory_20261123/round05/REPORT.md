# Hyperelliptic genus130 excluded; genus131 reduced to one class

2026-11-23. Same ECC2K-130 factor A and draft PR5079. Scope:
hyperelliptic curves over F2, including every isogeny representative.
Written arguments and native arithmetic below are unsigned; they have
not received independent signed verification. [pcc:unverified]

## Findings

No genus130 hyperelliptic C/F2 has Jacobian isogenous to A. Such a
Jacobian predicts exactly2 F4-points on C, whereas every degree2 cover
of P1/F2 has at least3 F4-points. This does not require any integral
endomorphism, map-degree or natural-representative assumption.
[pcc:unverified]

For hyperelliptic genus131, if J(C) is isogenous to A*E with E elliptic,
only trace(E)=-1 survives. All other four elliptic classes are excluded
below. The survivor is the Weil-restriction class Res(E0), and it must
be ordinary with one rational branch point and one branch orbit of
length131. Survival is not existence. [pcc:unverified]

This gives a concrete normalized equation family for every surviving
hyperelliptic candidate. One specified sparse member matches the
required counts through degree5 but fails in degree6:53 points versus
the required65. This rejects only that member. The general
nonhyperelliptic genus130 and genus132–300 questions remain open.
[pcc:unverified]

## 1. An elementary fiber obstruction

Let pi:C->P1 be a finite degree2 cover over Fq. Each of the q+1
rational base points has a nonempty degree2 fiber. Every closed point
in that fiber has residue degree1 or2, so all of its support is
rational over F(q²). Different base points give disjoint fibers.
Therefore

    #C(F(q²)) >= q+1.

If there are b rational branch points, the sharper count is
#C(F(q²))>=2*(q+1)-b. The weaker bound suffices here. Also for every m,

    #C(F(q^m)) <= 2*(q^m+1).

These statements count support points, so wild ramification causes no
exception. A geometrically hyperelliptic curve over a finite field also
has such a map: its unique involution descends, and the genus0 quotient
has a rational point over the finite field, hence is P1. [pcc:unverified]

For E0:y²+xy=x³+1 put s0=2,s1=-1 and
s_(m+2)=-s_(m+1)-2*s_m. The trace-zero factor has

    Tr(F^m|A) = -s_m                 if131 does not divide m,
    Tr(F^m|A) = 130*s_m             if131 divides m.

Since s2=-3, a genus130 curve with this Weil polynomial would have
N2=4+1+s2=2. The fiber lower bound gives N2>=3. Isogenous abelian
varieties have the same Weil polynomial, so this excludes every
hyperelliptic Jacobian in the entire A isogeny class. It goes beyond
the parent integral-zeta obstruction, but does not exclude a
nonhyperelliptic Jacobian. [pcc:unverified]

## 2. Every genus131 hyperelliptic elliptic complement

A dimension1 complement over F2 has trace t in{-2,-1,0,1,2}. Write
b0=2,b1=t,b_(m+2)=t*b_(m+1)-2*b_m. Then
N_m=2^m+1-Tr(F^m|A)-b_m. In particular

    N1=2-t,   N2=6-t²,   N3=14-t³+6*t.

For t=2 or-2, N2=2 violates the degree2 fiber lower bound. For t=1,
N3=19 violates N3<=2*(8+1)=18. These are independent of endomorphism
orders and induced elliptic-cover degrees. [pcc:unverified]

For t=0 use the branch set W of the hyperelliptic map. The
characteristic-two facts needed are:

- #W=f+1, where f is the Jacobian's2-rank.
- If c_d is the number of Frobenius branch orbits of length d, then
  #W(F(2^m))=sum_(d|m) d*c_d.
- N_m is congruent to #W(F(2^m)) modulo2.
- Odd ramification breaks d_w satisfy
  g+1=sum_(w in W)(d_w+1)/2.

The first three are the branch-count/parity facts in Borodin–May §2.1;
the ramification formula is also recalled by Elkin–Pries §1. These are
theorems and elementary counting statements, not the conjectures in
the Borodin–May paper. [pcc:unverified]

A is ordinary of2-rank130. The trace0 elliptic class has2-rank0, so
f=130 and #W=131. Native coefficient-parity calculations replay both
ranks. Here N1=2 is even, while N131 is odd: s131 is odd and b131 is
even. As131 is prime,

    #W(F(2^131)) - #W(F2) = 131*c131.

The parity difference makes c131 odd. With only131 geometric branch
points, this forces exactly one131-cycle and no other branches.
Frobenius preserves the ramification break, so every branch has the
same odd break d. The genus formula then requires

    g+1 = 131*(d+1)/2.

But g+1=132 is not divisible by131. This excludes trace0, including
isogenous representatives losing maximality atell; that escape from
round03 does not affect this hyperelliptic obstruction. [pcc:unverified]

## 3. Necessary normal form for the survivor

Only t=-1 remains, hence E is in the E0 isogeny class. The full Weil
polynomial is

    P_C(X)=P_A(X)*(X²+X+2)
          =X^262-s131*X^131+2^131.

It predicts N_m=2^m+1 whenever131 does not divide m. Both factors are
ordinary, so f=131 and #W=132. Now N1=3 is odd and N131 is even;
the same parity argument forces a131-cycle and the remaining single
rational branch. Since #W=g+1, every ramification break is1.
[pcc:unverified]

Move the rational branch to infinity. Reduced Artin–Schreier form
therefore gives every such hyperelliptic curve an equation

    y²+y = x+b+R(x)/Q(x),
    b in F2; Q monic irreducible of degree131;
    0 != R in F2[x], degree(R)<131.

Nonzero R is automatically coprime to Q. The poles are simple and
are precisely infinity and the131 conjugate roots of Q. Conversely
these presentations give ordinary genus131 curves with this branch
shape; that converse does not assign them the required Weil polynomial.
[pcc:unverified]

There are (2^131-2)/131 choices of Q and2^131-1 choices of R.
Including b, the exact number of normalized presentations is

    2*(2^131-2)*(2^131-1)/131
      =113140362002873839803183863214595818360322324302212115654994876121039356639404.

This counts presentations, without quotienting by isomorphisms. It is
not a runtime estimate or an executed enumeration. [pcc:unverified]

## 4. A specified curve fails after five matching extensions

The frozen fixture uses b=0,R=1 and
Q=x^131+x^13+x²+x+1, the registered source-field polynomial. Native
binary polynomial arithmetic and a separate PARI calculation both
check irreducibility. Thus this is an actual ordinary genus131 curve,
with the necessary branch shape, not merely a proposed Weil polynomial.
[pcc:unverified]

| Extension degree | Fixture points | Required for A*E0 |
|---|---|---|
|1|3|3|
|2|5|5|
|3|9|9|
|4|17|17|
|5|33|33|
|6|53|65|
|7|115|129|
|8|337|257|

The degree6 mismatch rejects this curve as a carrier. The checker
constructs each small finite field from a trial-checked irreducible
polynomial. It counts the fixture both by absolute traces and by
enumerating the quadratic equation's y values. E0 is checked separately
in its original Weierstrass equation against the trace recurrence and
its Artin–Schreier presentation. The optimized and UBSan builds give
identical output. These are unsigned producer checks. [pcc:unverified]

## What remains

The genus130 hyperelliptic realization is excluded by the written
fiber argument. General genus130 Jacobians may be nonhyperelliptic;
the parent polarization/lattice questions remain relevant there.
Genus131 hyperelliptic carriers are reduced to the displayed A*E0
class and normal form, but none has been constructed with that class.
The one fixed curve above is a negative example, not a general search.
No subgroup map or attack cost is supplied. The2^37.17 model cannot
be realized through the excluded genus130 hyperelliptic case; it is
not thereby a proved cost for another curve type or genus.
[pcc:unverified]

Sources: Matvey Borodin and Liam May, arXiv2508.16886, §2.1 and §2.4:
https://arxiv.org/pdf/2508.16886. Arsen Elkin and Rachel Pries,
Algebra and Number Theory7(2013), §1:
https://msp.org/ant/2013/7-3/ant-v7-n3-p01-s.pdf.
The source A and prior nonhyperelliptic scope are pinned in the parent
REPORT.md and round03/REPORT.md. This round does not modify them.
