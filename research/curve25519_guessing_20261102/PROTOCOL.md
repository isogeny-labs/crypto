# Curve25519-family hybrid guessing: optimum G per (form, t) — protocol

Frozen 2026-11-02 before any data in this directory.

## What changed in the code

`pkm-semaev` `guess=G` now also applies to the **isogeny** form. The first
`t−G` Vélu maps are applied to every element of `V`; `V` is grouped by the
value `c'` reached at level `t−G` (exactly `2^G` groups of `2^(t−G)`). Each
group is the zero set of a sub-tower of height `t−G` with top rule
`y_(t−G−1)² = (c'+ξ) y − c'ξ − τ`. One branch system per group (summand 1 on
the sub-tower, the other summand unchanged). Bits guessing is unchanged.
`G ≥ t` is refused. Tests: F4 verdict == grid == exact decomposable set for
isogeny `G ∈ {1,2}`. The optional second-summand guess (`guess2`) is not
implemented.

## Measurement

`curve25519_family f4cost 20 <form> <t> <G> 100 2026111400+t 600`, k = 20
miniature, m = 2, mode full, 100 random subgroup targets per cell, per-cell
wall cap 600 s (a capped cell reports the targets it finished).

- bits: t ∈ {6,7,8}, G ∈ {3,4,5,6,7}, G < t.
- isogeny: t ∈ {6,7,8}, G ∈ {0,1,2,3,4}.

Reported per cell: F4 multiply-adds per target (all branches charged),
F4 calls per target, `C_solve` = multiply-adds per F4 call (mean branch
cost), `C(G) = 2^G · C_solve` (the cost of a full refutation), solving
degree, undecided systems, hits and verified hits. Per-relation cost =
muladds per target / λ, with λ from
`../curve25519_algebraic_ladder_20261102/analysis/support.jsonl`
(k = 20; form `xw` for bits, `isogeny` for isogeny):

| t | λ bits (xw) | λ isogeny |
|---|---|---|
| 6 | 0.009527 | 0.009527 |
| 7 | 0.049892 | 0.062651 |
| 8 | 0.179462 | 0.246092 |

Only F4 elimination multiply-adds are counted (the oracle's unit); system
specialisation and grid reading are uncharged native work, listed in the
`native` field. Under AGENTS.md §5a that work would be priced at 0/0.1/1 GAE
in any cross-method comparison; this experiment compares arms of one oracle
and does not rank against rho.

## Preregistered predictions (from the existing G = 0..3 data)

Optimum G = argmin of muladds per target (ties: smaller G).

- **P1 (bits).** t=6: G* = 3 (G=2 → 3 gave 0.26G → 0.17G, near the floor);
  t=7: G* = 4; t=8: G* = 5. Optimum rises by about one per unit of t.
- **P2 (isogeny).** t=6: G* = 1; t=7: G* = 2; t=8: G* = 3. The tower
  already linearises part of the system, so the optimum is lower than bits.
- **P3.** Isogeny + guessing beats bits + guessing on per-relation cost at
  equal t for t = 6, 7, 8 (isogeny has larger λ for t ≥ 7 and cheaper
  unguessed solves).
- **P4.** The best per-relation cost overall is isogeny at t = 7 or 8 with
  its optimum G, below the previous best (isogeny t=6, G=0, ≈ 5.9·10⁹).
- **P5.** C(G) is U-shaped: falls while the guessed levels cut the solving
  degree, rises as 2^G dominates.

Classification: P-claims are judged true/false on the measured argmin and
ordering; a cell whose cap truncates the sample or whose cost differences
are within the spread of its own targets is reported as inconclusive.
Verified hits must equal hits in every cell.

## Amendment 1 (before any data, coordinator)

The ladder's λ counts sums of the 8-projected base, so it is the yield of a cofactor-aware
oracle, about 8× the yield of `pkm-semaev` as run on subgroup targets (lead 3's exact
coverage: t = 6 isogeny 0.0013 vs λ 0.0095; lead 2 measured that 8 independent lifts cost ×8).
Per-relation cost is therefore reported **twice**: with λ as frozen above, and with the
harness's `coverage_exact` (exact fraction of the order-r subgroup that is a sum of 2 base
points, m = 2). P3/P4 are judged on both; where they disagree the result is reported as
inconclusive. The previous-best reference for P4 at exact coverage is 4.2·10¹⁰ (isogeny t = 6,
G = 0, lead 3 README). The optimum-G predictions (P1, P2, P5) use multiply-adds per target and
are unaffected.
