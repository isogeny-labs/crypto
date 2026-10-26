# E2 cofactor ladder: known-log base against the abscissa base at h = 1 to 16

A browsing copy. The source of truth is
[`research/generator_effect_ic_20261019/E2.md`](../../research/generator_effect_ic_20261019/E2.md),
with the frozen PROTOCOL, the pre-registered predictions, 40 ecbench sessions
(122 880 records, all verified), the aggregation JSON and the plot source.

40 curves over 24-bit `p` (`a = −3`, `#E = h·r`, 8 per `h ∈ {1, 2, 4, 8, 16}`),
768 planted targets per curve, `B = ⌈#E^{1/3}⌉`, `mitm` `m = 2`
negation-folded:

- The online yield ratio of a random-scalar known-log base over the
  smallest-abscissa base is 1.008, 2.015, 3.997, 7.832 and 14.99 at
  `h` = 1, 2, 4, 8, 16. Measured over the pre-registered subgroup-sumset
  prediction it reads 1.002 to 1.017, and every 90% interval contains 1.
  The `N₀` prediction holds at `h` ≤ 8 and over-counts by 2.5% at
  `h = 16`, as pre-registered. The A/A control misses by 0.0002 at `h = 4`,
  which voids `h = 4` under the frozen rule.
- Total S at T = 1: known-log over abscissa is 0.897 to 0.967 at w = 0 and
  0.626 to 0.770 at w = 1, below 1 across the §5a band at every `h`. Both
  arms are 8 to 54 times rho-neg.
- Shared macOS host, isolation L0, wall time not used. No security claim;
  P-192/P-224 have `h = 1`, so E2 says nothing about them.

- [Graph (SVG)](e2.svg), [PNG](e2.png)
- [Report PDF](E2.pdf), [run report PDF](run-report.pdf)
