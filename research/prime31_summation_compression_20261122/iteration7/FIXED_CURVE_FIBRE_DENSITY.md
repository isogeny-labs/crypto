# A dense Dickson fibre on a fixed odd-order source curve

Date: 22 November 2026 America/Los_Angeles / 23 November 2026 UTC.
Parent-proposed averaging argument, independently checked and written by the
higher-arity mathematical worker. **[pcc:unverified]**. This is a mathematical
derivation and algorithm specification; no numerical or cryptographic
experiment, curve construction, point count, or native test was run.

## Theorem and its exact scope

Let p>=4096 be prime, let E/Fp be a fixed nonsingular short-Weierstrass curve

    E: y^2=f(x)=x^3+ax+b

whose rational group has odd order, and let q be a power of two satisfying

    4 <= q <= sqrt(p)/16,       4q | (p+1).

Let T be the norm-one subgroup of F_(p^2)^*, and put

    C_reg = { w+w^(-1) : w in T^q, w not in {1,-1} },
    X_c = { x in Fp : D_q(x)=c },
    B(c) = #{ x in X_c : f(x) is a square in Fp }.

Here T^q={z^q:z in T}. Each X_c is a split squarefree q-point fibre. There
are exactly M=(p+1)/(2q)-1 distinct regular fibres. Uniform c in C_reg obeys

    | E_c[B(c)/q] - 1/2 |
       <= (3 sqrt(p)/2 + q/2 + 1) / ((p+1)/2-q)
       <= 8/sqrt(p) <= 1/8.                            (1)

Consequently at least one sixth of all c in C_reg satisfy B(c)>=q/4. Such a
fibre can be selected by a Las Vegas procedure with expected
O(q log p) field operations under ordinary exponentiation, plus field
inversion costs, and O(q) field-element storage, after the certified p and
specified curve are supplied. Bit complexity adds the chosen field-arithmetic
cost. **[pcc:unverified]**

This theorem changes the factor-base fibre, not the source elliptic curve.
It applies to an already supplied prime-order source as a special case.
It does not construct that source, certify a large prime subgroup, establish
sumset distribution, or solve a relation target.

## 1. Count liftable coordinates on the norm-one trace set

Let chi be the quadratic character of Fp, with chi(0)=0, and set g(X)=X^2-4.
The trace image of T is

    T_x = {x in Fp : chi(g(x))=-1} union {2,-2}.

Indeed, u and u^(-1) are roots of U^2-xU+1. A nonsquare discriminant gives
conjugate roots with product one; the zero discriminant gives u=+/-1. A
nonzero square discriminant gives two Fp roots, which cannot be norm-one
unless u=+/-1. Thus |T_x|=(p+3)/2.

Odd #E(Fp) implies f has no Fp zero: a root would give a rational point of
order two. Hence f(2)f(-2)!=0. Nonsingularity makes f squarefree, and f is
coprime to g, so fg is a squarefree polynomial of degree five.

Define

    S_f  = sum_(x in Fp) chi(f(x)),
    S_fg = sum_(x in Fp) chi(f(x)g(x)),
    epsilon_+ = chi(f(2)),  epsilon_- = chi(f(-2)).

The genus-one and genus-two Hasse-Weil bounds give

    |S_f| <= 2 sqrt(p),       |S_fg| <= 4 sqrt(p).       (2)

For the second inequality use the smooth projective model of Y^2=fg: its
squarefree degree-five polynomial gives genus two and one rational point at
infinity. In each odd-degree model the point count equals p+1 plus the
character sum. References S4-S5 below specify the point-count convention.

Write A_T for the number of x in T_x with chi(f(x))=1. Since f(x)!=0 for all
rational x, its liftability indicator is (1+chi(f(x)))/2. The torus trace
indicator is

    1_(T_x)(x) = (1-chi(g(x)))/2 + 1_(g(x)=0)/2.

