# Exact discriminant factorization for the translated rational S3 divisor

Date: 2026-11-23. Continuation of PR #4977 at source parent f684c35a7ba58e1bdcdf94c0526fbbefe5199a89.
Scope: **producer mathematical derivation and symbolic check, [pcc:unverified]**. This is neither independently signed verification nor a faster index.

## Result

Let E/F_p be y²=f(x)=x³+ax+b, with odd p>3, and let G be an odd-order subgroup of E(F_p). Take a sign-closed nonidentity subset U of G with distinct abscissae X of size D>=1. Then U is exactly the two rational lifts of each x in X. Set

$$H(Z)=\prod_{u\in X}(Z-u),\quad F=\operatorname{Res}(H,f)=\prod_{u\in X} f(u),\quad \mathcal D=\operatorname{Disc}(H)=\prod_{u<v}(u-v)^2.$$

Let R be a nonidentity point in G with r=x(R) not in X; in particular f(r) and H(r) are nonzero. Define the ordinary rational doubling coordinate

$$d(r)=\frac{(r^2-a)^2-8br}{4f(r)}=x(2R).$$

Define the **monic translated divisor** and the off-diagonal pair-spectrum polynomial:

$$P_r(V)=\prod_{u\in X}\frac{S_3(u,V,r)}{(u-r)^2}
=\prod_{T\in U}(V-x(R-T)),\qquad
\operatorname{Off}_X(s)=\prod_{\{u,v\}\subset X,\;u\ne v}S_3(u,v,s).$$

The off product takes each unordered distinct pair once. Then the following *exact field identity* holds:

$$
\boxed{\operatorname{Disc}_V(P_r)
=\frac{16^D f(r)^D\,F\,\mathcal D^2\,[4f(r)]^{2D(D-1)}}{H(r)^{8D-4}}\;
\operatorname{Off}_X(d(r))^2.}\tag{1}
$$

The prefactor is nonzero under the stated hypotheses. Thus

$$P_r\ \text{is squarefree}\quad\Longleftrightarrow\quad
\operatorname{Off}_X(x(2R))\ne0.$$

For the target S=2R, with R outside U, the right-hand side is exactly the existing nonzero, off-diagonal S3 pair-spectrum miss predicate. The diagonal pair-sum cases S=2T for T in U force R=T in an odd-order group and are exactly the excluded r in X branch. Identity S=O and any target in the zero-subtotal branch remain separately handled. **The new discriminant interface is not a distinct accelerated solver**; it reduces to the already-known pair-spectrum off product.

## Derivation, including normalization

Use the characteristic-not-two Semaev cubic with variables x,y,z:

$$S_3(x,y,z)=(x-y)^2 z^2-2[(x+y)(xy+a)+2b]z+(xy-a)^2-4b(x+y).$$

As a quadratic in V, set Q_u(V)=S_3(u,V,r) and L_u(V)=Q_u(V)/(r-u)^2. Its discriminant is

$$\operatorname{Disc}_V(L_u)=\frac{16 f(u)f(r)}{(r-u)^4}.\tag{2}$$

For distinct u,v, the second exact polynomial identity is

$$
\operatorname{Res}_V(Q_u,Q_v)
=(u-v)^2[4f(r)]^2\,S_3(u,v,d(r)).\tag{3}
$$

Equation (3) has **no denominator**: its right side is understood with the two factors of 4f(r) cleared. It follows from the universal quadratic resultant formula

$$\operatorname{Res}(AV^2+BV+C,EV^2+FV+G)
=(AG-EC)^2-(AF-EB)(BG-FC),$$

after substituting S3's three coefficients. This is an identity in the polynomial ring over Z[a,b,u,v,r], not merely an equality of zero sets on rational curve points. It is also compatible with the four signed elliptic addition branches.

Since L_u is monic, equation (3) gives

$$\operatorname{Res}(L_u,L_v)
=\frac{(u-v)^2[4f(r)]^2}{(r-u)^4(r-v)^4}\,S_3(u,v,d(r)).\tag{4}$$

Apply the usual discriminant-of-product identity to P_r=product L_u:

$$\operatorname{Disc}(P_r)=\prod_u\operatorname{Disc}(L_u)
\prod_{u<v}\operatorname{Res}(L_u,L_v)^2.$$

