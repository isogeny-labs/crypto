= E1 dry run: the known-log base against the abscissa base
<e1-dry-run-the-known-log-base-against-the-abscissa-base>
#strong[SCRATCH, not data.] This is the PLAN §5 step 2 dry run.
`PROTOCOL.md` is not frozen, the host is L0 (no CPU pinning, no
isolation lock, other load allowed), and nothing here is a result of
record. Operation counts and group-addition equivalents (GAE) are exact
and deterministic; wall time is not used anywhere. #strong[No security
claim.] The known-log base is a generic precomputation table in the
Bernstein--Lange shape. No exponent moves.

== What ran
<what-ran>
#figure(
  align(center)[#table(
    columns: (50%, 50%),
    align: (left,left,),
    table.header([item], [value],),
    table.hline(),
    [curves], [two 24-bit prime-order curves, `a = −3`, cofactor 1,
    found by `examples/a_minus_3_prime_order_curves.rs` (seed 20261019);
    generator = smallest-abscissa point],
    [c1], [`p = 8582641`, `b = 6723661`, `r = 8580037` (2^23.03), B =
    205],
    [c2], [`p = 13903607`, `b = 5849518`, `r = 13902853` (2^23.73), B =
    240],
    [B], [`ceil(r^(1/3))`, the family optimum of PLAN §4],
    [arms], [`ic-abscissa` (`prime-abscissa:size=B`), `ic-knownlog`
    (`known-log:size=B`), `rho-neg`\; both IC arms
    `mitm:negation_folded=1,m=2`, walk targets],
    [targets], [16 `planted_shared` per curve, 2 measured rounds after 1
    warm-up, so 32 runs per arm per curve, all verified (288 of 288
    records including warm-up)],
    [sessions], [c1 `ECBS1hdfb4b2694578`, c2 `ECBS1h3e4cb8782f3c`],
    [specs,
    records], [#link("dryrun/spec-c1.json")[`dryrun/spec-c1.json`],
    #link("dryrun/spec-c2.json")[`dryrun/spec-c2.json`],
    #link("dryrun/records/")[`dryrun/records/`]],
    [aggregation], [`examples/generator_effect_dryrun_summary.rs` writes
    #link("dryrun/summary.json")[`dryrun/summary.json`]\; 10 000
    bootstrap resamples, percentile 90% intervals, seed 20261019],
    [graph], [#link("dryrun/dryrun.svg")[`dryrun/dryrun.svg`], source
    #link("dryrun/plot_dryrun.py")[`dryrun/plot_dryrun.py`] (plots
    recorded values only)],
  )]
  , kind: table
  )

#strong[Pricing (AGENTS.md §5a).] The pinned calibration prices group
additions only. Lookups, target-guard probes, square roots, Legendre
symbols and `row_ops` have no pinned ratio on these fresh curves, so
they are priced at `w` ∈ {0, 0.1, 1} GAE per operation and every number
is shown at all three. S = GAE / √r. At `w = 0` the abscissa base's
construction and its linear algebra cost exactly 0 GAE, because they are
only unpriced native work.

== Result 1: the baseline's set-up and linear-algebra share is small
<result-1-the-baselines-set-up-and-linear-algebra-share-is-small>
Share of the `ic-abscissa` total S (measured counts, derived shares).

