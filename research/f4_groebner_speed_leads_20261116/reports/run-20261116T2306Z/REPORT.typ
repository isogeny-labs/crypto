#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= Run report: F4 / Gröbner speed leads, serial A/B (run-20261116T2306Z)

*Standing:* every number is `[pcc:unverified]` — one shared 4-vCPU cloud VM, isolation L0, no PMU, no pcc claim filed.
*Scope:* wall-clock and deterministic cost of the F4 engines (`f4_fp` over $FF_p$, the GF(2) Macaulay builder). No counted
unit changed, so no index-calculus S value or gap to rho moves.

== What ran

- *Commit measured:* `929c01c601` (integration of f4-parallel, f4-memory, gf2-m4ri) against baseline `a53cc49711`.
  Binary SHA-256 in `data/binaries.sha256`. The landed default (f4-memory off) equals the measured `MEM` old arm.
- *Host:* Intel Xeon \@ 2.10 GHz, 4 vCPU KVM, AVX-512F, 15 GB; load 0.35 at start; only the runner active. Isolation L0 (no
  pinning, no L2 admission, SMT topology unknown).
- *Protocol:* frozen before data (`PROTOCOL.md`, 2026-11-16 23:06 UTC). Eight ABBA/BAAB blocks interleaved across pairs and
  thread counts. Paired bootstrap of the ratio of medians (2,000 resamples). A row is significant when its 95% CI excludes 1
  and |ln ratio| exceeds that case's A/A spread at the same thread count.
- *Workloads:* `f4_fp_bench 3 large` (13 fixed-seed $FF_p$ systems; each case already a median of 3);
  `koblitz_f4_kernel_bench 5 x|sym` (degree-3 Macaulay/RREF, Koblitz $n = 31$ dimension-16 cell);
  `crossbred_bench 9 11 13`.
- *Pairs:* AA (noise), CROSS (all toggles vs baseline binary, expected 1), PAR (`F4_FP_PAR_OLD=1` as old), MEM
  (`F4_FP_MEM_OLD=1` as old), ALL (all toggles as old, same binary), TOTAL (baseline binary as old); M4RI
  (`KIC_M4RI_OLD=1` as old) for the GF(2) benches; RAYON_NUM_THREADS 1, 2, 4 (crossbred 1 and 4, decided up front).
- *Runs:* 1,440 requested, 1,440 completed, 0 failed (`data/runs.jsonl`, raw logs `data/raw-logs.tar.gz`). Every run's
  fingerprints matched the baseline (`data/fingerprints.json`; red-team re-check `redteam/fpcheck-summary.json`).

== Results

#figure(
  image("ratios.svg", width: 100%),
  caption: [Median old / median new per case (>1: new faster) with paired-bootstrap 95% CI, 16 trials per arm for
  `f4_fp_bench`; filled = significant under the frozen rule. Values from `data/rows.json`, plotted by
  `scripts/plot_ratios.py`; run-20261116T2306Z.],
)

#table(
  columns: (auto, auto, auto, auto),
  align: (left, right, right, right),
  table.header([*measured, same binary unless noted*], [*1 thread*], [*2 threads*], [*4 threads*]),
  [TOTAL quad_n9 (integrated vs baseline binary)], [1.119 [1.07, 1.15]], [1.195 [1.06, 1.29]], [1.193 [1.11, 1.26]],
  [TOTAL sum of f4 times], [1.077 [1.03, 1.11]], [1.220 [1.14, 1.29]], [1.266 [1.19, 1.33]],
  [PAR sum of f4 times (f4-parallel marginal)], [1.062 [1.00, 1.13]], [1.052 [1.02, 1.13]], [1.132 [1.05, 1.18]],
  [PAR quad_n9], [1.042 [0.99, 1.11]], [1.047 [0.99, 1.15]], [1.115 [1.02, 1.21]],
  [PAR quart_n3 (small case)], [1.017 [0.91, 1.16]], [0.857 [0.67, 0.95]], [0.835 [0.73, 1.00]],
  [MEM quad_n9 (f4-memory marginal; predicted 1.2–1.4)], [1.062 [0.97, 1.12]], [0.974], [1.024 [0.98, 1.10]],
  [MEM overdet_n12 (not predicted)], [0.979], [1.358], [1.666 [1.60, 1.76]],
  [MEM quad_n5 (small case)], [0.797 [0.70, 0.98]], [—], [0.909],
  [M4RI kernel_x (gf2-m4ri marginal)], [1.488 [1.17, 1.99]], [1.403], [1.440],
  [M4RI kernel_sym], [1.42], [1.39], [1.43],
)

*Shipped default vs baseline* (derived, unpaired: old arm of TOTAL over old arm of MEM; `par_alone_vs_baseline.md`):
sum of f4 times 409.5 → 403.4 ms at 1 thread (1.015) and 336.3 → 296.0 ms at 4 threads (1.136); quad_n9 168.0 → 145.1 ms at
4 threads (1.158); small cases at 4 threads slower: quart_n3 0.74 → 0.84 ms (0.882), cubic_n4 1.68 → 1.81 ms (0.926).

*Deterministic (measured):* f4-parallel cut the one-thread share of counted row operations on quad_n9 from 10.2% to 2.5%
and on cubic_n5 from 97.9% to 12.4% (`F4_FP_PAR_PROF=1`). gf2-m4ri: callgrind `macaulay_profile` 71.1 M → 54.4 M Ir
(1.31×), builder 37.9 M → 21.2 M Ir (1.79×); production stage ladder −0.17% / −0.50% Ir (rungs 5 / 2). f4-memory:
accumulator words per field operation 1.0 → 0.131–0.142.

== Controls and limits

A/A spread per case is in `data/rows.json` (`aa_tau_ln`); CROSS ratios were 0.97–1.05 (toggles restore the baseline).
About 300 comparisons without multiplicity correction, no single primary endpoint, analysis code written after data
collection began, one host. No msolve arm. quad_n10, quad_n11 and cubic_n6 were not timed. f4-memory's overdet gains rest
on a weak output check (basis {1}). Red-team verdicts: `redteam/verdicts.json`.

== Raw data

`data/runs.jsonl` (1,440 runs), `data/raw-logs.tar.gz`, `data/rows.json`, `data/fingerprints.json`,
`data/binaries.sha256`; integration correctness matrix `integration/correctness-matrix.tar.gz`; lead notes `leads/*/`.
