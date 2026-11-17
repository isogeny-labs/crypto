# Prime31: sparse degree-nine RR9 unit certificates for two public negative residuals

2026-11-17 UTC. Producer claim standing **[pcc:unverified]**; separate certificate replay passed, signed independent PCC not yet recorded.

## The result

For the actual ordinary cofactor-one elliptic curve
\[
 E/\mathbb F_p: y^2=x^3+847514846x+2087444724,
 \quad p=2147483647,\quad \#E=2147446831,
\]
and the fixed eight-point factor-base abscissas (0,1,2,3,5,6,8,9), the original Rust RR9 norm-chart equations now admit **explicit degree-nine Bézout unit identities** for two frozen off-support public residuals:

- R0 = (625037715, 2100627251) (the actual generator).
- R1 = (1923055775, 307731458).

The input to the candidate is only the public R, H_B and the same original curve; no signed factor-base leaves, hidden scalar or group-sum enumeration is passed. The binary of the candidate reuses the existing native prime31 Rust RR9 coefficient equations and the original prime31 arithmetic, not a new Python ECDLP solver.

Let z=(a0,a1,a2,a3,v0,v1,v2) and V(X)=X^3+v2 X^2+v1 X+v0. For R=(r,s), define D_R(X)=X^2+rX+r^2+a_E and A(X)=a3 X^3+...+a0. Write
\[
 q_j(z)=[X^j]\big(D_R(X)V(X)^2-(X-r)A(X)^2-2sA(X)V(X)-H_B(X)\big),\quad 0\le j<8.
\]
The exact, original finite-field construction gives, separately for R0 and R1, a coefficient array w indexed by (j,monomial m of total degree at most seven) satisfying
\[
 \boxed{\displaystyle
 1 = \sum_{j=0}^7 q_j(z)\!\sum_{|\alpha|\le7}w_{j,\alpha}z^\alpha
 \quad\text{in }\mathbb F_{2147483647}[z_1,\ldots,z_7].}
\]
This proves the **absence of an affine normalized RR9 B8 coefficient solution even over the algebraic closure**, under this chart's assumptions. The polynomial identity is independently checkable with only modular polynomial multiplication; it does not require trusting the Wiedemann solver, rho, the signed support oracle or any expensive Macaulay rank computation.

For these two residuals, the earlier D5 and D6 dense experiments had no negative unit certificate; degree nine is the first now-computed sound exact separating degree. The positive B8 fixture from #4550 has a separate known-support test-only degree-nine nonzero right kernel and is not misclassified as a unit.

## How the Rust algorithm constructs an exact witness

Build the D9 Macaulay coefficient matrix M with eight quadrics times all monomials of total degree <=7: **27,456 rows × 11,440 columns, 240,240 nonzeros**. Form only sparse operators for A=M^T D M over Fp, where D is seeded nonzero diagonal. Use 22,880 Krylov scalar moments and Berlekamp–Massey. In both frozen negatives, scalar linear complexity is N=11,440 with nonzero terminal connection coefficient; this certifies A and hence M full column rank.

Moreover, if \(\chi_A(t)=t^N+c_1t^{N-1}+\cdots+c_N\), define \(x=-c_N^{-1}(A^{N-1}+c_1A^{N-2}+\cdots+c_{N-1}I)e_0\) and \(w=DMx\). The Rust code performs 11,439 additional Gram applications and then directly verifies \(M^Tw=e_0\); the latter is exactly the displayed Bézout identity. A **separate Rust example** reconstructs the public quadrics independently and verifies the same identity from the emitted w without running BM or using the candidate's sparse Macaulay generator.

Only one-way full-rank results are interpreted. Over a finite field, M full rank need not imply M^T D M is nonsingular for an arbitrary D because of isotropic subspaces; an inconclusive BM run does not establish that a positive relation exists.

## Frozen raw evidence

| Actual public residual | BM degree | BM final coefficient | D9 unit-replay | Fully charged Rust wall |
| --- | ---: | ---: | --- | ---: |
| R0 (actual generator) | 11440 | 804920363 | Identity verified | 27.142 s |
| R1 | 11440 | 948224476 | Identity verified | 27.190 s |

Every unit witness has **27,456 nonzero modular coefficients**. Both have polynomial-identity check constant 1 and 11,439 zero nonconstant coefficients. The separately constructed direct arithmetic validation also checked all 256 B8 signed group sums and independently confirmed neither negative lies in that support; the single known-positive test point is one of the 256. That generic oracle is strictly for checking, never an extractor input.

