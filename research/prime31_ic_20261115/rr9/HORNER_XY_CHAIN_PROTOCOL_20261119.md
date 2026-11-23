# Frozen Prime31 signed weighted x/y Horner-chain gate — 2026-11-19

Status: protocol frozen before implementing or executing the candidate. This is a **constraint-construction and correctness** study, not an ECDLP solver speedup.

## Exact question
Can the 8-slot powers-of-two weighted relation
`Q = P0 + [2]P1 + [4]P2 + ... + [128]P7`
be represented as a low-total-degree polynomial system with fewer explicit addition/doubling steps than the previously studied split `U + [16]V` chain, while **retaining both signs** of unknown factor-base points and rejecting inverse-denominator extraneous solutions?

Use unchanged original ordinary cofactor-one `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. Reuse the native Rust Arithmetic and first-31 rational-x factor-base points, four windows [0..7], [8..15], [16..23], [23..30]. Source pinned to original crate `1df714645aa683cf975bc76fcacb9842cb4780fd`.

## Algebraic construction
Define `T7=P7` and `Ti=Pi+[2]T(i+1)` for i=6 down to 0. Seven shared doublings and seven additions replace independent left/right Horner plus four extra middle doublings. For each ordinary affine step introduce two slopes, two slopes' inverses, the doubled coordinates and the new sum coordinates. Eight polynomial equalities per step:

- `(2Y)λ=3X²+a`, `(2Y)ρ=1`,
- `X2=λ²−2X`, `Y2=λ(X−X2)−Y`,
- `(xi−X2)μ=yi−Y2`, `(xi−X2)σ=1`,
- `Xnew=μ²−X2−xi`, `Ynew=μ(X2−Xnew)−Y2`.

Per leaf require `H_window(xi)=0`, `yi²=xi³+axi+b`, where `H_window` is the public product of the eight x-coordinate linear factors. End with `(X0,Y0)=Q`. All 74 displayed equations are over Fp. The 56 non-leaf equations have degree at most 2; leaf equations have degrees 8 and 3. There are 72 scalar variables before eliminating two terminal coordinates. The inverse-witness equations *saturate* denominators on the ordinary chart; they must not be silently dropped.

## Frozen controls and accounting
- Generate 64 signed eight-leaf tuples in each of four original B8 windows (256 total) from a deterministic SplitMix seed `0x4858_595f_4348_4149`, never from hidden target logarithms. Calculate Q using original native point arithmetic.
- Validate each leaf factor predicate and curve equation, all eight identities for each of seven steps, inverse-witness certificates and public-target equality, independently from the existing group operations.
- Record exceptional affine charts (zero/infinity denominator) as explicit failures to certify; never report these as absence of a relation. Require at least 240/256 ordinary-chart passes, otherwise investigate.
- For each passing witness, reject the **same witness** at mutated public target `Q+G`; this does NOT prove the mutated Q has no other witness. Also mutate a nonzero doubling slope; demand an algebraic residual.
- Compare counted group additions of Horner vs literal eight weighted scalar multiplications on *identical* witnesses; prices are a **witness construction control**, not nonenumerative solver cost. Report exceptional cases, positive certificates, negative mutation checks, and equation residuals.
- Run original 40 Rust tests plus focused example tests and produce JSON in release. Cross-check selected instances with independently written Python affine arithmetic, if possible.
- No generic sum-table lookup inside the constraint constructor. Signs and x choices are allowed *only as planted oracle controls*; the system description itself takes public B8 x values and public Q.

## Falsification and stopping rules
- Any nonzero identity residual, incorrect terminal Q, accepted mutated slope, or unhandled exceptional point invalidates a claimed ordinary-chart certificate.
- The chart is incomplete unless exceptional cases are handled separately. Algebraic F4 efficiency is unknown; the 72-variable system is not automatically better than RR9 or F3.
- No claim of a complete, public-target-only, non-generic B8 extractor until it recovers unknown x indices without planted witness, signs, or generic tables, verifies negatives, and passes the 46+/18− corpus.
- No complete ECDLP advantage until it is integrated into the rank31/1,024-log Rust solver and fully charged alongside original-runner mixed IC, optimized 2^20 shared BSGS, and rho.

## Prior work and publications
Repository owner #4692 (RR9 public-only extractor); #4809 (Semaev F3 algebraic pair primitive, avoid duplication); #4810 (x-only [16] permutation); #4808 (block-scale U+[16]V, generic grading); #4728 (rank31/1024 generic-table IC); #4532 (170.113ms total setup+online matched BSGS baseline); #4801 and #4803 (interpolation and unweighted coverage obstructions).
Published background: Petit–Kosters–Messengur (PKC 2016) rational-map factor bases; Amadori et al. (2018) summation-polynomial variant; Kudo et al. (2018) hybrid Gröbner attack; journal analysis 'Complexity bounds on Semaev's naive index calculus method for ECDLP' (2020). Existing quadratic x/y addition equations and Horner double-and-add are standard; **no novelty-by-absence claim**. The shared research-KB service was not available through the accessible tools; GitHub and public primary-source fallback used.

Protocol frozen by the initial protocol-only GitHub commit before source edits. [pcc:unverified]
