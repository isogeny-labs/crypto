# Public-point rational lift search on two prime reductions

The canonical xedni rational lift search finds the known rational points in both prime reductions at heights 4 and 8. For each of six seeded public points it exhausts height 8,192: 35,658–53,304 exact discriminant tests per point, with no rational lift under the frozen height law. The independent audit checks all ten search outcomes and replays each same-point rho scalar with separate finite-field group arithmetic.

## Frozen experiment

[`protocol.json`](protocol.json) fixes the curve `y² + 94xy + 188y = x³ + 1738x + 3805`, rational `Q=(-4,19)`, reductions `(p,q)=(1531,1553),(2287,2251)`, and the height law `x=a/b²`, `H=max(|a|,b)`. Targets are reduced `Q`, reduced `2Q`, then three SHA-256-seeded multiples per field. Only the public `(x,y)` enters the lift search; fixture scalars are held for replay. The search filters `a/b²` by the public x coordinate, checks the exact integral quadratic discriminant, and accepts the first rational point reducing to the public point. The rho arm starts from the same public point and verifies its recovered scalar.

The protocol and runner were committed and pushed as `88d91149dba` before execution. The checked launcher command was:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_public_target_lift_20261120/audit.json
```

The launcher's runtime receipt, source hashes, fixture rows, every arm row, result digest, and audit result are retained here. The run used an ordinary Darwin arm64 host with 14 visible CPUs; wall times are diagnostic and do not carry an isolation-tier speedup claim. The target point was constructed from a known scalar outside both arm intervals. Lift time includes candidate generation, exact checks, and found-point verification or cap. Rho time includes the walk through scalar verification. These arms have different output types: a rational lift is a point, while rho returns a scalar.

## Results

| p | Target | Public point | Lift status | Height | Exact tests | Lift seconds | Verified rho scalar | Rho ms |
| ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1531 | Q control | (1527,19) | found | 4 | 1 | <0.001 | 1 | 48.871 |
| 1531 | 2Q control | (8,572) | found | 8 | 1 | <0.001 | 2 | 0.923 |
| 1531 | seed 0 | (1002,485) | height cap exhausted | 8192 | 53,304 | 6.957 | 579 | 1.293 |
| 1531 | seed 1 | (1342,39) | height cap exhausted | 8192 | 53,243 | 7.102 | 758 | 0.794 |
| 1531 | seed 2 | (894,686) | height cap exhausted | 8192 | 53,250 | 6.925 | 144 | 0.845 |
| 2287 | Q control | (2283,19) | found | 4 | 1 | <0.001 | 1 | 1.466 |
| 2287 | 2Q control | (8,1328) | found | 8 | 1 | <0.001 | 2 | 1.102 |
| 2287 | seed 0 | (961,1446) | height cap exhausted | 8192 | 35,658 | 7.198 | 1438 | 2.359 |
| 2287 | seed 1 | (331,2157) | height cap exhausted | 8192 | 35,701 | 7.018 | 1613 | 1.498 |
| 2287 | seed 2 | (1680,22) | height cap exhausted | 8192 | 35,673 | 7.252 | 92 | 3.419 |

All ten rho arms finished with independently replayed scalars. Each of six seeded lift arms reached the height cap before the 180-second wall cap. The reported lift wall times measure a capped stage, not a complete scalar recovery interval. [`audit.py`](audit.py) constructs the entire modular-x candidate set, sorts it by `(H,b,a)` independently of the runner's boundary enumeration, checks rational equations exactly using `Fraction`, and checks group/scalar arithmetic directly modulo each prime. [`audit.json`](audit.json) reports `PASS` for all ten fixtures.

## Next proof and experiment step

The two reductions give a finite, reproducible height frontier for the six seeded points. Extend the same public-coordinate protocol to independently selected curves and predeclare a sequence of height caps. Measure lift discovery as a censored stage and, for every found rational point, separately account for the work required to express it in the relation-generating subgroup before treating it as a discrete-log method. A finite exhaustive negative statement for these exact targets and height bound is a candidate for a Lean certificate once a tractable certificate encoding is chosen; this packet records the complete enumerator and audit rather than asserting that formalization is done.
