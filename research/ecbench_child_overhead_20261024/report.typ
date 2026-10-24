#set page(paper: "a4", margin: 18mm)
#set text(font: ("Liberation Sans", "DejaVu Sans"), size: 9.5pt)
#show heading: set text(size: 11pt)
#show table: set text(size: 8.5pt)

#text(size: 15pt, weight: "bold")[The cost of one measured ecbench child]

2026-10-24 · `research/ecbench_child_overhead_20261024/` · measured: Callgrind instruction counts; wall time descriptive (shared 4-vCPU Xeon VM, L0, no PMU) · class: *engineering* (GAE unchanged, solves identical across binaries)

Every measured run of every ecbench session is a fresh `ecbench exec` process. Arms: before #2184 (`1b74250e9`), after its child-path commits (`0f5d9c272`), `main` (`004b792b7`), and this PR (`645666691`). This PR removes the alias-table parse that `main` had grown into every index-calculus child. Inputs: the first 18 measured executions of `docs/ecbench/specs/smoke-prime.json` (20-bit prime-order curve, every method). Protocol: `PROTOCOL.md`, frozen before timing.

#figure(image("figures/overhead.svg", width: 100%), caption: [Instructions (Callgrind, whole process, every dump part) and wall time per child. Data: `callgrind/totals.tsv`, `wall/wall.json`.])

= Instructions per child

#table(columns: 5, align: (left, right, right, right, right),
  [*method (input)*], [*before*], [*after \#2184*], [*`main`*], [*this PR*],
  [`rho.negation` (008)], [194.7M], [4.26M], [4.64M], [4.64M],
  [`bsgs.negation` (012)], [194.2M], [3.53M], [3.89M], [3.89M],
  [`ic.pipeline` (001)], [228.1M], [37.96M], [169.47M], [*9.86M*],
)

#2184 stopped each child from parsing the whole 4.7 MB curve registry; a child now parses its one entry (190M instructions, about 98% of a rho or BSGS child). This PR stops each IC child from parsing the whole 2.8 MB alias table to match one curve name against the 25-row frozen calibration table. That cost 27M instructions at `0f5d9c272` and 154M on `main`. Each name is now resolved by binary search over the sorted text, with answers equal to the map's on every name (`curve_id::tests`).

= Wall time per child (30 rounds, 2,000-resample round bootstrap)

#table(columns: 4, align: (left, left, right, left),
  [*comparison*], [*inputs*], [*ratio of medians*], [*95% interval*],
  [after / before], [all 18], [*0.0786* (37.20 → 2.92 ms)], [[0.0769, 0.0802]],
  [after-aa / after (A/A)], [all 18], [1.0032], [[0.9820, 1.0288]],
  [this PR / `main`], [all 18], [0.9910], [[0.9754, 1.0083]],
  [this PR / `main`], [`ic.pipeline` 001], [*0.1457* (31.65 → 4.61 ms)], [[0.1403, 0.1534]],
  [this PR / `main`], [`ic.pipeline` 010], [*0.1400* (30.82 → 4.31 ms)], [[0.1322, 0.1499]],
  [`main` / after], [all 18], [1.1460], [[1.1278, 1.1685]],
)

The pooled A/A holds. Per input, host noise is about ±6%: one BSGS input's A/A reads 0.94 [0.89, 0.99]. The IC ratios are far outside that. `main` is about 15% slower than `0f5d9c272` on rho and BSGS children, with about 9% more instructions, from the measurement-hygiene work merged since (allocator trim, per-solve observations). That is not addressed here. Still spent before a rho solve under this PR (about 3.2M instructions): the registry entry search (1.2M, a linear scan because the registry is not sorted by slug), the argument parser (0.8M) and the curve's ICV1 identity (0.35M). The solve itself is about 1.1M.
