# Frozen performance-gains dashboard audit — 2026-10-21

**Scope:** the [frozen performance page](../../../../docs/performance-gains.html),
[data](../../../../docs/performance-gains/data.json), and
[existing chart](../../../../docs/performance-gains/summary.svg). This is a
read-only audit of the 2026-09-14 measurements, not a new benchmark or a
current leaderboard validation. The [native checker](../../../../examples/performance_gains_audit.rs)
reads committed Git blobs and changes no source record.

| Requested aspect in this scope | Status and evidence |
|:--|:--|
| Legitimate frozen inputs | **Verified for this page.** Four `sourceHashes` match exact current Git blobs; both comparison records say `complete: true` with zero failures (342 ablation runs, 140 confirmation runs). |
| Displayed data | **Verified for this page.** All 804 displayed rows equal the frozen source arrays; the HTML's embedded JSON equals `data.json`. The checker independently recomputes all 766 pair ratios and 32 input-cluster geometric point ratios. All 32 reported intervals are identical to the frozen comparison JSON and include their point estimates. |
| Arithmetic panel | **Verified for this page.** The p=67, seed=11, repetition=0 reference and Horner output files match the hashes in `raw.jsonl`; their `specialization_fp_muls / (targets × evaluation_repetitions)` values are 1,342 and 1,199. The existing chart's 143-product (10.7%) difference is consistent with those counts. |
| Graph and dashboard freshness | **Historical page checked; no new chart data.** The SVG, PNG and PDF exports were checked and left unchanged because no new run or correction changed this page's source. The chart labels local specialization separately from full-DLP confirmation and states that full-DLP improvement remains unproven. Current site publication and the separate IC leaderboard require their own checks. |
| Lean coverage of all historical experiments | **Open beyond this audit.** This page contains empirical timing and exact-output checks; this check does not formalize its implementation or all other experiments. |

The [checker output](audit.json) records the exact counts. A scratch copy of
`data.json` with one local ratio changed was rejected with `FAIL: dashboard
local differs from frozen source`; no committed input was edited. The four
source hashes in the page are:

| Frozen source | SHA-256 |
|:--|:--|
| `research/summation_polynomials_20260914/run-01/comparison.json` | `721e88a01c78cdcfd20ad942bafcfb3560b57a0f1fe05e9e928d29cb9befae83` |
| `research/summation_polynomials_20260914/confirmation-01/comparison.json` | `8dfc552eaa8f0fb49208687a4968fa11d0b2ad99eb0f09a3e61c357be6563077` |
| `research/gaudry_allocation_20260914/run-01/comparison.json` | `7f4ab8e26eedca1670317c54e9747858029181969a0497af5def682d233b42b5` |
| `gpu/macaulay/benchmarks/pipeline-20260914/run-01/summary.json` | `9e1a123796334918db83da649bb1d9cae898ee3ce384cdc78a8a0f8a5fa5b009` |

The required `cargo test --release --lib` passed on the final source:
**4,474 passed, zero failed, 119 ignored**. The first parallel run on the
pre-fix source had one timing-profile assertion count two calls instead of
one; its focused run passed. This PR confines the opt-in support-local
profiling switch to the requesting thread and adds a cross-thread control.
The full parallel suite then passed. This changes profiling isolation for
the documented single-threaded solve; it does not change solver arithmetic
or the frozen performance measurements.

## Reproduction and limits

```sh
cargo run --release --example performance_gains_audit --quiet
cargo test --release --lib
```

The checker reads `HEAD:<source path>`, so a later source change makes the
unchanged page fail rather than silently read sparse-checkout omissions. Its
ratio and source checks are native Rust. The confidence intervals are copied
exactly from the frozen comparison JSON; the checker tests their bounds but
does not rerun the September benchmark or independently regenerate its
bootstrap distribution. The interactive JavaScript test was unavailable on
this host because Node was absent. This audit does not establish an
end-to-end cryptanalytic speedup, a current public deployment, or a clean
repository-wide validation gate.

The `perf history` pull-request check now runs the native checker when this
page, its frozen source records, the checker, or relevant Rust code changes.

The existing [SVG chart](../../../../docs/performance-gains/summary.svg),
[PNG](../../../../docs/performance-gains/summary.png),
[chart PDF](../../../../docs/performance-gains/summary.pdf), and
[page data](../../../../docs/performance-gains/data.json) were checked and
left unchanged. [This audit PDF](report.pdf) embeds the existing chart. No
canonical graph changed because the source measurements and conclusions for
this historical page did not change.