Also sum_x chi(x^2-4)=-1. One elementary proof counts solutions to
y^2=x^2-4: the equivalent equation (x-y)(x+y)=4 has p-1 solutions because
2 is invertible. Expanding the indicators therefore gives the exact identity

    A_T = [p+3+S_f-S_fg+epsilon_++epsilon_-]/4.         (3)

Combining (2)-(3), with epsilon_+,epsilon_- in {1,-1}, yields

    | A_T - |T_x|/2 | <= 3 sqrt(p)/2 + 1/2.            (4)

The displayed derivation, including its endpoint correction, is independent
of any random-curve or random-coordinate heuristic. **[pcc:unverified]**

## 2. Partition into full fibres and account for both exceptions

The cyclic group T has order p+1, and T^q has order h=(p+1)/q. Since 4q
divides p+1, h is a multiple of four, so both 1 and -1 belong to T^q.

For w in T^q minus {1,-1}, all q solutions of u^q=w lie in T. Their trace
coordinates form a full q-point fibre of D_q(X)-(w+w^(-1)); inversion pairs
the preimages of w with those of w^(-1), without identifying two elements
inside one q-coset. Thus the two w values give the same fibre, and all other
regular w values give disjoint fibres. There are (h-2)/2=M of them.

The exceptional c=2 fibre comes from u^q=1. Its q elements include both
fixed points u=1 and u=-1 of inversion, so it has q/2+1 trace coordinates.
The exceptional c=-2 fibre comes from u^q=-1. It has no fixed point of
inversion and hence q/2 trace coordinates. Their union has exactly q+1
coordinates. Consequently the regular trace coordinates have total size

    K = |T_x|-(q+1) = (p+1)/2-q = Mq.                 (5)

Let A_exc be the number of liftable exceptional coordinates, and put
A_reg=A_T-A_exc=sum_c B(c). Because 0<=A_exc<=q+1,

    | A_reg-K/2 |
       <= |A_T-|T_x|/2| + |A_exc-(q+1)/2|
       <= 3 sqrt(p)/2 + q/2 + 1.                      (6)

Divide (6) by K=Mq to obtain the first inequality in (1).

For the stated cap, K >= p/3 and

    3 sqrt(p)/2 + q/2 + 1 <= (49/32)sqrt(p)+1.

Thus the relative error is at most 147/(32 sqrt(p))+3/p, which is at most
8/sqrt(p) for p>=3. With p>=4096 it is at most 1/8. If rho is the fraction
of fibres with B(c)/q>=1/4, the elementary bound

    E[B(c)/q] <= 1/4 + (3/4)rho

and E[B(c)/q]>=3/8 imply rho>=1/6. The constants are conservative and fully
explicit; no sampled observation is used. **[pcc:unverified]**

## 3. Exact sampling and setup recurrence

Because p=3 mod4, use F_(p^2)=Fp[i] with i^2=-1. The map

    t in P^1(Fp) -> (t+i)/(t-i),     infinity -> 1

is a bijection onto T. It supplies an exact uniform norm-one sample with
ordinary field arithmetic and a charged inversion. Alternatively, a uniform
nonzero extension element raised to p-1 is uniform in T.

Find a primitive q-th root zeta by taking uniform u in T, setting
zeta=u^((p+1)/q), and testing zeta^(q/2)!=1. The image is uniform in the
cyclic q-subgroup, and exactly half its elements have order q, so the
expected number of trials is two. No odd-part factorization of p+1 is
required. This is target-independent setup.

For a fibre trial, choose an independent uniform z in T and compute
c=z^q+z^(-q). Reject c=+/-2. Each regular fibre has exactly 2q seeds z:
q preimages of w and q preimages of w^(-1). Conditional sampling is
therefore uniform on C_reg, rather than biased by fibre size.

The bad-seed probability is exactly 2q/(p+1), at most 1/512 under the stated
p and q bounds. A good fibre has conditional probability at least 1/6;
therefore an unconditional seed produces an acceptable fibre with probability
at least 511/3072, and the expected number of seeds is less than seven.

