# K1107 cold factor base wins the frozen K1103 comparison

The K1107/root11 dynamic base reduced mean fresh-process cold time by **1.195 seconds** against K1103 on 16 new N83 public points. The paired two-sided 95% Student-t interval for `K1103 - K1107` is **[0.957, 1.433] seconds**, so the preregistered cold selector chose `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h8c736ae9c7b4` within this fixed-layout pair. The GitHub Actions Tier 1 run and a second local execution of the exact verifier both admitted all 64 IC arms and 16 same-point rho arms; the local analysis equals the archived analysis as parsed JSON.

| Base | Exact candidate ID | Usable points | Folded columns | Mean cold process | Mean setup | Mean rank stage | Mean target stages | Geometric mean one-target online |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| K1103/root11 dynamic | `IC1N83Ckb1a1fb183098PDP4rootRCguidedLAgaussTDdirectISO0hab4dd05a14ae` | 183,098 | 1,103 | 114.950 s | 46.818 s | 67.999 s | 0.129 s | 91.264 ms |
| K1107/root11 dynamic | `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h8c736ae9c7b4` | 183,762 | 1,107 | **113.755 s** | 47.166 s | 66.486 s | 0.099 s | 62.750 ms |

The online values are diagnostics from 32 fresh IC processes per base, two per point. The same-point rho online geometric mean across the 16 points was 2,334.964 ms; the paired geometric-mean rho/IC ratios over those repeated IC processes were 25.58 for K1103 and 37.21 for K1107. This cold-selection protocol did not select an online winner. All A/B cold spreads were below the frozen 2% limit: maxima were 0.559% for K1103 and 1.338% for K1107. Setup, rank, target and residual process costs sum to the cold interval for each arm; the mean residual was under 0.004 s for either base.

The exact curve is `EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis GF(2^83), modulus terms `[0,2,4,7,83]`, `y² + xy = x³ + x² + 1`, subgroup order `8569786107849059`, and cofactor `1128547018`. The [protocol](PROTOCOL.md) froze the 16-point scalar law, source and manifest hashes, mirrored A/B order, 480-second IC and 120-second rho caps, exact base hashes, and the paired decision rule before the run. The host was a GitHub Actions Linux x86-64 Tier 1 allocation exposing four logical CPUs on two Intel Xeon Platinum 8370C cores and approximately 15.6 GiB RAM. Affinity, cgroup, topology, execution-order and per-arm telemetry gates passed. This comparison varies the exact usable factor base while holding the root11 dynamic index layout and implementation fixed.

## Evidence and replay

- [Run 39551089734](https://github.com/aburan28/crypto/actions/runs/39551089734) executed at PR head `849dc0e45946c71cb149c7cea96a01fb4cc0ea82`; provider artifact ID `12339508726`. The raw ZIP is 279,321,506 bytes, SHA-256 `d3f0d65a27e57d8d60ebfd49d04ec8ff693f06ef0db1803060f0e5ca56846c44`.
- The [archived analysis](results/analysis.json) has SHA-256 `cb48efaedc1bc02a9650f962348f06c7af1aa39b410a9b23ffa29e8a49a086a8` and status `COLD_WINNER_CONFIRMED_WITHIN_PAIR`. It preserves every point, arm, phase, resource receipt, A/A control and paired difference.
- An independent local run of `verify.py` from the same source snapshot exited 0, wrote zero stderr bytes, and reproduced that analysis as parsed JSON. The local stdout SHA-256 is `42f01dfd53aba16a3f1485bbb84ea679cf166a510c757e995f5397e3be826da3`.
- The [custody receipt](results/custody.json) binds the provider ZIP to an S3 object. A fresh S3 download matched the provider SHA-256. Restore the raw ZIP, extract its `results/` directory, and run `python3 research/n83_cold_base_confirm_20261021/verify.py /path/to/results` from the frozen PR source snapshot.

K1107 was also the minimum of the earlier K1102–K1120 exposed-point screen. This holdout confirms its cold advantage over K1103 on new points. The separate five-base current-source cold finalist panel is the next cross-family comparison; its selector remains subject to both-stratum local replay and archive binding.
