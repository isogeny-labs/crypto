# End-to-end toy index calculus vs Pollard rho

Measures the whole pipeline, not one step, to give the distance to rho. Both
methods are counted in point additions (group ops) on the same prime-order
curves. Exploratory runs, not bench.record runs (no S3 credentials in the
session). All numbers are [pcc:unverified].

`ic_mt.py p seed n1,n2,...`:
- Curve: random y^2 = x^3 + Ax + B over F_p with prime order r (brute-force point count, seeded).
- Index calculus: the factor base is the n points with smallest x. Building the pair table x(F_i +- F_j) costs about n^2 adds. Targets come from an r-adding random walk (1 add each). Each target is decomposed as R = +-F_i +-F_j +-F_k: about 2n adds per target, plus one add per hit to recover the sign. The run collects n + 5 relations, then does Gaussian elimination mod r (not counted in group ops). The recovered log is asserted equal to the true w.
- Rho: r-adding walk with 20 multipliers and Brent cycle detection, mean of 5 runs, checked against w. No negation map.
- Outputs: `runs/`, plot `ic_vs_rho.{png,svg,pdf}` (`plot.py`).

Mean over 3 curves per p; IC is at its best n of the grid:

| p | rho ops | IC best total ops (n) | IC / rho | sqrt p | p^(2/3) |
|---|---|---|---|---|---|
| 10007 | 163 | 1288 (30) | 7.9 | 100 | 464 |
| 100003 | 501 | 5008 (60) | 10.0 | 316 | 2154 |
| 1000003 | 1752 | 22647 (120) | 12.9 | 1000 | 10000 |

- IC cost grows like about p^0.63 and rho like about p^0.51 over this range, so the gap widens with p (7.9x, 10.0x, 12.9x) [pcc:unverified].
- Every IC and rho log recovered was correct (asserted).
- This is the expected outcome for a decomposition step that is generic group arithmetic: the total is about p^(2/3), and no tuning of n changes the exponent. Speeding up the decomposition step further (as in `sym_m4/`) cannot close the gap. Only a decomposition whose per-target cost grows more slowly than n, which no encoding here has shown, could [pcc:unverified].
- Not counted: linear algebra (n^3 field ops) and the one-time field cost of building the factor base. Counting them would only widen the gap.

## Large-prime variation (2026-11-15)

`ic_mt.py p seed n,... --lp`. Each target already computes 2n leftover points
L = R -+ F_k. If none decomposes, they are stored by x(L). When a later target
produces the same x, the two partial relations give a full relation in 2
factor-base elements: R - s F_k = +-(R' - s' F_k'). If no usable dependency
appears, the run collects 5 more relations and retries; those additions count.
Outputs in `runs_lp/`. Numbers [pcc:unverified].

| p | rho ops | LP-IC best total ops (n) | LP-IC / rho | plain IC / rho |
|---|---|---|---|---|
| 10007 | 163 | 344 (2) | 2.1 | 7.9 |
| 100003 | 501 | 1033 (4) | 2.1 | 10.0 |
| 1000003 | 1752 | 3110 (2) | 1.8 | 12.9 |

- The large-prime variant costs about p^0.48, the same class as rho. The gap stays at about 2x instead of growing, which is 4-7x fewer group operations than plain IC at these sizes [pcc:unverified].
- It still does not beat rho. The best n is tiny (2-4), and almost every relation comes from leftover-point collisions. At that point it is a birthday-collision algorithm with an almost empty factor base, so it is bound by the generic sqrt(p) limit (Shoup) like rho. The remaining factor of about 2 is a constant: rho takes one addition per step, while this pays 2n additions per target.
- Every log was checked correct. Across 45 runs (3 primes, 3 curves, 5 values of n) the solver retried 6 times. Rho has no negation map here; adding one would widen the gap by about sqrt(2).

## Stronger rho reference, minimal slack, and the triple-large-prime question (2026-11-15)

Two stronger rho arms (`ic_mt.py`, always run): `rho_store` stores every visited
point and solves at the first repeat, with no cycle-detection overhead.
`rho_store_neg` also matches +-X on x-coordinate (the negation-map gain). LP-IC
now runs with `--slack=1` (collect n + 1 relations, not n + 5). Outputs in
`runs_lp_slack1/`; mean of 3 curves; numbers [pcc:unverified].

| p | sqrt r | rho Brent | rho store | rho store+neg | LP-IC best (n) | vs store+neg | vs Brent |
|---|---|---|---|---|---|---|---|
| 10007 | 100 | 162 | 129 | 91 | 220 (1) | 2.40 | 1.35 |
| 100003 | 316 | 500 | 382 | 350 | 755 (1) | 2.15 | 1.51 |
| 1000003 | 1000 | 1751 | 1500 | 871 | 1985 (4) | 2.28 | 1.13 |

- Against the strongest rho reference the gap is about 2.2-2.4x and does not shrink with p [pcc:unverified]. The earlier "~2x" was measured against the weaker Brent rho.
- The constant matches a count. At n = 1, each target costs 3 additions (1 walk step, 2 leftovers) and yields 2 stored points, so 1.5 additions per point. It also needs n + 1 = 2 collisions instead of rho's 1, a factor of sqrt(2) in a birthday search. So 1.5 x sqrt(2) is about 2.1. As n goes to 0 the method becomes `rho_store_neg`, so in this generic setting it can approach strong rho but not pass it [pcc:unverified].
- Triple (and double) large primes are not tested, because they have no meaningful generic analogue here. In Gaudry/Thériault-style methods, large primes are elements of a bounded larger set (for example x < B2) that the decomposition can target algebraically. Here decomposition is a hash join, so leftovers are arbitrary points, and the single-large-prime variant already uses every one. Multi-way variants would need 3-way collisions (L1 + L2 + L3 = O), which take more points than pairwise birthday collisions in a prime-order group. A bounded large-prime set would make only a B2/p fraction of leftovers usable, which is strictly worse. Triple large primes pay off only where decomposition is algebraic and fast, as with Weil descent over F_{q^n}.
