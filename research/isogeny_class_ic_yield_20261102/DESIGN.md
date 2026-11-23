# Isogeny-class IC parameters: ECCp-131 and ECC2K-130 (design, 2026-11-02)

Question (user, 2026-11-02): across the isogeny classes of ECC2K-130 and
ECCp-131, do index-calculus relation yield, solve time, degree of regularity
(Dreg), solving degree and related parameters differ between curves?

Preregistered expectation: null. Within an isogeny class only transport cost,
automorphisms and model presentation can differ
(`research/isogeny_class_difficulty_20261008`). A null result closes a lead;
it is not an improvement over the IC baseline (CLAUDE.md success criterion).

## Class structure (computed 2026-11-02, PARI 2.17 via cypari2; agrees with ecdb curve 22)

| | ECC2K-130 | ECCp-131 |
|---|---|---|
| trace | -22283658519494248867 | 29529377007934231835 |
| fundamental disc. | -7 | -5328143084860676590809120034441032302419 |
| conductor of Z[pi] | 263 * 146505763881528721 | 1 (crater only, h ~ 2^63.6) |
| computable neighbours | 262 at level 263 | Elkies l<=150: 5,7,13,17,29,37,61,73,83,89,107,131,149 (2 each), 67 (1, ramified) |

Prior work (do not redo): `cryptanalysis/experiments/ecc2k130-isogeny-class/`,
`research/isogeny_conductor_gap_ic_20261010` (+ rounds), koblitz-isogeny ICC
study, `cryptanalysis/ecc2k130/research/volcano-m83/`.

## Parts

- **P1 ECCp-131 full width.** Build the 27 Elkies neighbours by verified Velu,
  carry one planted DLP. Per curve: Dreg / solving degree, Macaulay rank
  profile, F4 time per decomposition attempt (m=2,3, fixed bounded system),
  with an isomorphic-model control and an A/A repeat. Full-width yield is
  predicted, not observed (exact-curve screens found zero relations).
- **P2 ECCp-131 scaled analogues.** Primes of 24-40 bits whose Frobenius order
  has conductor 1, full horizontal class enumerated; per curve empirical
  yield, Dreg, F4 time; ICC curve share with planted-effect positive control.
- **P3 ECC2K-130 analogue + full width.** Koblitz analogue volcano at n=41..61
  with a split conductor prime, all level-1 curves, m=3 yield / Dreg / F4 time;
  plus full-width Dreg on E0 and the 262 level-263 curves over a k grid.

Execution: model tiers (AGENTS.md): Sonnet builders write harness and
protocol; experiment-director compiles task cards; executor-mechanical runs
them; the main session interprets. Route frozen by the director (isolab-eks
default).
