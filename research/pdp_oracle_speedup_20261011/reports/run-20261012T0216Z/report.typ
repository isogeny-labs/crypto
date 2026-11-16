#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-3 run report, host A: bit-sliced group-law kernel at 64, 256, 512 lanes] \
  #text(size: 10pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps3_bitsliced/raw/` --- 2026-10-12, 02:16--02:52 UTC]
]

= What ran

`ps3_bitsliced/run_ps3.sh` (correctness, programs, utilisation, product micro-benchmark, then the interleaved timing stage), `examples/pdp_speedup`, one release binary for every arm (SHA-256 `c40434d6...a6c1`), runtime dispatch, no global `target-cpu`. Host A: Intel Xeon 2.80 GHz KVM guest, 4 vCPU, AVX-512 F/BW/CD/DQ/VL/VNNI, PCLMULQDQ, kernel `6.18.44-fc-v114`; timed runs on CPU 3 through `tools/isolated_bench.py run --cpus 3` (a shared container, not the repository's receipt service). Arms: `baseline` (`decompose`), `baseline2` (the A/A), `group-scalar` (PS-2), `bs64`, `bs256`, `bs512`. Five rounds requested per rung, every round runs every arm once.

*Runs requested and completed.* 180 timed rows requested (6 arms x 5 rounds x 6 rungs `l = 8..13`); *169 completed*. The container stopped during `l = 13`, round 4; `l = 13` has three complete rounds. 4 of the 169 are marked contended by the tool (`baseline2-l8-r2`, `group-scalar-l8-r2`, `group-scalar-l9-r2`, `bs64-l9-r2`); they are kept in every figure and table, and `ab_summary_excl_contended.md` drops their rounds. 9 starts were refused by the pre-flight and retried (`raw/refused_starts.log`). The thread-scaling stage did not run on this host.

= Observations

#figure(
  image("ns_per_target.svg", width: 92%),
  caption: [Wall nanoseconds per refuted target by arm and rung (source `raw/times.jsonl`; hollow marks are runs the tool marked contended; black bars are per-arm medians; log scale).],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto),
  align: (right, right, right, right, right, right, right),
  table.header[*l*][*baseline ms*][*group-scalar*][*bs64*][*bs256*][*bs512*][*rounds*],
  [8], [11.13], [0.351], [0.808], [0.652], [0.802], [5],
  [9], [48.88], [1.248], [2.906], [1.524], [1.862], [5],
  [10], [206.44], [3.504], [9.574], [4.035], [3.796], [5],
  [11], [897.32], [15.863], [38.494], [14.711], [14.498], [5],
  [12], [3839.87], [60.292], [160.481], [59.347], [50.358], [5],
  [13], [16167.60], [245.456], [749.956], [262.647], [245.700], [3],
)

Medians over rounds, milliseconds per refuted target. A/A (baseline against baseline) run-to-run spread: 7.95%, 8.72%, 5.42%, 5.01%, 9.30%, 2.06% at $l = 8..13$.

#figure(
  image("ns_per_pair.svg", width: 92%),
  caption: [Wall nanoseconds per own pair (an arm's pairs are its own: the baseline visits every abscissa of $V$, the others the liftable base points).],
)

At $l = 11$ the baseline against each arm, per target (B/A medians, reciprocals derived): group-scalar 0.0175 (57.1x), bs64 0.0420 (23.8x), bs256 0.0165 (60.6x), bs512 0.0160 (62.5x). Against the scalar group kernel, per own pair, $l = 11$: bs256 0.934, bs512 0.935 (min..max 0.841..1.038; the A/A spread of the scalar arm is 7.48%).

Verdict digests equal PS-0's on every rung for all six vector arm/thread configurations (30 records, 0 mismatches), plus six $l = 13$ sample records. The recorded PS-2 program hashes reproduce on every rung (`programs_equal_ps2.txt`). Lane utilisation at $l = 11$: 0.913, 0.780, 0.653 (64, 256, 512 lanes).

= Missing or not done

The thread stage; $l = 13$ rounds 4 and 5; the 4-thread isolated run (the tool refuses to reserve all four vCPUs, see the host B report); `bs512` under `callgrind` (valgrind 3.22 does not execute AVX-512). Wall figures are exploratory in the repository's isolation vocabulary. Nothing end to end; nothing for ECC2K-130. Host B rows are in a separate report and are not pooled with these.
