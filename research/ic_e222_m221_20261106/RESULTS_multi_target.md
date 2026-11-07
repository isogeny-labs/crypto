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

## Larger T: no crossover (24 and 25-bit groups, valid regime only)

`run_T_ext.py` (`results/T_ext/`): T = 4096 and 16384, 4 instances x seed 1 per cell, same frozen IC fb rule and the same kappa rule for batch rho. A first attempt including 20 and 22-bit groups hung and was stopped: the reference builds its database with restarted walks that reach only a limited set of distinct points, so a database larger than that never fills (20-bit, T = 16384, kappa = 2); the database must stay a small fraction (about 5%) of the x-classes, which restricts the valid cells to 24 and 25-bit groups (kappa = 2 at T = 16384 timed out in all 8 attempts there and rho used the best kappa that completed). 16 IC runs verified; 40/48 rho runs verified and 8 timeouts.

| group bits | T | IC fb | batch rho GAE/target (best kappa) | IC @0 / @0.1 / @1 | IC / rho at 0 | at 0.1 | at 1 | IC cheaper in |
|---|---|---|---|---|---|---|---|---|
| 24 | 4096 | 512 | 144 | 252 / 257 / 308 | 1.75 | 1.79 | 2.14 | 0/4 |
| 24 | 16384 | 512 | 93 | 149 / 153 / 188 | 1.60 | 1.64 | 2.02 | 0/4 |
| 25 | 4096 | 512 | 190 | 297 / 307 / 395 | 1.56 | 1.61 | 2.07 | 0/4 |
| 25 | 16384 | 1024 | 119 | 228 / 231 / 255 | 1.93 | 1.95 | 2.15 | 0/4 |

The ratio does not keep falling with T: about 1.7 (T = 1024), 1.6 to 1.75 (4096), 1.6 to 1.9 (16384). Four instances per cell and one seed: the cells are not ranked against each other. The multi-target IC stays 1.6x to 2.2x above the counted batch rho in the valid regime.

## v2: cheaper table build (measured holdout)

The pair-sum table was built with 4 group additions per pair; F_i + F_j and -F_i - F_j (and F_i - F_j and -F_i + F_j) are negatives of each other, with the same x and flipped y, and negation is free, so 2 additions per pair give all four sign combinations (table cost m^2 instead of 2m(m+1)). Relations, trial counts, determined columns and recovered logs are identical (spot-checked: W20-2, W22-3, W24-5; old build still available with `MT_FULL_TABLE=1`). Full rerun of the holdout (`run_multi_holdout_v2.py`, `results/multi_holdout_v2/`, same instances, seeds and frozen fb rule; 132 configs, 132 verified), compared with the recorded batch-rho costs:

| group bits | T | IC v1 @0 | IC v2 @0 | v2 / v1 | batch rho (best kappa) | IC v2 / rho at 0 [boot95] | at 0.1 | at 1 | IC cheaper in |
|---|---|---|---|---|---|---|---|---|---|
| 20 | 128 | 450 | 320 | 0.711 | 193 | 1.67 [1.61, 1.69] | 1.77 | 2.69 | 0/22 |
| 20 | 1024 | 152 | 135 | 0.893 | 87 | 1.56 [1.54, 1.57] | 1.63 | 2.19 | 0/22 |
| 22 | 128 | 773 | 643 | 0.832 | 339 | 1.89 [1.86, 1.94] | 2.04 | 3.36 | 0/22 |
| 22 | 1024 | 259 | 195 | 0.751 | 142 | 1.38 [1.37, 1.39] | 1.44 | 1.93 | 0/22 |
| 24 | 128 | 1809 | 1293 | 0.715 | 640 | 2.04 [1.99, 2.09] | 2.18 | 3.42 | 0/22 |
| 24 | 1024 | 425 | 360 | 0.848 | 250 | 1.44 [1.43, 1.46] | 1.53 | 2.35 | 0/22 |

A measured 10% to 29% reduction in the multi-target IC's counted additions (a real operation saving, not an accounting change), narrowing the distance to counted batch rho to 1.4x to 2.0x at price 0 and 1.9x to 3.4x at price 1. Still not cheaper than batch rho in any configuration. The batch-rho reference was built the same way (x-only lookups; negation free) from the start, so v2 removes an IC-only inefficiency.

## v3: pure-G arms with b = 1, and a pricing fix (symmetric §5a pricing)

**Change.** A target walk does not need arms shifted by multiples of Q_t: if every step adds a multiple of G, the relation is a G + 1 Q_t = +-F_i +-F_j with b fixed at 1, the start carries Q_t, and the per-target table of Q multiples (7 additions) and the 32 per-target arm additions disappear. Holdout `run_multi_holdout_v3.py` (`results/multi_holdout_v3/`, same instances, seeds and frozen fb rule as v1 and v2; 132 configs, all 132 verified).

