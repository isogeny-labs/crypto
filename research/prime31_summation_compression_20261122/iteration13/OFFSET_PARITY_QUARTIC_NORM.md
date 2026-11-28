# Exact half-degree offset selector and signed quartic norm

2026-11-23. Draft PR #5064; read-only mathematical continuation from head 41e71c3d48629636e99225b464a297a14dd69da6. [pcc:unverified] No cryptographic execution, timing, or independent signature. This develops the previous uncommitted symmetric quartic observation and explicitly accounts for the missing parity, endpoint and sign guards.

## 1. Inputs and exact offset partition

Let p>3, N=2^k>=2, 4N | p+1, T the norm-one torus in F_(p^2), z in T with c=z^N+z^(-N) not in {2,-2}, gamma a supplied primitive Nth root in T. Put t_i=z gamma^i and x_i=t_i+t_i^(-1) for i modulo N. The N traces are distinct: if t_i=t_j^(-1), then z^(2N)=1, contrary to regularity. Thus these are precisely the roots of the monic D_N(X)-c. The actual base is a filtered subset M of these roots with rational nonzero odd-order group lifts; the whole fibre is NOT assumed to lift. Torus coordinates do not encode elliptic scalar residues.

Every unordered leaf-index pair can be written {j+d,j} with unique d in 0..N/2, unique j modulo N unless d=N/2, when j and j+d describe the same pair. At d=0 there are N diagonal pairs, at 0<d<N/2 there are N each, and at d=N/2 there are N/2. Total N(N+1)/2, with no off-diagonal omitted. Choose v in T with v^2=gamma^d (exists since 2N | p+1; v^N=(-1)^d). Let u=t_j v, lambda=v+v^-1, w=u+u^-1. Then x_j+x_(j+d)=lambda*w, x_j*x_(j+d)=lambda^2+w^2-4. Both lambda,w are rational. Changing v to -v simultaneously changes both traces' signs and preserves the pair. For each d, the N distinct w values are exactly D_N(w)=(-1)^d c, by the same regularity argument. No extraction of elliptic discrete logs is used.

## 2. A degree-halved selector

Define delta=lambda^2-2=gamma^d+gamma^(-d), sigma=w^2-2=u^2+u^(-2), s=lambda*w, t=delta+sigma=x_j*x_(j+d). Then

    s^2=(delta+2)(sigma+2),   (x_j-x_(j+d))^2=(delta-2)(sigma-2).
    D_(N/2)(delta)=2(-1)^d,
    D_(N/2)(sigma)=(-1)^d c.

The N/2+1 distinct delta values constitute the traces of the Nth roots of unity, not the N roots counted with multiplicity in D_N(delta)-2. Fix d, hence parity. The monic degree-N/2 selector L_d(S)=D_(N/2)(S)-(-1)^d c has exactly N/2 distinct roots: w and -w form pairs because N is even and no w=0 occurs (D_N(0) is 2 or -2). All of those roots have sigma+2 a nonzero square, because they arise from actual rational w. Thus this selector is exact without a second general finite-field factorization or a nonsquare rejection. The signs s are BOTH possible for delta!=-2, since w and -w are present in the full fibre. The actual filtered M may retain neither, one or both; test each separately. At delta=-2 only s=0 occurs.

## 3. Exact signed norm and fixed-degree residue

Use the short-Weierstrass S3 from the pinned pencil note, with curve coefficients a,b and nonidentity target abscissa r. Define polynomials in S (for fixed delta,r):

    A=(delta-2)(S-2)r^2 + (delta+S-a)^2 - 4br
    C=2r(delta+S+a)+4b
    Q=A^2-C^2(delta+2)(S+2).

Substituting the symmetric leaf sum s and product t in S3 gives A-C*s. Q is its product over both signs of s. It is monic quartic in S: A is monic quadratic and the subtracted term has degree <=3. This remains true for all r and all delta. For every root S of L_d, both signs are rational as above, so Q=0 iff at least one full-fibre pair satisfies S3. For C!=0 recover s=A/C; for C=0 require A=0 and retain both allowed s (or one if s=0). Recover the unordered leaf abscissae as roots of Z^2-sZ+(delta+S). The torus proof ensures these roots are rational on the full selector, even when equal. At delta=-2 use A, a monic quadratic, in place of Q=A^2; only s=0. At delta=2 the pair is diagonal, retain the true tangent and no phantom identity target. The target O is separate: precisely the opposite-sign pairs of actual eligible leaves. For every affine candidate require both abscissae in actual M, reconstruct both signs in the specified source subgroup, and replay the exact signed group equality. This final guard is necessary; full-fibre norm acceptance alone is not an actual-base hit.

