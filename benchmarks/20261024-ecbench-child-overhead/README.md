# ecbench child overhead (2026-10-24)

Copies for browsing. The study, with raw data, inputs, protocol and tools, is
[`research/ecbench_child_overhead_20261024/`](../../research/ecbench_child_overhead_20261024/README.md).

- `report.pdf`: the report;
- `overhead.svg`, `overhead.png` (150 dpi): instructions per measured
  child (Callgrind) and wall time per child for `1b74250e9`, `0f5d9c272`,
  `004b792b7` and `645666691`.

Headline: Callgrind instructions per `ecbench exec` child fell from 194.7M
to 4.3M (rho) with #2184. They fell from 169.5M to 9.9M (IC) with this
entry's fix. Wall time per child, pooled: 37.20 → 2.92 ms. Per IC child:
31.65 → 4.61 ms against `main`.

**No `benchmarks/README.md` row yet.** The validation gate
(`docs/VALIDATION_GATE.md`) admits a row only with a replay certificate, an
independent validator report and a matched rho reference. This entry is
engineering: every solve is identical across the binaries, so no ECDLP
cost changes and no rho comparison applies. The row is added when a
validator has re-measured the Callgrind counts.
