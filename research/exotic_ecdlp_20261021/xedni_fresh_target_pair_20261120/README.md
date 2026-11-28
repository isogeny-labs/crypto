# Fresh-target rational search and same-point rho

All 18 arms on six frozen, previously unseen public points returned verified
scalars. In this ordinary-host panel, rho's online wall interval is smaller
than the rational search interval on every paired point. The rational search
uses 462–8,578 height-ordered candidate evaluations per target; the
finite-group control visits the identical first-match labels without exact
rational-point construction. A Lean theorem proves their candidate-point
identity in scalar coordinates once the verified relation `R̄=λQ̄` is
available.

## Frozen input and measured pairs

The [protocol](protocol.json) and [runner](run_sage.py) were pushed in
`8c6e5421f5f` before execution. A SHA-256 seed fixes three distinct public
targets on each of the two prime-order reductions. Known scalars construct
the fixture points outside every arm interval; solvers receive the public
point. The arms rotate order across target indices. Each arm has a
120-second cap and retains its own status. `λ=11` and `λ=1238` are verified
setup inputs from the prior collision and radius packets.

| p | q | Target index | Fixture scalar | First matching label index | Rational online ms | Finite control ms | Rho online ms |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1531 | 1553 | 0 | 433 | 8,578 | 10,860.466 | 73.359 | 0.734 |
| 1531 | 1553 | 1 | 133 | 1,558 | 248.363 | 12.180 | 2.034 |
| 1531 | 1553 | 2 | 1300 | 4,579 | 1,935.660 | 36.545 | 0.699 |
| 2287 | 2251 | 0 | 1822 | 650 | 61.253 | 5.666 | 2.436 |
| 2287 | 2251 | 1 | 1070 | 7,369 | 9,474.617 | 67.444 | 0.930 |
| 2287 | 2251 | 2 | 807 | 462 | 39.111 | 3.842 | 0.926 |

Each row's three arms use the **same public point**. The
[raw arm rows](arm-rows.jsonl) retain point coordinates, order, start/stop
timestamps, exact nanosecond intervals, candidate counts, rho walk updates,
collision details, restarts, and status. The [fixture rows](target-fixtures.jsonl)
retain the deterministic input law. All arms are `verified`, with scalar
replay against the public point. The [result](result.json) pins the protocol,
source and raw-row digests, source hash, host, and setup interval. Accepted
[Sage runtime information](sage-runtime-info.json) was saved before the run.

The online interval begins at the first target-dependent candidate or rho
walk computation and ends after scalar recovery and public-point
verification. It includes all failed candidates and rho restarts. Curve,
pairing, and label setup, source-`λ` discovery, fixture generation, and
process startup are outside it. Global curve/pairing setup took 55.832 ms;
cell setup, including label ordering, took 71.292 ms at `p=1531` and
13.584 ms at `p=2287`. Those are reproducibility data, not an assembled cold
total, because this packet does not time `λ` discovery. The panel ran on an
ordinary Darwin ARM64 host without a Tier 0/1 isolation receipt. Its wall
intervals are exploratory; no promoted IC/rho speedup follows from them.

## Independent audit and proof map

The [integer group-law audit](audit.py) independently rebuilds all six
target points, recomputes each first matching coefficient label, and replays
all six deterministic rho collision walks. Its [result](audit-result.json)
is `PASS` for all 18 arm rows and both raw-file digests. The prior
[rational-prefix audit](../xedni_threshold_rational_cost_20261120/audit-result.json)
reconstructed every point through full coverage at these radii, a longer
prefix than any target search here.

[ScalarControl.lean](formal/ScalarControl.lean) proves for every modulus `q`
and scalar coordinates `a,b,λ,G,R` that `R=λG` implies
`aG+bR=(a+λb)G`. The actual elliptic-curve-to-scalar bridge and each `λ`
are checked by Sage and independent group-law replay. The pinned
[Lean build](formal/build.sh) succeeds, and [axiom output](formal/build-output.txt)
lists only `propext` and `Quot.sound`. `lean4checker --fresh` was unavailable
on this host.

The next experiment is a Tier 1 same-point comparison with fresh targets on
an allocated cloud instance, including calibration and the complete
resource-envelope receipt. The current local result also motivates a
formal cost comparison for rational-point arithmetic versus the equivalent
finite-group label scan when `λ` is already available.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_fresh_target_pair_20261120/formal/build.sh
```
