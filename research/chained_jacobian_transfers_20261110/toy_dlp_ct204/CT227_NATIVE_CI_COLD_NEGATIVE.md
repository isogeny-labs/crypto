# CT227 — two hosted native composite-J cold-cost replays: audited negative

2026-11-19 UTC · [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) · **[pcc:unverified]**

## Decision and measurements

The pending CT226 native Rust test **passed** in two distinct GitHub Actions pushes on 2026-11-19 UTC. Both runs executed the real native genus-4 Cantor composition / signed offset relations / modular rank 54 / source-point target recovery on the finite F101 known-answer example, with three alternating-order cold pairs against direct inline elliptic BSGS each.

| Quantity | Six hosted observations |
|---|---:|
| Median native transferred index-calculus elapsed | **44,901,277.5 ns** (44.901 ms) |
| Median native direct BSGS elapsed | **1,733 ns** (1.733 microseconds) |
| Median **paired** direct / transfer ratio | **0.0000385453** |
| Median **paired** transfer / direct slowdown | **26,020.0×** |
| Range of paired transfer / direct slowdown | **21,359.6–28,456.7×** |
| Mean fraction of transferred time spent in relation phase | **98.9188%** |
| Median transferred non-relation time, with measured relations hypothetically free | **491,294.5 ns** |
| Median paired non-relation / BSGS ratio (empirical diagnostic) | **274.37×** |

The medians of separate arms and the median of paired ratios are **different estimators**; never divide the two medians and call that the median paired ratio. The last two rows are an **Amdahl-style diagnostic for this frozen implementation**, not a rigorous implementation-independent lower bound. Optimizing relations alone cannot make the *measured, unchanged other phases* competitive. Do not extrapolate this to large p, a different factor base or a better native solver.

Both hosted runs report the same 1,421 candidates, 62 fully split relations, rank 54/54, 14 offsets, 1,435 Cantor additions, 4 target additions, and 14 inline BSGS additions. Their field-operation counters agree: 1,273,478 additions, 956,007 multiplications, 17,786 squares and 24,532 inversions. All six records have exact integer phase-sum equality and ratio rounding consistency. These are producer-instrumented counts, not calibrated standalone primitive wall prices.

## Verified source and reproducibility

- [First successful run #44467973031](https://github.com/isogeny-labs/crypto/actions/runs/44467973031) — exact source `95d374fe1e31214b5eff586866404a1a3dc53216`, [native job 135370252212](https://github.com/isogeny-labs/crypto/actions/runs/44467973031/job/135370252212).
- [Second successful run #44468520835](https://github.com/isogeny-labs/crypto/actions/runs/44468520835) — exact source `9b203b4bb12922dbfa56b9d5eb14b543a990c5e3`, [native job 135372027935](https://github.com/isogeny-labs/crypto/actions/runs/44468520835/job/135372027935).
- [Frozen six records](ct227_native_ci_pairs.json) were transcribed **from the corresponding job logs**, not synthesized from expected values. Each record includes the run ID, native job ID, exact head, phase timings, rank, counters and original decimal ratio.
- [Offline auditing program](ct227_native_ci_cost_audit.py): validates two distinct expected workflow SHAs, six unique alternating rounds, all raw constants, exact cost summation, published-ratio tolerance, baseline dominance and six synthetic mutation controls.

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct227_native_ci_cost_audit.py --self-test
# To reproduce the native toy solve, rather than only audit the GitHub log:
cargo test --locked --release --test ct226_native_genus4_transferred_ic -- --nocapture
```

Local Python normal-mode self-test of the auditor against the six frozen rows succeeded with `CT227_SELFTEST_PASS 6 6`. This is a **new audit of producer CI logs**, not an independent algebraic implementation, signed PCC, blind-target challenge or independent remeasurement. A separate scoped cheap Python CI workflow has been added; its exact-head hosted verdict must be checked after publication.

## Caveats and next action

- The tiny direct-BSGS times (approximately 1.6–2.1 microseconds), unisolated GitHub machines and only six observations make precision estimates and confidence intervals inappropriate. These are fixed-target same-job diagnostics, with two distinct Ubuntu runner images, **not** a matched isolated performance qualification.
- The code contains public known-answer assertions for source point Q and scalar 37. They are validation of the fixed demonstration, not evidence of blind ECDLP recovery on arbitrary targets. No unproven source-generator logarithm anchor is used in the linear solve.
- Actual peak memory, general source-target divisor norm/projection cost, optimizer-grade rho and frontier index-calculus references, all original-model 695/701 records, and calibrated `C169/C13a/C13b/D` prices remain absent. The authenticated original fixture is documented blocked by S3 403 and EKS paid STOP. No substituted fixture is represented as original.
- Given the **negative complete toy benchmark**, do not spend the next iteration squeezing only its relation walk. The next authorized measurable lead is a *distinct, larger* composite-order source-subgroup index-calculus panel using existing native framework and strongest same-host refs, or real original-model norm-reuse replay when authorized. Enforce setup/transfer/relation/LA/recovery, memory, failed attempts and cold baseline accounting. Compare against best optimized rho/BSGS/IC, not only BSGS. Freeze a protocol before data. No paid compute or protected merge.

**Outcome:** CT226 moved from native-pending to native-executed and quantitatively **negative**. No complete single-target prime-field ECDLP advantage or independent PCC is established.
