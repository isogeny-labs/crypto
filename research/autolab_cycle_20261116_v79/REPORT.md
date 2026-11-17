# C79: event-driven completion fidelity and unchanged-code A/A

Hypothesis: kernel exit readiness removes fixed-sleep completion quantization without changing root code, deadlines or output. The new supervisor uses pidfd/select followed by wait4, stamps completion before bookkeeping, retains a full-lifetime deadline and process-group kill, and fails explicitly if pidfd is unavailable. Post-exec RSS sampling remains a lower bound; wait4 retains inherited/pre-exec peak scope. No old timings are corrected retrospectively.

Independent native controls passed24 normal observations plus8 nonzero/malformed/partial-timeout/memory controls. CLOCK_MONOTONIC stamps immediately before fflush/_exit supplied an independent exit-tail reference. Median observed tail was4.4337275ms for legacy polling and0.0927400ms for pidfd. That tail includes remaining output/exit work and scheduling, not pure observer overhead. The below1ms development prediction was met; this is timing-fidelity evidence, not a root-solver speedup.

Both observers retained nonzero exit7, rejected malformed output, killed/retained partial output on deadline and measured the8MiB allocation control. Sampled memory peak was8832KiB while lifetime wait4 peak was12672KiB, illustrating why these scopes cannot be conflated.

The pidfd A/A cohort used the identical frozen reference function and identical input arrays in two fresh processes per pair. Startup, every solve, complete original verification and serialization were charged. Eleven pairs produced22 batch processes and110 reference conditional solves; all roots, candidate shifts and polynomials matched. One of11 pairs failed abs(A-B)/median(A,B)<=10%. F65537 pair4 spread was16.659%; all three P521 pairs passed, with spreads0.973%,0.700%,3.007%. The full calibration therefore fails the all-pairs rule. It is neither qualified isolation nor an optimized-code comparison.

Failed-pair CPU/wall diagnostics:

```json
[
  {
    "field": "F65537",
    "rep": 4,
    "runs": [
      {
        "label": "A",
        "wall_seconds": 0.035201282,
        "child_CPU_seconds": 0.03405
      },
      {
        "label": "B",
        "wall_seconds": 0.041598338,
        "child_CPU_seconds": 0.040143
      }
    ]
  }
]
```

The remaining variation cannot be attributed to different root algorithms: A/B execute identical code. Its precise environmental cause is not established. Observer correctness does not erase startup, scheduling or I/O variability, and neither pair nor field may be dropped to claim qualification.

Next: a separately preregistered startup-amortization curve with all batch sizes retained can test whether a representative multi-query workload has stable observation. Keep per-instance hardness separate; do not bundle work to manufacture the one-second gate. Independent exact polynomial-squaring and binary trace-jump research can proceed while performance qualification remains unresolved.

Two frozen90-second,4-GiB,one-core experiments passed their functional checks through resume/run-ready. This worker success does not mean the A/A gate passed. No novel root algorithm, full original search, genuine F4/F5/F6 superiority or50x is claimed. Scientific goal active. Budget receipts retain actual coordinator/publication time separately.
