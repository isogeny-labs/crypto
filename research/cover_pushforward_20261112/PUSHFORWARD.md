# Cover push-forward: what a bielliptic cover gives on the embedded elliptic subgroup

Protocol (frozen before data, with two dated amendments): [PROTOCOL.md](PROTOCOL.md), hashes in `frozen.sha256`.
Code: `src/cryptanalysis/chained_transfer/pushforward.rs`, `src/bin/cover_pushforward.rs`.
Data: `pushforward.json` (primary run, 3 seeds per prime), `pushforward_15seeds.json` (supplementary, 15 seeds), `run.log`, `pushforward.svg`.
Bench record: `benchmarks/records/20261112T1550Z-cover-pushforward.json` (left open).
Every number below is a counted group operation or a count from those files and carries [pcc:unverified]. Counts are deterministic and exact; no timing claim is made.

## Verdict against the frozen rule

**The cover supplies no logarithm on the embedded elliptic subgroup and no factor base smaller than the group.** [pcc:unverified]
All three conditions of the frozen rule hold at all 7 primes (211 to 12841):
(1) 37,866 relations were pushed through `phi1_*` and `phi2_*`, 0 were invalid on `E1` and 0 on `E2`, and 0 pushed `E2` relations involve the target; (2) `|B cap <G_E>| / r` lies between 0.4071 and 0.5505, never below 0.4; (3) IC on `J(C)`, IC on `E1` over `B` with the same stop rule, and the generic two-term IC on `E1` over `B_r` each cost more than the matched rho on `E1` at every prime and at all three prices of the unpriced linear algebra (gap ratios 1.24 to 43,658 depending on arm and price, tables below). [pcc:unverified]

The owner's sentence that "IC on `J(C)` should be exactly two-term IC on `E1` over `B`" is half right and half wrong, and the wrong half is the sharper result: the pushed relation is not an independent two-term decomposition. It is the degenerate `[2]Q = Q + Q`, so the genus-2 index calculus on the pulled-back instance is a collision search on `E1` that discards half its trials. [pcc:unverified]

## The mechanism, stated precisely

Notation: `C : y^2 = F(x^2)`, `phi1(x, y) = (x^2, y)`, `iota(x, y) = (-x, y)`, `G = phi1^*(G_E)`, `T = phi1^*(T_E)`, `B = phi1(affine degree-1 places of C)`.

1. **`B` is the half of `E1` with square abscissa.** A point `(X, Y)` of `E1` has a rational preimage exactly when `X` is a square in `F_p`. So `|B|` is the number of affine points of `E1` with `X` a square or zero, about `#E1 / 2`. Each image has two places over it, `x` and `-x`, except `X = 0`, where `x = 0` is one place (ramified). Measured: `|B| / #E1` between 0.4707 and 0.5367 over the 7 primes, the count of `B` from the places of `C` equal to the count from `E1` alone at every prime, fibre sizes only 2 and 1, with exactly 2 images of fibre size 1 (the two points with `X = 0`) at every prime. [pcc:unverified]
2. **The reduced class of `[alpha]G + [beta]T` is a fibre.** `Bielliptic::pull(Q)` returns the divisor `P + iota(P)` of the two preimages of `Q`. The reduced Mumford representative of a degree-2 class is unique, and `phi1^*` is a homomorphism, so `[alpha]G + [beta]T = phi1^*(Q)` with `Q = [alpha]G_E + [beta]T_E` is that fibre. It splits into degree-1 places exactly when `X(Q)` is a nonzero square (probability about 1/2), and then its two places `P, iota(P)` have the same image `phi1(P) = phi1(iota(P)) = Q`. Measured: 37,866 of 37,866 relations collected by the genus-2 IC are of this fibre type (primary run), and 189,330 of 189,330 in the 15-seed run. [pcc:unverified]
3. **Pushed through `phi1`, the relation is `[2]Q = Q + Q`.** The pushed relation `phi1(P1) + phi1(P2) = [2 alpha]G_E + [2 beta]T_E` holds in every case (0 invalid in 37,866 and in 189,330), but its two terms are the same point of `B`. It carries one factor-base element, not two.
4. **The matrix is a collision table.** Every J(C) row has two entries of equal sign, on the columns of `x` and `-x`. The columns actually used are exactly twice the columns of the native `E1` arm (58 against 29 at p = 211 and 1,608 against 804 at p = 12841, medians; the factor 2 holds at every seed of both runs), and a row can only combine with another row on the same `+-Q`. The elimination returns the log through a repeated `X(Q)`, that is a birthday collision on `<G_E>`. The cover contributes the acceptance test "`X(Q)` is a square", which discards about half of the trials. [pcc:unverified]
5. **Through `phi2` the pushed relation is trivial.** `phi2_* phi1^* = 0`: for every relation `phi2_*(G)` and `phi2_*(T)` are the identity, so the pushed `E2` relation reads `sum = O` whatever `alpha, beta` are (0 of 37,866 rows have a non-identity `phi2_*(G)` or `phi2_*(T)`; the two places of a row never have equal `phi2` images, 0 of 37,866). It holds (0 invalid) and says nothing about the target. [pcc:unverified]

