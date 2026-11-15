#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.5pt)
#set par(justify: true)

= ecbench round 2 run report: walk key ring, 32-bit inversion, raw-clock IC trace

*Run start:* 2026-11-01T20:10Z (UTC). *Study:* `research/ecbench_walk_trace_ir_20261101/` (README with the tables and the reasoning). *Class:* engineering; every child report identical apart from timing fields.

== What ran

- *Binaries.* E: #3144 head (sha256 0038806674a3af21…); F: E + `TunedWalker` key ring, jump wrap without division, 32-bit `invmod` (b6cd58faf502e51a…); G: F + the IC phase trace on raw `CLOCK_MONOTONIC` nanoseconds (1efe6b068214f08e…).
- *Inputs.* The exported 32-bit child inputs (`data/input-32bit-*.json`, byte-identical under every binary) and the 20-bit `audit-ir` seq 0–2 of `spec-p20-profile.json`.
- *Instruments.* Callgrind (`--cache-sim=no --branch-sim=no`): the marker split for the rho child, the whole process (`Collected`) for the IC child (its per-relation dump parts are discarded), `audit-ir` at 20 bits. Wall: the child's own `solve_wall_ns`, interleaved rounds E / G / E, `taskset -c 3`, host load about 1.
- *Host.* Shared 4-vCPU cloud VM, Intel Xeon @ 2.10 GHz, no PMU; L0. valgrind 3.22.0, rustc 1.97.0.
- *Completed.* 3 arms × (rho marker run, rho whole, IC whole) at 32 bits; 3 arms × 3 `audit-ir` children at 20 bits; 9 + 7 wall rounds. No failures. `verify --replay-all --exit-code` on the 36- and 4-record scratch sessions under F and G: all reproduced. `cargo test` (ic_boundary, workload, curve_id, ic_measurement): 77 passed, 0 failed; clippy clean apart from the pre-existing unknown-lint warning.

== Measured values

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, right, right, right, right),
  [*Callgrind Ir*], [*E*], [*F*], [*G*], [*E → G*],
  [32-bit `rho.negation` solve (34,156 GAE)], [30,092,881], [24,782,352], [*24,781,851*], [−17.6%],
  [32-bit `rho.negation` whole child], [32,598,131], [27,330,279], [*27,318,553*], [−16.2%],
  [32-bit `ic.pipeline` N=1024 whole child], [916,254,438], [845,735,563], [*776,313,670*], [−15.3%],
  [20-bit `rho.negation` solve (1,706 GAE)], [1,222,979], [1,054,933], [*1,054,467*], [−13.8%],
  [20-bit `ic.pipeline` N=32 solve (14,101 GAE)], [15,144,353], [14,373,889], [*12,409,320*], [−18.1%],
  [20-bit `ic.pipeline` N=45 solve (10,632 GAE)], [10,267,488], [9,737,258], [*8,551,931*], [−16.7%],
)

Wall (median, descriptive): 32-bit `rho.negation` solve 6.57 ms → 5.84 ms (A/A 6.51 ms, 9 rounds); 32-bit `ic.pipeline` solve 278.6 ms → 261.7 ms (A/A 279.6 ms, 7 rounds); round-to-round spread ±10–15%.

#figure(image("../../figures/round2.svg", width: 100%),
  caption: [Recorded values only (`data/ir_32bit.tsv`, `data/ir_20bit.tsv`, `data/wall_rounds_32bit.tsv`). Left: 32-bit Ir relative to E with absolute labels. Middle: 20-bit solve Ir per GAE. Right: every wall round with its median.])

== Raw records

`data/ir_32bit.tsv`, `data/ir_20bit.tsv`, `data/wall_rounds_32bit.tsv`, `data/input-32bit-rho-negation.json`, `data/input-32bit-ic-pipeline.json`. Scratch sessions as in `research/ecbench_child_walk_ir_20261101/` (not committed).
