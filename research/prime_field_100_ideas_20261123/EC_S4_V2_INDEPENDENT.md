# Independent readback of #24 S4 V2 emitted rows

Date: 2026-11-24 (America/Los_Angeles). Conductor task `T-12212`. This is a separate verifier for the already emitted [`S4_MATCHED_V2_STDOUT.txt`](S4_MATCHED_V2_STDOUT.txt), not a new S4 search or a modification of the producer. The producer stdout SHA-256 at readback was `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f`; its source SHA-256 was `2c6c948e86b2225e4c341117582ff984bd4e74ee52c8cec9d3cfe4d5cb627657`. Producer files were left untouched.

The distinct [GP verifier](EC_S4_V2_INDEPENDENT_GP.gp) read all 80 output lines. It parsed the 16 printed factor-base points and both target points, checked each on `E/F_1009: y²=x³+x+14`, rebuilt each 17-entry coefficient row from the printed triple indices and signs, and checked every one of the 57 signed point sums against its printed target. It then recomputed the rank of each emitted-row matrix over `F_1013` using a modular Gaussian-elimination routine in this verifier, rather than the producer's `matrank` call. The first signed witness examined in each arm was `z=0: [3,5,12], [-1,1,-1]` and `z=320: [1,5,10], [1,1,-1]`; all subsequent emitted witnesses passed the same independent check.

Command from repository root:

```sh
timeout 60 gp -q < research/prime_field_100_ideas_20261123/EC_S4_V2_INDEPENDENT_GP.gp > research/prime_field_100_ideas_20261123/EC_S4_V2_INDEPENDENT.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_S4_V2_INDEPENDENT.stderr.txt
verify_status=$?
printf '%s\n' "$verify_status" > research/prime_field_100_ideas_20261123/EC_S4_V2_INDEPENDENT.exit.txt
```

The [raw stdout](EC_S4_V2_INDEPENDENT.stdout.txt) reports `z=0: 24 rows, rank 4` and `z=320: 33 rows, rank 6`, matching the producer's summaries. [Stderr](EC_S4_V2_INDEPENDENT.stderr.txt) is empty and the [exit file](EC_S4_V2_INDEPENDENT.exit.txt) contains `0`. GP was PARI/GP 2.17.3 on arm64. No timing or resource advantage is inferred.

| Artifact | SHA-256 |
|---|---|
| `EC_S4_V2_INDEPENDENT_GP.gp` | `c666d3d1c470804a80cdef272a4d5a2d827d8dca7914cb7d8177fbef23a6bb94` |
| `EC_S4_V2_INDEPENDENT.stdout.txt` | `377b6674fb82b2657dbab88aca4a08bf7f999bcbdbbb649e91c4e2601dd956c4` |
| `EC_S4_V2_INDEPENDENT.stderr.txt` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `EC_S4_V2_INDEPENDENT.exit.txt` | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |

This readback checks the printed rows, their signed group witnesses, and their modular ranks. It does not independently recompute the S4 polynomial, the 4096 polynomial evaluations per target, zero-set completeness, the coefficient support counts, Horner proxies, or a full relation-collection or target solve. The source output and this verifier still lack a signed repository proof-carrying verdict.
