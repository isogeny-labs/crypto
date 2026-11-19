# Frozen x-only signed Horner/F3 chart and sign-lifting gate (2026-11-19)

Status: protocol committed before source, validation, or execution. Follow-up in the existing #4812 Horner lane; not a solver speed claim.

## Research question and original-curve custody
Can the signed powers-of-two eight-factor relation
`Q = P0 + [2]P1 + ... + [128]P7`
be represented with substantially fewer polynomial variables than the existing 72-scalar x/y Horner chart, without accidentally losing **sign coherence**, and while preserving a sound post-solve signed witness recovery algorithm?

Use the **unchanged original** native Rust curve: p=2147483647, a=847514846, b=2087444724, n=2147446831 (prime), cofactor=1, G=(625037715,2100627251), original crate source ref `1df714645aa683cf975bc76fcacb9842cb4780fd`. First-31 rational-x factor base, four fixed overlapping B8 windows [0..7], [8..15], [16..23], [23..30]. Rust library `Arithmetic`, `PRIME31`, `check_curve` are reused unchanged.

## Candidate algebraic system (no group tables)
Let each leaf x_i be a root of H_i(X)=product of (X-x(P)) over its public eight-point window. Write t_7=x_7 and let t_i be the x-coordinate of the Horner suffix T_i=P_i+[2]T_{i+1}. Introduce d_i=x([2]T_{i+1}) for i=0,...,6. Define
`f(X)=X^3+aX+b`,
`N(X)=X^4-2aX^2-8bX+a^2`,
`D(X)=4 f(X)`,
`F3(u,v,r)=(r-u)^2 v^2 - 2((r+u)(ru+a)+2b)v+(ru-a)^2-4b(r+u)`.

Equations for each i=0,...,6:
`D(t_{i+1}) d_i - N(t_{i+1})=0` (where t_7=x_7),
`F3(x_i,d_i,t_i)=0`.
Add eight public leaf equations `H_i(x_i)=0` and the terminal equation `t_0=x(Q)`. That is 8 leaf variables + 7 suffix t variables (t0..t6) + 7 doubled d variables = **22 variables, 23 equations**; eliminating t0 reduces to **21 variables and 22 equations**. Doubling and F3 equations have total degree at most four; H_i is degree eight.

### Scope of the soundness theorem being tested
For a rational solution whose leaves lift to the specified base points, and for which all intermediate points are finite/nonzero on E(Fp), there exists a signed choice of each public base leaf with weighted sum **Q or -Q**. The final global sign can be flipped to get Q. The proof is inductive: start with either sign of P7; for each F3 step, because its other two x-arguments lift to curve points, t_i is x(P_i + [2]T_{i+1}) or x(-P_i+[2]T_{i+1}); select a compatible leaf sign. For a nonzero finite point R, x(R)=x(Q) implies R=±Q; flip all leaf signs to match Q. Doubling equation uniquely determines d_i because the odd prime-order group has no rational 2-torsion, hence f(t_{i+1}) != 0. This is a theorem only for the finite nonidentity chart. If any intermediate suffix is infinity, handle it as an **exceptional branch**; it must NOT be silently treated as nonmembership. No assertion that F4 can efficiently solve the system or that it is a novel algebraic attack.

## Frozen experiment and falsifiers
- Deterministic SplitMix64 seed `0x584f4e4c595f4348`, 256 generated signed eight-leaf tuples per window, total 1,024 planted witnesses. Public x leaves and public x(Q) are input to polynomial checker; signed leaves are allowed only in fixture generation and independent post-solve verification.
- Count complete ordinary x-only charts separately from infinity exceptions, insist `ordinary + exceptional = 1024` and at least 1,000 ordinary (unexpected failures invalidate the predeclared acceptance gate).
- For each passing tuple, verify every H_i root, seven doubled-coordinate equations and seven F3 equations in native finite field arithmetic; require all residuals zero.
- Recover signed leaves using only (public x_i, public suffix t_i, public Q, public factor-base point table), with a post-solve group-law replay and global sign flip. Never use planted signs in this recovery. Verify [sum weights] == exact Q. Fail on any wrong or missing signed recovery.
- Poison d_i by +1 for the first step, and poison the final target x-coordinate by +1. Require an algebraic residual in each case, but don't claim those altered public targets lack other witnesses.
- Add two focused Rust tests and a release executable using the original native crate. Independently verify a sample of native reported certificates using Python bigint EC arithmetic, checking exact polynomial residues and recovered signed replay.
- Report equation/variable counts, observed chart exceptions, native field multiplications and group counts as descriptive diagnostics, **not** a cold whole-solver benchmark; wall timing is not a primary claim.
- No generic 4+4 table or any precomputed group-sum table in the proposed polynomial checker. Do not confuse test fixture generator (group operations) with non-generic index extraction.

## Prior work and interpretation
Read existing #4812 x/y chart, #4809 batched F3 (the F3 identity is standard), #4810 Kummer doubling permutation, #4692 RR9/F4 owner, #4808 weighted block-scale gate, #4728 complete grader, #4532 optimized shared BSGS. Compare to existing `research/ic_tree_gluing_20261103` (S3 cherry trees) and `research/prime31_ic_20261115/native/M8_UNIFORM_RANK_CEILING_AND_F4_PILOT.md`. Published prior art: Yokoyama et al., *Complexity bounds on Semaev's naive index calculus method for ECDLP* (2020, https://doi.org/10.1515/jmc-2019-0029) cautions against equating low polynomial degree with fast Gröbner solving. The Semaev identity and x-only Kummer quotient are not claimed novel. Shared KB server unavailable in the connected tool roster; repository code search and current public literature used as limited fallback.

## Decision gate
If certificate parity passes, send the **21-variable** system to the existing #4692 owner for strict public-target-only elimination on the frozen 46-positive/18-negative B8 controls, charging all failed queries, solving degree, sparse fill and memory, then try full rank31/1,024 logs against same-runner strongest BSGS/IC/rho frontier. If sign lifting fails or chart exceptions dominate, stop and fix the mathematical model. Signed independent PCC remains required. No full index-calculus advantage inferred from planted fixtures.
