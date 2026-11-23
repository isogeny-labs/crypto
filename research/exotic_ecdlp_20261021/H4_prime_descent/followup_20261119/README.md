# H4 n=13 Macaulay profile extension — 2026-11-19

## Measured result

Raising the row cap from 9,000 to 30,000 completed degree 10 on both
deterministic `n=13`, `m=3`, subspace-dimension-5 inputs. The planted arm
gave quotient dimension `Q_10=5,706` against three enumerated solutions;
the ordinary-query arm gave `Q_10=5,705` against zero enumerated solutions.
Both arms stopped at the 180-second per-arm deadline before degree 11.
Under the frozen projected-rank method, the solving-degree diagnostic is
therefore `d_solv>=11` for each of these two inputs. The first-fall degree
was seven in both arms. This extends the earlier degree-9 profile and
leaves the H4 cost and solving-degree decision inconclusive.

| Arm | Exact solutions in `V^3` | `D=9`: rows, rank, Q | `D=10`: rows, rank, Q | `D=10` profile time | Status |
| --- | ---: | --- | --- | ---: | --- |
| Planted | 3 | 7,488; 7,488; 20,336 | 25,233; 25,121; 5,706 | 196.26 s | Censored by arm deadline after D10 |
| Ordinary query | 0 | 7,488; 7,488; 20,336 | 25,233; 25,122; 5,705 | 208.35 s | Censored by arm deadline after D10 |

The planted witness has zero equation residual and is in the exact
enumerated solution set. The ordinary-query instance has no solution in
the searched `V^3`; it is retained as a zero-yield arm. Enumeration and
the original Boolean system builder are reused without changing the
frozen source. The rank projection is probabilistic, so these dimensions
are measured diagnostics of that projection, not certified ranks of the
unprojected Macaulay matrices.

## Protocol, resources, and replay

[`protocol.json`](protocol.json) was frozen before execution (SHA-256
`729071687d1285c2ce6e97843f2efff0d9c90c6f09d39438b075ba2d65314f93`).
It fixes instance index 5, seed formula, two arms, source hash, cap,
deadlines, controls, and claim boundary. [`run.py`](run.py) imports the
unchanged builder and profile functions from the frozen H4 runner at
SHA-256 `46fb1a8a311f36907427ac35b40066d88cc0948c7f8b522fbc87c8338682e11a`.
[`runtime-info.json`](runtime-info.json) records the accepted checked Sage
runtime before the workload.

Replay from this repository with the checked launcher:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/followup_20261119/runtime-info.json
timeout 480 /usr/bin/time -l /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/followup_20261119/run.py > research/exotic_ecdlp_20261021/H4_prime_descent/followup_20261119/run-output.txt 2>&1
```

The one-process local macOS run exited successfully in 445.82 s real,
with maximum resident set size 6,503,514,112 bytes from `/usr/bin/time -l`.
The requested 12-GiB `RLIMIT_AS` was rejected by the host and recorded as
`address_space_limit_enforced: false`; the process deadline and observed
memory remained within the stated envelope. The per-arm deadline is
checked between profile steps, so the degree-10 rank operations overran
180 s before each arm was marked censored. The resulting timings are
algorithmic diagnostics on this ordinary host, not paired performance
measurements.

[`result.json`](result.json) is the checkpointed structured result;
[`events.jsonl`](events.jsonl) preserves each completed profile degree;
[`run-output.txt`](run-output.txt) contains the process receipt. The next
proof or experiment step is to certify the rank/quotient transition for
these inputs or run a bounded complete solver at `n=13`, then price any
decomposition improvement in the full relation and target pipeline.
