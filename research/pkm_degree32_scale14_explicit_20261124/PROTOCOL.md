# Explicit-curve 14-bit PKM degree-32 panel — 2026-11-24

The roster-based 14-bit panel stopped before timing because all CLI seeds selected one curve with interval/candidate sizes 15/16. This amendment uses explicit EFD curves and does not pool the earlier store. A read-only diagnostic observed only curve validity and factor-base sizes; no performance result was inspected.

## Frozen deterministic roster

Enumerate odd primes `p` below 2^14 in descending order. For each, scan `a=1..29`, then `b=1..79`, and retain the first nonsingular `y²=x³+ax+b` with exactly counted prime order. Choose the least `x` with nonzero quadratic-residue RHS and least nonnegative square root `y`. Index `i` uses target seed `20262000+i`.

Each row below is `i p a b r X Y Z seed interval_size candidate_size`:

```text
1 16381 1 17 16381 2 384 1 20262001 13 17
2 16369 1 14 16319 1 4 1 20262002 19 13
3 16363 1 35 16453 0 181 1 20262003 16 16
4 16361 1 3 16249 1 1464 1 20262004 13 18
5 16349 1 66 16249 0 7709 1 20262005 16 14
6 16339 1 4 16451 0 2 1 20262006 17 7
7 16333 1 15 16453 1 7903 1 20262007 15 23
8 16319 1 3 16547 0 1411 1 20262008 18 22
9 16301 1 14 16087 0 6768 1 20262009 20 17
10 16273 1 7 16427 1 3 1 20262010 15 20
11 16267 1 53 16417 0 1980 1 20262011 20 17
12 16253 1 26 16061 2 6 1 20262012 13 14
13 16249 1 16 16069 0 4 1 20262013 16 16
14 16231 1 31 16369 2 2434 1 20262014 17 18
15 16229 1 22 16061 1 5817 1 20262015 18 10
16 16223 1 1 16333 0 1 1 20262016 16 16
```

Use `--efd-model shortw --efd-chart jacobian`, exact p/a/b/r/generator, cofactor 1, 16 repeats, and the frozen target seed. Omit `--bits` and `--family`. Binary SHA-256 is `06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`.

Rerun both bases for all sixteen curves with `subtract`, one trial and zero rho runs. Require exact instance/order and frozen sizes. Append selection indices 3, 13 and 16 before timing.

## Timed panel and gates

Candidate is `prime-isogeny:t=5,seed=5,aux=ordinary` with m=2 F4 chain degree 8. Comparator is `prime-abscissa:below=32` with the exact five-bit interval equation. Timing is a separate explicit resume. Before it, require cargo, rustc, full-cost work and other `ic bench` measurements to be idle and append a load receipt with local time, load averages, CPU count and CPU-heavy process snapshot.

Run interval, candidate, candidate, interval for each selected curve, with 1,000 trials, five seconds per solver call, one rho context run and a 600-second outer cap. Require four successful 16-row invocations, exact instance/order/base size, verified corresponding scalars and 16 distinct scalars.

Each curve passes only when both mirrored charged-GAE ratios are below 0.9, wall/CPU ratios below 1, combined GAE below 0.9, and each arm's repeat max/min GAE and wall are at most 1.5. Overall passage requires all three curves plus aggregate GAE/wall/CPU below 1. Report relations, trials, relation probability, relations per charged GAE, F4 operations, solver time, nanoseconds per operation, systems and median `s_over_rho`. This is a deterministic size-matched component panel, not a random curve sample or a complete attack comparison.