#figure(
  align(center)[#table(
    columns: (12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%),
    align: (left,right,right,right,right,right,right,right,),
    table.header([curve], [w], [total S], [base construction], [linear
      algebra], [#strong[set-up + LA]], [pair-table build], [target draw
      \+ probes],),
    table.hline(),
    [c1], [0], [19.14], [0], [0], [#strong[0.0%]], [75.3%], [24.6%],
    [c1], [0.1], [20.03], [0.11%], [0.24%], [#strong[0.35%]], [72.0%], [27.6%],
    [c1], [1], [28.07], [0.76%], [1.73%], [#strong[2.5%]], [51.4%], [46.1%],
    [c2], [0], [20.17], [0], [0], [#strong[0.0%]], [76.9%], [23.1%],
    [c2], [0.1], [21.07], [0.09%], [0.19%], [#strong[0.29%]], [73.6%], [26.0%],
    [c2], [1], [29.10], [0.68%], [1.40%], [#strong[2.1%]], [53.3%], [44.6%],
  )]
  , kind: table
  )

The cost of the baseline at this size is the pair table (42 230
additions on c1) and the walk that feeds it, not the base's algebra.
Removing set-up and LA entirely can save at most #strong[0% to 2.5%] of
S. Rho on the same curves: S = 1.13 / 1.24 / 2.14 on c1 and 0.95 / 1.04
\/ 1.81 on c2 at w = 0 / 0.1 / 1, so both IC arms sit at about 13 to 21
times rho at T = 1 (derived).

== Result 2: the online yield is about 100 times lower, not equal
<result-2-the-online-yield-is-about-100-times-lower-not-equal>
Relations found per target tried, pooled over 32 runs per arm, 90%
bootstrap interval.

#figure(
  align(center)[#table(
    columns: (25%, 25%, 25%, 25%),
    align: (left,left,left,left,),
    table.header([curve], [abscissa], [known-log], [#strong[ratio
      known-log / abscissa]],),
    table.hline(),
    [c1], [9.75e-3 \[9.45e-3, 1.006e-2\]], [9.43e-5 \[7.32e-5,
    1.266e-4\]], [#strong[0.0097 \[0.0074, 0.0130\]]],
    [c2], [8.25e-3 \[8.04e-3, 8.46e-3\]], [7.61e-5 \[5.93e-5,
    1.032e-4\]], [#strong[0.0092 \[0.0072, 0.0126\]]],
  )]
  , kind: table
  )

PLAN §4 E1 predicted a ratio of 1.00 with a 90% interval inside \[0.97,
1.03\]. #strong[That prediction is false, and structurally so.] The base
`{±[i]G}` is an arithmetic progression, so every sum of two of its
members is one of `±i ± j`, an integer multiple of G in `[−2B, 2B]`. The
pair table holds at most `4B + 1` distinct sums (821 on c1, 961 on c2),
where an unstructured base of B points holds about `C(2B, 2)` (about 84
000 on c1). The oracle tests membership of a #emph[sum], and sums of a
progression collide. The predicted hit probability per target,
`(4B + 1) / r`, is 9.57e-5 on c1 and 6.91e-5 on c2 (derived), against
9.4e-5 and 7.6e-5 measured, both inside the interval. PLAN §1D and §2
said that the MITM oracle makes the base "an arbitrary set of B points".
It does not: it makes the base's #emph[sumset] the object, and the
sumset of `F_G(B)` has size O(B), not O(B^m).

== Result 3: total S at T = 1 is a tie, by coincidence of trial counts
<result-3-total-s-at-t-1-is-a-tie-by-coincidence-of-trial-counts>
Total S of `ic-knownlog` over `ic-abscissa`, paired by workload and
round.

#figure(
  align(center)[#table(
    columns: (25%, 25%, 25%, 25%),
    align: (left,left,left,left,),
    table.header([curve], [w = 0], [w = 0.1], [w = 1],),
    table.hline(),
    [c1], [0.973 \[0.922, 1.030\]], [0.965 \[0.908, 1.030\]], [0.921
    \[0.819, 1.031\]],
    [c2], [0.968 \[0.917, 1.022\]], [0.960 \[0.902, 1.022\]], [0.913
    \[0.809, 1.021\]],
  )]
  , kind: table
  )

Every interval contains 1. Under PLAN §4's decision rule (interval below
1 at every weight) this is #strong["accounting, no improvement"] at T =
\1. The ratio is not a saving from removed algebra (Result 1 caps that
at 2.5%). It is two trial counts that happen to be close: the abscissa
run collects about 118 relations at 1% each, about 12 000 targets; the
known-log run needs one relation at 0.01% each, about 10 600 targets.
The pair-table build, 14.4 and 15.5 S, is identical in both arms.

== Result 4: with more than one key the known-log base loses by a growing factor
<result-4-with-more-than-one-key-the-known-log-base-loses-by-a-growing-factor>
Model built from the measured parts. The first key is the measured run.
Each further key under the same G costs the relations phase per relation
found: the abscissa matrix has been solved once, and the known-log base
never needed one. S per key at `T` keys, ratio known-log over abscissa,
90% interval.

