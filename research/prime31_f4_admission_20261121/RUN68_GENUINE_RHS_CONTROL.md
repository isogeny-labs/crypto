# Prime31 run68 — genuine labelled relations and a complete generic calibration (NOT an admitted index-calculus win)

**Date:** 2026-11-22. Existing [draft PR #4901](https://github.com/isogeny-labs/crypto/pull/4901). Original Rust source: [prime31_run68_rhs_control.rs](../prime31_ic_20261115/native/examples/prime31_run68_rhs_control.rs). [Dedicated GitHub Actions workflow](../../.github/workflows/prime31-run68-rhs-control.yml).

## New question and experimental design

Run67 measured *synthetic* signed-pair constraint ranks; no right-hand-side-labelled actual EC relations were available, so none of its 1,424 quotient certificates recovered a target log. Run68 isolates that missing integration gate by constructing **genuine** public relations, without planting their right-hand sides:

1. Build the unchanged ordinary, cofactor-one curve over \(\mathbb F_{2147483647}\), with \(a=847514846,\ b=2087444724,\ n=2147446831,\ G=(625037715,2100627251)\). Reuse the original Rust crate \`prime31-algebraic-ic\`, original Dickson depth-10 B509, and the frozen 168-extra-point B677 extension.
2. Recheck each of the previous **88 homogeneous factor-base relations**, using the actual original Rust EC group law. Row 30 contains a **coefficient +2** and must not be misparsed as a sign; the source explicitly accepts \(\pm1,\pm2\) and handles multiplicities.
3. Build all 916,469 distinct **generic** signed-pair group-sum keys; this is an explicit meet-in-the-middle *reference oracle*, not Semaev F3/F4, not blind RR9, and **not an admissible non-generic extraction arm**.
4. Walk public \([k]G\), k=1,2,...; whenever \([k]G=\epsilon_i P_i+\epsilon_jP_j\), collect the authentic **labelled** equation \(\epsilon_i\ell_i+\epsilon_j\ell_j=k\bmod n\). Keep all misses and hits; accumulate affine signed-DSU constraints and then specialize the 88 true homogeneous equations to unanchored roots. Only stop if the projected system has full rank. Reject inconsistent equations.
5. Recover *all 677 factor-base logarithms* and check \([\ell_i]G=P_i\) independently with original Rust arithmetic.
6. Generate 32 held-out targets from private-to-the-harness deterministic scalars. The solving function receives only EC points, uses a generic signed three-sum lookup after public shifts, reconstructs \(\ell_Q\) from the **genuine RHS** and recovered base logs, and independently checks \([\ell_Q]G=Q\). Check the output scalars against harness secrets *after* solving; no secret is passed into the solver.
7. Charge original-Rust B677 setup, table build, **every** relation collection shift, rank/recovery, target shifts, and explicit target checks. Run a same-process original-arithmetic BSGS reference for 32 targets; it is not an optimized negation-map/rho or the strongest IC frontier.

**Admission gate:** This is an end-to-end **generic calibration** designed to expose implementation and measurement errors in the rank/recovery stage before attaching #4822/#4812/#4692. Even if the calibration happens to beat one BSGS implementation, it is **not** a qualifying novel/non-generic prime-field ECDLP win, due to its quadratic generic pair table. Full cold/amortized comparison against best index calculus and optimized rho/BSGS still required.

## Independent control and source review

- The independent Python affine EC replay on frozen \`run66\` B677 and \`run57_basis.json\` confirms **88/88** homogeneous identities **when the coefficient +2 is treated correctly**. An initial sign-only verifier flagged row 30, leading to the necessary correction in the new original-Rust source **before accepting any CI result**. This is a fixture-parsing correction, not an invalid original relation.
- Checked preceding #4901 runs 55–67, #4692 registered selector/RR9 blind gate, #4822 four-variable sparse Krylov, #4812 transformed-slot F3, #4816 working nonenumerative *small-field* control plus negative Prime31 scaling, and #4891 repository F4/GB timing. No competing PR duplicated.
- Relevant primary work: Semaev's summation polynomials (IACR ePrint [2004/031](https://eprint.iacr.org/2004/031)); Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), 168–182, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009), already introduces a one-relation prime-field method. This calibration does **not** claim that method as new. Research-KB MCP was not present in this tool runtime; accessible code/PR records and these primary sources are the fallback.

## Reproduction and evidence status

Command: \`cd research/prime31_ic_20261115/native && cargo run --locked --release --example prime31_run68_rhs_control\`

Actions: [run68 workflow](../../.github/workflows/prime31-run68-rhs-control.yml), Rust 1.100.0. Initial source was fixed to handle the \`+2\` fixture before accepting any run results. **No pass, execution time, completed DLP, or superiority claim should be made until the run68 CI source-specific output is checked.** Limits: 12 million public relation shifts and 1,024 shifts per target; if not complete, record censoring as failure instead of inventing a cost.

Expected stdout fields: \`RUN68_RELATIONS\` (actual shifted queries/hits/independent labelled equations), \`RUN68_COMPLETE\` (32 target verifications, setup/collection/online time), \`RUN68_BSGS\` (matched reference times), and final \`RUN68 PASS\`. A pass means **correctness of genuine labelled generic calibration only**, not S3/RR9 polynomial root extraction.

## Prioritized follow-up experiments

**P0 — non-generic blind root admission:** Reuse #4692/#4812/#4822 original Rust to recover *unknown* indices/signs from public target + factor-base polynomial constraints on registered 46-positive/18-negative corpus. Negative roots, timeouts, solver degree, polynomial operations, and memory must be charged. No access to run68's generic dictionary inside candidate process.

**P1 — actual non-generic RHS → quotient solver:** Replace run68's dictionary oracle with the admitted polynomial extractor, retain authentic \([k]G\) RHS and 88 free rows, and feed *all* returned distinct roots to the rank engine. Check every scalar \([\ell]G=Q\), not just factor-base identities.

**P2 — qualified timing:** Same host, original Rust, same curve and targets, cold 1 / batches 32,128,1024, full setup + online + failures + rank + verification. Compare against the **strongest existing index calculus** first and optimized shared BSGS and negation rho. Require statistically repeatable advantage over the frontier, not an isolated component ratio. Keep PR draft until passed.


## Measured native result — exact Rust source CI PASS ×2

The source-fixed version (\`197de2a7cd01d96c92151e38dca28938456e29c0\`) ran under the original crate using pinned Rust 1.100.0. [First passing Actions run #45055226592](https://github.com/isogeny-labs/crypto/actions/runs/45055226592), job 137308851779; [second source-identical run #45055266777](https://github.com/isogeny-labs/crypto/actions/runs/45055266777), job 137308981036. Both passed and printed **32/32 actual Prime31 target scalar logs checked via original Rust \`[k]G=Q\`**, plus **677/677 original factor-base logs**. The second had only unrelated documentation commits versus the first; the Rust source was unchanged. Exact source output and scope frozen in [RUN68_RESULTS.json](RUN68_RESULTS.json).

| Measure | Passing run 1 | Passing run 2 |
|---|---:|---:|
| Actual public relation shifts | 8,267,474 | 8,267,474 |
| Genuine RHS-labelled signed-pair hits | 3,520 | 3,520 |
| Independent pair rows before 88 free rows | 672 | 672 |
| Original-Rust verified base-point logs | 677/677 | 677/677 |
| Verified 32-target logs | 32/32 | 32/32 |
| Generic pair-table/base setup | 290.163 ms | 287.847 ms |
| Genuine RHS collection + rank + base-log check | 2,353.420 ms | 2,327.579 ms |
| 32-target online generic lookup + verification | 56.177 ms | 55.397 ms |
| **Full pipeline including target generation/control harness** | **2,900.834 ms** | **2,871.313 ms** |
| Same-process original-Rust BSGS setup + 32 online logs | 200.484 ms | 199.930 ms |
| Indicative complete generic control / reference BSGS ratio | **14.47× slower** | **14.36× slower** |

The actual query walk had 8,267,474 points to recover all 677 base logs, versus run57's idealized synthetic-query model (different distribution and stopping implementation). This is **one deterministic cohort and walk** replayed twice, not an independently sampled distribution over targets or walks. The previous 88 homogeneous rows were checked against EC group law and their coefficients, including +2, handled exactly. Pair-table precomputation is charged; no polynomial extraction occurred.

**Conclusion limited to this evidence:** a complete correct *generic control* now genuinely recovers 32 Prime31 logarithms with authentic relation right-hand sides in the original Rust crate, revealing a >14× deficit against the same-process reference BSGS for this single run. This control is **not** novel or non-generic and therefore does not satisfy the desired index-calculus advantage. Same-runner optimized rho and strongest index calculus were not run, and PCC remains unverified. The decisive next integration is to replace the **generic** relation-pair dictionary with a measured public-only nonenumerative RR9/F3 extractor while retaining the now-verified real-RHS rank/recovery stages.
