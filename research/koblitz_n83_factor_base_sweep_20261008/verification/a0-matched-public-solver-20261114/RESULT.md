# N83 a=0 same-point solver pair — 2026-11-14 UTC

The locked comparison measured one ordinary public query on each of two
factor-base constructions. Both searches reached the one-million-conflict
limit with terminal `INDETERMINATE` status. The retained raw transcripts,
commands, host samples and receipts passed the separate byte and terminal
audit in `evidence/audit.json`. The original solver stdout is retained as
deterministic gzip in each arm's evidence directory, and the audit checks its
decompressed bytes against the run receipt. [pcc:unverified]

The exact primary model is `icv1-f2m83-tm6151469093347-debefd74`:
`y² + xy = x³ + 1` over the polynomial basis
`F₂[z]/(z⁸³ + z⁴⁵ + z² + z + 1)`, subgroup order
`2417851639230796216685689`, cofactor 4. The query was sealed public
fixture 1, point
`(5809312440517013698248638,3568442427236746854022671)`.
Both arms used one thread on Darwin arm64, CryptoMiniSat 5.14.7 binary
SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`,
seed `20261114`, VSIDS, one-million-conflict and 600-second solver limits,
and a 660-second process guard. K2048 used the frozen 25,000-column Gaussian
ceiling; D18 used its frozen default ceiling. [pcc:unverified]

| Factor base and PDP | Usable points | Folded columns | Terminal | Conflicts | Solver process wall / CPU | Peak RSS | Preflight load 1m |
| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| Full D18, ordered m4 | 5,418,738 | 32,643 | `INDETERMINATE` | 1,000,001 | 427.082 / 425.508 s | 522,158,080 B | 12.43, 12.05, 11.44 |
| K2048, m5 wide XOR | 339,968 | 2,048 | `INDETERMINATE` | 1,000,002 | 156.853 / 156.098 s | 394,444,800 B | 3.87, 4.41, 4.76 |

The times are process-wall stage observations on a shared Mac host with
different background load. The two methods have different summand counts
and Gaussian column settings; the table records the whole paired solver
policies. Each raw receipt keeps `verified_relation`, `novel_rank`,
`single_target_online_ms`, and `selected_best_total_runtime` as null.
The next experiment needs relation-producing ordinary queries, checked
relation rows and novel rank across the common corpus, followed by complete
same-target IC/rho timing in an admitted performance tier. [pcc:unverified]
