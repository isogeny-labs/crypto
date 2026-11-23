# Prime31 W8 complementary-zeta/Walsh sign evaluation — prospective Rust experiment (2026-11-18)

Tracking: existing [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728); original Prime31 cofactor-one curve `y²=x³+847514846x+2087444724` over p=2147483647, group order 2147446831. Reuse pinned original-crate Rust arithmetic at `1df714645aa683cf975bc76fcacb9842cb4780fd`.

## Input boundary and exact algebraic prediction

The **eight distinct positive slot abscissae and ordinates are supplied**. This is neither unknown-index recovery nor a complete non-generic DLP attack. The public target is R=(r,s), and sign mask eps selects +/- each supplied point. Let U run over the 70 four-subsets of [8], S=U-complement, V(S)=product_(i<j in S)(x_j-x_i), A_S=product_(i in S)(x_i-r), and Y_T=product_(i in T)y_i. Set

    h_U = (-1)^(sum_(i in S)i) V(S)V(U) A_S   if |U|=4; h_U=0 otherwise.

Define upper subset zeta H_T = sum_(U superset T) h_U. Then the coefficient of the sign monomial eps_T in the 8x8 target-reduced Riemann–Roch determinant is **exactly**

    c_T = H_T * Y_T * s^(4-|T|)   for |T|<=4, and c_T=0 otherwise.

A standard 8-bit Walsh-Hadamard transform of all c_T evaluates 256 signed determinants. This follows by swapping the order of the 70-term Laplace and subset expansions from W8_SIGN_FOURIER_PROTOCOL_20261118.md. Zeta takes 8*128=1024 modular additions; Walsh takes another 1024 butterflies; no inversions or group additions inside either determinant evaluator. This is a classical Boolean-lattice transform, not a novel ECDLP theorem. Pair only with actual point-sum truth checks; supplied x/sign fixtures may not leak into a proposed unknown-index candidate.

## Prior-art and research-state review before source change

- Existing source [#4728](https://github.com/isogeny-labs/crypto/pull/4728): 70 complementary Vandermonde minors, 1120 sign-monomial updates and 256-point Walsh transform; source-bound producer run reports 179200 field multiplications for 32 cases. This experiment **reuses** that implementation as the paired baseline.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4717](https://github.com/isogeny-labs/crypto/pull/4717) and [#4720](https://github.com/isogeny-labs/crypto/pull/4720): target-conditioned selector/F4, multiplicity and pair-projector leads; no claimed public-only B8 solution.
- [#4714](https://github.com/isogeny-labs/crypto/pull/4714), [#4722](https://github.com/isogeny-labs/crypto/pull/4722): full 1024-Q weighted M6 Rust pipeline and exact yield/formal-collision obstruction; M6 previously lost to shared BSGS.
- [crypto-autoresearcher SA-04 and SA-18](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md): negative determinant/norm gates. Prime-field summation-polynomial precedents: [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009). [#4746](https://github.com/isogeny-labs/crypto/pull/4746) retracts an optimistic PKM exponent. Research-KB MCP/wrapper inaccessible from current connector runtime: no novelty claim based on missing hits.

## Frozen test protocol, before native timing

Use the same first-31 positive canonical x-coordinate base, 8 weights 1,3,5,7,11,13,17,19, 16 planted signed x-index fixtures and 16 target+G shifted negatives from W8_SIGN_FOURIER_PROTOCOL_20261118.md. Confirm all 248 positive weighted x-coordinates are distinct. For all 32 targets x 256 masks, exact residue equality holds between (a) direct Gaussian 8x8, (b) prior 70-minor Fourier evaluator, and (c) new complementary-zeta/Walsh evaluator; in parallel independently check original-curve group-sum truth for all masks (16 positive fixtures with exactly one zero each and 16 negative with no zeros).

Record total and per-case instrumented Fp multiplication/inversion/addition counts; A/B alternate order for the two fast methods, *include* baseline direct method separately, and include diagnostic runtimes without treating them as ECDLP solve performance. Acceptance: 8192/8192 equal exact residues and group identities, all inherited 40 library tests and new Rust example test pass, standalone Python arithmetic checker passes, zero inversions for both fast arms, and new-counted Fp multiplication strictly below old. Fail closed on new false positives or duplicate coordinates. This is an **operation-count component optimization only**.

Independent local preflight prior to this protocol: a separate Python BigInt proof-of-feasibility implementation checked 2048 determinant residues on four planted/shifted original-curve fixtures (not prospective native timing, not PCC verification). Future native measurements occur only after this protocol is committed.

## Stop/progression rule

If the operation-count experiment passes, keep results at the supplied-x component scope. **No amount of fixed-x sign speed qualifies the ECDLP goal** without public-only unknown-index recovery. Next bounded gate: construct an algebraic selector using the precomputed h_U tensor and polynomial slot-membership ideals at B4/B8, forbid any generic 4+4 grader table and all seeded x-index charts, and test on the frozen 46-positive/18-negative B8 natural panel. Kill at a matched full-cost component threshold; only advance to B31 rank31 and 1024 blinded unknown logs if extractor succeeds, charging all setup, failures and certification against mixed IC/shared BSGS/rho. Independent signed PCC pending; do not promote leaderboard rows on producer evidence.
