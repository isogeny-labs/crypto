# Prime31 RR9 D9 matrix-free Wiedemann rank gate — preregistered 2026-11-17

**Status: prospective; no computation or speedup yet. [pcc:unverified]**

## Inputs, unchanged from #4548–#4550

- Original ordinary nonsingular cofactor-one E/Fp: p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251).
- Fixed B8 x=(0,1,2,3,5,6,8,9), H_B monic product; public positive R=(1911579686,548820096), two public off-support negatives (625037715,2100627251) and (1923055775,307731458).
- Original **Rust** native prime31 arithmetic and `prime31_rr9_public_macaulay.rs` equation constructor, without support masks, secret k, or target cherry picking.
- Homogenized D9 Macaulay M (27456 rows × 11440 columns) for eight RR9 quadrics in seven normalized coefficients; each row is q_i times a monomial of total degree ≤7. The dehomogenized order corresponds bijectively to degree-nine homogeneous monomials in the extra h.

## New bounded algorithmic test

1. Build sparse indexed rows once per public residual. Assert row and column counts; deterministic sorted monomials, nonzero coefficients, field p and no exceptional x(R) in base.
2. Define A=Mᵀ D M for a deterministic PRNG-seeded **nonzero** random diagonal D over Fp. Use matrix-free M·v and Mᵀ·(D·) operations. Do not form dense M or A, and do not interpret a singular A as evidence that M is singular (it may be an isotropic preconditioner).
3. Generate exactly 2N scalar Krylov moments s_i=uᵀ A^i v, N=11440, with deterministic independent nonzero vectors u,v, and apply classical Berlekamp–Massey.
4. **One-sided certificate:** if scalar sequence linear complexity is exactly N and the order-N connection polynomial has nonzero last coefficient, A is nonsingular and thus M has full column rank. Combined with #4550 no-infinity theorem this proves absence of an affine B8 RR9 solution over an algebraic closure for that public R. Every other outcome is **inconclusive**, not a positive relation.
5. Run negative indices 0 and 1 independently, each with a 1500-second timeout, preserving completed JSON results; do not change seeds or budget after seeing results. The static seeds are 0xD990001 (negative 0) and 0xD990002 (negative 1). A timeout remains censored.
6. Unit tests first: direct sparse operator vs an independently built dense small D3 Macaulay, rank-2 nonsingular and rank-deficient toy matrices, original positive D9 evaluation witness from #4550 (no candidate access to witness), and bounded 16-step D9 smoke.
7. Measure sparse storage, row nonzeros, moments, iterations, wall and deterministic frozen source SHA in CI. Use Rust 1.100.0 pinned native source, 40 original library tests, focused wrapper tests. Do not claim a complete DLP or competitive end-to-end runtime.

**Proof of the one-sided rank rule.** For any N×N A, the scalar sequence has a linear recurrence of degree at most N by Cayley–Hamilton. If the 2N-term Berlekamp–Massey sequence has linear complexity N, its connection polynomial coincides with the characteristic polynomial of A (the order-N recurrence is uniquely determined by the nonsingular N×N Hankel matrix). If its constant term is nonzero, det(A)≠0. Since rank(A)≤rank(M), M has full column rank. False negatives possible from projection or preconditioning, but this success gate admits no false full-rank result with correct field operations.

**Scientific frontier.** A rank certificate is only a relation-absence component. Positive coefficient recovery, RR9 B31 multiplicities and exceptional branches, unknown-log linear algebra, and the full identical-Q cost comparison are separate unsolved tasks. #4596 is already running a separate native complete-IC 1024Q comparison; this PR does not duplicate it.

**Sources/previous leads:** native #4548/#4549 D5/D6; #4550 D9 separator; #4581 seven-equation quotient; #4575 B8 MITM; B31 4+4 oracle and S9_B31_FOUR_PLUS_FOUR_ORACLE_ARITHMETIC_RESULT_20261116.md; autoresearcher #3301/#3305; cryptanalysis #1536. Wiedemann and rank preconditioning are standard prior art, see Dumas–Kaltofen–Thomé 2015 arXiv:1507.01083, Eberly 2016 arXiv:1607.04514, Dumas–Kaltofen 2014 arXiv:1401.4567. Amadori–Pintore–Sala 2018 DOI:10.1016/j.ffa.2018.01.009 is relevant prime-field IC prior art. Research-KB tool/CLI unavailable here; bounded source fallback, no novelty claim.

**Stop/go:** If no valid full-rank negative certificate within frozen cap, record the null/censored result and prioritize rank-guided sparse F4 or quotient extraction (not blind dense degree ≥10). No leaderboard promotion without independent PCC and complete-solve cost.


## 19:20 UTC evidence-driven extension (not in original preregistration)

Source-pinned GitHub Actions run 44156703850 completed **both** originally registered negative cases at D9 with BM complexity 11440, nonzero terminal coefficients and approximately 20.7 seconds per case, without any curve, seed, protocol or outcome changes. The producer-level summary alone is not a replayable algebraic certificate. Before implementing this follow-on, register a **constructive unit-certificate extension** on the **same** two negatives (not a new search or performance comparison):

- From the full BM recurrence C(z)=z^N+c1 z^(N−1)+...+cN, with cN≠0, derive an inverse action on the constant monomial e0:
  x=−cN^(−1)·(A^(N−1)+c1 A^(N−2)+...+c_(N−1) I)e0,
  where A=Mᵀ D M and N=11440.
- Compute w=D M x, and **replay the full polynomial identity Mᵀ w=e0** using the original public RR9 quadrics and all 27,456 degree≤7 multipliers, independently of the Krylov moment test. In affine form this is explicit 1=Σ_i w_i m_i q_i, a degree≤9 Bézout certificate for each negative public target.
- Record every coefficient of w in frozen Macaulay row ordering as a CI artifact, with exact source pinned. A separate independent reviewer can rebuild M from public R,H_B, multiply Mᵀ w and check e0 without trusting BM, field arithmetic counters or negative-support enumeration.
- If inverse construction or certificate replay fails, fail CI and preserve the error; do not report a negative certificate based solely on rank flags. Allow up to 1500 seconds per frozen negative as originally declared. Not a complete ECDLP solve or a new algorithmic exponent.