#figure(
  align(center)[#table(
    columns: (12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%),
    align: (left,right,right,right,left,left,left,left,),
    table.header([curve], [w], [per-key online limit,
      abscissa], [per-key online limit, known-log], [T = 1], [T = 4], [T
      \= 16], [T = 64],),
    table.hline(),
    [c1], [0], [0.040], [4.13], [0.97 \[0.92, 1.03\]], [1.61 \[1.42,
    1.84\]], [4.08 \[3.31, 4.95\]], [12.9 \[10.0, 16.0\]],
    [c1], [1], [0.110], [11.4], [0.92 \[0.82, 1.03\]], [2.11 \[1.72,
    2.56\]], [6.60 \[5.11, 8.31\]], [21.2 \[16.0, 27.0\]],
    [c2], [0], [0.036], [3.94], [0.97 \[0.92, 1.02\]], [1.55 \[1.35,
    1.76\]], [3.80 \[3.03, 4.62\]], [11.9 \[9.15, 15.0\]],
    [c2], [1], [0.101], [11.0], [0.91 \[0.81, 1.02\]], [2.02 \[1.63,
    2.45\]], [6.25 \[4.74, 7.88\]], [20.3 \[15.0, 25.8\]],
  )]
  , kind: table
  )

PLAN's E1 prediction that "as T grows both tend to the online limit and
the ratio tends to 1" is also wrong. The online limits differ by about
\100. In the same model the abscissa arm's amortised S per key is below
the rho reference at T = 64 at every weight, and at T = 16 only at w = 1
(one G, 24 bits only); the known-log arm stays at 4 to 11 S per key and
never gets there. Nothing here says anything about 192 or 224 bits.

== What changes in PLAN
<what-changes-in-plan>
+ #strong[§2 and E1 yield prediction.] Replace "ratio 1.00" by the
  sumset law: hit probability about `(2mB + 1) / r` for an
  arithmetic-progression base at `m = 2`, `(4B + 1) / r`. The interval
  \[0.97, 1.03\] cannot be met and should not be the pass condition.
+ #strong[E1 "total S ratio \< 1 by the set-up and LA share".] The
  measured share is 0% to 2.5%. The decision rule can still be run, but
  it will read "accounting" unless the arms differ in trials, which only
  a chance match of counts produced here.
+ #strong[E1 multi-target block.] The ratio rises with T (about 12 to 21
  at T = 64), it does not tend to 1.
+ #strong[Family optimum.] `B ≈ r^(1/3)` is the optimum for an
  unstructured base. For the known-log base the pair table is mostly
  redundant, so the comparison that matches it is a baby-step table of
  `±[i]G` probed by one lookup per target (online cost about `r / 4B`
  targets), which is `bsgs.negation`. The next row should be known-log
  against that arm at equal memory, not against the abscissa base.
+ #strong[N.] The known-log arm stops at its first relation, so each run
  adds one hit. A 90% interval of ±3% on its yield needs about 3000
  hits, so about 3000 targets per curve, not 64. Either raise N, or let
  the known-log arm keep collecting relations after the first (a
  full-rank-style stop) so one run measures the rate.
+ #strong[E2, E4, E3, E5.] Not run. E2's `N₀` argument is unchanged for
  the abscissa base; for the known-log base the same sumset collapse
  applies on top of it.

== Checks
<checks>
- All 288 records verified (`[d]G = Q` checked by the runner).
- `ic-knownlog` linear algebra: 0 group operations, 0 `row_ops`, rank 1,
  and GAE 0 in every record (`phases[linear_algebra]`).
- Base construction of the known-log arm: B − 1 charged additions
  (`known_log_additions`), 0.064 to 0.070 S.
- Unit tests, `src/cryptanalysis/ic_framework/mod.rs`: logs recovered on
  a 24-bit prime-order curve; digest identical for the same G and
  different for another; `±[i]G` closed under negation.
- Not done: E0 coupled-seed checks, `ic-abscissa-online`, A/A control,
  `m = 3`, the planted `B + 1` control, ecbench band against rho, any
  other bit size.
- Registry: the two curves are not in `docs/curves/registry.json`
  (AGENTS.md §11); register them before any citation in prose outside
  this scratch note.
- No canonical graph or leaderboard is changed: this is a scratch row
  and it lands no improvement (the known-log base is not faster than the
  baseline).

#page(flipped: true, margin: 1.2cm)[
#figure(image("dryrun/dryrun.svg", width: 100%, alt: "E1 dry run"),
  caption: [E1 dry run. A: mean S per target by phase (90% bootstrap interval on the total). B: relations found per target tried. C: S per key against keys T under one G, derived from the measured parts. Sessions ECBS1hdfb4b2694578 and ECBS1h3e4cb8782f3c, scratch dry run.]
)
]