**Pricing fix.** The earlier tables priced the IC arm's unpriced operations (sparse-solve operations, one hash lookup per trial, factor-base construction) at 0.1 and 1 GAE but left the batch rho's hash operations at 0, although it does one hash insert or lookup per walk step. The symmetric version below prices both arms' unpriced operations at the same 0 / 0.1 / 1 GAE (`results/multi_holdout_v3/analysis_symmetric_pricing.txt`, which also reprices v1 and v2). It moves the price-1 column down by about 0.3 to 0.9 in the ratio and leaves price 0 unchanged.

Symmetric pricing, IC / batch rho (median [bootstrap 95%] at price 0; configs where IC is cheaper out of 22):

| version | group bits | T | at 0 | at 0.1 | at 1 | cheaper at 0 / 0.1 / 1 |
|---|---|---|---|---|---|---|
| v1 | 24 | 1024 | 1.70 [1.68, 1.72] | 1.65 | 1.43 | 0 / 0 / 0 |
| v2 | 24 | 1024 | 1.44 [1.43, 1.46] | 1.41 | 1.29 | 0 / 0 / 0 |
| v3 | 20 | 128 | 1.47 [1.42, 1.50] | 1.45 | 1.36 | 0 / 0 / 0 |
| v3 | 20 | 1024 | 1.12 [1.12, 1.13] | 1.11 | 1.08 | 0 / 0 / 0 |
| v3 | 22 | 128 | 1.81 [1.76, 1.86] | 1.80 | 1.76 | 0 / 0 / 0 |
| v3 | 22 | 1024 | 1.12 [1.11, 1.13] | 1.10 | 0.97 | 0 / 0 / 21 |
| v3 | 24 | 128 | 1.99 [1.92, 2.02] | 1.95 | 1.76 | 0 / 0 / 0 |
| v3 | 24 | 1024 | 1.28 [1.26, 1.31] | 1.27 | 1.20 | 0 / 0 / 0 |

v3 saves a further 3% to 28% (median ratio v3 / v2 of the counted additions 0.72 to 0.97; the saving is largest at T = 1024). IC is not cheaper than batch rho at any price in any cell except the single (22 bits, T = 1024, price 1) cell, where the order does not hold across the band (0 of 22 at prices 0 and 0.1) and therefore is not a ranking under §5a. The distance to batch rho is 1.1x to 2.0x at price 0 and shrinks with T. Not an improvement over batch rho.

## v3 at large T: statistical near-parity with batch rho, unranked under §5a (22 paired configs per cell)

