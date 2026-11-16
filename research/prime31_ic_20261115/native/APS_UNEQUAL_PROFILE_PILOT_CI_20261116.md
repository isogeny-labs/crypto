# Prime31 APS unequal partitions — first complete cold Rust panel (2026-11-16)

**Exact executable evidence:** [GitHub Actions #43817341546](https://github.com/isogeny-labs/crypto/actions/runs/43817341546), job `133132277587`, source `fbf312c09005080f3032d4f2232a7cbd41c2bea1`, Rust 1.100.0 release, successful. All 24 public Q have independently replayed complete unknown scalar recovery in each of four arms (96/96). Full exact original program JSON, including all source labels, target points, per-round counters, cold timings, and caller-side known answers, is frozen as [APS_UNEQUAL_PROFILE_EXACT_CI_20261116.json](APS_UNEQUAL_PROFILE_EXACT_CI_20261116.json). **Producer/development evidence, [pcc:unverified].** Does NOT establish a novel non-generic ECDLP or rank against the external PKM/APS/CAS frontier.

## Same-curve, same-host complete unknown-log result

Same ordinary prime-order curve `E/F_{2147483647}: y²=x³+847514846*x+2087444724`, `n=2147446831`, cofactor 1; target scalars never enter the Rust solver. Each arm uses the same 64-step public additive-walk source law, source-specific 3 partitions, and the same S3 Mersenne field solver and signed full-point/scalar replay. Each solver starts cold, pays full public point generation, failed rounds, algebraic query work and verification. Arm order alternates across fresh Q. This trial compares **published APS methodology specializations** with generic MITM-equivalent finite pair-root scanning.

| Arm (A,B,C) | Correct logs | Total cold wall, 24 Q | S3 quadratic pairs | Source group additions | Versus control |
|---|---:|---:|---:|---:|---:|
| Control (1536,1536,1536) | 24/24 | **1,335.138 ms** | 9,254,400 | 259,163 | — |
| Unequal (256,256,32768) | 24/24 | **380.740 ms** | 415,744 | 947,529 | 3.507x less wall |
| **Unequal (384,384,16384)** | **24/24** | **258.271 ms** | 624,000 | 560,740 | **5.170x less wall** |
| Unequal (256,256,32768), A/A repeat | 24/24 | 369.338 ms | 415,744 | 947,529 | 3.615x less wall |

Both unequal candidates beat the incumbent on 22/24 individual public Q. The 256 A/A repeat is ~3.0% different in aggregate, within preregistered 5% drift threshold, but **the 384 candidate has no same-arm repeat in this first panel**. The previous #4383 1024-point win reversed on fresh holdout; therefore the 384 result requires an independent new-target, paired A/A confirmation before promotion.

### Why the improvement occurs, and why it is not a new non-generic attack

A three-list one-relation S3 query forms roots of `S3(x_i,x_j,Z)` for pairs `i in A, j in B`, and tests each root for membership in C. Enlarging the third list raises the hit probability per tested algebraic pair at the cost of producing more public labelled points. The measured 384 arm performs ~14.8x fewer S3 pair tests than equal T1536, but ~2.16x more public-source group additions; the *inclusive whole-cold* gain is ~5.17x. These are measured development ratios, not a proven cost exponent or security attack; actual success probabilities of correlated public walks are empirical.

The method still enumerates finite S3 pair-root support. It is mathematically **generic group meet-in-the-middle equivalent** despite using summation-polynomial Fp arithmetic. The completed 24-Q result is an optimization of our full published APS-style Rust implementation, not a new non-generic index calculus or a claim of superiority over optimized external solvers, mature F4, PKM or best rho. No independent PCC, isolated A/A, external CAS comparison, EKS or paid job ran.

## Prior-lead audit and next tests

- `research/aps_cost_leads_20261106/PROTOCOL.md` already includes *unequal profiles* at smaller prime sizes. This prime31 result instantiates and measures that concept; **do not claim novelty of the idea**. APS 2018 and PKM 2016 remain mandatory method controls.
- #4383 equal-size T1024 appeared faster on 12 targets but was slower on 24 fresh, motivating preregistered fresh confirmation. #4391 Mersenne field, additive-walk source and exact S3 controls are reused unchanged.
- #4373 m8 balanced/chained S3 F4 all censored; anchored m6 support ceiling 3.7407% and 0/16 natural witnesses; #4228/#4349 compact certified quotient lacks completed prime31 m8 original-ideal roots. Earlier generic S3 pair-root indexing is not a non-generic new method. `isogeny-labs/cryptanalysis/suite/src/cryptanalysis/symmetrized_semaev.rs` already implements classical Faugère–Gaudry–Huot–Renault symmetric polynomials; any Vieta/symmetric proposal must reuse that prior art.
- **Immediate:** 24 **new** public targets in a fresh 3-arm repeated 384-versus-control experiment with A/A (see `APS_UNEQUAL_PROFILE_384_CONFIRM_PROTOCOL_20261116.md`). Demand every heldout log be correct, equal group and finite-field semantics, full cold and all misses.
- **Separate genuinely non-generic path:** original m8 `H^8 mod f=0` symmetric finite-base membership + target-conditioned symmetrized S9 (or balanced certified resultant circuit); compare exact original rational-root sets on B4/B8 before B31. For B31 the 8-leaf orbit space is `C(38,8)=48,903,492` multisets (vs `31^8=852,891,037,441` ordered x tuples); this is a finite quotient **dimension count**, not a practical solver or guaranteed speed. Repeated roots require `H^8`, not `H`, and sign/infinity cases require explicit full-point checks. The classical symmetric reduction is prior art; proposed specialization is exploratory.

**Decision:** 384/384/16384 is the strongest development engineering candidate and is worth independent confirmation. Continue the distinct m8 non-enumerative algebraic program without misclassifying the APS component as a new attack.
