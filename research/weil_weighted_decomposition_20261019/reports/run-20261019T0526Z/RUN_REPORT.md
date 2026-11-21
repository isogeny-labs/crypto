---
title: Weighted Weil-restriction decompositions
date: 2026-10-19
geometry: margin=18mm
fontsize: 10pt
---

# Native larger-curve diagnostic

This run uses the repository's native cubic-field arithmetic and S3 resultant pair solver. It constructs no covering curve and no whole-field or whole-group operation table. Each decision is cross-checked with a separate signed-pair dictionary unavailable to the solver.

The local reference is the identical hybrid with c=1, not the best complete IC pipeline. Counts below are stage diagnostics, not end-to-end speedups. Complete S, ratios to rho and external fastest-tool comparisons are unestablished. Wall time is not admitted at L0.

![Recorded pair queries per verified relation; native-20261019-primary](counts.png)

| p | construction seed | group order | unknowns | weight | verified / targets | pair tests | pair tests / relation |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 31 | 20261019 | 30047 | 11 | 1 | 6 / 128 | 2714 | 452.333 |
| 31 | 20261019 | 30047 | 11 | 2 | 24 / 128 | 2537 | 105.708 |
| 31 | 20261019 | 30047 | 11 | 4 | 17 / 128 | 2605 | 153.235 |
| 31 | 936451 | 29629 | 11 | 1 | 2 / 128 | 2788 | 1394.000 |
| 31 | 936451 | 29629 | 11 | 2 | 21 / 128 | 2564 | 122.095 |
| 31 | 936451 | 29629 | 11 | 4 | 20 / 128 | 2581 | 129.050 |
| 61 | 20261019 | 227893 | 30 | 1 | 15 / 128 | 7034 | 468.933 |
| 61 | 20261019 | 227893 | 30 | 2 | 38 / 128 | 6483 | 170.605 |
| 61 | 20261019 | 227893 | 30 | 4 | 48 / 128 | 6143 | 127.979 |
| 61 | 936451 | 227111 | 34 | 1 | 27 / 128 | 7230 | 267.778 |
| 61 | 936451 | 227111 | 34 | 2 | 60 / 128 | 6385 | 106.417 |
| 61 | 936451 | 227111 | 34 | 4 | 69 / 128 | 6313 | 91.493 |
| 127 | 20261019 | 2047933 | 63 | 1 | 24 / 128 | 13849 | 577.042 |
| 127 | 20261019 | 2047933 | 63 | 2 | 52 / 128 | 12848 | 247.077 |
| 127 | 20261019 | 2047933 | 63 | 4 | 53 / 128 | 12395 | 233.868 |
| 127 | 936451 | 2048113 | 62 | 1 | 18 / 128 | 14197 | 788.722 |
| 127 | 936451 | 2048113 | 62 | 2 | 57 / 128 | 12055 | 211.491 |
| 127 | 936451 | 2048113 | 62 | 4 | 56 / 128 | 11768 | 210.143 |
| 241 | 20261019 | 13993069 | 130 | 1 | 19 / 128 | 29373 | 1545.947 |
| 241 | 20261019 | 13993069 | 130 | 2 | 71 / 128 | 22303 | 314.127 |
| 241 | 20261019 | 13993069 | 130 | 4 | 52 / 128 | 25741 | 495.019 |
| 241 | 936451 | 14001257 | 116 | 1 | 14 / 128 | 27131 | 1937.929 |
| 241 | 936451 | 14001257 | 116 | 2 | 58 / 128 | 22333 | 385.052 |
| 241 | 936451 | 14001257 | 116 | 4 | 53 / 128 | 22813 | 430.434 |
| 499 | 20261019 | 124249159 | 249 | 1 | 19 / 128 | 56670 | 2982.632 |
| 499 | 20261019 | 124249159 | 249 | 2 | 47 / 128 | 52049 | 1107.426 |
| 499 | 20261019 | 124249159 | 249 | 4 | 52 / 128 | 49357 | 949.173 |
| 499 | 936451 | 124261523 | 258 | 1 | 19 / 128 | 58878 | 3098.842 |
| 499 | 936451 | 124261523 | 258 | 2 | 51 / 128 | 50621 | 992.569 |
| 499 | 936451 | 124261523 | 258 | 4 | 52 / 128 | 51520 | 990.769 |
| 1009 | 20261019 | 1027186051 | 495 | 1 | 21 / 128 | 110487 | 5261.286 |
| 1009 | 20261019 | 1027186051 | 495 | 2 | 45 / 128 | 99211 | 2204.689 |
| 1009 | 20261019 | 1027186051 | 495 | 4 | 47 / 128 | 100052 | 2128.766 |
| 1009 | 936451 | 1027286801 | 495 | 1 | 22 / 128 | 109779 | 4989.955 |
| 1009 | 936451 | 1027286801 | 495 | 2 | 42 / 128 | 106812 | 2543.143 |
| 1009 | 936451 | 1027286801 | 495 | 4 | 52 / 128 | 100150 | 1925.962 |

