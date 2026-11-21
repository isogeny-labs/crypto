# H4 n=17 native XOR solver comparison — 2026-11-20 UTC

## Measured result

Preserving each Boolean equation as one native XOR-DIMACS constraint gave
verified first witnesses in **33.39 seconds** on the planted system and
**34.85 seconds** on the ordinary-query system. The matched Tseitin-CNF
encoding was censored at 180 seconds on the planted system and solved the
ordinary query in **155.57 seconds**. The paired ordinary-query solver-time
ratio is `155.5717 / 34.8492 = 4.46` on this local host. One input per arm
does not provide a variance estimate or an isolated-host performance claim.

| Input | Encoding | Variables | CNF clauses | Native XOR rows | Solver time | Result |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Planted | Tseitin CNF | 88,644 | 374,258 | 0 | 180.01 s | Censored at cap |
| Planted | Native XOR | 9,387 | 57,213 | 17 | 33.39 s | Verified witness 148582 |
| Ordinary query | Native XOR | 9,387 | 57,213 | 17 | 34.85 s | Verified witness 189699 |
| Ordinary query | Tseitin CNF | 88,992 | 375,650 | 0 | 155.57 s | Verified witness 13614 |

All systems have exactly six `V³` solutions by exhaustive enumeration.
The planted fixture has zero residual and belongs to its solution set.
Every returned model satisfies all regenerated CNF and XOR constraints;
its 18 input bits have zero residual in the original equations and belong
to the corresponding exact solution set. The independent replay is in
[`audit.py`](audit.py), with output in [`audit.json`](audit.json).

The two encodings share the same 9,369 nonlinear monomial AND gates. The
CNF arm expands parity into binary XOR chains with four clauses per gate;
the native arm passes the 17 original parity rows directly to
CryptoMiniSat. The [solver's documentation](https://github.com/msoos/cryptominisat)
specifies its XOR-DIMACS extension. The specific negative-literal parity
convention was checked on all four two-variable assignments before this
run: `x-1 2 0` accepts equal input bits.

## Protocol and evidence

[`protocol.json`](protocol.json) was frozen before execution with SHA-256
`398833772d0e5bd69e37cbaa75cb03c7fd66581db8b5f45868e713f0a5371b3d`.
It pins the original H4 builder, the prior CNF encoder, CryptoMiniSat
5.14.7, instance index 0, the four-cell order, a 180-second solver cap
per cell, and an 840-second process cap. The checked Sage runtime is in
[`runtime-info.json`](runtime-info.json). The full run exited successfully
in 406.38 seconds, with 422,019,072 bytes maximum resident set size from
`/usr/bin/time -l`.

[`result.json`](result.json) preserves every status, input hash, build and
solver time, witness, and constraint check. The four normalized solver
outputs retain the three complete models; the raw generated `.cnf` files
are reproducible byte-for-byte from [`run.py`](run.py) and are ignored by
Git under the repository's artifact policy. The audit regenerates each
input, checks its SHA-256 against the frozen result, and replays every
available model. The old CNF input hashes agree with the prior H4 SAT
follow-up, so the comparison used exactly the same Boolean systems and
CNF encoding.

Replay from this repository:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/xor_followup_20261120/runtime-info.json
timeout 840 /usr/bin/time -l /Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/xor_followup_20261120/run.py > research/exotic_ecdlp_20261021/H4_prime_descent/xor_followup_20261120/run-output.txt 2>&1
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/xor_followup_20261120/audit.py
```

This is a component-level diagnostic on two fixed `n=17` inputs. The next
experiment should freeze several independent ordinary queries at larger
degrees, count zero-yield and censored attempts, and charge the resulting
PDP costs into relation collection and a same-target end-to-end comparison.
