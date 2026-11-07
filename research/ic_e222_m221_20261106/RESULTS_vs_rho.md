# IC walk vs rho: a wall-time "parity" that is an artifact, and the counted-operation comparison (local, unvalidated)

**Status:** unvalidated (pre-gate). Local host. No replay certificate, no validator on this panel, no §5a 0 / 0.1 / 1 GAE band applied. Not a leaderboard row.

## 1. The matched wall-time panel (192 runs, 192 verified) and why it proves nothing about rho

12 fresh W instances per size at 20, 22, 24 and 25 bits (`results/instances_vsrho`, seed `e222m221-vsrho-v1`), solver seeds 1, 2, the walk with fb frozen per size before the run (`results/vsrho_frozen.json`, commit 6c1f309, 02:42:10Z), against the same binary's same-target rho (`pollard_rho_ecdlp`). `run_vs_rho.py`, data `results/vsrho/`.

| group bits | rho median ms | IC walk median ms | paired IC/rho median [bootstrap 95%] | IC faster in |
|---|---|---|---|---|
| 20 | 19.2 | 26.0 | 1.28 [0.92, 1.76] | 8/24 |
| 22 | 76.5 | 77.7 | 1.00 [0.84, 2.27] | 12/24 |
| 24 | 206.7 | 192.7 | 0.99 [0.84, 1.25] | 12/24 |
| 25 | 370.1 | 341.7 | 0.91 [0.82, 1.11] | 13/24 |

Taken at face value this is statistical parity with rho. **It is not, and the earlier lines of this file's predecessors that read "table 4 to 16 times slower than rho" (`RESULTS_scaling.md`) and "walk close to parity" (session log) are both wrong as statements about rho.** Two defects:
- The in-binary reference is a textbook walk: 3 branches, Floyd cycle finding (three point additions per iteration), no negation map, no distinguished points, on `BigUint` arithmetic. Its time scales with a log-log slope of 0.84 against the group order (theory 0.5), which is itself the sign of a weak reference.
- The repository's matched reference (`ic rho`, the tuned negation walk on {P, -P}) is a counted, machine-word implementation. `ic rho --prime-bits 24` runs all three of its walks in about 2.1 ms per run, against about 200 ms for one solve of the in-binary rho. Wall time cannot be compared across the two implementations at all.

Comparisons between IC variants, which all run on the same arithmetic in the same binary (sweep vs hoisted sweep vs table vs walk), are not affected by this; only the IC-vs-rho statements are.

## 2. The comparison in the valid unit (counted group operations)

Matched negation-map rho at sizes near these: S = ops / sqrt(r) = 1.41 at 20 bits and 1.115 at 23.4 bits (`ic rho --prime-bits 20,24 --runs 16`; the 22-bit value 1.28 is interpolated). IC walk additions are counted from the recorded trials of `results/scale_walk` at the best-time fb of each size: trials x (1 + restart cost / 32) + 2 fb^2 (table build) + the 32 arm points. Table `results/vsrho/gae_comparison.txt`:

| group bits | IC walk fb | trials | est. group additions (lower bound) | matched rho ops (S sqrt r) | IC / rho |
|---|---|---|---|---|---|
| 20 | 64 | 9,219 | 36,617 | 1,444 | 25x |
| 22 | 64 | 32,131 | 108,706 | 2,622 | 41x |
| 24 | 128 | 63,097 | 240,140 | 4,568 | 53x |

The IC figure is a lower bound: linear algebra (fb^3 modular multiplications mod n), the factor-base construction and hashing are left unpriced, and §5a prices such unpriced native work at 0, 0.1 and 1 GAE per operation, which can only raise the IC arm. **In the valid unit the IC walk is at least 25 to 53 times more expensive than the matched rho and the gap widens with size, consistent with an IC exponent above 1/2 (table/walk around p^(2/3) against rho p^(1/2)).** No claim of parity or crossover with rho is supported.

## 3. What stands and what does not

Stands (same-arithmetic comparisons, frozen tune/eval, validator-reviewed for the table): the table over the hoisted sweep (0.446 solver / 0.664 process vs the tuned hoisted baseline), and the restarted walk over the table (0.249 solver / 0.720 process; 0.124 / 0.520 vs the best hoisted sweep). Does not stand: any statement that the table or walk are near, at or within a small factor of rho. The distance remaining to the matched rho, in counted operations, is about 25x to 53x at 20 to 24 bits and growing.

## 4. Error record

The first matched panel was started with a rho arm that also ran the full shipped-default IC at fb 8 (minutes per run at 24 to 25 bits); noticed from the log rate, killed, and fixed by starving the IC part (1 trial, 1 attempt) so that only `rho.wall_ms` is read. The wall-time comparison itself was then set aside as above after reading `pollard_rho_ecdlp` and the `ic rho` timings.
