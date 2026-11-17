# Congruence-supported pair moments: a finite-support reduction

2026-10-30 UTC. Pure symbolic accounting for a proposed two-axis packet
construction. No implementation or scientific computation. All conclusions
are conditional on the stated real-slot support and geometric coefficient
valuation premises. This is not a universal mode-count lower bound.

## Premises and coordinates

At selected level i, let `D=L/2^i`, and write tile coordinates as

    0<=n1,n2<N=D/2,
    n1 == n2 mod 2^k,   k=u-i-1>=0.

Assume `a>=u`, `L=2^a`, so N is a multiple of `2^k`. Introduce the integer
coordinates

    n=n2,   z=(n1-n2)/2^k.

The possible z values are in

    [-Q,Q],   Q=2^(a-u)-1,
    H=2Q+1=2^(a-u+1)-1=D/2^k-1.

The support need not be a Cartesian product in (n,z); this interval bound
alone suffices below.

Suppose a raw pair-moment term with exponents `(b1,b2)` has coefficient
valuation at least `b1+b2+2`, as in a product of two source geometric
expansions. Then

    n1^b1 n2^b2
      =sum_(h=0)^b1 binom(b1,h) 2^(kh) z^h n^(b1+b2-h).

With `j=b1+b2-h`, the coefficient for `n^j z^h` is consequently divisible
by at least

    2^(2+j+(k+1)h).

The binomial coefficient can give additional divisibility; ignoring it is a
valid conservative bound. Without any finite-support reduction, this new
mode index set has about `w/(k+1)` types at a large weight w. That particular
change of coordinates alone does not cancel an exponential `2^k` density.
It would be incorrect to promote this observation into a general barrier,
because a further exact reduction is available.

## Finite support removes high difference degrees

Define the monic integer polynomial

    R(Z)=product_(e=-Q)^Q (Z-e),   deg R=H.

Every real source slot satisfies `R(z)=0`. Reduce every `z^h` modulo R:

    z^h=sum_(ell=0)^(H-1) c_(h,ell) z^ell

on the real support, with integer coefficients. For `h<H` this is just the
original monomial. For `h>=H`, all retained ell are smaller than h. Hence
the old coefficient valuation `2+j+(k+1)h` is at least the advertised new
weight `2+j+(k+1)ell`. No division by a power of two occurs. Combining
different contributions cannot invalidate a divisibility lower bound.

Thus it is legitimate to use only moments

    M_(j,ell)(X)=sum_(real tile slots) n^j z^ell X_slot,
    j>=0, 0<=ell<H,

with weights `2+j+(k+1)ell`. At any one exact weight there are at most H
moment types per active tile. This is an exact representation on the real
support, not an assumption that formal moments are independent.

The construction is charged: R has H+1 integer coefficients of polynomial
bit length in the source parameters; all needed polynomial reductions and
coefficient regroupings can be performed modulo the working precision.
Only exponents below that polynomial working precision need appear.
Moments are evaluated from their integer coordinate descriptions. No
exponential table or unstated non-Boolean oracle is used in this local
algebraic reduction.

## A valid resulting counting envelope

Suppose the packet construction has at most

    T=O((M/D^2)*2^(u-i))
     =O((M/D^2)*2^(k+1))

active tiles of this kind, including a fixed number of colors. After the
finite-support reduction, the number of mode variables of any one weight
is at most

    T H=O(M/D).

The source's weighted-partition generating-product argument therefore gives
at most

    exp(O(sqrt((M/D)*p)))

tuples below precision p. When the recovery proof supplies
`p=O(MD/u)`, this is

    exp(O(M/sqrt(u))).

Accordingly the raw exponential occupied-tile penalty need not survive this
particular congruence support: an explicit finite-support basis recovers an
envelope of the old one-axis scale. This by itself does not give the hoped
for two-axis exponent `M/u^(2/3)`.

For more precise accounting, the retained mode product before discarding
its stronger weights is bounded using

    log Z(theta)
      <=T sum_(m>=1) exp(-2m theta)/m
          * (1-exp(-(k+1)m theta H))
            /((1-exp(-m theta))*(1-exp(-(k+1)m theta))).

The preceding one-axis estimate simply bounds the finite difference-degree
sum by H. Keeping that finite sum, factorial divisibility, additional
support relations or cancellations may improve an envelope in a particular
parameter regime. No lower bound excluding such improvements is proved.

## Scope of the conclusion

The safe statements are constructive and conditional: a simple congruence
coordinate change adds weight `(k+1)h`; monic finite-support reduction caps
the difference degree at `H-1`; and the stated tile density then admits the
displayed one-axis counting bound. These steps do not prove a complete
packet construction, child-size invariant or public-entry recursion.
Neither the raw density nor the displayed envelope is a universal lower
bound on two-axis contractions or prime-field ECDLP.
