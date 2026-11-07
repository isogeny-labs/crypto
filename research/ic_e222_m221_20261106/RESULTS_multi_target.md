# Multi-target prime-field index calculus in counted group operations (local, unvalidated)

**Status:** unvalidated (pre-gate). Local, toy sizes (group order 2^20 to 2^24), counted operations; no replay certificate and no validator on these panels. The references are the matched single-target negation walk from `ic rho` (T independent solves) and, added after the first draft, a counted anchored batch rho (`examples/ic_batch_rho.rs`). Not a leaderboard row.

## What is new
A single-target solve pays for the factor-base logarithms every time. With T targets on one curve the logarithms y_i = log_G F_i are computed once, from relations on G alone (a G = +-F_i +-F_j), and each target then needs one relation a G + b Q_t = +-F_i +-F_j with y_i, y_j known, giving x_t = b^-1 (+-y_i +-y_j - a) mod n. Implementation: `examples/ic_multi_target.rs` (does not touch the `ic` binary). It reuses the pair-sum table and the restarted r-adding walk (starts are sums of three of 64 bank points; per-target arms are bank points plus small multiples of Q_t, so a target costs about 40 setup additions). The linear system has two +-1 entries per row, so it is solved by a near-linear signed-graph propagation (`sparse_solve`; first version had a sign error that gave 72/128 correct logs and was caught by the recovered-log check; the fixed version equals the dense Gaussian solver in all tests, using about 2.3k instead of 8.4M modular operations). Group additions are counted exactly in the code; a scalar multiplication is priced at 1.5 bits(n) additions, and every recovered log is verified by one priced scalar multiplication.

## Cost accounting
Counted: factor-base pair-sum table (2 fb^2 additions), the bank, the G-relation walks, per-target setup and walks and verification. Unpriced work (sparse-solve modular operations, one hash lookup per trial, factor-base construction about 4 fb bits) is reported separately and priced at 0, 0.1 and 1 GAE per operation (§5a band); the best fb is chosen per price in the exploratory sweep, and a frozen rule is used in the holdout.

## Exploratory sweep (`results/multi/`, one instance per size, 60 configs, all verified)
Best fb follows fb ~ (p T)^(1/4) (64 to 512). Analysis table `results/multi/analysis.txt`.

## Holdout (`results/multi_holdout/`, 132 configs, all 132 verified)
Frozen before the run (commit ce1ebd4, 03:32:29Z): fb = 2^round(log2((p T)^(1/4))). Instances: `results/instances_vsrho` idx 1 to 11 (the sweep used idx 0), 11 instances per size x 2 seeds = 22 configs per cell. Reference: T independent solves by the matched negation-map rho, S sqrt(n) operations each (S = 1.41, 1.28 interpolated, 1.115 at 20, 22, 24 bits from `ic rho`).

| group bits | T | fb | IC GAE/target (0 / 0.1 / 1) | rho ops/target | IC / rho at 0 [boot95] | at 0.1 | at 1 | cheaper in cfgs (price 1) |
|---|---|---|---|---|---|---|---|---|
| 20 | 128 | 128 | 450 / 470 / 647 | 1,444 | 0.312 [0.310, 0.315] | 0.325 | 0.448 | 22/22 |
| 20 | 1024 | 128 | 152 / 157 / 206 | 1,444 | 0.105 [0.104, 0.106] | 0.109 | 0.143 | 22/22 |
| 22 | 128 | 128 | 773 / 823 / 1,273 | 2,621 | 0.295 [0.290, 0.299] | 0.314 | 0.486 | 22/22 |
| 22 | 1024 | 256 | 259 / 267 / 336 | 2,621 | 0.099 [0.099, 0.100] | 0.102 | 0.128 | 22/22 |
| 24 | 128 | 256 | 1,809 / 1,896 / 2,680 | 4,567 | 0.396 [0.395, 0.399] | 0.415 | 0.587 | 22/22 |
| 24 | 1024 | 256 | 425 / 448 / 652 | 4,567 | 0.093 [0.092, 0.093] | 0.098 | 0.143 | 22/22 |

