# Same-public-point timing favors rho in an exploratory 43-point panel

**Five alternating pairs on the public point `T=(2,2)` all recover and independently verify scalar `8`; the rational-lift search has median online wall time 0.762 ms and Floyd rho 0.255 ms in the corrected ordinary-host panel.** The rational arm constructs and reduces 35 rational points per pair before its collision and target lift determine the scalar. Each rho arm uses 27 walk updates, with five distinct starting seeds. The first rational-arm call takes 48.878 ms after setup, while its four subsequent calls take 0.744–0.836 ms, exposing a large first-use effect that the raw records preserve.

| Pair | Order | Rational lift online ms | Rho online ms | Verified scalar |
| ---: | --- | ---: | ---: | ---: |
| 1 | lift, rho | 48.878 | 0.370 | 8 |
| 2 | rho, lift | 0.836 | 0.255 | 8 |
| 3 | lift, rho | 0.753 | 0.257 | 8 |
| 4 | rho, lift | 0.762 | 0.254 | 8 |
| 5 | lift, rho | 0.744 | 0.238 | 8 |

The [frozen protocol](protocol.json) specifies `E(F₄₇): y²=x³+46x+45`, subgroup order 43, `G=(43,19)`, and the same public point for both arms. The rational arm uses the exact rank-two lift curve and height-pairing order from the [previous packet](../xedni_height_order_prefix_20261120/README.md); the pairing, rational curve, `Q,R`, finite curve, and candidate score order are prepared before the online clock. Its online clock starts immediately before the first target-dependent rational candidate construction and stops after scalar replay. Rho's clock starts before its first target-dependent state construction and stops after its scalar replay. The corrected panel's target-independent setup interval is 62.422 ms, kept separately from both online intervals. Discovering the rational generators is outside that setup receipt.

The [runner](run_sage.py) and original protocol were pushed before measurement in `ff8cd05a88d`. The initial five-pair panel accidentally used seeds separated by 43, so the rho starting states repeated modulo the subgroup order. That panel, its runtime receipt and raw rows are retained as [initial evidence](initial-result.json). Commit `16aa159648e` changed the frozen rho schedule to `17×pair+restart`, recorded each effective seed, and preserved the initial panel before the corrected run. The [corrected raw rows](pair-rows.jsonl) contain each arm's monotonic start/stop timestamps, elapsed nanoseconds, order, work counts, verified scalar and status; the [result](result.json) fixes their digest, host summary, setup interval, and medians. The accepted [Sage runtime](sage-runtime-info.json) and stderr are retained. An [independent Python/F₄₇ audit](audit.py) checks pair order and timing boundaries, verifies five distinct rho seeds, replays all ten scalar answers by point addition, and matches both panel digests; its [result](audit-result.json) passes. The previous height-order packet independently checks candidate order.

This panel is an ordinary-host exploratory diagnostic on a reused 43-point target selected during method development. It has no Tier 0 isolation or Tier 1 fresh-target/noise receipts. The first-use effect, missing generator-discovery cost, and submillisecond intervals limit timing interpretation. The exact result here is that both algorithms verified the same target in every recorded pair, with the stated raw intervals and setup. A promotion study needs fresh frozen targets, a complete candidate manifest and stage accounting, measured cold setup, and a matched Tier 0 or Tier 1 resource envelope under the `cryptanalysis/docs/IC_BENCHMARK_TIERS.md` contract.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_same_target_timing_20261120/audit.stderr
```
