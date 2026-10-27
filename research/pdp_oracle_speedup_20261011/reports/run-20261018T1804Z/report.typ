#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-8 run report: meet-in-the-middle pair table against the pairs-and-solve kernels] \
  #text(size: 10pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps8_mitm/raw/` --- 2026-10-18, 18:04--18:32 UTC]
]

= What ran

`ps8_mitm/run_ps8.sh` (correctness, then the interleaved timing stage), `examples/pdp_speedup` arm `mitm` (`mitm.rs`), one release binary for every arm (SHA-256 in `raw/binary_hashes.txt`), protocol `protocols/PS-8.md` frozen before the first measurement. Host: Intel Xeon 2.10 GHz KVM guest, 4 vCPU, kernel `6.18.44-fc-v139` (the second host class of PS-3); timed runs on CPU 3 through `tools/isolated_bench.py run --cpus 3` (a shared container, not the repository's receipt service). Arms: `baseline`, `baseline2` (A/A), `group-scalar` (PS-2), `bs512` (PS-3), `mitm`. Five rounds, every round runs every arm once.

*Runs requested and completed.* 150 timed rows requested, 150 completed, 0 marked contended, 0 refused starts. Correctness: 13 `verify-arm` records (`mitm` and `group-scalar` on $l = 8..12$, 80 targets each; `baseline`, `group-scalar`, `mitm` on the 3-target $l = 13$ sample), 0 mismatches.

= Observations

#figure(
  image("ns_per_target.svg", width: 92%),
  caption: [Wall nanoseconds per refuted target by arm and rung, probe stage only (source `raw/times.jsonl`; black bars are per-arm medians; log scale). The table's build is not in these rows.],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  align: (right, right, right, right, right, right, right, right),
  table.header[*l*][*group-scalar ms*][*bs512 ms*][*mitm probe µs*][*mitm build s*][*keys*][*MiB*][*N\* vs group-scalar*],
  [8], [0.226], [0.653], [2.4], [0.0007], [16,383], [0.4], [3.0],
  [9], [0.812], [1.577], [6.9], [0.0026], [64,008], [1.5], [3.3],
  [10], [2.584], [3.587], [23.4], [0.0126], [215,295], [6.0], [4.9],
  [11], [11.854], [13.430], [86.5], [0.0634], [1,014,048], [24.0], [5.4],
  [12], [43.455], [44.739], [170.6], [0.2388], [3,968,063], [96.0], [5.5],
  [13], [181.363], [211.016], [502.8], [1.4068], [16,418,703], [384.0], [7.8],
)

Medians over five rounds (build: median of the five runs' setup). $N^*$ is derived: build seconds divided by the per-target saving. A/A (baseline against baseline) run-to-run spread: 5.38%, 8.01%, 6.74%, 10.22%, 3.14%, 3.53% at $l = 8..13$.

#figure(
  image("table_build.svg", width: 88%),
  caption: [Table build seconds per run (left; five runs per rung, bars are medians) and table size in MiB (right), source `raw/times.jsonl`.],
)

Probe against the scalar group kernel per target (B/A medians): 0.0106, 0.0087, 0.0094, 0.0074, 0.0039, 0.0028 at $l = 8..13$. The hypotheses of the protocol (identical outputs; probe at most 1/10 of the kernel's at $l >= 10$; break-even at most 100 targets; at most 40 bytes per key) all held on this host.

= Missing or not done

One host class; the repository's own `ic_boundary::PairTable` was not run on this corpus; no multi-threaded build or probe; nothing about relation yield, linear algebra or the counted unit $S$ (PS-6); nothing for ECC2K-130 (memory grows as $|B|^2$). Wall figures are exploratory in the repository's isolation vocabulary.
