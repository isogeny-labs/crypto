# H4 n=17 complete SAT solver follow-up — 2026-11-19

## Measured result

A Tseitin CNF encoding of the frozen `n=17`, `m=3`, `dim(V)=6` Boolean
decomposition system yielded one independently verified ordinary-query
solution in **147.97 seconds of solver time**. The solver's 88,992-variable,
375,650-clause assignment satisfies every CNF clause, its 18 input bits
give zero residual in the original equations, and it belongs to the
exactly enumerated six-element solution set. The planted control had six
solutions and a valid planted witness by exact enumeration, but the SAT
solver did not find one within its 180-second cap. These are two fixed
inputs and a first-witness stage measurement, not a full relation-yield
or target-recovery measurement.

| Arm | Exact `V³` solutions | CNF variables | CNF clauses | CNF build | Solver | Result |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Planted | 6 | 88,644 | 374,258 | 0.400 s | 180.01 s | Censored at solver deadline |
| Ordinary query | 6 | 88,992 | 375,650 | 0.404 s | 147.97 s | Verified witness `13614` |

The nonzero solution counts are correctness controls, not natural relation
rates. A complete first-witness result at this degree shows that the
previous Macaulay profile's early row cap was not a solver impossibility
result. It does not fix the solving degree, provide a scaling law to
`n=131`, or establish a useful cost per verified relation.

## Frozen method and evidence

[`protocol.json`](protocol.json) fixes the original builder SHA-256
`46fb1a8a311f36907427ac35b40066d88cc0948c7f8b522fbc87c8338682e11a`,
instance index 0, both arm laws, a 180-second solver deadline per arm,
and a 480-second process deadline. Its SHA-256 is
`4c9bdaf37566e56dcfff2a14d09f663a1da6397b6d71c3e93973493645b2b6e6`.
[`run.py`](run.py) imports the original system builder and exact `V³`
enumerator. It introduces one shared AND gate per distinct nonlinear
monomial and four clauses per binary XOR gate. The gate truth table was
checked on all eight assignments before execution.

The local checked Sage runtime is in [`runtime-info.json`](runtime-info.json).
CryptoMiniSat 5.14.7 was pinned by binary SHA-256
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`.
The exact generated inputs are retained as [`planted.cnf.gz`](planted.cnf.gz)
and [`random.cnf.gz`](random.cnf.gz), with uncompressed SHA-256 digests in
[`result.json`](result.json). The ordinary-query assignment was checked
against all 375,650 clauses independently of the solver return code.
[`audit.py`](audit.py) replays the source and solver hashes, exact solution
counts, original-equation witness residual, and every CNF clause;
[`audit.json`](audit.json) records its successful output.
[`run-output.txt`](run-output.txt) records the 330.88-second process receipt;
the compressed per-arm solver outputs retain emitted text and the random
arm's model. The planted timeout status is in `result.json`.

Replay from this repository with the checked launcher:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/sat_followup_20261119/runtime-info.json
timeout 480 /usr/bin/time -l /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/sat_followup_20261119/run.py > research/exotic_ecdlp_20261021/H4_prime_descent/sat_followup_20261119/run-output.txt 2>&1
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/sat_followup_20261119/audit.py
```

This was an ordinary local macOS diagnostic, not a CPU isolation tier or
same-point IC/rho comparison. The process stayed below the protocol's
12-GiB memory observation limit; the largest child-process resident set
reported by the runner was 378,830,848 bytes. The next cost experiment
should freeze several ordinary-query instances and a stronger SAT or
algebraic encoding, retain every zero-yield and censored arm, and measure
verified relation yield and total charged work at increasing degrees.