**CI:** pinned original Rust 1.100.0 on GitHub-hosted Ubuntu, exact original arithmetic source 1df714645aa683cf975bc76fcacb9842cb4780fd, 40 original native library tests, sparse D3 vs dense construction, 2×2 full-rank/singular controls and original positive D9 null witness. Source-specific proof/replay workflow run [#44157953813](https://github.com/isogeny-labs/crypto/actions/runs/44157953813) completed all steps successfully including both independent Rust unit replays. [Full 426KB-per-target modular witness artifact #14362356558](https://github.com/isogeny-labs/crypto/actions/runs/44157953813/artifacts/14362356558), independently downloaded with ZIP SHA-256 `7072196fa51a60b7b0559c3caa88ac7e0491938e8c9749d5f062c0a6133fbbe4`. The source-specific GitHub Actions run #44157634359 produced the initial full witnesses (artifact #14360619996, SHA-256 `1bcb1c9653f59b6c40aa31f2c511b208ab74808a4f87a33bb158acb00ade563f`), each separately verified in a second, independently reassembled modular arithmetic program. Full coefficient vectors live in these exact-source artifacts (30-day retention), while a compact permanent summary is checked into the repo. No signed PCC assertion or external peer review has been obtained.

## Prior leads excluded / experiments to prioritize

- **D5–D6 dense Macaulay (#4548/#4549):** rank 736/792 at D5 and 1688/1716 at D6 matches the regular-sequence Hilbert coefficients; no unit certificates. Do not just push the same dense elimination to D10.
- **D9 original theorem/positive witness (#4550):** its no-infinity argument plus the Hilbert series explains sufficiency of degree nine; #4604 adds actual negative unit identities rather than only a test-only positive evaluation vector.
- **B8 signed fibres (#4545, #4556, #4575) and seven-of-eight quotient (#4581):** all expose branches equivalent to signed point enumeration; the 4+4 generic control wins at B8. Not a non-generic asymptotic acceleration.
- **B31 exact eight-leaf oracle and multiplicities:** the existing original Rust signed 4+4 oracle is a correctness reference only. The actual Dickson31 24-Q panel had 22 eight-sum supports and 2 genuine negative residuals, so repeated-root/exceptional handling and randomized residual transforms are necessary for a complete solver. Squarefree-only support is not complete.
- **Cross-repository negative/censored leads:** crypto-autoresearcher #3301/#3305 (Semaev alternatives, Vieta/m6/m8, lead atlas), cryptanalysis #1536 (auxiliary-base IC) and crypto #4373/#4228/#4349 (sparse F4, quotient, rootfinding).
- **Published prior art:** Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018), DOI 10.1016/j.ffa.2018.01.009, and Dumas–Kaltofen–Thomé (arXiv:1507.01083) on Wiedemann certificates. This is an application of known linear algebra and a validated RR9 test, not a new asymptotic ECDLP algorithm. Research-KB unavailable in this runtime; bounded code/primary-source review, no assertion that the literature has no prior solution.

## Priority experimental queue and falsifiers

1. **Sign-free positive B8 extraction:** use the existing Rust sparse F4/quotient machinery to recover normalized coefficients for the positive fixture without knowing any sign/support. Negative unit certificates are now robust controls. Failure to recover the positive or a high phase cost retires the candidate configuration.
2. **Full B31 natural-target algebraic extraction:** same original Rust systems and target-conditioned Dickson H31/K961/S5 machinery, with repeated roots, tangent jets, x(R) exceptional cases, B31 first-x and Dickson bases kept distinct. Demand exact matching of the registered 22-positive/2-negative Dickson panel and independent group-law replay; count all misses.
3. **Complete unknown-target cost:** on at least 96 blinded targets, and then the frozen 1,024-Q prime31 corpus, collect sufficient verified independent relations, solve all base logs plus target logs and charge setup, elimination, failed trials, memory, verification and rootfinding. Compare against strongest equal-advice shared BSGS, optimized rho and published prime-field IC. The separate concurrent batched IC/BSGS Rust benchmark is already PR #4596; do not duplicate it.
4. **Independent standing:** replay the raw Bézout vectors via separately built polynomial coefficients (Rust verifier in this PR); then obtain signed PCC verdict before leaderboard promotion or theorem-level claims.

**Performance verdict at this scope:** the new algebraic negative-certificate phase costs approximately 27 seconds per target; this is **not competitive with** the B8 generic membership component, much less a complete logarithm solver. Exactly zero new blinded unknown prime31 discrete logarithms were solved in this experiment. A cheaper, complete genuinely non-generic prime-field ECDLP solver is **not** established here. [pcc:unverified]
