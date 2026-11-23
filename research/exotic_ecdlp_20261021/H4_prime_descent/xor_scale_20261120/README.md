# H4 n=19 native XOR scale check — 2026-11-20 UTC

## Measured result

All four frozen `n=19`, `m=3`, `dim(V)=7` solver cells reached their
300-second caps. The planted system has six exact `V³` decompositions;
neither encoding returned a witness. The ordinary-query system has zero
exact `V³` decompositions; neither encoding returned an UNSAT certificate
within the cap. This is a censored scale result for both encodings, so it
does not extend the `n=17` native-XOR timing ratio to `n=19`.

| Input | Encoding | Exact solutions | Variables | CNF clauses | Native XOR rows | Solver status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Planted | Native XOR | 6 | 22,120 | 137,641 | 19 | Censored at 300.015 s |
| Planted | Tseitin CNF | 6 | 232,130 | 977,700 | 0 | Censored at 300.011 s |
| Ordinary query | Tseitin CNF | 0 | 232,354 | 978,596 | 0 | Censored at 300.014 s |
| Ordinary query | Native XOR | 0 | 22,120 | 137,641 | 19 | Censored at 300.016 s |

The `n=17` follow-up showed verified first witnesses for both native-XOR
inputs in about 34 seconds; this `n=19` run shows that the same encoding
and solver binary did not finish within 300 seconds on the next frozen
degree. It sets an observed cap-dependent boundary, not a complexity
exponent. The zero-solution ordinary query is especially informative:
exhaustive `V³` enumeration certifies the count, while the SAT processes
remain censored as solver measurements.

## Frozen protocol and replay

[`protocol.json`](protocol.json) fixed both input laws, instance index 0,
the order `native XOR / CNF / CNF / native XOR`, 300-second solver caps,
and a 1,440-second process cap before execution. Its SHA-256 is
`dece0cec7f66ba57d2fcc79841910d73c224085734e07e10bb7fe31aa8fb9844`.
The original H4 builder, previous CNF encoder, previous native-XOR encoder,
and CryptoMiniSat 5.14.7 are pinned by SHA-256 in that protocol.
[`runtime-info.json`](runtime-info.json) records the checked Sage runtime.
The process exited successfully in 1,204.19 seconds real, with
1,145,176,064 bytes maximum resident set size from `/usr/bin/time -l`,
inside the 12-GiB observation envelope.

[`result.json`](result.json) preserves each censored arm, input size and
digest, exact solution count, build and solver timing, and cumulative child
peak RSS. [`audit.py`](audit.py) independently regenerates every input,
checks its hash, re-enumerates `V³`, and verifies the planted residual;
[`audit.json`](audit.json) records the successful replay. The normalized
solver outputs in Git retain the emitted terminal text. Full raw solver
output files are local and ignored; no model was emitted by any cell.

Replay with the accepted launcher:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/xor_scale_20261120/runtime-info.json
timeout 1440 /usr/bin/time -l /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/xor_scale_20261120/run.py > research/exotic_ecdlp_20261021/H4_prime_descent/xor_scale_20261120/run-output.txt 2>&1
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/xor_scale_20261120/audit.py
```

The next algorithmic step is to change the decomposition method at this
scale—such as a geometry-aware pair index or an algebraic reduction—and
compare its full verified attempt cost on the same frozen ordinary queries.
The current SAT encoding remains a measured component with a sharp
resource-capped transition from `n=17` to `n=19`.
