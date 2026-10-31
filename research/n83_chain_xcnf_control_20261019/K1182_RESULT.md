# Primary N83 K1182/866 same-query XCNF gate

The higher-coverage K1182/866 domain exports the first ordinary public query
to a 13,197,849-byte native-XOR XCNF. CryptoMiniSat 5.14.7 reached
`UNKNOWN_solver_cap` at 5,000, 100,000, and 1,000,000 conflicts. The
[independent audit](k1182_audit.py) checks the pre-call protocol, retained
source and input hashes, the query in the original curve group, all XCNF
rows, and every raw solver receipt. The result measures one point-decomposition
access path; useful relation yield and rank have no observed values at these
caps.

The [protocol](k1182_protocol.json) was pushed in
`b1482bac70f3fc261c5caa76d4d00a6d02248865` before the solver calls.
It holds the query fixed against the K600 Gray/hash panel: public target
`(1834913121178197242856238, 8797964190732846213981377)`, query
coefficients `a=1833078656608906180176350` and
`b=163089659398503559150141`, and resulting curve point
`(3547082699798970859000627, 6877207017147849837699235)`. The curve
is `y² + xy = x³ + 1` over GF(2^83), polynomial basis with modulus
`z^83 + z^45 + z² + z + 1`; the subgroup order is
`2417851639230796216685689`.

The first three x-coordinate summands use 1,182 signed-Frobenius orbit
representatives and the last two use 866; these unfold to 196,212 and
143,756 x-coordinate values before sign folding. The exporter uses the
frozen `identity_mask=0`, unsigned first-pair order, and four chained S3
equations. The formula has 101,636 variables, 714,072 CNF clauses, and
4,980 native XOR rows. Its SHA-256 is
`7354890e494a30e2ac3259eae8876fdb0235284a5808a04b366a3f7492ebde20`.
The compressed XCNF and exporter receipt are in `evidence/k1182/`.

| Conflict cap | Solver wall ms | Result |
| ---: | ---: | --- |
| 5,000 | 1,282.647 | `UNKNOWN_solver_cap` |
| 100,000 | 17,926.475 | `UNKNOWN_solver_cap` |
| 1,000,000 | 120,893.437 | `UNKNOWN_solver_cap` |

Each call used one thread on the local arm64 macOS host, with wall caps of
60, 120, and 180 seconds respectively. Solver wall time includes input
loading; the separate formula export took 376.738 ms. The host is an L0
stage-diagnostic environment under the benchmark tier contract. All three
first-attempt stdout, stderr, commands, binary digests, and resource
receipts are retained. `candidate_id` and `selected_best_total_runtime`
remain null because the observed panel does not supply natural relations,
useful rank, final matrix solving, or one-target online timing.

The next discriminating step is an access algorithm that resolves an
ordinary query at this same point under a sealed resource cap, followed by
verified natural relation yield and rank over independent frozen queries.
Only then can the complete factor-base pipeline be compared with its
same-point rho reference in a Tier 1 or Tier 0 environment.

From the repository root, replay the archived input and receipts with:

```sh
python3 research/n83_chain_xcnf_control_20261019/k1182_audit.py
```