# Synthetic one-target recoveries

| p | seed | weight | residuals | independent rank | status |
|---:|---:|---:|---:|---:|:---|
| 31 | 20261019 | 1 | 211 | 12 | verified_recovery |
| 31 | 20261019 | 2 | 81 | 11 | verified_recovery |
| 31 | 20261019 | 4 | 66 | 12 | verified_recovery |
| 31 | 936451 | 1 | 146 | 11 | verified_recovery |
| 31 | 936451 | 2 | 64 | 10 | verified_recovery |
| 31 | 936451 | 4 | 43 | 12 | verified_recovery |
| 61 | 20261019 | 1 | 210 | 31 | verified_recovery |
| 61 | 20261019 | 2 | 75 | 29 | verified_recovery |
| 61 | 20261019 | 4 | 97 | 30 | verified_recovery |
| 61 | 936451 | 1 | 171 | 32 | verified_recovery |
| 61 | 936451 | 2 | 64 | 30 | verified_recovery |
| 61 | 936451 | 4 | 47 | 29 | verified_recovery |
| 127 | 20261019 | 1 | 431 | 59 | verified_recovery |
| 127 | 20261019 | 2 | 161 | 61 | verified_recovery |
| 127 | 20261019 | 4 | 164 | 63 | verified_recovery |
| 127 | 936451 | 1 | 526 | 59 | verified_recovery |
| 127 | 936451 | 2 | 148 | 56 | verified_recovery |
| 127 | 936451 | 4 | 126 | 58 | verified_recovery |
| 241 | 20261019 | 1 | 584 | 117 | verified_recovery |
| 241 | 20261019 | 2 | 248 | 119 | verified_recovery |
| 241 | 20261019 | 4 | 251 | 121 | verified_recovery |
| 241 | 936451 | 1 | 724 | 105 | verified_recovery |
| 241 | 936451 | 2 | 296 | 105 | verified_recovery |
| 241 | 936451 | 4 | 292 | 110 | verified_recovery |

Completed 24 recoveries; 0 budget-exhausted recoveries. Total independently checked PDP decisions, including A/A controls and recovery attempts: 11370. The independent comparison is against a separate dictionary, not an independent host.

# Limits and next checks

The legacy pair API does not emit internal lookup, polynomial addition/division, allocation or RNG counts. Its F_p multiplication counter also uses legacy inverse accounting. These missing counts prevent a complete pricing band, S, or ranking against rho. No strongest-external-solver comparison was run: Magma, msolve and FGb were not run here. Base and weighted-slot setup, verification group operations, residual-generation group operations and modular elimination multiplications have separate ledgers. Dictionary construction, reference arithmetic and fixture generation are excluded validation/setup work without complete operation ledgers. No binary-field or ECC2K transfer was tested.

The next discriminating step is complete native counters plus matched ecbench one-target rho measurements, with independent-host replay and the fastest available external algebraic reference. These obligations remain open; verified recoveries alone do not discharge them.

# Reproduction and provenance

See config.json, host.txt, SHA256SUMS, records.jsonl, the committed PROTOCOL.md and the native example. Seed 936451 is the holdout. Inputs and outputs are public synthetic fixtures. Histories from the earlier Python experiment remain historical and are not runtime baselines. All A/A decisions and deterministic counters were asserted equal.

# Run record

Run: native-20261019-primary. Source fc13c52e82202b44a13f9e32dca2fac2401e320e;
protocol 6095144c. Report analysis: 040b8e1a. Requested and completed:
12 curves, 48 PDP arms (including A/A), 128 targets per arm, 24 recoveries.
No budget exhaustion or failed fixture was observed. These are descriptive
counts; no uncertainty interval was computed. The report graph plots the
Rust-recorded values without additional experimental calculation.

Host: AMD EPYC 9V74 under KVM, four-CPU cgroup quota, 16 GiB cgroup memory.
Isolation L0 with concurrent work. No wall time or host-speed claim.

Exact command, budgets and exclusions are in
[config.json](../../runs/native-20261019-primary/config.json).
[Raw records](../../runs/native-20261019-primary/records.jsonl),
[provenance](../../runs/native-20261019-primary/PROVENANCE.md),
[PARI results](../../runs/native-20261019-primary/pari_checks.jsonl), and
[source snapshot](../../sources/primary.rs) retain the evidence.