No step computes a logarithm on the embedded subgroup other than by finding two equal multiples `Q = +-Q'` inside `<G_E>` itself, which needs no cover. The control arm (A6) makes this explicit: the same stream on `E1` with no filter finds the same collisions at or before the cover arms (below).

## Results

Primary run, 3 seeds per prime, seed 0 reproduces the `results_v3.json` rows (IC on J(C) group ops 3,419 and 8,475 at p = 211, 419; rho 428 and 357); the copy of the relation collector reproduces the library's `genus2_index_calculus` exactly (ops, trials, relations, recovered log, LA multiplications) at the 6 primes up to 6421 where it was cross-checked. All 7 x 3 x 7 arm results were certified by an independent double-and-add. [pcc:unverified]

### Factor base and pushed relations (all cells [pcc:unverified])

| p | r | #E1 | abs B | B / #E1 | B∩<G_E> / r | places per image 2 : 1 | X=0 images | pushed rows valid E1 | valid E2 | E2 rows carrying target | fibre-type rows |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 211 | 109 | 218 | 117 | 0.5367 | 0.5505 | 115 : 2 | 2 | 372 / 372 | 372 / 372 | 0 | 372 / 372 |
| 419 | 113 | 452 | 223 | 0.4934 | 0.4071 | 221 : 2 | 2 | 693 / 693 | 693 / 693 | 0 | 693 / 693 |
| 821 | 71 | 852 | 401 | 0.4707 | 0.4789 | 399 : 2 | 2 | 1227 / 1227 | 1227 / 1227 | 0 | 1227 / 1227 |
| 1619 | 401 | 1604 | 773 | 0.4819 | 0.4638 | 771 : 2 | 2 | 2340 / 2340 | 2340 / 2340 | 0 | 2340 / 2340 |
| 3203 | 313 | 3130 | 1535 | 0.4904 | 0.5112 | 1533 : 2 | 2 | 4626 / 4626 | 4626 / 4626 | 0 | 4626 / 4626 |
| 6421 | 3191 | 6382 | 3141 | 0.4922 | 0.4951 | 3139 : 2 | 2 | 9447 / 9447 | 9447 / 9447 | 0 | 9447 / 9447 |
| 12841 | 3169 | 12676 | 6379 | 0.5032 | 0.5080 | 6377 : 2 | 2 | 19161 / 19161 | 19161 / 19161 | 0 | 19161 / 19161 |

### Comparison per prime (medians over 3 seeds, group operations; all cells [pcc:unverified])

