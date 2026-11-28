# Five distinct target IC/rho panel

The frozen n23 candidate recovered five distinct subgroup target scalars in five paired trials. Sage's 20-partition rho independently recovered each same-point scalar. The median paired online ratio was **15.97** (`rho / IC`), with per-pair ratios from 10.35 to 36.87. These are exploratory timings from one ordinary macOS host; all five pre/post calibration ratios passed the frozen 1.25 gate, but the host has no Tier 0 or Tier 1 allocation receipt.

| Target index | Order | IC online (ms) | Rho online (ms) | Rho/IC | Pre/post calibration ratio | Result |
| ---: | --- | ---: | ---: | ---: | ---: | --- |
| 0 | IC, rho | 1.223 | 14.695 | 12.02 | 1.103 | both verified |
| 1 | rho, IC | 0.981 | 20.764 | 21.16 | 1.031 | both verified |
| 2 | IC, rho | 1.113 | 17.782 | 15.97 | 1.022 | both verified |
| 3 | rho, IC | 1.108 | 40.839 | 36.87 | 1.114 | both verified |
| 4 | IC, rho | 0.944 | 9.764 | 10.35 | 1.036 | both verified |

The exact five targets, scalar generation law, workload IDs, and exclusions from prior relation queries are in [`fixtures.json`](fixtures.json). The complete candidate is `IC1N23Ch4randfb250PDP3mitmRCsampleLAgaussTDdirectISO0hee3873de904e`; its curve ID is `EC1N23Ch4randh9d96eba41b3a`, with subgroup order 232,891. Candidate and source identities, arm order, caps, calibration rule, and claim boundary were committed before measurement in `af76ab583d3` and are recorded in [`protocol.json`](protocol.json). The checked Sage runtime receipt was saved before the timed run.

Each IC online interval begins with the public point and a ready factor-base pair index and verified factor-base logs. It includes the candidate's exclusive query, point decomposition, relation check, descent, and scalar recovery check phases. Each rho online interval includes its target-dependent walk and scalar replay. The shared reconstruction and log replay cost was **963.339 ms**, outside both online intervals; it is a warm precomputation-state restoration, not a new cold candidate preparation. Fixture construction and the 512-addition pre/post calibrations were also outside both online intervals. The original candidate packet retains the separate complete cold preparation receipt.

[`result.json`](result.json) retains every pair, timing phase, cap, calibration, status, and scalar certificate. [`runs.jsonl`](runs.jsonl) contains five single-target v2 contract rows. The independent [`verify.py`](verify.py) rebuilds the curve, base, and logs; checks frozen hashes and workload identities; replays the IC witness equations and rho scalars; and checks phase sums and calibration. [`validation.json`](validation.json) records its result and the exact bootstrap interval for the five paired ratios. The bootstrap median-ratio 95% interval is [10.35, 36.87]; with five observations it has limited precision.

Reproduce the audit with the repository's checked Sage launcher:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/local_panel_20261120/verify.py
python3 -c 'import json,sys,pathlib; sys.path.insert(0,"/Volumes/SSD990/cryptanalysis/experiments/ic-candidate-catalog"); import analyze; p=pathlib.Path("research/exotic_ecdlp_20261021/H4_prime_descent/local_panel_20261120/runs.jsonl"); rows=[json.loads(s) for s in p.read_text().splitlines()]; [analyze.validate_run(r) for r in rows]; print(len(rows), "v2 rows passed")'
```

The prior candidate packet maps its pair-lookup and cofactor-transport results to checked Lean declarations. This panel adds measurements and witness replay, not a new mathematical theorem. The next measurement is an isolated Tier 1 same-target panel with the same candidate, fixture law, alternating order, and calibration gate.
