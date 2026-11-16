// N83 K1107 against matched precomputed rho: run 4 report.
// Every number is read from records/run-20261028T1511Z/analysis.json or the
// run's host/ and */time.txt receipts; derived sums are labelled.
#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[N83 Koblitz: K1107 index calculus against precomputed-table rho]\
  #text(size: 10pt)[Run 4, `records/run-20261028T1511Z` · source `de31b7c99` · 2026-10-28 · L0 shared VM]
]

= Question and decision

Does the current N83 cold IC baseline (K1107/root11 dynamic, 2026-10-21) beat
Pollard rho online once rho gets matched precomputation (a Bernstein–Lange table of
`G`-only distinguished points)? Protocol: `PROTOCOL.md`, frozen before any run, with
three amendments for runs 1–3, which failed on infrastructure and are kept, not combined.

*Decision under the frozen rule (§4): precomputed rho is faster online in all three
matched arms.* Each 95% interval of the median paired ratio IC/precomputed-rho lies
entirely above 1. This matches the frozen prediction. It is a reproduction diagnostic
on this host only, not an improvement, gate result or leaderboard row.

= Inputs and host

- Curve `EC1N83Ckb1a1haacb69ee9f37` ($y^2 + x y = x^3 + x^2 + 1$ over $FF_(2^83)$), r = 8569786107849059; the 16 public points of the 2026-10-16 control (sha256-checked).
- IC: `koblitz_orbit_dlp_fast construct:83:1:1107`, rank seed 29, root11 dynamic environment, `taskset -c 0-3`, one process for all targets.
- Rho arms: configs of the 2026-10-16 control; (a) 650 s table walks, 1 thread; (a′) walks budgeted at IC target-independent wall − 2 s = 149.6 s, 1 thread; (b) table bytes = IC max RSS (2,032,291,840 B), 4 threads; plain empty-table rho as positive control.
- Host: Intel Xeon @ 2.10 GHz, 4 vCPU, about 15 GB, no PMU. Load gate passed (1-min load 0.21 at start).
- Analysis: `n83_precomputed_rho_analyze` (exit 0): every recovered scalar replayed on the generic library curve; all arms agree; all match the hosted reference answers.

= Online results (measured)

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, right, left, left, right),
  [*arm*], [*threads*], [*online median ms [95% CI]*], [*median paired IC/arm [95% CI]*], [*IC slower*],
  [IC K1107], [4 / 1], [115.0 [44.6, 379.7]], [], [],
  [precomputed (a) 650 s], [1], [3.69 [1.99, 4.87]], [*40.9* [7.58, 141.9]], [15/16],
  [precomputed (a′) IC wall], [1], [15.2 [11.9, 30.7]], [*4.10* [1.58, 24.8]], [12/16],
  [precomputed (b) IC RSS], [4], [4.21 [2.16, 9.39]], [*33.1* [6.60, 74.4]], [15/16],
  [plain rho (control)], [1], [2,226 [1,561, 2,654]], [rho/IC 10.7 [6.33, 88.8]], [1/16],
)

#figure(image("../../figures/online-and-ratio.svg", width: 100%),
  caption: [Run 4 (run-20261028T1511Z). Left: online interval per target for every arm. Right: per-target paired ratio IC/precomputed rho; diamonds are medians with 95% bootstrap intervals (10,000 target resamples). Values from `analysis.json`; layout by `plot.py`.])

= Cold cost (derived: setup + sum of 16 online intervals)

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, right, right, right, right),
  [*arm*], [*setup s*], [*online total s*], [*cold s*], [*peak RSS*],
  [IC (4 threads, 314 s user CPU)], [151.6], [3.87], [155.5], [2.03 GB],
  [precomputed (a)], [651.0], [0.07], [651.0], [0.32 GB],
  [precomputed (a′)], [149.8], [0.42], [150.2], [0.08 GB],
  [precomputed (b) (4 threads)], [586.2], [0.11], [586.3], [2.04 GB],
  [plain rho], [0], [36.96], [36.96], [],
)

Reported beside the decision, not ranked. Plain rho solved all 16 targets in about a
quarter of IC's whole-process wall on this host.

= Operation counts (§5a)

Median paired IC/precomputed-rho count ratio at w = 0 / 0.1 / 1 GAE per unpriced
native operation: (a) 0.0005 / 2.04 / 8.18; (a′) 0.0001 / 0.41 / 1.64; (b) 0.0009 /
3.76 / 15.0. The order flips inside the band for every arm, so no ranking on counts.
IC's group-addition count is a lower bound.

= Limits

L0 shared VM, one run, public points (reproduction panel). Hosts differ from the
2026-10-16 control (K3300 3.74–40.6, K600 81.5–396), so cross-study numbers are not
paired. No replay certificate or validator report: nothing enters the leaderboard or
`benchmarks/README.md`.