Enumerate x_j=z zeta^j+(z zeta^j)^(-1), j=0,...,q-1, in O(q) extension-field
operations. Inverses in T are Frobenius conjugates. For each x_j evaluate
f(x_j) and perform an exact quadratic-residue test; in p=3 mod4 a verified
square root is also available by ordinary exponentiation. Keep the fibre
when B(c)>=q/4, otherwise discard it and try another seed.

If C_T denotes the priced torus sample plus O(log p) exponentiation cost,
and C_enum(q)=O(q) while C_lift(q,p)=O(q log p) under ordinary field
exponentiation, the expected target-independent work is bounded by

    C_generator + 7[C_T + C_enum(q) + C_lift(q,p)],

where C_generator uses two generator trials in expectation. Memory is O(q)
field elements for the current fibre and accepted point labels. Outputs are
verified by exact curve equations, and both signs may be retained. Failed
trials are included in this recurrence. Bit costs multiply by the actual
field-arithmetic/inversion costs. **[pcc:unverified]**

## 4. Compatibility with the Linnik family

In the reviewed family let Q=2^k, p the least prime congruent to -1 modulo
4Q, and p<=C(4Q)^L. Choose L>=2 by enlarging the fixed exponent if necessary,
and define q as the largest power of two at most min(Q,sqrt(p)/16).

Then q<=Q, so 4q divides p+1. Also, for sufficiently large p,

    q >= (1/2) min(Q,sqrt(p)/16)
      >= min(1/(8 C^(1/L)),1/32) p^(1/L).

Thus q still grows polynomially in p and eventually is at least four. The
density theorem applies to every specified odd-order E over each such field.
The family construction does not require an infinite Mersenne-prime family.
**[pcc:unverified]**

## 5. What this establishes and what it does not

The chosen fibre has q/4<=B(c)<=q, so B(c)=Theta(q) uniformly. This removes
the rational-liftability gap for a selectable full fibre on a fixed odd-order
curve. It works on the original source curve when that source satisfies the
hypotheses; no isogeny, twist switch or elliptic-to-torus DLP transfer is used.
The torus serves only to construct abscissa sets. **[pcc:unverified]**

The theorem leaves separate obligations:

1. Finding/certifying the primes and constructing/certifying a suitable
   source curve are input/setup costs. Waterhouse existence alone does not
   give the required efficient prime-order source family.
2. On a supplied prime-order curve, every rational base point is in the
   whole target group. On an odd composite curve, a prescribed proper prime
   subgroup may contain fewer of these points; this theorem does not bound
   that intersection. Cofactor clearing changes the x-coordinates.
3. Density is existential and constructible by choosing c; it is not a
   guarantee for c=0 or any previously frozen fibre.
4. There is no signed relation-yield, independent-rank, empty-target query,
   root-solving, or scalar-recovery bound here. The expected near-linear
   work constructs a base; it is not a faster relation solver.
5. The polynomial lower bound on q does not guarantee every desired
   factor-base exponent is available. Complete asymptotic optimization must
   use the actual supported size range and all setup costs.

## Sources and provenance

The density argument is a checked application of standard point-count bounds
to a genus-two auxiliary curve, together with the exact torus partition. No
literature-wide novelty claim is made. It is not a machine-checked proof.

* **S4:** Andre Weil, *Sur les courbes algebriques et les varietes qui s'en
  deduisent* (1948), Theorem 13, Corollary 3, through the explicitly labelled
  2022 LaTeX transcription by Denise Vella-Chemla, PDF p. 61:
  https://denisevellachemla.eu/Weil-courbes-varietes.pdf .
* **S5:** Steven Galbraith, *Mathematics of Public Key Cryptography*, author
  extended/corrected Chapter 10, Theorems 10.7.1 and 10.7.5, Exercise 10.7.9,
  and the odd-degree hyperelliptic model conventions:
  https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf .
* Dickson identity and Linnik inputs are independently checked in
  [INDEPENDENT_REVIEW.md](INDEPENDENT_REVIEW.md), with primary links and
  immutable source identity for the hourly task's family note.

The derivation has unverified PCC standing. All iteration-six attestation-
pinned mathematics, source files, and correctness evidence remain unchanged.