Rule fixed before the run (commit 1c7e5d7, 04:18:38Z): `run_large_T_holdout.py`, IC v3 with the frozen fb rule fb = 2^round(log2((pT)^(1/4))); batch rho with kappa in {0.5, 0.7, 1.0} (0.7 added after a kappa scan, `results/large_T_holdout`, showed rho's minimum at 0.5 to 0.7, the earlier {0.5, 1, 2} grid having under-tuned rho by up to 5%; see the note below the table); best kappa per configuration at each price; valid cells only (24 and 25-bit groups, database <= ~5% of the x-classes); instances_vsrho idx 1 to 11 x seeds 1, 2. 352 runs (88 IC + 264 rho), all 352 fully verified. Pricing is symmetric (IC: sparse-solve operations + one hash per trial + factor-base build; rho: one hash per walk step; both at the same 0 / 0.1 / 1 GAE). Data `results/large_T_holdout/`, `analysis.txt`.

| group bits | T | IC fb | IC GAE/target at 0 | rho at 0 | IC / rho at 0 [boot95] | at 0.1 | at 1 | IC cheaper in (0 / 0.1 / 1) |
|---|---|---|---|---|---|---|---|---|
| 24 | 4096 | 512 | 151 | 142 | 1.057 [1.052, 1.062] | 1.023 | 0.847 [0.841, 0.853] | 0 / 0 / 22 of 22 |
| 24 | 16384 | 512 | 96 | 93 | 1.027 [1.022, 1.031] | 1.012 | 0.923 [0.917, 0.928] | 0 / 0 / 22 of 22 |
| 25 | 4096 | 512 | 198 | 189 | 1.047 [1.045, 1.049] | 1.021 | 0.887 [0.885, 0.889] | 0 / 0 / 22 of 22 |
| 25 | 16384 | 1024 | 127 | 119 | 1.072 [1.071, 1.074] | 1.028 | 0.788 [0.787, 0.790] | 0 / 0 / 22 of 22 |

(Correction of the wording in the paragraph above: the kappa scan shows the earlier rho grid {0.5, 1, 2} was slightly too coarse for rho, so earlier IC/rho ratios were up to 5% lower than the best-tuned rho would give; the table here includes kappa = 0.7 and is the fair version.)

**Reading.** At large T the multi-target IC v3 costs 3% to 7% more than the counted batch rho when unpriced work is free, 1% to 3% more at 0.1 GAE, and 8% to 21% less at 1 GAE, where it wins in every configuration. The order therefore does not hold across the §5a band and **no ranking is claimed**; the correct statement is that the two are within about 10% of each other in counted cost in this regime. The price-1 result arises because the batch rho does about one hash operation per walk step (as many as its additions), whereas the IC performs one per trial plus a small amount of other work, and the IC's trial count is lower; a hash operation costs far less than an elliptic-curve addition in practice, which makes the realistic price much closer to 0 than to 1.

**This is not yet an improvement over the strongest attack.** The cost is at parity, not below it at the realistic price; group sizes are toy (2^24 to 2^25), T is large (4096 to 16384) and the comparison is against a memory-unbounded rho reference that I wrote; both arms are counted by my own code (no independent counting harness, no validator on these panels); one rho family only. The distance to batch rho in this regime has been reduced from 1.7x to 2.8x (v1) to 1.0x to 1.1x (v3).


## CORRECTION after the independent validator (2026-11-07, `results/multi_holdout_v3/VALIDATOR_REPORT.md`): the price-1 "win" and "unranked" claims are withdrawn

Disposition: the row-level measurements are validated (the large-T headline reproduces exactly; 176 configurations replayed with identical counters, including a fresh build of HEAD; an instrumented copy of the library matches the examples' counters in 36 of 36 runs; 80,000 recovered target logs and 6,753 factor-base logs verified by independent code with 0 failures; the sparse-solver sign handling is correct; all 48 instances checked independently). The headline as worded gets **insufficient_evidence and overstates IC's position**. What changes:

1. **The price-1 "win" and "unranked under the band" were a ledger artifact, and I withdraw them.** The IC's pair-sum table does about 2 m^2 hash inserts (524k to 2.1M per run) that my unpriced counter did not include, while I charged rho one hash operation per walk step. In the 25-bit, T = 16384 cell the omitted inserts equal 101% of the IC's counted additions; the sentence "the IC performs one per trial plus a small amount of other work" was wrong. With the table inserts counted (the validator's ledgers B, C, D) the IC / rho ratios are, at price 0 / 0.1 / 1:

| cell | as committed (A) | + table inserts (B) | + exact rho hash count (C) | + per-step exponent update on both arms (D) |
|---|---|---|---|---|
| 24 b, T = 4096 | 1.057 / 1.023 / 0.847 | 1.057 / 1.107 / 1.370 | 1.057 / 1.110 / 1.391 | 1.057 / 1.068 / 1.113 |
| 24 b, T = 16384 | 1.027 / 1.012 / 0.923 | 1.027 / 1.044 / 1.141 | 1.027 / 1.046 / 1.151 | 1.027 / 1.028 / 1.030 |
| 25 b, T = 4096 | 1.047 / 1.021 / 0.887 | 1.047 / 1.084 / 1.271 | 1.047 / 1.087 / 1.291 | 1.047 / 1.055 / 1.084 |
| 25 b, T = 16384 | 1.072 / 1.028 / 0.788 | 1.072 / 1.130 / 1.445 | 1.072 / 1.131 / 1.459 | 1.072 / 1.083 / 1.124 |

   In ledgers B, C and D the IC is cheaper in **0 of 22 configurations at every price in every cell**. Only a hypothetical x-only IC table (not implemented) keeps the flip. The correct statement: the multi-target IC v3 costs about 3% to 7% more than the anchored batch rho at price 0 and 3% to 46% more across the band when its table inserts are counted; it is not cheaper than rho anywhere.
2. **My batch rho is not the strongest generic reference.** The validator computed that a sequential-database anchored rho would be 7% to 12% cheaper (merged random-walk steps waste 5% to 11%), and measured a **multi-target baby-step giant-step (BSGS) that is 25% to 32% cheaper than my batch rho**, with every target verified by its own counter. Against that, IC v3 is **1.38x to 1.55x** (price 0). "Within about 10% of a counted batch rho" is true only for "the anchored random-walk rho as implemented" at prices 0 and 0.1. A crude collision-based distinguished-point model came out worse than the anchored scheme, so there is no evidence that a distinguished-point rho does better at these T.
3. **Smaller corrections accepted:** the kappa = 0.7 gain is at most 3.9% per configuration (cell medians at most 1.2%), not "up to 5%"; the 22-bit matched S (1.28) in the older independent-rho tables is stale and the cross-regime sentence "from 1.7x to 2.8x to 1.0x to 1.1x" compares different T regimes and should not be read as a single trend; IC wastes 2 counted additions per target on a discarded first restart and carries a dead `b` update (removing them would lower the ratio by 0.010 to 0.021); the factor-base-build charge `4 m pbits` is a hand estimate; the rho hash proxy overstates its phase-1 inserts by about 7%.
4. **Disclosures:** v2 and v3 were designed after seeing the v1 and batch-rho results on the same instances, so they are post-selected variants, not holdouts; the large-T panel overlaps the earlier T_ext_v3 and kappa-scan instances in 4 of 22 configurations per cell (the subsets differ by at most 0.007, against IC); the frozen fb rule is about 10% above the best size at 25 bits, T = 16384; rows carry no commit, binary hash or environment; the kappa-scan raw data and the analysis scripts that produced the committed tables are not in the repository.

**Net statement of what the multi-target work shows.** The counted multi-target IC is a real improvement over the repository's single-target IC and over T independent matched-rho solves; it is 3% to 7% (price 0) above an anchored random-walk batch rho at large T and about 1.4x to 1.55x above a multi-target BSGS, and it is not cheaper than a competent generic multi-target attack under any pricing in the committed data. It is at best a tie within the accuracy of the counters, not an improvement over the strongest attack.

## Final multi-target comparison: complete ledger, strongest generic arm (BSGS added), reproducible script

`examples/ic_multi_bsgs.rs` (counted multi-target baby-step giant-step; x-only baby table of M = mfactor sqrt(n T / 4) entries, giant step S = 2M + 1; every recovered log verified and priced) with its large-T runner (`run_bsgs_large_T.py`, mfactor in {0.7, 1.0, 1.4} fixed in commit 51b2643, 05:04:17Z, before the rows existed; 264 runs, all 264 verified; `results/bsgs_large_T/`). `analyze_large_T.py` (committed; `results/bsgs_large_T/analysis.txt`) reproduces every ratio from the raw rows with the **complete, symmetric ledger**: IC unpriced = 2 m^2 table inserts + one hash per trial + sparse-solve operations + 4 m pbits (hand estimate for the factor-base build); rho unpriced = exact database-phase steps + target steps (the identity adds_db = 64 ceil(1.5 bits) + 2 ceil(steps/32) + steps is solved exactly); BSGS unpriced = baby inserts + giant lookups. Best kappa / mfactor per configuration at each price (favours the generic arms). Medians over 22 paired configurations per cell:

| group bits | T | price | IC / rho | IC / BSGS | BSGS / rho | IC / best generic [boot95] | IC cheaper |
|---|---|---|---|---|---|---|---|
| 24 | 4096 | 0 / 0.1 / 1 | 1.057 / 1.110 / 1.391 | 1.498 / 1.580 / 2.025 | 0.705 / 0.702 / 0.687 | 1.498 [1.493, 1.509] / 1.580 / 2.025 | 0/22 at all prices |
| 24 | 16384 | 0 / 0.1 / 1 | 1.027 / 1.046 / 1.151 | 1.405 / 1.441 / 1.663 | 0.729 / 0.723 / 0.691 | 1.405 [1.402, 1.414] / 1.441 / 1.663 | 0/22 |
| 25 | 4096 | 0 / 0.1 / 1 | 1.047 / 1.087 / 1.291 | 1.531 / 1.594 / 1.922 | 0.684 / 0.682 / 0.671 | 1.531 [1.528, 1.539] / 1.594 / 1.922 | 0/22 |
| 25 | 16384 | 0 / 0.1 / 1 | 1.072 / 1.131 / 1.459 | 1.532 / 1.628 / 2.187 | 0.699 / 0.694 / 0.667 | 1.532 [1.531, 1.534] / 1.628 / 2.187 | 0/22 |

**Result: against the strongest counted generic multi-target attack (here BSGS), the multi-target IC v3 costs 1.40x to 1.53x at price 0 and up to 2.19x at price 1, and is cheaper in none of 22 configurations in any cell at any price.** This closes the "near-parity" reading: parity held only against the weaker anchored rho.

**Symmetry lead (j = 0 curves, a = 0): not pursued, by a cost model.** With the order-6 automorphism group, the IC's pair-sum table has 6 sums per pair (3 m^2 classes out of n/6), yield per trial rises 9x while the table build cost rises 3x, giving an amortized-cost factor of about 1/sqrt(3); BSGS and rho gain the full 1/sqrt(6) from the same symmetry (Wiener-Zuccherato, Gallant-Lambert-Vanstone). The IC/generic ratio would therefore get worse, from about 1.4x to about 2x. Not implemented; the model is an estimate, not a measurement.
