# Public-point rational lift search on four selected curves

The frozen height search finds all eight rational controls across four independently selected curves, then exhausts height 4,096 on each of twelve seeded public points. Each capped search performs 20,091–20,242 exact square tests. An independent integer audit recounts the curve orders, reproduces every selected target and lift-search status, and replays all twenty same-point rho scalars.

## Method and evidence

[`protocol.json`](protocol.json) fixes the short Weierstrass family `E_t: y²=x³+t*x+1`, its rational point `Q=(0,1)`, the primes 1009 and 1013, and ascending `t=1..64` eligibility testing. For each prime, the first two good-reduction curves with prime group order and Q generating the reduction group are selected. The 63 tested `(p,t)` cells, including unselected ones, are in [`selection-rows.jsonl`](selection-rows.jsonl). Curve selection reads neither lift nor rho outcomes.

For each selected curve, the targets are reduced Q, reduced 2Q, and three distinct SHA-256-seeded public points. The lift arm receives only public coordinates and searches increasing `H=max(|a|,b)` for `x=a/b²`, `y=c/b³`, up to `H=4096`, using modular x filtering and exact integer square tests. Rho solves the same public point with a separately verified scalar. Fixture construction and curve setup are outside both arm timers. These ordinary-host times diagnose the stage work; the lift arm and rho arm return different objects.

Protocol and runner were committed and pushed as `4458ba8f7ce` before measuring. The checked local commands were:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/audit.json
```

## Measured panel

| p | t | q | Q control height | 2Q control height | Seeded lift status | Exact tests per seeded point | Seeded lift seconds | Verified rho arms |
| ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| 1009 | 5 | 1039 | 1 | 25 | 3 height-cap exhausted | 20,123–20,242 | 1.697–1.706 | 5/5 |
| 1009 | 9 | 1033 | 1 | 81 | 3 height-cap exhausted | 20,122–20,125 | 1.723–1.767 | 5/5 |
| 1013 | 38 | 1033 | 1 | 361 | 3 height-cap exhausted | 20,091–20,134 | 1.699–1.716 | 5/5 |
| 1013 | 54 | 977 | 1 | 729 | 3 height-cap exhausted | 20,099–20,122 | 1.676–1.709 | 5/5 |

All twelve seeded lift arms completed the height cap before their 120-second wall caps. Their same-point rho arms each returned a scalar in 0.603–1.578 ms on this host. The raw [`arm-rows.jsonl`](arm-rows.jsonl) preserves every target, order, start/stop interval, status, and work count. The four Q controls had one exact square test each; 2Q controls required 3, 9, 168, and 647 tests. The first rho control includes a 51.267 ms first-use interval, retained in the raw row.

[`audit.py`](audit.py) computes each curve order by an independent Legendre-symbol point count, checks prime-order selection and scalar targets with a separate group law, globally sorts all modular-x candidate triples instead of following the runner's height-boundary loops, and checks found rational coordinates with exact fractions. [`audit.json`](audit.json) records `PASS` for 63 selection rows and twenty targets. The checked Sage runtime and source/output SHA-256 digests are retained in the packet.

## Next experiment

The four selected curves extend the height frontier beyond the earlier one-curve reductions while leaving the height law fixed. A subsequent preregistered cap ladder should estimate the first-hit height distribution with censored targets retained and should measure expression of a found rational lift in the rational relation generators as a separate stage. General coverage or complexity conclusions require that additional distribution and expression work.
