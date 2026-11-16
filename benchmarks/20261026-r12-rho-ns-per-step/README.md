# R12: rho ns/step, ecbench versus fold backend (2026-10-26)

Study (source of truth): [`research/r12_rho_ns_per_step_20261026`](../../research/r12_rho_ns_per_step_20261026/README.md).

Graph: [ns_per_step.svg](ns_per_step.svg) ([PNG](ns_per_step.png), [PDF](ns_per_step.pdf)).

| Pair | ns/step A | ns/step B | B / A (ratio of means) | Losing cells |
|---|---:|---:|---:|---|
| ecbench tuned rho.signed_frobenius vs strong, n = 41 | 95.0 | 519.9 | 5.48 | tuned never faster (0/6) |
| same, n = 53 | 92.2 | 861.5 | 9.34 | 0/6 |
| same, n = 61 | 102.5 | 979.7 | 9.56 | 0/6 |
| N83 worker wide vs fold, training_1 | 201.7 (fold) | 217.7 (wide) | 1.08 | fold ahead on mean; ranges overlap |
| N83 worker wide vs fold, training_2 | 185.4 (fold) | 197.8 (wide) | 1.07 | fold ahead on mean |

Host: Apple arm64, L0 (busy, unpinned): descriptive, not an admitted wall claim.
The audit's 599 versus 240 ns/step was not reproduced; see the study README for
the explanation (backend 1.07 to 1.76x, batched versus per-step inversion 5.5 to
9.6x), the list of affected comparisons (MEASURED-UNMATCHED) and what was not done.
Not admitted to the results table until it has a `VALIDATION.json` (this is a
diagnostic, not a speedup).
