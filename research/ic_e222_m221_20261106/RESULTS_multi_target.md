# Multi-target prime-field index calculus in counted group operations (local, unvalidated)

**Status:** unvalidated (pre-gate). Local, toy sizes (group order 2^20 to 2^24), counted operations; no replay certificate and no validator on these panels; the rho reference is the matched single-target negation walk from `ic rho`, not a batch rho. Not a leaderboard row.

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
