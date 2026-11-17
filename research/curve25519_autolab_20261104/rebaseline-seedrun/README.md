# Re-baseline: wwalk incumbents with a fresh table per run

Follow-up to `../r4-i1/`. Claim `PCC-20261104-d91a51`, standing `unverified`.

`ecbench` now accepts `seed=run` on an oracle seed: the oracle draws its random table from the run seed, so every run resamples the table as well as the probes. Explicit numeric seeds, and arms without one, keep their fixed table and replay unchanged.

Each size ran 32 fresh public targets x 2 rounds per arm: the incumbent with its fixed table, the same oracle with `seed=run`, an A/A copy, and rho-neg. The audit checked 384 of 384 records at each size and reproduced 8 of 8 replays.

| k | incumbent | fixed table S | seed=run S | rho-neg S | seed=run distance |
|--:|:--|--:|--:|--:|--:|
| 24 | `wwalkGg-x09-n2` | 1.723 | 2.105 | 1.084 | 1.94x |
| 28 | `wwalkGg-x09-n2` | 1.616 | 1.883 | 1.038 | 1.81x |
| 32 | `wwalkG-x06-n2` | 1.606 | 1.842 | 1.010 | 1.82x |

These are S at w = 0. The fixed-table figures differ from round 33 only through new targets; the table is the same draw. The seed=run figures agree with R4-I1's seed averages (1.78 at k = 32, 1.97 at k = 28) [pcc:unverified]. The ledger incumbents at k = 24, 28 and 32 now point at the seed=run arms, with these sessions as evidence, so later rounds compare challengers against the method's expected cost rather than a favourable draw.
