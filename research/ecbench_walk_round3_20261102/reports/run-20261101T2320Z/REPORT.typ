#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.5pt)
#set par(justify: true)

= ecbench round 3 run report: unrolled 32-bit inversion, walk states in the key ring

*Run start:* 2026-11-01T23:20Z (UTC). *Study:* `research/ecbench_walk_round3_20261102/`. *Class:* engineering; every child report identical apart from timing fields.

== What ran

- *Binaries.* M3: `main` 8931f24b88 with this round's file reverted (sha256 4af2a84a89695625…); H: with it (4842ce99db727bd1…). Same tree otherwise.
- *Inputs.* The exported 32-bit child inputs (`data/input-32bit-*.json`, byte-identical under both binaries and round 2's) and the 20-bit `audit-ir` seq 0–2.
- *Instruments.* Callgrind (`--cache-sim=no --branch-sim=no`): marker split for the rho child, whole process for the IC child, `audit-ir` at 20 bits. Wall: the child's `solve_wall_ns`, interleaved M3 / H / M3, `taskset -c 3`, host load about 1.
- *Host.* Shared 4-vCPU cloud VM, Intel Xeon @ 2.10 GHz, no PMU; L0. valgrind 3.22.0, rustc 1.97.0.
- *Completed.* 2 binaries × (rho marker, rho whole, IC whole, 3 `audit-ir` children); 9 + 7 wall rounds. No failures. `verify --replay-all --exit-code` under H: 36/36 and 4/4 reproduced. `cargo test` (ic_boundary, workload, curve_id, ic_measurement): 77 passed, 0 failed; clippy clean apart from the pre-existing unknown-lint warning.

== Measured values

#table(
  columns: (auto, auto, auto, auto),
  align: (left, right, right, right),
  [*Callgrind Ir*], [*M3*], [*H*], [*change*],
  [32-bit `rho.negation` solve (34,156 GAE)], [24,782,086], [*22,368,502*], [−9.7%],
  [32-bit `rho.negation` whole child], [27,311,486], [*24,902,908*], [−8.8%],
  [32-bit `ic.pipeline` N=1024 whole child], [776,961,624], [*707,398,246*], [−9.0%],
  [20-bit `rho.negation` solve (1,706 GAE)], [1,054,694], [*993,690*], [−5.8%],
  [20-bit `ic.pipeline` N=32 solve (14,101 GAE)], [12,399,287], [*11,647,495*], [−6.1%],
  [20-bit `ic.pipeline` N=45 solve (10,632 GAE)], [8,547,378], [*8,026,386*], [−6.1%],
)

Wall (median, descriptive): 32-bit rho 6.04 → 5.98 ms (A/A 6.46 ms); 32-bit IC 287.6 → 284.0 ms (A/A 263.2 ms). No difference resolved: the A/A spread exceeds it.

#figure(image("../../figures/round3.svg", width: 100%),
  caption: [Recorded values only (`data/ir_32bit.tsv`, `data/ir_20bit.tsv`, `data/wall_rounds_32bit.tsv`).])

== Raw records

`data/ir_32bit.tsv`, `data/ir_20bit.tsv`, `data/wall_rounds_32bit.tsv`, `data/input-32bit-rho-negation.json`, `data/input-32bit-ic-pipeline.json`.
