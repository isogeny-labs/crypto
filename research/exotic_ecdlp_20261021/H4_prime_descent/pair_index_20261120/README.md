# H4 group pair-index diagnostic — 2026-11-20 UTC

A group pair index recovered all six ordered decompositions of the fixed
`n=19` planted target and found none for the paired ordinary target. Both
sets agreed exactly with exhaustive evaluation of the original `S_4`
Boolean system. This gives a direct group-law check on the six algebraic
solutions and a measured alternative decomposition component on this input.

| Arm | `S_4` ordered x-triples | Group-realizable x-triples | Lifted factor-base points | Ordered pair entries | Pair-index setup | Target query | Exhaustive `S_4` enumeration |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Planted | 6 | 6 | 119 | 14,161 | 0.221 s | 0.00174 s | 0.625 s |
| Ordinary | 0 | 0 | 119 | 14,161 | 0.208 s | 0.00134 s | 0.626 s |

The target query tests every lifted third point against `R-C` in the pair
table. Each returned ordered x-triple passes the original Boolean equations;
the pair table keys are exact Sage group points. The ordinary query is a
zero-yield cell, not a solved decomposition. The pair table had 7,019
distinct group sums. Setup includes lifting every `x` in the 7-dimensional
subspace and forming every ordered pair sum; target query is charged
separately. Curve/system fixture construction and the independent exhaustive
enumeration are outside those two intervals. Times are one local macOS
diagnostic run, without an isolation gate or uncertainty estimate.

The original H4 `build_instance` function resets Sage's random seed to
`20261022 + 10n + instance_index` and Python's generator to
`20261022*1000 + 10n + instance_index`. The runner extracts the source
functions by AST and changes only the returned dictionary to retain the
already constructed `E` and `R`. It records the curve coefficients, field
modulus, subspace basis, full target coordinates, system digest, source hash,
checked Sage runtime, and separate stage times in
[`result.json`](result.json). [`protocol.json`](protocol.json) was frozen
before the run. A second independent process reproduced every structural
field of the result, including the full target coordinates and both
solution-set counts; its timings were 0.247/0.231 seconds for setup and
0.00198/0.00130 seconds for the planted/ordinary queries. The run used:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/pair_index_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/pair_index_20261120/run.py
```

The indexed group search is a component diagnostic. Its 119 points are
all curve lifts above the selected `x` subspace, with no subgroup filter;
they are not an `IC1` factor base. These results do not price relation
collection, final linear algebra, target descent across a campaign, or a
matched single-target reference. The next discriminating step is to run
frozen natural queries on one fixed curve and base, retaining zero-yield
queries and charging pair-table setup once under the declared workload.