The numerator combines 16^D f(r)^D product f(u), the source pair factor product (u-v)^4=Disc(H)^2, [4f(r)]^{4 binom(D,2)}, and Off_X(d(r))². The denominator has (r-u) exponent 4+8(D-1)=8D-4 for every u, giving (1). No cancellation, leading-coefficient choice, or multiplicity factor is omitted.

Each x(R-T) lies in F_p and, since translation is injective and x has fibres of size at most two on G, each P_r root has multiplicity at most two. Thus the previously recorded formula deg gcd(P_r,P_r')=N_U(2R)/2 is consistent with (1). The sign-specific T backpointers must still be recovered; equality of abscissae is not a relation row by itself.

## Scope and cost

Source setup: constructing X, duplicate/backpointer buckets and H from B signed leaves is bounded by the inherited O(B²) pair chart and quasi-linear-in-D polynomial operations. Signed labels and original curve group work are charged separately.

On a blind nonidentity query outside U, one can construct the degree-2D translated polynomial from D quadratic factors and run a derivative/gcd in O-tilde(D) field work, plus target conversion and root/output recovery. Alternatively, direct evaluation of the factored Off_X uses binom(D,2) S3 evaluations, **Theta(D²)** field operations. Explicitly materializing its target polynomial has degree D(D-1), hence Theta(D²) dense coefficient slots; an expanded index requires that setup and storage to be charged. Identity (1) alone gives neither a near-linear-space sublinear online emptiness test nor a complete cold-log cost improvement.

Because every nonzero coefficient in the prefactor is a square on rational sign-closed U and f(r) is itself a rational-point square, the nonzero discriminant in (1) has square quadratic character. A nonsquare discriminant test cannot act as an additional miss filter on this domain. A **zero** discriminant remains an exact positivity signal, but finding it cheaply is exactly the unresolved source-specific index problem. This restriction is not a general lower bound on all algebraic data structures.

## Symbolic and rational producer checks (not native Rust)

1. Expand the universal quadratic-resultant expression from S3's explicit A,B,C coefficients and subtract (u-v)^2 times S3(u,v,d(r)) with [4f(r)]² cleared. SymPy Poly in (u,v,r,a,b) returned the zero polynomial.
2. On rational scalars a=b=1,u=2,v=3,r=5 (an **algebraic test**, not a certified rational elliptic fixture), the monic product is V⁴-743 V³/9+1188 V²-9923 V/9+2173/9. The directly computed discriminant and formula (1) both equal 133759167502286123600/531441; the cross-resultant identity also held exactly.
3. These are same-coordinator symbolic checks. They are not an independent source-frozen Rust check, prime-field curve qualification, formal proof, or performance measurement.

### Validation queue after a Rust source freeze

On genuinely odd-order toy source groups and the exact Prime31 arithmetic, compare the discriminant/off predicate against the complete signed pair-scan oracle for **every available small-field R**, including misses, zero/identity branches, R in U, repeated leaves, duplicate abscissae (deduplicate X first) and the collision/sign backpointer buckets. Charge computing d(r), H(r), squarefreeness/gcd, output recovery, group replay and useful-rank filtering. Only apply (1) on the nonexceptional domain, and retain adverse cases as fixtures. No new relation-finder implementation was selected here because this formula gives no admitted query improvement.

## Existing work and provenance

Inputs: iteration9/PAIR_SPECTRUM_RESULTANT.md; iteration10/S3_PENCIL_CONTRACTION.md; iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md; iteration10/translated_discriminant_followup/SQUAREFREE_DOUBLING_GATE.md; PR #4979 double-resultant review. Semaev, *New algorithm for the discrete logarithm problem on elliptic curves* (2015), https://arxiv.org/abs/1504.01175, Section 2–3, already supplies the S3 and auxiliary-variable context. Resultant/discriminant-of-product identities are classical. This is an exact reconciliation of two existing interfaces; **no novelty claim is made**. Shared KB and Conductor are unavailable in this runtime, so the literature review is bounded, not exhaustive.

Coordinator roles this invocation: algebra derivation and symbolic challenge completed; Rust implementation deferred as no unclaimed asymptotically admissible step exists and no rustc/cargo were installed; validation plan prepared, without independent worker execution. No native worker slots were exposed. No EKS call, new PCC claim, signed verifier action or benchmark was performed. [pcc:unverified]
