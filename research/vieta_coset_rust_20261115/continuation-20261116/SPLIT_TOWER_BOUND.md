# Extending the lift to split rational-map towers

**Hand argument and construction, awaiting independent/formal validation
[pcc:unverified]. No novelty claim.** This is a sibling extension of the original
LAST_FALL_BOUND.md; that PCC-pinned file is unchanged. It is separate from the
frozen arithmetic experiment and is not implemented by its qualifier.

## Precise hypothesis and bound

Over F_p, p>3, let L_i=N_i/D_i for i=0,...,d−1, with coprime N_i,D_i,

    deg N_i = q_i >= 2,     deg D_i <= q_i−1.

Fix a finite terminal value c. Starting with c and working backwards, require
**every** encountered fiber N_i(X)−a_(i+1)D_i(X) to have exactly q_i distinct
roots in F_p. This is a full-splitting hypothesis on the entire fiber tree,
not just a claim that one rational path exists. Coprimality excludes a zero
denominator at any root of these finite-fiber equations.

Put M=product(q_i), S=sum(q_i−1), and u_d=c. One membership chain consists of

    F_i = N_i(u_i) − u_(i+1) D_i(u_i) = 0.

Use two copies of that chain, with initial variables x_1,x_2, and the same
quadratic Vieta core for R=(t,r):

    L² − x_1 − x_2 − t = 0,
    (x_1−t)(x_2−t) − 3t² − A − 2rL = 0.

For this exact generating system, the argument below gives

    d_F <= 2S+2 = 2 sum(q_i−1)+2.

In particular, a full split tower of degree-two rational maps has the same
proposed upper bound **2d+2 = 2 log2(M)+2** as the binary Kummer case. The
bound includes r=0 using local quadratic algebras, without assuming radicality.
It concerns this generating system, not Boolean/field-equation encodings or
the joint three-term system. Identity targets remain a separate branch.

## 1. Conditional projectors stay small in lifted variables

