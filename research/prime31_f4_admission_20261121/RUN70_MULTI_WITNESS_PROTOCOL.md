# Prime31 Run70 — fixed-budget multiple target certificates, authentic RHS rank (preregistration)

Date: 2026-11-22 UTC. Owner: draft PR #4901, existing Prime31 Rust lane. **This is a generic reference experiment, not a qualified non-generic ECDLP attack.** This protocol precedes the new source and CI measurements.

## Question and exact mathematical rule

Run69 uses only the first signed three-leaf certificate for each public target Q. A second independent signed decomposition represents the same target logarithm by another affine linear form. At any fixed true-RHS relation row space, Q is recoverable as soon as *any one* correct certificate's coefficient vector is in the row space. Therefore allowing K certificates (K=1,4,16) must weakly improve the authentic-relation stop count, if the algorithm chooses from the first K certificates of a single deterministic ordered search and uses the same true relation stream.

For Q+jG = ε₁P_a+ε₂P_b+ε₃P_c, record the signed indices and public j. On a DSU plus the 88 genuine homogeneous equations, a certificate is **determined** iff its residual coefficient vector vanishes after RREF reduction. The candidate scalar is the fully reduced affine constant; independently require [scalar]G=Q. Do not use secret scalars, generic BSGS, or group enumeration in the row-space decision.

## Unchanged baseline and paired design

- Original Rust crate: `research/prime31_ic_20261115/native`, original `Arithmetic, Base, PRIME31`, Rust 1.100.0; frozen curve p=2147483647, a=847514846, b=2087444724, G=(625037715,2100627251), prime subgroup order n=2147446831, cofactor 1.
- Rebuild exact 509+168 = 677 canonical points and verify 88 public homogeneous EC identities, including the +2 multiplicity row.
- Reuse the exact deterministic run69 32-point public-Q cohort (xorshift seed 0x68B677A1D1), but never let secret scalars enter the candidate search, relation source, rank solver, or witness-choice logic.
- Use run69's **explicitly generic** signed 916,469-key pair table only as a calibration oracle, identical for all arms. For each Q, enumerate public shift j=0,...,1023 and signed leaf t=0,...,1353 in that order; retain first 16 valid *distinct* (terms,j) signed-three-leaf certificates; stop when 16 found. If a Q has <16 certificates within cap, mark the experiment censored. Charge every rejected leaf, shift, proof, and setup cost.
- Walk identical actual public R=[k]G for k=1,...,12,000,000 and collect labelled signed pair relations R=εP_i+δP_j as in run69 (count every miss). Every 32 successful hits, use *the same* true-RHS DSU and 88 EC-verified homogeneous rows to test all three arms on the 32 held-out targets. Arm K=1 sees only the first certificate, K=4 the first four, K=16 the first sixteen; each target may use its first rank-visible certificate, not necessarily its first chronologically. Freeze earliest per-arm shift and actual hit/independent-rank counters. Continue until all arms have recovered all 32 (or cap).
- Verify all 96 output scalars (32 × 3 arms) independently by original Rust [k]G=Q and against post-solve harness scalars and an independent original-Rust BSGS control. Record whether K=1 reproduces run69's exact 5,378,231 public relation shifts / 2,272 hits / 651 independent pair rows; if not, fail closed and diagnose.
- Measure wall-time per stage, K=16 certificate discovery, relation/rank scan and the same-process BSGS; record source commit/hash, workflow, runner and limitations. No extrapolated qualification from components. Both library tests and focused example must pass.

## Admission/falsification

A successful control must have exact verified K=1 replay, valid monotonic stopping (shifts_16 <= shifts_4 <= shifts_1), independent log checks, unambiguous number of attempted public leaf probes, complete measured cold-path cost and matching original-Rust arithmetic. A failed compile, censored certificate, incorrect EC relation, inconsistent RHS, missed target, or mismatch with the registered control is a FAIL or CENSORED, never a claimed improvement.

Even a large rank-stop reduction does not establish a cheaper *non-generic* solver: the generic pair/triple table is not allowed in an index-calculus candidate. The next qualifying experiment replaces both relation and target certificate enumeration with the public-only unknown-index RR9/F3 extractors in #4692, #4822, #4812; evaluate registered 46 positive/18 negative corpus, then same-runner complete index calculus frontier, optimized rho and BSGS.

## Prior-work boundary

- Run59/67 synthetic quotient, run68 real-RHS full rank, run69 genuine target-specific rank: #4901 (these are existing pieces, not new root-finding claims).
- #4809 native quadratic S3 root lane; #4692 RR9/F4 blind gate blocked by checkpoint transport; #4822 sparse RR9/Krylov; #4812 transformed-slot Horner/F3; #4816 small-field nonenumerative control. Do not duplicate their extractor code and do not call generic witness enumeration an unknown-index rootfinder.
- Semaev, ePrint 2004/031; Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009, already discusses a one-relation approach. These references provide prior-art context, not independent validation of this experiment.
- Shared research-KB MCP and local Rust are not mounted in this execution; direct exact-source GitHub files, same-day session log, and published source metadata were reviewed as fallback.

Current status: **PROTOCOL ONLY; no new Rust run, certificate count, timing, or advantage measured yet.**