IC J(C) and IC E1 fibre use the same random stream and the same stop rule (relations = J(C)'s). The two-term arm builds `B_r` by `r - 2` counted additions (in parentheses) and is charged one counted subtraction per candidate.

| p | r | rho E1 | IC J(C) ops | trials | rel | trials/rel | IC E1 fibre (same stream) ops | IC E1 two-term ops (base build) | trials/rel | rel | cols |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 211 | 109 | 441 | 3354 | 220 | 124 | 1.77 | 3354 | 761 (107) | 1.00 | 38 | 30 |
| 419 | 113 | 393 | 8694 | 567 | 231 | 2.45 | 8694 | 679 (111) | 1.00 | 31 | 23 |
| 821 | 71 | 377 | 11353 | 866 | 409 | 2.12 | 11353 | 469 (69) | 1.00 | 25 | 17 |
| 1619 | 401 | 485 | 34521 | 1653 | 780 | 2.12 | 34521 | 2754 (399) | 1.01 | 101 | 93 |
| 3203 | 313 | 614 | 60059 | 3062 | 1542 | 1.99 | 60059 | 2194 (311) | 1.00 | 88 | 80 |
| 6421 | 3191 | 963 | 188130 | 6331 | 3149 | 2.01 | 188130 | 28546 (3189) | 1.00 | 798 | 790 |
| 12841 | 3169 | 942 | 372157 | 12528 | 6387 | 1.96 | 372157 | 28897 (3167) | 1.00 | 813 | 805 |

### Gap to matched rho at the three prices of linear algebra (median total / rho, w = 0 / 0.1 / 1 GAE per `Z/r` multiplication; all cells [pcc:unverified])

| p | IC J(C) | IC E1 fibre | IC E1 two-term |
|--:|--:|--:|--:|
| 211 | 7.61/10.92/40.79 | 7.61/8.48/16.32 | 1.73/2.63/10.37 |
| 419 | 23.74/37.72/156.75 | 23.74/25.36/39.19 | 1.73/2.43/8.79 |
| 821 | 30.50/73.76/467.20 | 30.50/32.54/50.90 | 1.24/1.68/5.55 |
| 1619 | 70.50/194.98/1315.29 | 70.50/85.78/223.29 | 5.68/16.49/113.80 |
| 3203 | 97.13/482.88/3954.64 | 97.13/117.47/300.55 | 3.62/8.48/56.07 |
| 6421 | 198.21/1225.97/10475.79 | 198.21/451.96/2735.72 | 29.47/446.60/4200.83 |
| 12841 | 393.59/4720.07/43658.40 | 393.59/940.08/5858.48 | 30.37/485.52/4585.04 |

The gap is the distance remaining. In the primary run no arm is below rho at any price in any prime. In the 15-seed run the two-term arm was below rho on the same seed in 1 of 15 seeds at p = 821 (472 against 475 on that seed), and the full-stop arms in 0 of 105. [pcc:unverified]

### Stopping at the first collision (supplementary run, 15 seeds; group operations; all cells [pcc:unverified])

This is the J(C) relation list (equivalently the native `E1` fibre arm; the two have identical group-operation counts at every seed) read at the first repeated `X(Q)`, against the control that uses no cover.

| p | r | rho E1 median (range) | J(C) IC stopped at first collision median (range), seeds below rho | E1 birthday control median (range), seeds below rho |
|--:|--:|--:|--:|--:|
| 211 | 109 | 425 (310-516) | 221 (47-548), 13/15 | 149 (47-280), 15/15 |
| 419 | 113 | 417 (327-519) | 231 (45-491), 14/15 | 147 (45-261), 15/15 |
| 821 | 71 | 308 (240-475) | 114 (40-250), 15/15 | 102 (40-218), 15/15 |
| 1619 | 401 | 559 (382-949) | 543 (81-1512), 7/15 | 448 (81-665), 13/15 |
| 3203 | 313 | 517 (384-888) | 524 (111-728), 7/15 | 325 (111-644), 11/15 |
| 6421 | 3191 | 925 (690-1970) | 2254 (652-4694), 2/15 | 1644 (413-3090), 3/15 |
| 12841 | 3169 | 876 (584-2649) | 1699 (751-5953), 3/15 | 1176 (587-2929), 6/15 |

The cover arm is at or above the control in every seed by construction (it accepts a subset of the control's trials). At the three smallest primes both are below the median rho (the cover arm below rho on 13, 14 and 15 of 15 seeds, the control on 15 of 15 at each). At p = 6421 (r = 3191) the cover arm is above rho on 13 of 15 seeds and the control on 12 of 15; at p = 12841 (r = 3169) 12 of 15 and 9 of 15. [pcc:unverified] Rho's counted cost contains a fixed start-up (sixteen jump points, each built from two scalar multiplications; not measured separately here), which is the likely reason a birthday search wins at small `r`; the trend in `r` (birthday search about `sqrt(r)` trials of two scalar multiplications each, rho about `sqrt(r)` steps of one addition) is the expected one, not a property of the cover. As a measured gap to the IC baseline on `J(C)`, stopping at the first collision costs between 0.46% (p = 12841) and 6.33% (p = 211) of the operations of the full relation set (median over 15 seeds). It uses no genus-2 structure beyond the acceptance filter, and the control without the filter is never worse. [pcc:unverified]

Local wall-clock ratio of one Cantor addition to one elliptic addition in this implementation (unisolated host, no timing claim): 15.8 to 24.5 over the 7 primes. [pcc:unverified] The J(C) and E1-fibre arms have identical group-operation counts, so on an equal-operation basis `E1` is the cheaper place to run the same search. The transfer (cover, `phi^*`, instance) is a one-time cost of 0.3 to 8.9 ms per curve and is in no total.

## Prediction against outcome

| id | frozen prediction (probability) | outcome |
|:--|:--|:--|
| P1 | `|B|/#E1` in [0.45, 0.55] (0.90); `|B cap <G_E>|/r` in [0.40, 0.60] (0.85) | held: 0.4707 to 0.5367 and 0.4071 to 0.5505 |
| P2 | two places per image except `X = 0` (0.97) | held at all 7 primes |
| P3 | pushed relations valid on `E1` and on `E2` (0.98, 0.97) | held: 0 invalid in 37,866 and in 189,330 |
| P4 | `phi2_*` of generator and target are the identity, 0 pushed `E2` rows carry the target (0.98) | held: 0 rows |
| P5 | every genus-2 IC relation is fibre type (0.95) | held: 100% |
| P6 | J(C) IC and two-term E1 IC have the same trials per relation and relation count (0.15) | did not hold, as expected: trials per relation 1.77 to 2.45 against 1.00 to 1.01, relations 3.3 to 17.5 times as many on J(C) |
| P7 | A1 and A4 accept identical trials (0.93) | held at every seed of both runs, and identical group operations |
| P8 | full-stop arms above rho everywhere and at all prices (0.92); A4 first collision below rho at some cell (0.25) | first held on the primary run; second occurred (primary-run median below rho at p = 211 to 3203) |
| P9 | the two-term base build alone exceeds rho in at least one cell (0.60) | held at p = 6421 (3,189 against 963) and 12841 (3,167 against 942) |
| A6 | control first collision below the cover arm's at every cell (0.85) | at or below in all 7 cells, strictly below in 3 of 7 medians on the primary run (equal at 211, 1619, 6421, 12841 where the filter did not delay the first collision), strictly below in 7 of 7 on the 15-seed run; judged held on the weak reading and not on the strict wording of the frozen text |

All values [pcc:unverified].

## What a cover would have to do differently

The owner's rule is that a shortcut must say how it computes logarithms on the embedded elliptic subgroup. For the degree-2 bielliptic covers measured here the answer is: it does not, because the relations it produces on `J(C)` are images of single points of `<G_E>`, and the only information they contain is which multiples of `G_E` have a square abscissa. To help, a cover has to supply all of:

1. an explicit, evaluable map from the target's group into a place where relations can be found (here `phi^*`, supplied, and one-time);
2. a target-side factor base of size well below `r` (the bar: about `r^(1/2)` times a polylog, so that relation collection with a constant number of group operations per relation stays below the `sqrt(r)` of rho), on which relations are found at a constant rate;
3. relations whose terms are independent elements of that base. Here the base is half the group (`|B cap <G_E>| / r` about 0.5) and the two terms coincide.

A smaller Prym dimension does not enter: `E1` carries the whole discrete-log instance (`phi1_* phi1^* = [2]`), the Prym (the `E2` side here) is killed by `phi1_*`, and nothing computed on it can see the target. [pcc:unverified]

## Limits and what went wrong

- One bielliptic curve per prime (the first candidate of the bench's search loop), weight-2 relations only, degree-2 covers only, `p` up to 12841. Nothing here speaks for covers of higher degree or non-split Pryms.
- The independence of the push check: the `E1` side compares the place sum with `[2 alpha]G_E + [2 beta]T_E` computed on `E1` alone; the `E2` side compares it with `phi2_*` as computed by `Bielliptic::push`, whose formula `place_image` shares with `phi` (both written from the same model). The vanishing `phi2_* phi1^*` is also a unit test of `bielliptic.rs`.
- A smoke run (2 primes, no record) preceded the full run and led to Amendment 1 (control arm A6); the primary run produced noisy first-collision counts and led to Amendment 2 (15 seeds). Both are post-hoc to the first data and labelled as such in the protocol. The verdict rule was judged on the primary run as frozen.
- The strict wording of the A6 prediction did not hold on the primary run (above).
- `cargo clippy --release --all-targets -- -D warnings` fails for the crate for reasons outside these files: two `manual_is_multiple_of` errors in `endo_census.rs` and one `unnecessary_map_or` error in `koblitz_index_calculus.rs`; this crate's `pushforward.rs` and `cover_pushforward.rs` are clean.
