# Scaling of the pair-sum table and the hoisted sweep, with a rho reference (exploratory, local, unvalidated)

**Status:** exploratory (not frozen, 3 instances per size, seed 1), unvalidated (pre-gate). Local unisolated host, solver stage timer. No replay certificate, no §5a 0 / 0.1 / 1 GAE band. Not a leaderboard row.

**Design.** W instances of 18 to 24 bits (`results/instances_scale`, indices 0 to 2, generator seed `e222m221-scale-v1`, never used before). `run_scale.py`: hoisted sweep (extra 2) at fb 128, 256, 512; table (extra 2) at fb 64, 128, 256, 512; `run_scale_ext.py`: hoisted sweep at fb 1024 for sizes 21 to 24 (the first grid put the sweep's optimum on its edge at six of seven sizes; with fb 1024 added the optimum is interior, 256 or 512, at sizes 21 to 24; at 18 to 20 the edge value 512 was not re-checked). The `p` cap of `solve-prime-file` was raised to 4e7 for this. Rho reference: the same binary's same-target Pollard rho (`--rho-steps 5000000`), wall time, median of 3. 159 solver runs, all verified; all rho runs verified. Data: `results/scale/`, `results/scale_ext/` (`analysis.txt`, `rho_comparison.txt`).

| group bits | rho ms | best table ms (fb) | table / rho | best hoisted sweep ms (fb) | sweep / rho |
|---|---|---|---|---|---|
| 18 | 4.5 | 70 (128) | 15.6 | 188 (512) | 42.1 |
| 20 | 42.5 | 175 (128) | 4.1 | 1161 (512) | 27.3 |
| 22 | 73.6 | 490 (256) | 6.7 | 4464 (256) | 60.6 |
| 24 | 198.8 | 1406 (256) | 7.1 | 19977 (512) | 100.5 |

Fitted slope of log2(time) vs group bits: table 0.72, hoisted sweep 1.11 (sweep slope inflated by its grid cap at 512 to 1024), rho 0.86 (3 instances per size; rho's run length is random, so this slope is noisy and the theoretical value is 0.5).

**Reading.**
- The table's advantage over the best sweep grows with size (0.37, 0.24, 0.15, 0.18, 0.11, 0.08, 0.07 for 18 to 24 bits), consistent with a lower exponent: with a table, collection costs about p/fb trials-equivalents and the build about fb^2, so the best case is near p^(2/3) (measured 0.72 on log2 of the group order), against about p^(3/4) for the sweep (dense linear algebra, fb^3, adds to both).
- The table is still 4 to 16 times slower than rho in wall time on the same instances (n = 3 per size, no §5a pricing of unpriced work, and a plain walk without the negation map). An exponent above 1/2 means the gap to rho widens with size; nothing here suggests a crossover. The IC baseline's distance to rho is reduced (sweep/rho 27 to 100 vs table/rho 4 to 16), not closed.
- Within the measured range this is the first row in which the pair-sum table reaches the same order of magnitude as rho (4.1x at 20 bits).

**Limits.** 3 instances per size, one seed, rho from a single run per instance; the rho slope and the ratios are noisy. The table's memory (about 2 fb^2 entries) and build make it fb-limited at larger p. No isolab-eks run, no validation gate, no matched-rho band. The goal is not met.
