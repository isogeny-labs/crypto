#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= Run report: shipped F4 defaults, native confirmation (run-20261117T0022Z)

*Standing:* `[pcc:unverified]` — one shared 4-vCPU cloud VM, isolation L0, no PMU. *Scope:* wall-clock of the F4 engines
(`f4_fp`, the GF(2) Macaulay builder); no counted unit changed, so no index-calculus S value or gap to rho moves.
*Record:* `benchmarks/records/20261117T0022Z-f4-shipped-default-ab.json`. *Protocol:* `CONFIRM_PROTOCOL.md`, frozen
2026-11-17 00:19 UTC before any data.

== What ran

- *Source:* commit `b04b25832f` — f4-parallel and gf2-m4ri on by default, f4-memory opt-in (`F4_FP_MEM=1`). Binary SHA-256
  in `conditions.txt`.
- *Driver:* `examples/f4_ab_confirm.rs` (native Rust; statistics through `ecbench::stats`), launched under
  `tools/benchrec.py run`: `f4_ab_confirm <run>/data 12 f4_fp_bench koblitz_f4_kernel_bench`.
- *Arms:* `new` (no toggle) against `old` (`F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1`), same binary; A/A `new` vs
  `new`. Inherited `F4_FP_*`, `KIC_*`, `RAYON_NUM_THREADS` removed from every child.
- *Cells:* `f4_fp_bench 3 large` (13 fixed-seed $FF_p$ systems, each a median of 3) and `koblitz_f4_kernel_bench 5 x`
  (median of 5), RAYON_NUM_THREADS 1 and 4. 12 blocks, rotated cell order, ABBA/BAAB by parity: 24 (old, new) pairs per
  cell.
- *Runs:* 384 requested, 384 completed, 0 failed, 0 fingerprint mismatches across all arms (`data/runs.jsonl`).
- *Host:* Intel Xeon \@ 2.10 GHz, 4 vCPU KVM, AVX-512F; quiet before and after (`conditions.txt`).

== Pre-registered predictions and outcomes

#table(
  columns: (auto, auto, auto, auto),
  align: (left, left, left, left),
  table.header([*endpoint*], [*predicted*], [*measured: median old / new [95% CI]*], [*outcome*]),
  [*primary:* sum of f4 times, 4 threads], [[1.08, 1.20], significant], [1.141 [1.098, 1.158], significant], [met],
  [quad_n9, 4 threads], [[1.08, 1.25]], [1.161 [1.109, 1.209], significant], [met],
  [sum of f4 times, 1 thread], [[0.97, 1.06], not significant], [1.034 [0.981, 1.086], not significant], [met],
  [quart_n3, 4 threads], [≤ 1], [0.811 [0.687, 0.882], significant], [met (regression)],
  [cubic_n4, 4 threads], [≤ 1], [0.865 [0.645, 0.944], significant], [met (regression)],
  [GF(2) kernel_x, 1 thread], [[1.30, 1.60], significant], [1.335 [1.173, 1.540], significant], [met],
  [GF(2) kernel_x, 4 threads], [[1.30, 1.60], significant], [1.492 [1.367, 1.596], significant], [met],
)

A/A controls: sum of f4 times 1.006 [0.951, 1.037] (1 thread) and 1.004 [0.971, 1.035] (4 threads); quad_n9 0.985 / 0.984;
kernel_x 1.027 / 0.951.

#figure(
  image("ratios.svg", width: 100%),
  caption: [Median old / median new per case (>1: new faster) with bootstrap 95% CI, 24 pairs per cell; filled =
  significant (CI excludes 1 and |ln ratio| above the A/A spread). Values from `data/summary.json`, plotted by
  `scripts/plot_confirm.py`; run-20261117T0022Z.],
)

== Reading

At 4 threads the shipped default takes 300.1 ms where the old paths take 342.5 ms over the 13 systems (1.141); at 1
thread it is within the A/A spread. The cost is on the smallest systems at 4 threads: quart_n3 0.75 → 0.93 ms and
cubic_n4 1.67 → 1.93 ms, and the three `solve` paths (many small F4 calls) at 0.93–0.94, not significant. The GF(2)
cold Macaulay kernel is 1.34–1.49× faster. This matches the first run's integrated and derived figures
(`../run-20261116T2306Z/`), now as a paired same-binary measurement.

== Limits

One host, L0 isolation, unknown SMT topology; 2 thread counts; no msolve arm; quad_n10, quad_n11, cubic_n6 not timed;
the small-case regression is real and is not gated yet. Wall-clock only: the counted units (field operations, word
XORs) are unchanged by construction.
