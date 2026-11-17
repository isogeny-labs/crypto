# N83 external native-XOR Gauss settings

The frozen four-call panel completed on the original unpinned N83
known-witness XCNF. All calls reached `UNKNOWN_solver_cap` at 100,000
conflicts. Solver wall times were 7.652 s and 7.723 s for the two defaults,
7.685 s with wider matrix limits, and 7.744 s with forced Gauss. The
[independent audit](gauss_audit.py) binds the sealed runner source, exact
XCNF, solver binary, command options, caps, statuses and raw outputs.

The [protocol](gauss_protocol.json) was pushed at
`acf525e936a7fc87b428eb9a1800152536e1e144` before the four calls.
It uses local arm64 macOS, CryptoMiniSat 5.14.7, one thread, 100,000
conflicts and a 60-second solver wall cap. Wall time includes XCNF loading;
export and independent replay are outside that interval. The input SHA-256
is `c2cc36eee7a72dbc583a217a0d42d02c74d5d3fde241536941b6614fc565b81e`.

| Call order | Matrix setting | Wall ms | Status |
| --- | --- | ---: | --- |
| default A | library defaults | 7,652.159 | `UNKNOWN_solver_cap` |
| wide | `maxmatrixrows=10000`, `maxmatrixcols=50000` | 7,684.744 | `UNKNOWN_solver_cap` |
| forced | wide plus `autodisablegauss=0`, `gaussusefulcutoff=0` | 7,744.455 | `UNKNOWN_solver_cap` |
| default B | library defaults | 7,723.227 | `UNKNOWN_solver_cap` |

The two default calls bracket the tuned calls within 71.068 ms. Their
outcomes and the tuned calls' outcomes agree at the declared cap. The
earlier exploratory forced-Gauss 5,000-conflict call also returned
`INDETERMINATE`; its raw stdout and stderr are retained, while its wall time
was not recorded. All result rows are stage diagnostics on an unisolated
host. They supply no natural relation, useful-rank row, or complete DLP
timing; `candidate_id` and `selected_best_total_runtime` remain null.

Recheck from the repository root with:

```sh
python3 research/n83_chain_xcnf_control_20261019/gauss_audit.py
```
