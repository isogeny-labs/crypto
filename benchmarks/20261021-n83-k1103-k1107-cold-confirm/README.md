# N83 cold factor-base comparison: K1103 vs K1107

This entry records the frozen 16-point, fresh-process cold comparison under the fixed root11 dynamic layout. K1107 averaged **113.755 s** versus **114.950 s** for K1103. The paired difference K1103 − K1107 was **1.195 s**, with a two-sided 95% Student-t interval of **[0.957, 1.433] s** across 16 points. All paired point differences favored K1107.

![Paired cold-process differences by public point](paired-cold-difference.svg)

The result is bounded to these two factor bases, this implementation/layout, and this one-target cold workload. The same-point rho measurement in the protocol is online only, so it does not provide a matched cold IC/rho ratio. This result therefore belongs in the benchmark history but does not add a point to the IC/rho progress timeline or leaderboard and does not establish an online winner.

- [Full protocol, candidate manifests, raw rows and replay receipts](../../research/n83_cold_base_confirm_20261021/)
- [Admitted analysis](../../research/n83_cold_base_confirm_20261021/results/analysis.json), SHA-256 `cb48efaedc1bc02a9650f962348f06c7af1aa39b410a9b23ffa29e8a49a086a8`
- Graph: generated from `cold_process_ms_by_point` and `paired_interval` in the admitted analysis by `render_paired_cold_difference.py`; validate with `python3 benchmarks/20261021-n83-k1103-k1107-cold-confirm/render_paired_cold_difference.py --check`.
