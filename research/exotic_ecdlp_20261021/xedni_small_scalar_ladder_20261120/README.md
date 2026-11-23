# Small-scalar public-point lift height ladder

The frozen 48-target ladder locates a sharp measured frontier in the four selected curves. The public-coordinate search finds `kQ` for `k=1,2` on every curve and also finds `3Q` on `E_5` reduced at 1009, at height 1,064. Its other 39 searches exhaust height 4,096. Every same-point rho arm returns and verifies its scalar. An independent rational/finite-group replay confirms every target, known rational height, and height-search status.

## Frozen inputs and boundary

[`protocol.json`](protocol.json) pins the four prior curves `(p,t,q)=(1009,5,1039),(1009,9,1033),(1013,38,1033),(1013,54,977)` and scalars `k=1..12`. On each `E_t: y²=x³+tx+1`, `Q=(0,1)`. Fixture construction computes both the public reduced point `T=kQbar` and the rational point `kQ` before timing. The lift arm sees only `T`'s public coordinates; the separate known-rational height records `H(kQ)=max(|a|,b)` for reduced `x(kQ)=a/b²`. The pinned search tests rational `x=a/b²,y=c/b³` in height, denominator, then numerator order, with full public-point verification. A same-point rho arm recovers and verifies the scalar. The arms alternate order by odd/even `k`.

The protocol and runner were pushed as `07eff44199a` before running. Checked local commands:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_small_scalar_ladder_20261120/audit.json
```

The run is an ordinary-host stage diagnostic. Lift time includes all target-conditioned candidate and exact-square work through a found point or cap; rho time includes the walk through scalar verification. These outputs have distinct meanings, so the two intervals are retained side by side rather than divided into a speedup ratio.

## Results

| p | t | q | H(Q) | H(2Q) | H(3Q) | First lift frontier in k=1..12 | Found/capped | Rho verified |
| ---: | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: |
| 1009 | 5 | 1039 | 1 | 25 | 1,064 | k=1,2,3 found; k=4..12 capped | 3/9 | 12/12 |
| 1009 | 9 | 1033 | 1 | 81 | 5,896 | k=1,2 found; k=3..12 capped | 2/10 | 12/12 |
| 1013 | 38 | 1033 | 1 | 361 | 27,440 | k=1,2 found; k=3..12 capped | 2/10 | 12/12 |
| 1013 | 54 | 977 | 1 | 729 | 78,736 | k=1,2 found; k=3..12 capped | 2/10 | 12/12 |

The `3Q` rational heights on the last three curves exceed the cap, and the exhaustive public-coordinate search finds no other rational lift under that cap for those public points. On the first curve, the first found `3Q` lift is the known rational point. The minimum known-rational height at `k=4` is already 284,225; the search nonetheless checked all height-eligible candidates rather than treating the height of `kQ` as a lower bound on other rational lifts.

[`arm-rows.jsonl`](arm-rows.jsonl) retains all 96 arms, including start/stop intervals, exact test counts, statuses, and rho certificates. [`target-fixtures.jsonl`](target-fixtures.jsonl) retains all 48 public points and known rational x heights. The 39 capped lift arms each completed height 4,096 before the 120-second wall cap. The independent [`audit.py`](audit.py) uses exact `Fraction` arithmetic for rational multiples, a separate finite-field group law for public targets and rho scalars, and a globally sorted modular-x candidate list for the lift search; [`audit.json`](audit.json) reports `PASS` for all 48 targets. The source and output digests and accepted Sage runtime are retained with the packet.

## Next step

The first uncaptured scalar is `k=4` on the `t=5` curve and `k=3` on the others. A preregistered cap expansion can distinguish the height of the known rational `kQ` from the first height of *any* rational lift of the same public point. The recorded cap statuses remain censored until that expansion is run. For any found lift, subsequent work must separately measure expression in rational relation generators before it can enter a discrete-log pipeline.
