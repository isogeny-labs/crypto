# Run72 — exact univariate degree barrier for B677 signed pair support (2026-11-22)

## Verified-input algebraic deduction

Curve: E over Fp, p=2147483647, y^2=x^3+847514846x+2087444724. Original Rust n=2147446831 (odd prime), cofactor 1.

The original Rust pair_table in research/prime31_ic_20261115/native/examples/prime31_run69_genuine_partial_rank.rs starts from the canonical 677-point B, constructs the 1354 signed points A=B union (-B), enumerates each unordered signed-point pair including repetitions, and records its **group sum**. The resulting exact-size assertion is |A+A|=916469; earlier native Rust CI (Run68/69/71) verified this fixture including complete recovery. This is a frozen observed cardinality, not a conjecture about generic sizes.

**Lemma.** For any elliptic curve over an odd-characteristic field without nonzero rational 2-torsion, and any finite symmetric C=-C containing identity O, the finite x-coordinate image X(C) has precisely (|C|-1)/2 elements.

**Proof.** In C minus O, the involution P -> -P is fixed-point-free because 2P=O has no nonzero solution. Two affine points have equal x-coordinates exactly when they are P and -P, hence each x-coordinate has exactly two antecedents in C. QED.

Here C=A+A contains O (P+(-P)) and is negation-stable since A=-A. The prime-order original curve has no nonzero rational 2-torsion. Thus **the exact number of distinct pair-sum x-coordinates is (916469-1)/2 = 458234.**

**Corollary.** Any NONZERO univariate polynomial over Fp vanishing on all 458234 of those pair-sum x-coordinates has degree at least **458234**, by the elementary root-count bound. The direct product of linear factors has exactly that degree and 458235 coefficients in dense representation.

The 1354 signed points produce 1354*1355/2 = 917335 unordered pairs, just 866 more than the measured 916469 distinct group sums. This makes dense univariate encoding of all pair-sum x-coordinates almost a direct re-encoding of the generic dictionary.

## Scope and limitations

This is **only a polynomial degree bound** for exact full-support univariate vanishing predicates. It is not a complexity lower bound for fast algebraic circuits or structured high-degree polynomials, multivariate Semaev/RR9/F3 elimination, sparse resultants, prefilters, or ECDLP. The underlying pair dictionary is a GENERIC calibration and does not qualify as index-calculus or a novel relation extractor. No new Rust run is claimed in this memo; the exact input count is sourced to the existing Rust hard assertion and CI.

The result is a reason to avoid a single naive dense root polynomial as a replacement for generic pair lookup; it does **not** rule out a compact factorization, low-operation circuit, or an unknown-index multivariate algebraic extractor.

## Next falsifying experiment

Compare (a) exact generic group pair lookup, (b) direct dense univariate polynomial evaluation, and (c) a verified compact symmetric S3/FASS circuit on the same public samples, charging precomputation, arithmetic, memory, unsuccessful trials and amortization. A candidate qualifies for the user's complete non-generic solver only when actual public-only unknown-index roots from RR9/Krylov PR #4822, Horner/F3 PR #4812, selector/F4 PR #4692, or B8 coefficient PR #4919 pass their blind corpus and feed complete authentic-RHS recovery. FASS PR #4921 currently contains an unmeasured protocol, not an established advantage.

Relevant primary literature: Amadori, Pintore and Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009; Kudo, Yokota, Takahashi and Yasuda (2018), https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/.

Original native Rust source evidence: https://github.com/isogeny-labs/crypto/blob/research/prime31-f4-admission-run51-20261121/research/prime31_ic_20261115/native/examples/prime31_run69_genuine_partial_rank.rs; verification reference: https://github.com/isogeny-labs/crypto/actions/runs/45072185961.
