# Proposed next builder: Miller recurrence with a Frobenius anchor

Status: algebraically derived here; not implemented or timed in this packet.
Retain the current dense builder as the exact reference.

Let r be an odd prime, h=(r-1)/2, and g=product(X+a) for a square-free feature
with at most three shifts. If g(0)=0, factor out X once: g=X u. Build u^h
and shift by h. Here degree(u)<=2, so degree(u^h)<=r-1. The empty feature is 1.
Otherwise g_0 is nonzero. Put F=g^h=sum f_n X^n, d=degree(g)<=3, f_0=g_0^h.
From g F'=h g' F, coefficient comparison gives

    n g_0 f_n = sum_{i=1}^{min(d,n)} ((h+1)i-n) g_i f_{n-i}.

For n not divisible by r, this determines f_n using at most d previous
coefficients. This is the standard J.C.P. Miller power recurrence. In this
characteristic, naive division by n is invalid at n=r. Since d*h<2r,
there is at most one such positive singular index. For degree<=2 there is none.

The missing coefficient is anchored by an identity valid over F_r:

    g F^2 = g^(2h+1) = g^r = g(X^r).

The coefficient of X^r on the right is g_1. On the left, the only terms
involving unknown f_r are 2 g_0 f_0 f_r. Everything else uses earlier
coefficients. Let

    R = sum_{i=0}^d g_i sum_{j=0}^{r-i} f_j f_{r-i-j},

omitting (i,j)=(0,0) and (0,r), so no f_r occurs. Then

    f_r = (g_1-R)/(2 g_0 f_0).

The denominator is nonzero because r is odd and g_0,f_0 are nonzero. The
recurrence resumes at r+1 and determines every remaining coefficient through
d*h. Induction below r, the exact anchor, and induction above r establish
equality with g^h. A single O(r) convolution slice suffices for the anchor.
Using precomputed inverses for residues 1..r-1, each column therefore has
O(r+log r) field work for bounded d; the log term covers initial powers or
inverses. A shared inverse table can be built in O(r) field/integer operations
for prime r. Retained coefficient storage remains O(r) per column, and the
dense elimination cost is unchanged. These are derived algorithm bounds,
not implementation or performance results.

Do not differentiate only the reduced representative: formal differentiation
does not descend to F_r[X]/(X^r-X), since (X^r-X)'=-1. If F=P+(X^r-X)Q,
then the reduced differential equation has the extra term

    g P' - h g' P = g Q modulo (X^r-X).

Working with unreduced F until construction is complete avoids losing Q.

Literature: the recurrence matches FLINT's documented J.C.P. Miller formula:
https://fredrikj.net/fdoc/html/fmpz_poly/_fmpz_poly_pow_multinomial.html
(read 2026-11-05; canonical flintlib.org fetch and linked GitHub source fetch
failed). The page identifies integer exact division, not division in F_r,
and carries a notice about its ongoing documentation revision. The derivation
above supplies the finite-field argument explicitly; the characteristic-r
anchor is not asserted to follow from the integer implementation. No novelty
priority claim is made. Next implement this anchor and test every cubic column
against the dense builder, including zero shifts and n=r boundary cases.