For a leaf path a=(a_0,...,a_(d−1),a_d=c), write

    P_(i,a)(X) = N_i(X) − a_(i+1)D_i(X),
    gamma_(i,a) = 1 / P'_(i,a)(a_i),
    e_(i,a)(X) = P_(i,a)(X) / ((X−a_i) P'_(i,a)(a_i)),
    E_a(U) = product_i e_(i,a)(u_i).

The derivative is nonzero by the hypothesis. Each e has degree q_i−1 and
E_a has degree at most S in the lifted variables. It equals1 on path a and0
on any other valid path: take the highest index where the paths differ, so
their next coordinates agree and the relevant conditional Lagrange polynomial
vanishes. Also **sum_a E_a=1 as an exact polynomial identity**. Sum over the
lowest fiber first; its Lagrange polynomials sum to1 identically. Repeat
upwards through the tree. This does not invoke the gate equations.

## 2. Denominator factors fit the same degree budget

The key identity is

    (u_j−a_j)e_(j,a)(u_j)
      = gamma_(j,a) [F_j + (u_(j+1)−a_(j+1))D_j(u_j)].

Multiply by the other e factors, then apply the same identity to the remaining
next-coordinate term. The terminal has no remainder because u_d=a_d=c. Thus

    (u_j−a_j)E_a
      = sum_(k=j)^(d−1)
          [product_(i=j)^k gamma_(i,a)]
          [product_(i<j) e_(i,a)]
          [product_(i>k) e_(i,a)]
          [product_(i=j)^(k−1) D_i(u_i)] F_k.

Every term has total degree at most

    sum_(i<j)(q_i−1) + sum_(i>k)(q_i−1)
      + sum_(i=j)^(k−1)(q_i−1) + q_k = S+1.

Therefore each coordinate-projector relation belongs to V_(S+1), in the
standard constructible-space definition of last fall degree. This is where
deg D_i<=q_i−1 matters. An arbitrary rational map with a denominator of full
degree q_i is not covered by this same estimate.

For two chains, e_(a,b)=E_a(U_1)E_b(U_2) has degree at most2S, sums to1,
and its coordinate action relations belong to V_(2S+1).

## 3. The local slope argument and normal forms

Let T=2S+2. Multiply both Vieta equations by e_(a,b), and use the coordinate
relations within degree T. With

    q=a_0+b_0+t,
    K=(a_0−t)(b_0−t)−3t²−A,

this gives e(L²−q), e(2rL−K) in V_T. For r!=0, set ell=K/(2r).
If ell²!=q, multiplying e(L−ell) by L+ell discards that projector within T;
otherwise its block is F_p with slope ell. For r=0 discard K!=0 blocks and
retain F_p[L]/(L²−q) in each remaining block, including nonsplit/nilpotent cases.

Represent the resulting product algebra by the span W of good e, or e and Le
in quadratic blocks. W has degree at most T−1 and injects into that algebra.
For every coordinate z and basis element w, zw−NF(zw) is in V_T. Also
1−NF(1) is in V_T, by the discarded-projector relations and the exact partition
of unity. Induction on monomial degree gives

    f−NF(f) in V_max(T,deg f).

The induction uses the **actual** degree of f−NF(f), at most max(deg f,T−1),
so multiplying by the next variable stays inside the indicated bound. Every
original generator maps to zero in the product algebra. Hence f in (F) has
NF(f)=0, establishing the proposed last-fall-degree bound. This is the same
normal-form step as in the original binary proof, with the new rational-map
coordinate certificate supplying its missing hypothesis.

## 4. An interpolation-free target evaluator

This lift also admits a direct evaluator; it does not require building any
of the proof projectors. In the quadratic algebra

    B = F_p[L][Z] / (Z²−s(L)Z+v(L)),
    s=L²−t,  v=tL²+2rL+A+t²,

start U=Z,W=1 and compose each map homogeneously:

    U_next = W^q N_i(U/W),
    W_next = W^q D_i(U/W).

These expressions are polynomials in U,W, evaluated by homogeneous Horner
steps in B; there is **no division or interpolation**. Coprimality and the
degree hypotheses make each pair a map of the projective line, so the
composed U,W have no common geometric zero. After the terminal step, compute
H=U−cW=a(L)Z+b(L) in B. The underlying degree-M membership polynomial has
exactly the full split fiber as its roots. For distinct abscissae, candidate
slopes come from gcd(a,b), just as in the Kummer extractor.

Repeated abscissae require the separate branch: Delta=s²−4v=0 and
H(s/2)=0. Modulo Delta, the latter can be evaluated as a*s/2+b from the same
remainder. Requiring the repeated quadratic to divide a squarefree membership
polynomial would still be wrong. Decode and check the resulting points by
the group law and exact tower membership, including poles/identity exceptions.

The off-diagonal coefficient degrees still grow with M. The evaluator avoids
interpolation; it does not make the dense polynomial/GCD cost disappear.

## 5. A concrete rational-tower certificate

Over F_17, use L(X)=(X²+1)/(2X) twice, with terminal c=4. The last fiber is
{11,14}; their two preimage fibers are {10,12} and {2,9}. All denominators
are nonzero and all fibers split distinctly. M=4,S=2,T=6.

For leaf (a_0,a_1)=(2,14),

    e_0=12X+11,   e_1=6Y+2,
    E=(12X+11)(6Y+2),
    F_0=X²+1−2XY,   F_1=Y²+1−8Y.

The explicit degree-three coordinate certificates are

    (X−2)E = (4Y+7)F_0 + 8X F_1,
    (Y−14)E = (4X+15)F_1.

The other three leaves give the same partition-of-unity construction.
This example is a rational-map certificate, not a claim that this particular
map is an elliptic isogeny.

The Rust producer checker `native/split_tower_certificate.rs` expands all four
leaves' coordinate certificates and the partition of unity as literal F_17
polynomial identities. It also checks homogeneous membership composition
against the four linear factors. Its retained output is
`split-tower-certificate.json` [pcc:unverified].

On the separate toy curve y²=x³+2x+2 over F_17, exhaustive point enumeration
gives prime group order19. The same fiber supplies points with x=9,10;
(9,1)+(10,6)=(6,14). At line slope5 the membership polynomial factors as

    X⁴+X³+6X²+X+1 = (X²−2X+5)(X²+3X+7).

The checker retains both ordered relations and this divisibility identity.
This is a small exact example, not the large-field benchmark or a validation
of the general hand argument [pcc:unverified].

## Application and limits for prime curves

Normalized separable isogeny x-maps have numerator degree q and denominator
degree q−1. Thus the conditional theorem covers PKM auxiliary-curve isogeny
towers **when** their chosen finite fibers satisfy the full rational splitting
hypothesis. A rational kernel coset with distinct finite x-coordinates supplies
the intended kind of fiber; its certificate and construction/search cost must
still be supplied for any concrete prime. Existence and cheap construction
are not inferred from the degree argument.

The auxiliary curve supplies a structured subset of F_p. The decomposition
curve can be a separate prime-order curve over the same field. This is not a
claim of an isogeny or DLP transfer between curves of different group orders.
The M² projectors are proof witnesses only; constructing them would reintroduce
large enumeration/interpolation work and is not part of the target evaluator.

Sources: the original sibling proof and Huang–Kosters–Yang–Yeo definitions2.1,
2.5; Petit–Kosters–Messeng §3.3; repository ic_framework/prime_tower.rs and
pkm_smooth_tower.rs for auxiliary rational-map construction. KB unavailable;
primary/internal sources were read directly. Independent or formal validation
is still required, and no novelty, running-time or end-to-end gain is asserted.
