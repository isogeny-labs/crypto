# Run report: residue leads R1–R4, 2026-11-20 (UTC start ~20:50)

- **Commands.** `native/target/release/residue_leads r1|r23|r4|r2v2 ../results`; replay of `r1 r4 r3 r2v2` into a scratch directory.
- **Commits.** Protocol frozen at `7d8c0633`. v1 run (r1, r2, r3) from `94f6a75c`. r4 and r2v2 ran from the
  amendment-2/3 source, committed as `78657d08`. The replay from `78657d08` gives byte-identical r1, r4, r3,
  r2_v2 and aa_r2_v2 (sha256 prefixes fda289e2…, d2006c77…, ed941f37…, e2876f67…, 09331a6b…). Binary sha256
  `460b817dc1826ee7e1c9b7a811e63dcfcc59d2b6c01e0d7758ac7856b3c6faaf`.
- **Host.** Local container, 4 vCPU, rustc via cargo 1.97.0. Isolation level: none required, because every metric
  is an exact count with no clock. isolab-eks was unavailable (no `aws`, no identity), and the S3 checkpoint
  failed (no upload token).
- **Runs requested and completed.** R1, R2 v1, R3 and R2 v2 completed. R4 failed twice on harness defects (a
  non-terminating twist scan and two never-prime-order families), was fixed under Amendment 2, and then
  completed. Seeds: SplitMix64 from the protocol hash `0x5015e814565adcef`, with per-label streams.
- **Measured values.** `results/*.json`. Graphs and the PDF: `../../REPORT.pdf`, with figures in `../../report/`.
- **Controls.** All positive controls detected (see REPORT §2–§5). All A/A arms valid (KS ≤ 0.234). The
  exceptions: the R3 binary positive control at m = 11 is underpowered (z = 3.2), and those rows decide nothing.
