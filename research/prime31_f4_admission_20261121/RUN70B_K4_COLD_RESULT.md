# Prime31 Run70B — cold four-certificate genuine-RHS recovery, original Rust PASS twice

**2026-11-22 UTC** · Existing draft [PR #4901](https://github.com/isogeny-labs/crypto/pull/4901) · **GENERIC reference control, not non-generic index calculus**.

## Measured result

For 32 genuinely unknown-index public points on the unchanged ordinary cofactor-one curve \(E/\mathbb F_{2^{31}-1}\) with \((a,b)=(847514846,2087444724)\), \(G=(625037715,2100627251)\), \(n=2147446831\), the original Rust pipeline recovered **32/32 actual correct discrete logarithms**, independently verified by \([x]G=Q\), post-solve harness comparison and separate BSGS.

This is a *cold K=4-only* calibration, unlike prior Run70's joint K=1/4/16 experiment: the code really enumerates **only four** publicly verified, distinct signed three-point certificates per target and stops its actual RHS-bearing public \([k]G\) relation walk at **2,528,933** shifts. It uses **933,657 attempted public leaf probes**, **1,088** genuine labelled signed-pair RHS matches, **546** independent pair relations, the previously group-verified 88 homogeneous rows, and exact target-specific quotient rank. The K4 relation query count is **52.9783% lower** than Run69's same-cohort K1 5,378,231 shifts; the original Rust K=4 replay asserted and passed every registered count.

| Actual cold K4 measurement | [GitHub Actions #45066979236](https://github.com/isogeny-labs/crypto/actions/runs/45066979236) | [GitHub Actions #45066973351](https://github.com/isogeny-labs/crypto/actions/runs/45066973351) |
|---|---:|---:|
| Original Rust 1.100.0 release library tests | 41/41 PASS | 41/41 PASS |
| Generic B677 + 916,469-key table setup | 218.321 ms | 348.384 ms |
| Four public witness scans per target | 180.211 ms | 295.405 ms |
| Actual 2,528,933 public shifts, rank, verification | 536.272 ms | 828.398 ms |
| **Fully charged K4 pipeline + harness** | **1,086.016 ms** | **1,708.381 ms** |
| Same-process original-Rust 32-target BSGS, including setup | **150.769 ms** | **235.548 ms** |
| **Full cold generic K4 / BSGS** | **7.20× slower** | **7.25× slower** |

Both GitHub Actions executions succeeded with exact same Rust source commit \`083dc8caf2b673fdbe4a4c59212990ca41143280\`. Original Rust K4 source blob \`94a83b21ef32593bc76a67ef15bf831b010d5a42\`. The run uses one frozen public cohort and relation stream on different runners, **not independently sampled targets**. Only same-process BSGS comparisons are cost ratios; no pairwise cross-run timing claim is made.

## Why this is progress but not the requested victory

The K4 whole-pipeline measurement removes the earlier ambiguity about Run70's joint 16-witness/K1-long-walk cost. The target-specific rank algorithm can now indeed save 52.98% of actual required labelled relation queries while producing all 32 correct logs. This improvement is still attached to the **generic 916,469-key signed pair-sum dictionary**, both for relation collection and target certificate discovery. Generic meet-in-the-middle enumeration is prohibited by the owner's novel non-generic/index-calculus requirement. It is **not** a qualifying blind algebraic extractor and, even as a control, its complete cost is 7.2x slower than this particular unoptimized BSGS baseline.

There is a measurable tradeoff: Run69 K1 5.38M public shifts and 1 witness/Q were already 10.2x slower than its same-process BSGS; Run70B scans four witnesses/Q, cutting collection by 53%, but remains 7.2x slower. Those two whole-solver ratios were recorded in separate CI workflows/runners, so they must not be compared as a controlled same-host speedup.

## Rust reproduction and chain of evidence

- [Frozen protocol](RUN70B_K4_COLD_PROTOCOL.md) committed before code; [Run70B original Rust example](../prime31_ic_20261115/native/examples/prime31_run70b_k4_cold.rs) calls the existing included Run69 original Rust arithmetic/quotient code; [frozen machine-readable results](RUN70B_RESULTS.json); original Rust 1.100.0 workflow [prime31-run70b-k4-cold.yml](../../.github/workflows/prime31-run70b-k4-cold.yml).
- Original prior [Run69](RUN69_AUTHENTIC_TARGET_QUOTIENT_GATE.md) supplied genuine RHS quotient functionality and K1 same-cohort control; twice-passing [Run70](RUN70_MULTI_WITNESS_RESULT.md) supplied K1/4/16 exact rank thresholds.
- One independent original-Rust BSGS reference; original crate \`cargo test --locked --release --lib\` 41/41 PASS on each run; 32/32 original \`[x]G=Q\`; post-solve secret checks. Independent signed PCC is not yet available.

## Next independent, high-priority work

The now-derived [Run71 target-certificate difference-row theorem and experiment](RUN71_TARGET_DIFFERENCE_NEXT_EXPERIMENT.md) supplies actual RHS-bearing relations *without knowing a target scalar* by subtracting different public decompositions of the same Q. Implement and **measure real incremental rank**, RHS and reduced collection cost, not idealized row count. A K=4, T=32 panel offers at most 96 such difference rows; K=16 offers at most 480. Neither is guaranteed independent. This could turn previously expensive extra target decompositions into shared rank gains.

For a **qualifying** non-generic solver, replace the generic pair/triple dictionary with existing public-only unknown-index polynomial extraction in [#4692](https://github.com/isogeny-labs/crypto/pull/4692) (registered 46+18 positive/negative corpus), [#4822](https://github.com/isogeny-labs/crypto/pull/4822) (RR9/Krylov) and [#4812](https://github.com/isogeny-labs/crypto/pull/4812) (Horner/F3). Charge negative attempts, complete authentic RHS rank, setup, online target recovery and memory, then compare cold 1, 32, 128 and 1024 verified logs against best existing IC, optimized negation-rho and shared-BSGS **on the same host**. Prior one-relation work by Amadori–Pintore–Sala (2018), DOI 10.1016/j.ffa.2018.01.009, precludes a blanket claim of novelty.

**Admission status:** original Rust correctness and fully charged generic reference cost independently replayed twice; genuine nonenumerative root extraction and complete cheaper index-calculus solver **still not established**. Keep PR draft, [pcc:unverified].


## Cost-accounting erratum (Run71 source audit, 2026-11-22)

**The previously published 7.20×/7.25× comparisons above are conservative but mislabelled as cold K4 solver-only ratios.** The source `prime31_run70b_k4_cold.rs` computes its `whole.elapsed()` **after** executing the separate BSGS reference. Thus `whole_pipeline_ms` includes the entire BSGS control on the *solver* side of its ratio, despite dividing by that same BSGS cost. This is a reporting defect, not a cryptanalytic speedup.

Without altering the frozen original timing receipts, subtract the separately measured in-process BSGS setup+online times from the recorded whole duration:

| CI | Frozen reported whole including BSGS | Reference BSGS | Corrected source-structure solver-only estimate | Corrected solver / BSGS |
|---|---:|---:|---:|---:|
| #45066979236 | 1,086.016 ms | 150.769 ms | **935.247 ms** | **6.203× slower** |
| #45066973351 | 1,708.381 ms | 235.548 ms | **1,472.833 ms** | **6.253× slower** |

The *corrected* figures are **derived by subtraction**, not directly timed isolated cold stop boundaries. The individually timed setup + witness + relation stages provide a corroborating lower bound of 934.804 ms and 1,472.187 ms, respectively. The generic solver therefore remains markedly slower than matched BSGS; the prior 7.2× number overstates the gap by approximately one BSGS execution.

Run71 now logs a truly isolated `RUN71_COLD_DIFFERENCE` timestamp **before any reference BSGS or readout-only baseline RREF**. Use that source-specific measurement instead of repeatedly estimating cold costs from joint experiments. Do not rewrite the earlier frozen JSON; retain it as an immutable record with this erratum.
