# Prime31 two-leaf IC yield bound and non-generic experiment gates (2026-11-17)

**Classification:** exact elementary counting lemma and a falsifiable experiment queue; **not** a new speed measurement, not a complete non-generic ECDLP solver. [pcc:unverified]

## Fixed curve and prior full-cost frontier

Keep the existing ordinary cofactor-one curve over F_(2^31-1): p=2147483647, y²=x³+847514846x+2087444724, n=2147446831 prime, G=(625037715,2100627251), and the exact 1024 public targets from merged [crypto #4532](https://github.com/isogeny-labs/crypto/pull/4532).

Producer historical fully charged 1024-Q figures on one runner: shared BSGS m=1048576, **170.113 ms**; complete mixed-base IC with 61 basis logs, **2221.474 ms** (ratio **13.0588×**). This is historical comparison, not a new measurement of #4596. The new candidate in [#4596](https://github.com/isogeny-labs/crypto/pull/4596) uses the *existing* Rust prime_fast::ic_multi_solve_ex with table=true, batched=true, B=1920, extra=32, max-walk=500000 and seed=20261117. It has no non-generic relation oracle, but is a legitimate index-calculus pipeline.

## Exact generic signed-two-leaf ceiling

Fix *any* B distinct source-curve points P_1,...,P_B on an elliptic curve of odd prime order n. A signed two-leaf lookup table contains only group points of the form ±P_i±P_j, including i=j, and possibly O. The number of **different** group points is bounded by

    |S₂| <= 4*C(B,2) + 2*B + 1 = 2*B² + 1.

Proof: every unordered i<j contributes at most four signed sums; equal-index pairs contribute ±[2]P_i and O; group collisions can only reduce the image. This statement is independent of how the factor base was selected and does not assume a random curve.

At B=1920, |S₂| <= **7,372,801**, so a *uniform independent random residual* R in the actual group has

    Pr[R∈S₂] <= 7372801/2147446831 ≈ 0.00343328687,
    E[uniform independent queries before a hit] >= 291.2661.

Even granting one independent useful relation for every hit, reaching at least 1920 independent rows needs **at least 559,231 queries in expectation** under this *uniform independent residual sampling model*. This is an optimistic cost screen, not a universal complexity lower bound: correlated walks, target-conditioned sampling, exceptional group coincidences and algebraic non-uniform oracles fall outside the iid assumption; the realized collector may generate several useful rows per one residual, so the 559,231 row argument specifically assumes at most one counted independent row per query. The probability ceiling itself still holds for a single uniform independent residual. In particular, do not misuse the heuristic to reject a different, genuinely algebraic oracle.

This explains why microbenchmarking the pair hash lookup alone cannot establish an index-calculus advantage. Include table construction, useful-rank growth, duplicate relations, all unsuccessful walk steps, modular matrix solve, all 1024 descents, their verification and RSS. The actual Rust collector may exploit multiple signed hits; log observed hits/independent rank explicitly before drawing a sampling conclusion.

## Prior leads read / no-duplication decisions

| Source and direction | Established disposition | Reopen only with |
| --- | --- | --- |
| [#4532](https://github.com/isogeny-labs/crypto/pull/4532), shared 1024-Q BSGS vs mixed IC | Complete 1024/1024 for both; generic BSGS 13.06× cheaper | New same-run fully charged solver source or cost-reducing mechanism |
| [#4572](https://github.com/isogeny-labs/crypto/pull/4572), batched pair-table IC | Nearby 32-bit generated-curve complete one-target 0.403 s vs rho 0.0093 s; not the actual prime31/1024-Q cohort | #4596 actual curve, exact corpus, cached BSGS |
| [#4575](https://github.com/isogeny-labs/crypto/pull/4575), B8 4+4 vs RR9 | Generic 4+4 wins against enumerative signs | Algebraic extraction not choosing/enumerating signs |
| [#4581](https://github.com/isogeny-labs/crypto/pull/4581), seven-of-eight quotient | Split 128 signed-linear branches; no sign-free shortcut | Full eight-equation quotient elimination with **public R only** |
| [#4550](https://github.com/isogeny-labs/crypto/pull/4550), B8 D9 membership | Exact algebraic criterion, positive witness test; negative D9 ranks unmeasured | Replayable negative unit certificates and a true public-only positive extractor |
| [#4541](https://github.com/isogeny-labs/crypto/pull/4541), squarefree RR9 | Radical divisibility; squarefree-only support insufficient on prime31 | Repeated roots, exceptional x(R), unknown-Q recovery |
| [autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301), 39-lead atlas; [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), Semaev-alternative corrections | Prior nulls for Vieta, lattice, RR variants, sign enumeration, alternative presentations; none certified a complete prime-field advantage | A changed algebraic mechanism with independent cost and rank gate |
| [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) | Auxiliary/random-base comparisons already investigated | Source-target matched curve and generator with demonstrated cost advantage |

Research-KB tools and repository wrapper were not available in this connector runtime, so the above is a bounded audit of original repository PRs/files and the actual SA campaign SUMMARY/ERRATA; absence of a hit does **not** establish novelty. Prime-field prior art includes Petit–Kosters–Messeng (PKC 2016; DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala (FFA 2018; DOI 10.1016/j.ffa.2018.01.009), and Kudo et al. (CANS 2018; DOI 10.1007/978-3-030-00434-7_19). The APS paper's one-factor-base-relation variation is not a new lead by itself.

## Next distinct experiments (priority; none reported completed by this document)

1. **Finish current reproducible Rust batch frontier (#4596).** Same 1024 public Q, 3 BSGS arm sizes (46341, 262144, 1048576), include outer total wall and per-target failures and peak RSS. Count complete only when 1024/1024 scalar and point replays pass on original curve. Record rank and collector behavior; any failure is retained, not retuned.
2. **Public-only B8 RR9 full-eight sparse quotient/D9 gate.** Reuse existing Rust f4_fp_tower/quotient/rootfinding source. Fix three pre-registered residuals (one positive and two non-support negatives). No signed leaves, guessed masks, supplied coefficients or generic point law as input to candidate. Require positive coefficients *and* two independently replayable negative unit certificates; compare to generic 4+4 on same runner and charge all rank and witness work. The earlier positive-witness-only 6.846 ms is test oracle data, not a public algorithm.
3. **B31 all-multiplicity RR9 gate.** Replace squarefree M=H_B with radical divisibility and explicitly cover repeated roots and x_i=x(R). Build a blinded natural-positive/negative panel with same Fp field and prime-order group; report recovered polynomial, orientation and signed support independently rechecked with source group law. Require an algebraic *nonenumerative* coefficient mechanism, not one of the `C(31,8)*2^8` support loops.
4. **Complete non-generic ECDLP gate.** Reuse Rust relation matrix and target descent from the existing prime31 pipeline. Reach full factor-base log rank and recover each hidden scalar from public Q only, with deterministic seed and all failures/walks included. Match strongest cached BSGS, tuned negation-map rho, and a mature external IC reference on equal targets/host, including cold setup and 1/64/1024 target amortization. No leaderboard row until independent signed PCC replay.
5. **Hybrid x/y only after structural gate.** The SA campaign's Edwards-y and Vieta variants already have strong negative controls. Revisit a new y-coordinate factor-base polynomial *only* if it cuts the actual quotient degree, yields a correctly saturated solution set on positive/negative inputs, and beats the existing generic membership probe per useful relation. Merely changing coordinates is not a new solver.

**Decision threshold:** A new prime-field non-generic win requires actual complete logs and total matched cost below *both* the strongest generic comparator and current mature IC baseline; asymptotic-security claims additionally require a non-generic complexity argument, scaling data and independent validation. Until then, report producer diagnostics, not a breakthrough.
