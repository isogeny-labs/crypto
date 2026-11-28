# Hyperelliptic genus130/131 fiber and branch screen

2026-11-23. Continue the same objective and draft PR5079. This round
studies hyperelliptic C/F2 only; it does not replace the general curve
question with that restriction.

Write out the elementary degree2 bounds N2(C)>=3 and
Nm(C)<=2*(2^m+1). Replay the proposed A counts and all five elliptic
complement traces with native GMP. For trace0 in genus131, combine
branch parity at1/131, 2-rank130, and the odd-pole ramification genus
formula. Do not rely on integral zeta or maximal endomorphism orders.
References: Borodin–May arXiv2508.16886 Lemmas2.1/2.2, and
Elkin–Pries ANT7(2013) §1, doi10.2140/ant.2013.7.507.

For any survivor, specify the normalized ordinary branch131+1 family.
One fixed fixture is y²+y=x+1/Q(x), where
Q=x^131+x^13+x²+x+1 is the registered source-field modulus as a
polynomial over F2. Verify Q irreducible by the prime-degree Rabin
criterion. Count this curve over F2^m for m1..8 using explicit native
finite fields, with independent elliptic E0 point-count controls.
Any mismatching count rejects this one curve only. No adaptive search,
timing comparison or cryptographic subgroup solve is planned.

Compute the exact number of normalized (Q,R,b) presentations with
Q monic irreducible degree131, nonzero R of degree<131 and b in F2.
This is a combinatorial size, not a measured search cost or a proof
that every curve in that branch family is a carrier. All curve/map
construction and total attack-cost claims remain separate.
