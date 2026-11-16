# Large primes on the Koblitz compact-orbit pipeline, n = 53 and 61 (2026-10-29)

Atlas lead `LEAD-ICB-large-prime-variants`. Protocol: [`PROTOCOL.md`](PROTOCOL.md) (committed before the
sessions finished). Report: [`report.html`](report.html), [`REPORT.pdf`](REPORT.pdf), figure
[`large-prime-s.svg`](large-prime-s.svg) / [`.png`](large-prime-s.png). Evidence: `sessions/`,
`verify/`, `band/`, `results.json`, `validator-report.json`, `replay-certificate.json`; the matching
benchmarks entry is `benchmarks/20261029-icb-large-prime/`.

## Result

The two-large-prime collector (`mitm-frobenius-counted:m=3,large_primes=2,lp_domain_factor=c`, already on
`main`) lowers counted S of the classic single-target compact-orbit pair-table pipeline against the plain
table over the same envelope and against the best plain table in the bracket:

| size | best plain (columns, S) | best large-prime arm (S) | paired ratio | ratio at w = 0 / 0.1 / 1 | arm / rho at w = 0 / 0.1 / 1 |
|:--|:--|:--|:--|:--|:--|
| n = 53 (log2 r 44.3) | 440, 6.38 | `lp2-k40-c10`, 3.86 | 0.604 [0.599, 0.610] | 0.604 / 0.635 / 0.730 | 30 / 31 / 35 |
| n = 61 (log2 r 47.2) | 880, 7.62 | `lp2-k60-c10`, 4.26 | 0.559 [0.535, 0.578] | 0.559 / 0.623 / 0.831 | 47 / 52 / 64 |

One large prime is worse than the plain table (1.47-1.73x). Rho (strong signed-Frobenius) is ahead of every
arm at every weight. This is a constant factor inside the classic pipeline, in counted GAE; it does not touch
the leaderboard's Table A recipe (a different pipeline and unit) and is reported as a component result. Full
findings, limits and the control results are in `report.html`.

## Reproduce

```sh
python3 make_specs.py                      # specs/*.json
./run_ladder.sh 53 1 2; ./run_ladder.sh 61 1 2
ecbench band --dir sessions/n53-s1 sessions/n53-s2 --json band/n53-all.json   # and per session
./run_verify.sh n53-s1 n53-s2 n61-s1 n61-s2
python3 summarize.py && python3 validator.py && python3 make_certificate.py && python3 make_report.py && node render.js
```

Four n = 53 sessions were planned (`specs/n53-s1..s4`); two were run, the others dropped for time and
their specs are kept as written.