The ranking against independent matched rho holds at every price in the band (the sweep shows the same for T >= 128 and a loss at T = 16, 1.3x to 4.2x). The narrow intervals are the spread over instances of a nearly deterministic count, not a sampling confidence for other groups.

## What this does not show
- **Not a win over rho in the multi-target setting.** The right reference for many targets is a batch rho (distinguished points, total about S sqrt(n T)); that is not implemented here for prime curves. Against that idealized model (valid for T up to about n^(1/4); an extrapolation beyond) the IC cost per target is 3 to 5x larger (sweep: 2.9x to 5.9x at T >= 128). So the improvement is over the repository's single-target IC solved T times and over independent rho, not over the best known multi-target generic attack; the distance to that is about 3 to 5x.
- The scenario differs from the single-target baseline: at T = 1 the IC arm remains 25x to 53x the matched rho (`RESULTS_vs_rho.md`), and at T = 16 it loses to independent rho.
- Toy sizes only; one rho S per size (22 bits interpolated); group additions are counted by the code, not by an independent harness; field-operation costs inside an addition are not distinguished from inversions; a measured exponent in T was not fitted. Wall time is not used here.
- No isolab-eks run, no validation gate, no pcc claim.


## Measured batch-rho reference (added; supersedes the "3 to 5x" model above)

`examples/ic_batch_rho.rs`: anchored multi-target rho in the same unit (exact group additions, a scalar multiplication priced at 1.5 bits(n) additions, every recovered log verified and priced). Phase 1 stores the x-coordinate of every point visited by restarted r-adding walks on known multiples of G (database of kappa sqrt(n T) points; no distinguished-point overhead; memory unbounded, which favours rho); phase 2 walks each target Q_t through the same walk space until its x-coordinate is in the database (x-only lookups, so P and -P share a key), giving x_t = (+-a' - a) mod n. The rule (kappa in {0.5, 1, 2}; report the best kappa per configuration, which favours rho) was committed with the runner before the run (commit 5ebf6a2). Same instances and seeds as the IC holdout (`results/instances_vsrho` idx 1 to 11, seeds 1, 2); 396 runs, all 396 fully verified. Data `results/batch_rho/`, `analysis.txt`.

| group bits | T | batch rho GAE/target (best kappa) | IC GAE/target (0 / 0.1 / 1) | IC / batch rho at 0 [boot95] | at 0.1 | at 1 | IC cheaper in |
|---|---|---|---|---|---|---|---|
| 20 | 128 | 193 | 450 / 470 / 647 | 2.34 [2.27, 2.38] | 2.44 | 3.36 | 0/22 |
| 20 | 1024 | 87 | 152 / 157 / 206 | 1.75 [1.73, 1.76] | 1.81 | 2.38 | 0/22 |
| 22 | 128 | 339 | 773 / 823 / 1,273 | 2.28 [2.24, 2.34] | 2.43 | 3.75 | 0/22 |
| 22 | 1024 | 142 | 259 / 267 / 336 | 1.84 [1.82, 1.85] | 1.89 | 2.39 | 0/22 |
| 24 | 128 | 640 | 1,809 / 1,896 / 2,680 | 2.84 [2.78, 2.92] | 2.98 | 4.22 | 0/22 |
| 24 | 1024 | 250 | 425 / 448 / 652 | 1.70 [1.68, 1.72] | 1.79 | 2.60 | 0/22 |

**Result: the multi-target IC is 1.7x to 2.8x more expensive than a counted batch rho (4.2x at the high end of the pricing band) in every one of the 132 paired configurations; it is not an improvement over the strongest generic multi-target attack.** The earlier model (S sqrt(n/T), 3 to 5x) was wrong in both directions: it omitted the factor 2 of the two-phase scheme (so it understated rho's cost) and the IC constants. The ratio falls as T grows (about 2.3 to 2.8 at T = 128, about 1.7 to 1.8 at T = 1024), so whether it reaches 1 at larger T is the next test. The improvement that stands is over the repository's single-target IC and over T independent matched-rho solves (previous section), not over batch rho.