Evaluate gcd(Q,L_d) by reducing D_(N/2) modulo Q with k-1 Dickson doublings D_2(Z)=Z^2-2, then subtract parity*c. Each residue has dimension four (dimension two at delta=-2), so O(log N) field operations per offset, including gcd of constant-degree polynomials; exact gcd=1 proves a miss for this entire full-fibre offset. Factor only the nonconstant gcd, whose roots already lie in F_p. No claim that the full field polynomial was factored for free. The previous quartic in w and Q obey Q(w^2-2)=F_r(lambda,w)F_r(lambda,-w); degree and signs are testable by formal expansion.

## 4. Dimensions, complete cost, limitations

Supplied and verified torus seed/root and actual base permit O(N) recurrence work and storage for the N/2+1 offsets, plus charged root/seed generation and source/subgroup membership construction. The full selector degrees are N/2, not N. For inherited N=64/B=31 there are 33 offsets, each selector degree 32, including a quadratic norm at the antipodal offset. The full unordered trace-pair chart has 2080 entries, whereas the actual 31 eligible trace labels have 496 unordered pairs before elliptic signs. These are counts, not native measurements.

All offsets require O(N log N) basic field work on a full-fibre miss, plus O(N) offset accesses. If C_hit offsets have nonconstant gcd, price their constant-degree rational root isolation (e.g. expected polylog(p) randomized factoring), reconstruction/sqrt and label checks, and every emitted signed witness and proof byte. Even an actual-base miss may have C_hit>0 due to the filter. A deterministic completeness protocol must also account for root-isolation failure/timeout as indeterminate. For fixed lambda, F_r is monic quartic in w, so there are at most four candidate w values per offset, not eight; antipodal symmetry gives at most two unordered pairs there. Degenerate C=0 is not permission to drop either sign. A certificate can stream gcd Bezout identities for rejected offsets, and full candidate lists and signed checks for the rest, with O(N) blocks and all replay work charged.

In the density regime N=Theta(B), the O(N log N) construction is not an asymptotic improvement on the existing exact O(B) group-complement two-leaf scan. In particular, it does not remove the D=Theta(B^2) empty four-leaf query bottleneck. The degree-halving is an exact representation improvement for the two-leaf selector, not a complete solver or timing result. Enumerating offset pairs costs Theta(N^2) before solving; do not count a supplied tuple evaluation as extraction. A new gate is to batch or eliminate offsets with complete signed witness recovery and comparable setup without paying that product, including actual-M filtering.

## 5. Validation plan and provenance

Pinned source: iteration9/PAIR_SPECTRUM_RESULTANT.md blob e2348e91dc08141a663c5adb9798b6ca8f09424e; iteration10/S3_PENCIL_CONTRACTION.md da12c7a459405f4fdea44118cc03ade721d5eb90; iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md 953b48e251bdbd6fce9b7748cd4b05c5d60afb7c; iteration13/RANK_STAR_CONTRACTION.md b5b00271ed34ece5a48573b89c7406ee1d70bedf. Primary literature scope: Semaev 2015 arXiv:1504.01175 (summation chains); Wang–Yucas DOI 10.1016/j.ffa.2012.02.001 functional Dickson identities (accessible author text); Bhargava–Zieve DOI 10.1006/ffta.1998.0221 on bivariate Dickson factorization (metadata/abstract only). Standard identities, no novelty claim. Shared KB/Conductor not exposed and no repository checkout or Rust toolchain present. Prior root build failures remain historical. No production source, workflows, shared STATE or reserved report edited. Native agents unavailable; coordinator handles algebra and a separate post-freeze symbolic check, not independent signed validation.

After freezing, check the polynomial norm identity and monicity symbolically; later native qualification must enumerate all small-field offset/leaf charts against actual subgroup points and all targets, including c=0, diagonal, antipodal, zero/one/both sign branches, root collisions, outside-M roots, identity, duplicate tuples, and augmented rank. Use the original arithmetic and preserved pair spectrum as controls. No cryptographic computation authorized or performed by this note. [pcc:unverified]
