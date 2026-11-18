# F4 / Gröbner speed leads, 2026-11-16

**Type.** Component study: five leads run in parallel, integrated, measured by a
serial interleaved A/B, then red-teamed. **Standing.** Every number here is
`[pcc:unverified]`: one 4-vCPU shared cloud VM, isolation L0, no PMU, no pcc
claim filed. **Scope.** Wall-clock and deterministic cost of the F4 engines
(`f4_fp` over F_p, the GF(2) Macaulay builder); not an index-calculus result.
No counted unit changes, so no IC row, S value or gap to rho moves. The IC
default oracle for m = 2 is `DecompositionStrategy::PairTable`, which calls
neither engine.

Owner request (2026-11-16): "pursue all the leads here in parallel workflow",
after the assessment in `docs/session-logs/2026-11-16.md` (PR #4436).

## What landed

| change | default | toggle | evidence |
|:--|:--|:--|:--|
| f4-parallel: column-tiled `B'` build, tiled in-block finish, work-based spread gates, gated round-parallel symbolic preprocessing (`src/cryptanalysis/f4_fp.rs`) | **on** | `F4_FP_PAR_OLD=1` restores the old dispatch | serial A/B below |
| gf2-m4ri: sort-free parity row generation for cold GF(2) Macaulay builds (`src/cryptanalysis/koblitz_groebner.rs`) | **on** | `KIC_M4RI_OLD=1` restores the old builder | callgrind + serial A/B |
| f4-memory: K = 8 fused register-delayed lazy updates at three sites (`f4_fp.rs`) | **off** | opt in with `F4_FP_MEM=1` | its pre-registered endpoint failed; see below |

Every toggle restores the old path exactly. Output was identical to the
baseline in every arm: 57/57 integration comparisons, 1,440/1,440 timed runs,
and an 86-run red-team check at 1, 2, 3, 4, 6 and 8 threads with forced
knobs and fresh seeds. `cargo test --release --lib`: `f4_fp` 97 passed,
`koblitz_groebner` 95 passed (5 ignored), `inherited_f4` 18 passed (2 ignored).

## Headline: shipped defaults, paired confirmation (run-20261117T0022Z)

Pre-registered in `CONFIRM_PROTOCOL.md` before data; native driver
`examples/f4_ab_confirm.rs` under `tools/benchrec.py run`; record
`benchmarks/records/20261117T0022Z-f4-shipped-default-ab.json`; report
`reports/run-20261117T0022Z/REPORT.pdf`. Same binary (commit `b04b25832f`),
default against `F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1`, 12 ABBA/BAAB
blocks, 24 pairs per cell, 384 runs, 0 failed, 0 fingerprint mismatches.
Every pre-registered prediction held [pcc:unverified]:

| endpoint | predicted | median old / new [95% CI] |
|:--|:--|:--|
| **primary:** sum of the 13 `f4_fp_bench` times, 4 threads | [1.08, 1.20], significant | **1.141 [1.098, 1.158]**, significant (342.5 → 300.1 ms) |
| quad_n9, 4 threads | [1.08, 1.25] | 1.161 [1.109, 1.209], significant (172.6 → 148.7 ms) |
| sum of f4 times, 1 thread | [0.97, 1.06], not significant | 1.034 [0.981, 1.086], not significant |
| quart_n3 / cubic_n4, 4 threads | ≤ 1 | **0.811 / 0.865**, both significant: a real small-system regression |
| GF(2) `kernel_x`, 1 / 4 threads | [1.30, 1.60], significant | 1.335 / 1.492, significant |

A/A: sum of f4 times 1.006 (1 thread), 1.004 (4 threads).

## First run: integrated build, f4-memory on (run-20261116T2306Z, `reports/run-20261116T2306Z/`)

This run's driver and analysis were Python (`data/`, `redteam/`), which AGENTS.md
"Implementation language: no Python" does not allow for benchmark drivers; the
Python scripts are kept only as provenance inside `data/provenance-scripts.tar.gz`,
and the confirmation run above replaces it with the native driver.

Serial ABBA run, 8 blocks, 16 trials per arm for `f4_fp_bench 3 large`,
nothing else running, frozen protocol (`PROTOCOL.md`), paired bootstrap of the
ratio of medians. Full tables: `reports/run-20261116T2306Z/ratios_table.md`;
graph: `ratios.svg`; PDF: `REPORT.pdf`.

**Shipped default against the baseline binary** (derived, unpaired; old arm of
the TOTAL pair over the old arm of the MEM pair):

| case | 1 thread | 4 threads |
|:--|--:|--:|
| quad_n9_p65521 | 235.9 → 231.6 ms (1.019) | 168.0 → 145.1 ms (**1.158**) |
| quad_n8_p65521 | 1.101 | 1.192 |
| quad_n6 / quad_n7 | 1.086 / 1.100 | 1.340 / 1.157 |
| cubic_n5_p31 | 1.023 | 1.093 |
| overdet_n10 / overdet_n12 | 1.033 / 1.001 | 1.116 / 1.044 |
| quart_n3_p31 / cubic_n4_p31 | 1.090 / 1.034 | **0.882 / 0.926** (0.74 → 0.84 ms; 1.68 → 1.81 ms) |
| sum of f4 times | 1.015 | **1.136** |

**f4-parallel marginal** (same binary, paired): significant at 4 threads on
quad_n6–n9 (1.115–1.277), overdet_n6 (1.824), the sum (1.132 [1.05, 1.18]);
neutral at 1 thread. Small cases at 2–4 threads slow down (quart_n3 0.835,
cubic_n4 0.83–0.96; CIs exclude 1 at some thread counts but not the A/A spread).

**gf2-m4ri** (same binary, paired): cold Macaulay kernel x 1.49 / 1.40 / 1.44 and
sym 1.42 / 1.39 / 1.43 at 1 / 2 / 4 threads, all significant; callgrind
`macaulay_profile` 71.1 M → 54.4 M Ir (1.31×), the builder 1.79×. The
production stage ladder moves −0.17 % (rung 5) and −0.50 % (rung 2) in Ir,
because it mostly hits the cached column layout.

**f4-memory** (same binary, paired): pre-registered endpoint quad_n9 1.2–1.4×
**not met** (1.062 / 0.974 / 1.024 at 1 / 2 / 4 threads). Unpredicted
significant gains on overdet_n10 / overdet_n12 at 4 threads (1.220 / 1.666),
whose output check is weak (their basis is {1}, the same across fresh seeds;
only the counters carry the check there). Significant regression on quad_n5 at
1 thread (0.797). Hence off by default.

**Thread scaling** is still weak on this host: the baseline's cubic_n5 is
slower at 4 threads than at 1 (19.7 vs 16.4 ms), and so is the default (18.1 vs
16.1 ms). Against each arm at its best thread count (red-team), quad_n7 0.997,
cubic_n5 1.025, quad_n8 1.226, quad_n9 1.191.

## Negative and blocked leads

- **crossbred-m3** (`leads/crossbred-m3/`): pair-table root finding beats every
  algebraic engine at m = 3 on K_0 (40–6800× in wall time; 0 disagreements with
  exhaustive enumeration over n = 5, 7, 9, 13, 15, 31). Crossbred versus
  InheritedF4 is **unresolved**: the native counters price different loops and
  disagree with wall time at n = 31 (3.3× fewer counted ops, 1.44× slower).
  Production crossbred parameters (k = v/2) are far from the tuned ones
  (70–8000× more ops), and the tuning search was not charged. Harness:
  `leads/crossbred-m3/crossbred_m3_bench.rs` (not compiled into the crate).
  [pcc:unverified]
- **isolab** (`leads/isolab/`): a many-core confirmation run is blocked. Every
  Iso Lab launch on 2026-11-16 failed with `AccountLimitExceededException`
  (CodeBuild queue limit 0); this session's AWS keys are rejected by STS;
  ecbench cannot express env-toggle arms. A draft `f4_fp_ab` workflow mode and
  protocol are in `leads/isolab/draft/`; red-team found the draft's arms do not
  fit the integrated revision (the baseline needs `F4_FP_PAR_OLD=1
  F4_FP_MEM_OLD=1` and a cross-binary arm), and a command Job carries no L2/A-A
  admission, so it could not pass the validation gate as drafted.

## Red-team (`redteam/verdicts.json`)

f4-memory and f4-parallel did **not** survive as claimed: missed pre-registered
targets, a post-hoc effect on a weak output check, equal-thread comparisons
against a baseline that itself scales badly, cutoffs tuned on the evaluation
cases (quad_n10/n11 never timed), no msolve comparison (AGENTS.md §15). gf2-m4ri
survives at component scope. What the data do support, on this host only:
integrated quad_n9 1.193 [1.11, 1.26] at 4 threads; f4-parallel marginal
1.12–1.28 on quad_n6–n9 at 2–4 threads; gf2-m4ri cold kernel 1.40–1.49×.
Arithmetic bounds, thread-count independence and toggle fidelity all checked.

## Open next steps

1. Pre-registered confirmation of f4-memory with overdet_n12 at 4 threads as the
   primary endpoint, held-out seeds, a step-matrix hash instead of `basis_fp`,
   and a size gate; only then turn it on.
2. Time quad_n10, quad_n11 and cubic_n6 under f4-parallel's cutoffs (they were
   tuned on the evaluation cases) and add an msolve arm.
3. Fix the small-case cost of f4-parallel at 4 threads (quart_n3, cubic_n4),
   e.g. by keeping steps below a work threshold on one thread.
4. Many-core confirmation once the CodeBuild limit is restored.

## Layout

`PROTOCOL.md` frozen protocol; `data/` runs (`runs.jsonl`, 1,440 rows),
re-analysed rows (`rows.json`), fingerprints, binary hashes, raw logs
(`raw-logs.tar.gz`), and the first run's Python runner and analyses as
provenance only (`provenance-scripts.tar.gz`, written for a scratch layout:
`${SCRATCH}/measure/` is `data/` here); `scripts/` the plotting scripts (layout
of recorded values only) and the integration correctness matrix;
`CONFIRM_PROTOCOL.md` the confirmation run's frozen protocol; `integration/`
merge notes and the correctness matrix; `leads/` each lead's notes and result;
`redteam/` verdicts and the fingerprint re-check; `reports/run-20261116T2306Z/`
and `reports/run-20261117T0022Z/` the run reports, graphs and PDFs.
