# Eight fresh rho seeds on the public M31 target

2026-11-28. Diagnostic result, `[pcc:unverified]`. The [protocol](PROTOCOL.md) was committed locally at `ef8bcdb9` before these outcomes; the implementation and runner were committed locally at `8cb6bf24`. Git HTTPS push stalled, so these local commits were not remotely timestamped before execution. The protocol and result were later uploaded through the GitHub API; this is a source-order receipt, not an independently verifiable preregistration. The exact build and process inputs are in `results/manifest.json`, all 17 process records in `results/records.jsonl`, and gzip-preserved stdout and verbatim stderr beside them. `verify.py` checks each raw hash and independently computes `[d]G=Q`.

The curve is `p=2147483647`, `a=847514846`, `b=2087444724`, generator `(625037715,2100627251)` of order `2147446831`, and public target `(976979610,1228979761)`. The scalar `987654321` was recovered and verified on every run. The IC arm is the unchanged current 8-lane `mitm_four_fast.rs` at SHA-256 `098c6d3c…` with `40 12345678 4`. The rho arm differs from the [published diagnostic](../beat-rho-diagnostic-20261128/README.md) only by accepting a required seed argument. Its old fixed seed reproduced the published 44,428 steps, 45,177 additions and 355 walks exactly before the panel.

The Rust 1.100.1 release binaries were built offline with the committed lockfile and unchanged fixture/arithmetic modules in the small Cargo adapter. This avoids the unrelated full-package F4 source-hash failure; it is not a full-package or isolated L2 result. The local host exposed five AMD EPYC 9V74 vCPUs and was shared. Each pair used fresh processes in alternating order, with full setup, failed rho walks, point checks, and exit costs included.

| Measure | 4-thread IC | 64-jump negation rho diagnostic |
| --- | ---: | ---: |
| Completed and independently verified | 8/8 | 8/8 |
| Median external process wall | 25.465 ms | 10.222 ms |
| Wall range | 22.671–30.672 ms | 7.179–26.213 ms |
| Median child CPU | 44.815 ms | 10.094 ms |
| Relation/probe or walk work | 707 rows, 11 queries, 10,068,344 probes each | 14,130–75,214 steps; 123–622 walks |

Paired IC/rho wall ratios by seed order are `2.427, 4.010, 2.603, 3.576, 0.884, 2.126, 2.823, 2.543` (median `2.573`; 7/8 favor rho). The paired CPU ratio median is `4.370`. The fifth seed took 75,214 rho steps, so its rho process was slower than the IC process; it is retained. The separate seed-0 A/A process reproduced the rho scalar, steps, additions, walks, cycles and distinguished-point counts exactly.

This extends the earlier one-seed timing diagnostic with fresh walk seeds but still concerns **one fixed target** and a v5-inspired scratch rho policy. It does not qualify a cross-target, cross-curve or asymptotic claim, and it does not prove parity with the repository's `RhoV3::v5(true)` or its newer v6. The next discriminating implementation step is an exact repository-native tuned rho arm on this curve and target, followed by a held-out target panel under the same cold, independently replayed protocol. A change to IC relation collection or target descent would need its own prespecified gate.
