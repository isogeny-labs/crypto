# R4-I1: walk-seed sweep of the round 33 wwalk incumbents

Pre-registered in `../ideas/STRUCTURE-R4.md` (R4-I1) before data. Claim `PCC-20261104-c97d81`, standing `unverified`.

## Why

The `wwalk` table is seeded from `seed=0` and the two seed points' keys, so it is one fixed draw per curve and configuration. The ic_autolab holdouts vary only the probe phase. On the round 33 k = 32 holdout, total GAE minus probes lies between 22,392 and 22,701 across all 96 runs of the selected arm [pcc:unverified]. Choosing among x values therefore also chose among a few table draws.

## What was run

The round 33 incumbents under walk seeds 0 to 15, with 32 fresh public targets per seed and a matched rho-neg arm, one round (`specs/seeds-k32.json`, `specs/seeds-k28.json`). The audit checked 1,088 records and reproduced 8 of 8 replays.

| k | arm | seed-averaged S (w = 0) | 95% interval over seeds | SD | seed 0 | rho-neg | distance |
|--:|:--|--:|:--|--:|--:|--:|--:|
| 32 | `wwalk:x=0.6,g=1,grow=2`, walk probes | 1.781 | [1.602, 1.960] | 0.335 | 1.480 | 0.977 | 1.82x |
| 28 | `wwalk:x=0.9,g=1,grow=2`, stride-generator probes | 1.969 | [1.808, 2.131] | 0.304 | 1.707 | 1.059 | 1.86x |

The pre-registered prediction was 1.80 [1.65, 1.95] at k = 32 and 1.82 [1.66, 1.98] at k = 28. The falsification lines were a mean below 1.62 or 1.64, or an SD below 0.12. Neither was crossed [pcc:unverified].

## Reading

- The round 31 and 33 distances to rho at k = 28 and 32 (1.49x to 1.55x) reflect a favourable table draw. The expected distance of the method is about 1.8x to 1.9x [pcc:unverified].
- The method is still below the coefficient box incumbents it replaced (S 2.52 at k = 32, a deterministic table).
- Holdouts of randomised-table oracles must resample the table. Either mix the run seed into the oracle seed, or average over oracle seeds as done here.
