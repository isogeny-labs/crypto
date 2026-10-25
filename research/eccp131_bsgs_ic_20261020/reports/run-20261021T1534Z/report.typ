#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "Libertinus Serif", size: 8.5pt)
#set par(justify: true)

= ECCp-131 exact-probe benchmark

#text(size: 7.5pt)[Run start 2026-10-21 15:34 UTC · source `18550e9979` · macOS 26.6, Apple M4 Pro, 48 GiB · one Rayon thread · L0 descriptive wall]

== Exact-curve stage matrix

The frozen protocol ran twenty fresh `ic prime --curve eccp-131` processes. Five runs each used B256, B512a, B512b (the same B512 command, an A/A drift check), and B1024; every run checked 128 setup and 128 published-target candidates, starting target descent with the published Q. Eight planted decompositions passed in every run. All runs recorded zero natural rows, setup rank zero, and combined rank zero.

#figure(image("exact_stage.svg", width: 100%), caption: [Recorded stage intervals for all 20 runs. The probe interval rebuilds the table and checks 256 candidates. Source: `bench_20261021/exact/raw/`; graph source `plot.py`.])

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, right, right, right, right),
  inset: 3pt,
  [Arm], [n], [Pairs], [Median pair ms], [Median CLI ms],
  [B256], [5], [65,536], [24.47], [624.70],
  [B512a], [5], [262,144], [100.42], [835.47],
  [B512b], [5], [262,144], [83.07], [720.66],
  [B1024], [5], [1,048,576], [264.88], [1,051.25],
)

External maximum RSS ranged from 13.65 to 15.50 MB at B256, 24.49 to 24.79 MB at B512, and 65.63 to 66.13 MB at B1024. The whole CLI interval includes two table builds, planted controls, candidate checks, rank handling, serialization, and executable hashing. The B512 A/A spread under heavy host load makes wall comparisons descriptive only. The raw JSON omits missed scalar draws, so this matrix has no separate candidate-by-candidate replay.

#pagebreak()

== Separate native ecbench control

The standard `ecbench` prime instance uses `u64`, so its run used a registered 20-bit prime curve with two planted targets. It compared `rho.negation`, `ic.pipeline` with a 32-abscissa base and negation-folded MITM oracle, and an identical rho A/A arm. Session `ECBS1hcc2634f59aca` sealed 24 verified records; the full audit replayed all 18 measured records identically. This is a distinct code path and curve from the exact ECCp-131 probe.

#figure(image("ecbench_s.svg", width: 92%), caption: [Per-record `S` on the 20-bit control, six measured records per arm. All `S` values are lower bounds because native work is unpriced. Source: sealed `records.jsonl`; graph source `plot.py`.])

#table(
  columns: (auto, auto, auto, auto),
  align: (left, right, right, right),
  inset: 3pt,
  [Arm], [Verified], [Mean recorded S], [Lower bound],
  [rho-neg], [6/6], [1.1483], [yes],
  [ic-mitm], [6/6], [10.8027], [yes],
  [rho-neg-aa], [6/6], [1.1483], [yes],
)

The paired `ic-mitm / rho-neg` arithmetic ratio is 9.4076 with 95% cluster interval [7.2188, 12.9316] over six pairs. The deterministic rho A/A `S` ratio is 1.000. Pricing each unpriced counter on both arms at 0, 0.1, or 1 GAE gives ratios 9.4076, 9.8920, and 12.2958 for this 20-bit control only. No full-cost or ECCp-131 comparison follows from these values. Wall results are L0 and descriptive.

== Run limits and retained attempts

The `ecbench` child completed, but its shell wrapper failed while assigning zsh's read-only variable `status`; the sealed session and full audit establish the child result. The first jq pricing command used an unavailable `--argfile` option; both failed attempts are retained. Native source binaries and `Cargo.lock` are SHA-256 pinned in `bench_20261021/manifest.txt`. The original ECCp-131 problem record remains open: natural useful rank, published-target descent, scalar verification, and complete cold solution cost were not established by these bounded runs. No leaderboard or benchmark admission is asserted. Full commands and raw paths are in `REPORT.md` beside this PDF.
