# Prime31 Run70 — genuine RHS multi-certificate stopping measured in original Rust

**Date:** 2026-11-22 UTC. **Status:** 2 × source-identical original-Rust CI passes, one deterministic 32-target cohort. **Research classification:** mathematically exact target-rank improvement in a **GENERIC negative-control** solver; not yet a cheaper public-only non-generic index calculus algorithm.

## Key new measured result

On the unchanged ordinary cofactor-one curve over \(p=2^{31}-1\), keeping up to four *distinct* publicly group-verified signed three-leaf decompositions for each of the same 32 unknown-index targets reduces the number of **actual authenticated public relation shifts** needed to recover and independently verify all target logs by **52.9783492%** compared with one certificate per target. Going from four to sixteen gives **zero** further reduction on this cohort.

| Fixed certificate width / target | Public kG shifts at first complete rank-visible target set | Actual signed-pair RHS hits | Independent pair rows | Verified target logs |
|---|---:|---:|---:|---:|
| K=1 (exact Run69 replay) | 5,378,231 | 2,272 | 651 | 32/32 |
| K=4 | **2,528,933** | **1,088** | **546** | 32/32 |
| K=16 | 2,528,933 | 1,088 | 546 | 32/32 |

The three arms share **the same true-RHS relation trajectory, factor base, 88 checked free identities and quotient-RREF machinery**. Each K=4/16 target can choose its first coefficient vector that actually belongs to the current relation row space. All 96 arm/target scalars passed original Rust \([x]G=Q\), post-solve secret-harness comparison and independent same-process BSGS. This is not a synthetic-rank model. No factor-base full-rank assumption is used for target stopping.

**Subtle scope:** Multiple public certificates are **alternative readouts only** in this run; their differences have *not* been injected as extra relation rows. That more promising rank-source experiment is separately derived and preregistered in [Run71](RUN71_TARGET_DIFFERENCE_NEXT_EXPERIMENT.md).

## Original Rust, fully charged joint-control timings

| Measured quantity | Passing CI #45066573115 | Passing CI #45066578357 |
|---|---:|---:|
| Original crate \`cargo test --release --lib\` | 41/41 PASS | 41/41 PASS |
| Generic B677 + 916,469-key table setup | 223.840 ms | 202.384 ms |
| Public certificate scan to **16 per target** (3,661,605 attempted leaf probes) | 792.156 ms | 652.955 ms |
| Entire genuine-RHS walk through K=1 completion, with all three quotient arms | 1,215.847 ms | 1,104.987 ms |
| **Whole joint experiment including harness / checks** | **2,382.204 ms** | **2,115.936 ms** |
| Same-process reference BSGS setup + 32 logs | 149.856 ms | 155.195 ms |
| Whole **joint control** / BSGS | 15.90x slower | 13.63x slower |

**Do not misinterpret the last two rows as a cost measurement of a K=4-only solver.** The joint protocol deliberately collects all sixteen certificates, then continues the public relation walk until the K=1 arm also completes, so both certificate and collection costs are larger than a K=4-only implementation needs. K=4 cold-path timing is the highest-priority short experiment; it must count four-certificate enumeration, stop at 2,528,933, and charge setup/rank/recovery against same-host existing IC, BSGS and negation rho.

The two jobs used identical deterministic public targets, not independently sampled cohorts. Their absolute timings differ because the runners differ; the exact K=1/4/16 shift thresholds replicate identically.

## Exact-source evidence

- GitHub PR [#4901](https://github.com/isogeny-labs/crypto/pull/4901) (draft).
- Rust source: [prime31_run70_multi_witness.rs](../prime31_ic_20261115/native/examples/prime31_run70_multi_witness.rs); exact tested blob \`6534dec885c76aad9684c43b3bd39b00da22dccb\`.
- Original included Run69 helper blob \`80a335ca98f3f7345b091a9f792c788f2a2659bc\`; its four leading \`//!\` documentation lines were converted to ordinary \`//\` comments for Rust \`include!\` hygiene only; no executable expressions changed. Historical Run69 separately tested SHA remains in \`RUN69_RESULTS.json\`.
- Frozen preregistration: [RUN70_MULTI_WITNESS_PROTOCOL.md](RUN70_MULTI_WITNESS_PROTOCOL.md); full machine-readable numbers [RUN70_RESULTS.json](RUN70_RESULTS.json).
- Source-specific [CI pass 1 #45066573115](https://github.com/isogeny-labs/crypto/actions/runs/45066573115) job 137343963258; [CI pass 2 #45066578357](https://github.com/isogeny-labs/crypto/actions/runs/45066578357) job 137343972253; source commit \`9af5d90c8cf9f295ca37d2e3945275a3da037377\`.
- An earlier job #45066349920 failed compiler E0753 because original inner \`//!\` documentation comments could not appear inside an \`include!\` expansion; converted exactly four comment lines and reran source-specific CI to two passes. The 41 existing Rust library tests passed even on that initial failed run, but its new example was not accepted.

## Why it matters, and what is *not* demonstrated

Run69 established a real authentic RHS → DSU → quotient → target scalar pipeline, but waited for *one target witness* per Q to become determined. Run70 proves that the first public witness is not an adequate optimization target: selecting among four signed three-leaf certificates halves the required authentic relation walk, with no additional linear-algebra unknowns and with exact group verification.

The witness enumerator and relation source remain the forbidden-for-qualification **GENERIC** 916,469-key pair-sum dictionary. This result is a reusable target-quotient optimization, not proof that RR9/Horner F3 can obtain those certificates at a competitive price. The strongest 2026-11-22 outstanding scientific gate is public-only unknown-index nonenumerative root recovery from #4692 (registered 46-positive/18-negative corpus), #4822 (four-variable RR9/Krylov) and #4812 (14-variable Horner/F3), not more EC-shift microbenchmarking. #4809 separately owns the native quadratic S3 lane; #4816 is the small-field nonenumerative control.

The Amadori–Pintore–Sala one-relation prime-field index-calculus method (Finite Fields and Their Applications 51, 2018, DOI 10.1016/j.ffa.2018.01.009) predates this work, so there is **no blanket novelty claim**. Research-KB MCP and a local Rust compiler were unavailable in this execution; the exact-source GitHub files, prior PRs and published records were the checked fallback.

## Prioritized next falsifiers

1. **K=4-only cold original-Rust reference:** do not enumerate 16 then pretend the K=4 stop is an entire solve. Hold actual source/cohort and compare fully charged K=4 with K=1 and same-process best IC, BSGS and negation rho.
2. **Run71 authentic difference rows:** if one public Q has witnesses \(Q+j_r G=\sum_i c_{r,i}P_i\), then each pair \(\sum_i(c_{r,i}-c_{0,i})L_i=j_r-j_0\bmod n\) is a genuine labelled relation independent of Q's scalar. Check group identities and *incremental sparse modular rank*, not just K×T row counts; quantify the reduction in additional unknown-scalar relation search. Test K=4/16/32, T=1/32/128.
3. **Real algebraic oracle admission:** attach exactly the same target-row interface to nonenumerative public-only RR9/Horner/Semaev rootfinding; require positive/negative blind corpus, all failures/timeouts/memory, 1/32/128/1024 verified logs and fully charged same-host comparison against the strongest existing index calculus and optimized rho/BSGS.

**Result:** a verified **52.98% reduction in authentic relation queries** for 32 target logs under four public certificates, but no validated complete non-generic Prime31 cost advantage. PR remains draft.
