# Iteration 10 — exact double-resultant, symmetric-square and support census

2026-11-23 UTC. **Author mathematical derivation, not machine-checked or independently signed. [pcc:unverified]** No assertion of improved relation solving or cryptanalytic cost. This is a disjoint mathematics/review continuation of [merged #4951](https://github.com/isogeny-labs/crypto/pull/4951), not a rewrite of its native source, workflow or frozen evidence.

## Original-source assumptions and input

Take a nonsingular short-Weierstrass curve E/F_p: y^2=x^3+a x+b, p>3, and an odd-order subgroup G, with no rational 2-torsion in G. Let A={+/- P_1,...,+/- P_B} be 2B distinct nonidentity points of G, with pairwise distinct abscissae, and allow repeated leaves in four-summand decompositions. As in iteration9, let U=(A+A)\{O} as a **set of points**, X=x(U), D=|X| and H(V)=prod_(u in X)(V-u). Choose one actual rational lift S_i=(u_i,y_i) in U for each u_i in X. The group U is sign-closed and has exactly 2D points. All y_i are nonzero by odd order. The doubling map is injective on G, so x(2P_i) are distinct and **B<=D<=B^2**. The right inequality is iteration9's exact unordered-pair/sign orbit count; distinct output labels can collide in X.

The source polynomial, as implemented by `src/cryptanalysis/semaev_prime_poly.rs`, is

```text
S3(u,v,r) = (u-v)^2 r^2
           -2*((u+v)*(u*v+a)+2*b)*r
           +(u*v-a)^2-4*b*(u+v).
```

## Lemma A — exact rational factorization, including the diagonal

For i!=j, u_i!=u_j, and each point S_i+S_j and S_i-S_j is nonidentity. The affine addition law has nonzero denominator, hence

```text
S3(u_i,u_j,r) = (u_i-u_j)^2
                * (r-x(S_i+S_j)) * (r-x(S_i-S_j)).
```

Proof: x(S_i+S_j)=((y_i-y_j)/(u_i-u_j))^2-u_i-u_j, while
x(S_i-S_j)=((y_i+y_j)/(u_i-u_j))^2-u_i-u_j.
The leading term, sum and product of these two roots agree with the displayed S3 formula using y_i^2=u_i^3+a u_i+b. This is an **equality of polynomials over F_p**, not merely an algebraic-closure zero-set assertion. All roots are rational.

For i=j the quadratic coefficient vanishes. The proper degree-one identity is

```text
S3(u_i,u_i,r) = -4*y_i^2 * (r-x(2*S_i)),
x(2*S_i) = ((3*u_i^2+a)/(2*y_i))^2-2*u_i.
```

The constant coefficient check is
4*y_i^2*x(2*S_i)=(u_i^2-a)^2-8*b*u_i.
The leading coefficient -4*y_i^2 is nonzero. **Do not** divide by (u_i-u_i)^2, discard this factor, or count it as a quadratic. These statements fail without the odd-order/nonidentity guards, and need separate exceptional logic when characteristic 2, rational 2-torsion or O is admitted.

## Lemma B — exact double-resultant identity and its degree

For a target variable r let K_r(V)=Res_U(H(U),S3(U,V,r)) mod H(V) as in iteration9. Define

```text
Delta(r) = Res_V(H(V),K_r(V))
         = product_{i=1}^D product_{j=1}^D S3(u_i,u_j,r).
diag(r)  = product_i S3(u_i,u_i,r).
off(r)   = product_{i<j} S3(u_i,u_j,r).
```

Because H is monic and split, reduction mod H leaves evaluation at its roots unchanged. Symmetry in u,v gives the **exact** identity

```text
Delta(r) = diag(r) * off(r)^2,
deg diag = D,
deg off = D*(D-1),
deg Delta = 2*D^2-D.
```

The leading scalar of Delta is
`(prod_i (-4*y_i^2))*(prod_{i<j}(u_i-u_j)^4)`, and is nonzero. No hidden generic-position or squarefreeness assumption is needed for this product identity. Extra collisions among distinct sums create **additional multiplicities**; the stated exponents are degrees with multiplicity, not counts of distinct target abscissae. The diagonal factors do not square merely because the off-diagonal ordered pairs do.

For nonidentity R in G put r=x(R). Let

```text
T(r)=diag(r)*off(r),          deg T = D^2,
Q(r)=H(r)*T(r),              deg Q = D^2+D.
```

Then the **exact nonidentity target-support predicate** is

```text
there exists (a1,a2,a3,a4) in A^4 with a1+...+a4=R
    iff Q(x(R))=0.
```

To prove this, partition the ordered four leaves after leaf 2. If a two-leaf subtotal is O, the other is R and H(r)=0; conversely the sign-closed zero-pair table supplies a cancelling pair for any R in U. Otherwise both subtotals lie in U and admit representatives +/-S_i. Lemma A says S3(u_i,u_j,r)=0 exactly for one of their sums/differences up to sign. The sign closure and global negation recover the requested R. Conversely any such root provides actual rational pair subtotals, and their retained leaf preimages plus sign/group replay give the four leaves. This uses **Fp-point support**, not multiplicity. R=O is a separate group-semiring case; never evaluate x(O). No unknown Fp root is inferred from coefficients alone.

This also proves that materializing Q in **dense coefficient representation** requires exactly D^2+D+1 field slots, before auxiliary scratch, collision processing, labels or signed witness expansion. The dense Delta needs 2D^2-D+1 slots. These are representation-specific output lower bounds: a straight-line circuit, sparse representation, hash index or queried predicate need not write those coefficients. The identity provides **no general lower bound** for succinct data structures.

## Lemma C — a rank-three determinant is never an emptiness oracle

For the fixed r, the D-by-D matrix M_ij(r)=S3(u_i,u_j,r) factors as W C(r) W^transpose, with W_i=(1,u_i,u_i^2) and

```text
C(r) =
[ a^2-4*b*r     -2*(a*r+2*b)      r^2       ]
[ -2*(a*r+2*b) -2*(r^2+a)        -2*r      ]
[ r^2           -2*r              1         ].
```

Thus rank(M)<=3. If D>=4, det(M)=0 **for every target**. This does not imply that any entry M_ij vanishes! For any nonidentity target outside the support of Q, every M_ij(r) is nonzero while det(M)=0. The original B=31 setup guarantees D>=31, and the certified multiset support bound below guarantees an abundance of such targets when n is the inherited odd subgroup order. This is a concrete false-positive control for the tempting rank-three/determinant shortcut, **not** a lower bound against other rank-aware algorithms.

The componentwise zero-support query requires detecting at least one zero M_ij, not vanishing of the ordinary matrix determinant. Symmetric-square contraction replaces ordered pair multiplicities with i<=j factors but still has D(D+1)/2 factors, each of degree one (diagonal) or two (off-diagonal). A rank-3 bilinear description of entries alone does not establish sublinear empty-inclusive online evaluation.

## Lemma D — stronger exact uniform-target emptiness upper bound

There are M=binom(2B+3,4) unordered signed four-leaf **multisets**, allowing repeats. Global negation maps each multiset to its negative and maps its sum to the negative point. A fixed multiset under negation must contain equal numbers of +P_i and -P_i for every i, hence has group sum O. Such size-four fixed multisets are precisely multisets of two cancelling label-pairs, of count F=binom(B+1,2). Every other negation orbit has size two and maps to at most **one** nonidentity target abscissa, possibly with collisions and possibly mapping to O. Therefore, unconditionally for the specified signed base,

```text
number of reachable nonidentity target abscissae <= (M-F)/2.
uniform nonidentity target hit probability <= (M-F)/(n-1),
```

where n=|G| is odd and nonzero points of G split into (n-1)/2 +/- pairs. This does not require genericity or independent-row assumptions. It is an **upper bound on distinct RHS support**, not on the number of ordered witnesses or useful independent relations.

For inherited Prime31 B=31, n=2147446831 (not independently recounted here):
M=677040; F=496; reachable target abscissae <=338272; available nonidentity target abscissae=1073723415; missed >=1073385143. Thus a uniform nonidentity target misses with probability **at least 99.96849542486693%**, a more discriminating exact combinatorial cap than the earlier 99.31% bound from raw ordered-tuple counting. It is **not** a measured sample, factor-base yield experiment or claim about signed relation rank. The bound presumes the specified signed base contains exactly B distinct pairs; if an experiment uses a different base or target distribution it does not apply as stated.

## Cost and failed asymptotic shortcut

- Pair chart source setup: B^2 source-group additions/doublings (up to collisions), labels/indices and O(B^2) storage; H tree O(M(D) log D) field work with an O(D log D) ordinary full-tree storage envelope.
- Exact **direct complement** query: hash all 2D actual signed nonzero pair sums, handle the O pair separately, then for each S in U look up R-S. This is O(D) source-group additions and expected dictionary lookups **even on misses**, plus O(Z_R) actual ordered signed output and duplicate/certificate costs. It is a stronger mandatory native control than a dense polynomial product, and already follows from iteration8/9; no new group-complement mechanism is claimed.
- Explicit all-target Q: constructing all D^2 nonidentity chart expressions requires O(D^2) group additions (plus collision/labels), and writing Q densely requires Omega(D^2) field slots. An exact all-target hash index spends O(D^2) expected setup/space for expected O(1) membership and O(Z_R) output. No uncharged Fourier characters, DLP oracle, target-coordinate arrays or free field setup.
- The **single** low-rank matrix factorization does not compute the product of its D^2 entries for free, and det(M)=0 is an invalid predicate.

Under the same conventional uniform-target/independent-row **conditional model** as iteration8/9 and n=Theta(p), a B=p^beta signed base requires about p/B^3 relation attempts for B independent rows. Pair-complement cost B^2 source setup and B^2 per miss yields optimistic exponent min_beta max(2beta,1-beta)=2/3 at beta=1/3. A full B^4 all-target dictionary with constant-time queries yields min_beta max(4beta,1-3beta)=4/7 at beta=1/7, with B^4 setup/space. Both exceed 1/2 even before signed output, memory, target recovery and verification. These are architecture-specific conditional cost envelopes, **not unconditional ECDLP lower bounds** and not a claim that either base family exists at the balancing exponent.

The still-required new lemma is a **lossless and chargeable elliptic-target index** with setup exponent sigma<=2, empty-inclusive query tau<1 and O(B^2) or better space for m=4, including zero pairs, rational-root isolation, sign recovery, real output, all misses and useful rank. Merely writing S3 as a 3x3 matrix pencil or discarding square multiplicities does not supply it. If an alternative dependence/rank architecture is used, specify and recalculate its full budget rather than borrowing this criterion.

## Independent validation plan (prepared, NOT executed)

Freeze the original Rust arithmetic revision and add a separate test module/fixture owned by a later dedicated Rust session; do not alter iteration9's frozen source or report. On exhaustive small odd-order E/F_p models and all R in G including O:
(1) enumerate all A^4 signed **multisets** and independently all ordered tuples;
(2) build actual rational pair-sum S_i, including collision buckets and zero subtotals;
(3) compare each computed diagonal and off-diagonal S3 coefficient with the two curve addition factors and count exact degrees D, D^2, 2D^2-D;
(4) form Q/Delta and compare exact root support to brute force, evaluating every *miss*;
(5) check gcd(H,K_r) and any supplied extended-Bezout=1 negative certificate, isolate only known rational roots, then recover and replay every witness including signs and duplicate labels;
(6) ensure D>=4 gives det(M)=0 on a miss while every M_ij is nonzero; exercise P,P,P,-P and the nonrational projected-ghost negative control;
(7) cross-check the orbit bound and record actual support, collisions, useful-row rank and allocation/phase costs separately.
Only after source freeze may a different validator replay this program. No cargo/rustc, native worker launcher, authenticated EKS, independent signature or Lean execution is available in this invocation. **Zero new native tests or benchmark observations are claimed.**

## Sources and provenance

- Existing original-arithmetic `src/cryptanalysis/semaev_prime_poly.rs` blob `123ccac36794ed3ce043e01b3a9c27ed3e2ca096`, main read at `816f9c3a942efed569049618c056d12a0c56d693`.
- Iteration9 theorem `iteration9/PAIR_SPECTRUM_RESULTANT.md`, blob `e2348e91dc08141a663c5adb9798b6ca8f09424e`; iteration9 `STATE.json`, blob `9c1e877a50b020f4da70a83decca1a7e245f8f7a`. Parent PR [#4951](https://github.com/isogeny-labs/crypto/pull/4951) merged, not an active edit destination.
- [Semaev, *New algorithm for the discrete logarithm problem on elliptic curves* (2015)](https://arxiv.org/abs/1504.01175), sections 1–2, for summation-polynomial semantics and auxiliary-variable approach.
- [Karabina, *Point Decomposition Problem in Binary Elliptic Curves* (2015)](https://arxiv.org/abs/1504.02347), for rational-auxiliary polynomial-system prior art (a different field family).
- [Golovnev et al., *Data Structures Meet Cryptography: 3SUM with Preprocessing* (2019)](https://arxiv.org/abs/1907.08355), for genuine setup/query tradeoffs. Their integer-index model does **not** give a free lossless DLP-free reduction of elliptic addition.
- [Chan–Itakura–Ma–Yu (2026)](https://arxiv.org/abs/2610.42012) is a recent integer-kSUM preprint; its applicability to succinct elliptic group encoding has **not** been proved. The HTML full text could not be fetched in this runtime; no theorem is imported from it.
- Shared research-KB MCP and repository wrapper were not available through this runtime; repository files and the accessible primary arXiv abstracts/text above were used as fallback. No absence/novelty claim is made.

## Coordinator handoff

One coordinator performed the derivation and source review, with independent GitHub reads parallelized. **No native algebra, Rust or validator worker slots were exposed; therefore there are no separately executed worker artifacts, no claim of worker agreement and no elapsed-time speedup.** This disjoint iteration10 note belongs to the new PR only; #4951/#4962 are merged. The next work is a genuinely new empty-inclusive target index or independent Rust counterexample/positive-control replay, not a retest of the dense resultant.
