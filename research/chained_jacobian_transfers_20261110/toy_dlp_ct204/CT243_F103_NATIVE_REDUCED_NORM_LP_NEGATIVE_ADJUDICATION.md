# CT243 — F103 reduced norm/2LP cold single-target negative result (2026-11-19)

**Standing:** Original native Rust benchmark completed and archived; independent checks of the 15 structured records passed, but a distinct native solver replay, signed PCC, matched optimized rho/frontier IC and calibrated attributable peak memory do not exist. **[pcc:unverified]**.

## Provenance and exact reproducibility
- Frozen **preregistered** experiment: [CT241 native protocol](CT241_F103_NATIVE_REDUCED_NORM_LP_PROTOCOL.md) (published before the native source or measurement); full preflight and test via `cargo test --locked --release --test ct238_native_cubic_norm_compression ct241_paired_native -- --nocapture --test-threads=1` at source SHA `e52e8827e7cc54ce9ef67eb488526d858d36fe9f`.
- Actual hosted [CT241 GitHub Actions run #44592813955](https://github.com/isogeny-labs/crypto/actions/runs/44592813955), job **135803058003**, completed success on 2026-11-19; release Rust built and one native test passed.
- Original job log contains **15 authentic structured measurements plus 15 byte-identical `grep` replay echoes** (plus two shell-command references). Only the first 15 genuine observations are preserved in [ct241_f103_native_15_pairs.jsonl](ct241_f103_native_15_pairs.jsonl), initial raw-data freeze commit `67e725db759fd962920cc9186c334223aebebe44`. These observations were not recomputed or simulated.
- Reproducible record check: `cd research/chained_jacobian_transfers_20261110/toy_dlp_ct204 && python3 ct241_f103_native_cost_audit.py && python3 -O ct241_f103_native_cost_audit.py`. Auditor self-tests include 16 synthetic corrupt/missing/duplicate mutations. Its success after the file freeze establishes *record consistency*, not an independent Rust solver replay.
- A separate JavaScript extraction independently checked all 15 phase sums, signed LP ranks, fixed finite counts, wall ratios and exact equality of each original to its echo before publication.
- **Hosted exact-source record-audit CI passed:** [GitHub Actions #44602035872](https://github.com/isogeny-labs/crypto/actions/runs/44602035872), job **135835706727**, on frozen-record commit `67e725db759fd962920cc9186c334223aebebe44`. Decoded original log shows both ordinary Python and `-O` processed **15 real rows**, each reported `CT241_RECORD_ONLY_AUDIT_PASS`, and both rejected all **16 synthetic mutations**. This is independent record adjudication, not a second timed native solve or signed PCC.

## Five-arm native cold Q73 result

The source group is `E: y²=x³+x+24` over `F103` with order 109, generator `G=(1,51)` and public `Q=(42,71)=[73]G`. A single new cold target is solved in each of three alternated same-host rounds per base. The complete candidate time includes setup, online LP relation collection/elimination, backsolve, source recovery and verification. VmHWM is whole-process, not candidate-attributed.

| Base columns B | Excluded LP logs fully recovered | Genuine relation candidates | Median candidate | Median paired direct BSGS | Median paired BSGS/candidate | Logical matrix bytes | Median process-wide VmHWM |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 12 | 10 | 589 | 24.670 ms | 2.173 µs | 0.0000868134 | 3,088 | 3,292 KiB |
| 16 | 6 | 381 | 16.481 ms | 2.073 µs | 0.0001269934 | 3,280 | 3,304 KiB |
| 18 | 4 | 381 | 16.246 ms | 2.073 µs | 0.0001275977 | 3,472 | 3,312 KiB |
| 20 | 2 | 381 | 16.291 ms | 2.144 µs | 0.0001316045 | 3,728 | 3,324 KiB |
| 22 | 0 | 381 | 16.300 ms | 2.093 µs | 0.0001284030 | 4,048 | 3,336 KiB |

The median `BSGS/candidate` above is **the median of the three paired per-round ratios**, not the ratio of median times. The B12 factor-base matrix is smaller but ten LP pivots are solved too: **22 source norm logarithms total in all five arms**. B12 uses 589/381 = **1.546×** the relation candidates of B22 and its median cold time is **1.513×** B22. B18 is nominally 0.33% faster in median elapsed time than B22 and B20 0.055%, but these unblinded, in-process three-sample differences are too small to establish a reproducible improvement. At the same size all five are roughly 7,598–11,519× slower than paired toy BSGS.

**Bottleneck:** The `relation_online_la_ns` phase dominates: B18 median 14.819 ms out of 16.246 ms; even subtracting this entire phase leaves approximately 1.43 ms—hundreds of times the microsecond-scale direct source baseline. This is a conditional implementation-specific observation; it is not a lower bound on other algorithms.

## Scope, controls and mathematical interpretation
- Correctness checks: each run achieved full base rank, recovered every excluded norm LP and the public source scalar 73; no fabricated successful samples. The earlier CT242 independent Python screen also recovered *non-inverse-three* target logs 37 and 51 under reduced and full bases, but those are **not the timed Rust targets**.
- This test only compares to direct elliptic BSGS; matched optimized rho, existing IC frontier, independently repeated native timing and signed PCC are absent. The candidate includes cold target work and verified arithmetic, but the recorded memory is only process-high-water and matrix bookkeeping; do not claim fully calibrated memory economics.
- The original 695/701-record Jacobian norm-point reuse fixture, independently authenticated replay and *physical* `C169`, `C13a`, `C13b`, `N2_actual` and `D` operation prices were **not** produced by this small F103 toy. Crypto #4215 provides prospective counter algebra `C169 - 2*C13`, not a native measured cost saving. Do not merge these two experimental populations.
- Double-large-prime techniques are established: Gaudry–Thomé–Thériault–Diem, *Math. Comp.* 76 (2007), https://doi.org/10.1090/S0025-5718-06-01900-4; and Diem, https://arxiv.org/abs/math/0606607. Neither paper establishes an advantage on this transferred source ECDLP fixture.

## Research decision
**Stop optimizing the unchanged F103 norm-fiber collector**. The genuine reduced-base and two-LP experiment did not cross even the same-curve tiny BSGS cost floor, and splitting the same 22 unknowns across base/LP pivots did not reduce total solved logarithms.

Next high-value work is **original authenticated 695/701 norm-reuse custody + physical primitive pricing** (only if already authorized and source/budget gates pass), or an algebraically different cover/collector with a prospectively registered complete single-target cost model and matched optimized rho/BSGS/frontier IC. Preserve failed strategies, no EKS Spot charge or ledger bypass, no qualified speedup or PCC claims.
