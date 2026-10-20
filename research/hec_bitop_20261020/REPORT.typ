#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[The genus-4 sub-rho row, re-priced in bitop/v1]
  #v(2pt)
  `research/hec_bitop_20261020` · protocol frozen 2026-10-20 · report 2026-10-20
]

= Question

The scoreboard's only end-to-end measurement below rho is index calculus on a
genus-4 hyperelliptic Jacobian over $FF_61$ ($N = 16,790,591 approx 2^24$) at
0.24× rho. It rests on two choices:
- *The in-situ unit.* Its linear algebra and smoothness oracle enter through a
  wall-clock calibration.
- *An 8-bit rho walk.* Round twelve later found this walk slow on the largest
  rows.

Two questions follow:
- Does the row stay below rho when both arms are counted where they execute
  and priced in one accounting convention, against the full-width walk?
- Does the genus-4 exponent hold?

`bitop/v1` is a reproducible accounting convention with frozen gate-equivalent
weights. It is not a physical gate count or circuit cost; see the protocol's
interpretation amendment.

= Method

- *Instrument.* Every operation of both arms is counted where it executes:
  - the $FF_p$ additions, multiplications, squarings, inversions, square roots
    and Legendre symbols;
  - arithmetic modulo $N$;
  - hash probes and word operations.

  Both arms are priced by `src/cryptanalysis/bitops.rs`:
  - field operations at $"bits"(p)$;
  - mod-$N$ operations at $"bits"(N)$.

  The work is grouped in phases: precomputation, walk, oracle, linear algebra
  and verification.
- *Coverage diagnostic.* Under Callgrind, the two arms' instructions per
  counted gate are within 1.16×. The IC oracle alone runs 1.7–2× rho's,
  because `fp_inv`'s exponentiation is priced as one primitive.
  - Not counted: memory movement, generator internals beyond one word
    operation per draw, and loop-control tests. Both arms do these.
- *Rows.*
  - The 18 rows of round seven, in the fb-walk configuration.
  - Index calculus: seed 20260916, 5 runs per row.
  - Rho: seed `0xC0FFEE`, 40 runs per row, full-width walk.
  - Every logarithm verified.
- *Analysis.* `examples/hec_bitop_analyse.rs`, with 2,000 bootstrap resamples
  and seed 20261020.
  - Per row: the ratio of mean costs, with a 95% interval.
  - Per genus: the slope of $log_2("IC"/"rho")$ on $log_2 N$, with a two-stage
    bootstrap interval.

= Results

#figure(image("figures/ratio_vs_n.svg", width: 96%),
  caption: [IC/rho per row in bitop/v1 with 95% intervals (5 IC × 40 rho runs); crosses are round seven's in-situ ratios against the 8-bit walk.])

#table(columns: 4, stroke: 0.4pt, inset: 4pt,
  [*genus*], [*bitop/v1 slope [95%]*], [*round seven, in-situ*], [*verdict*],
  [4 (5 rows)], [−0.232 [−0.320, −0.082]], [−0.29 [−0.42, −0.16]], [advance holds],
  [3 (7 rows)], [+0.095 [+0.050, +0.148]], [−0.01 [−0.07, +0.05]], [rising, above round seven's band],
  [2 (6 rows)], [+0.269 [+0.138, +0.523]], [+0.21 [0.00, +0.41]], [rising],
)

*Genus 4, p = 61:* IC/rho = *0.114 [0.096, 0.138]*. The upper bound is below
1, so by the preregistered rule the row *survives*.

#figure(image("figures/p61_variants.svg", width: 92%),
  caption: [The p = 61 row under the primary measure and every reported variant; the grey bars are round seven's figures.])

#figure(image("figures/phases_g4_p61.svg", width: 92%),
  caption: [Where one solve's bitop/v1 goes at genus 4, p = 61 (recorded phase totals over runs, divided by runs).])

= What qualifies the result

- *Rho's set-up drives the small rows (post hoc).* The rho reference
  precomputes 16 full-width jump points.
  - At small $N$ that fixed cost dominates: 68% of rho at genus 2,
    $p = 251$.
  - Without it, every genus-2 row and the small genus-3 and genus-4 rows read
    above 1.
  - At genus 4, $p = 61$, it is 14% of rho, and the row reads 0.134 without
    it.
- *The code moved.* With the in-situ unit and the full-width rho, the genus-4
  rows read 1.22, 0.87, 0.61, 0.62 and 0.13. Round seven read 2.47, 1.79, 1.21,
  1.16 and 0.15.
  - The move from 0.24 to 0.11 combines later algorithm changes, the corrected
    walk and the unit.
  - At the same head, the unit alone moves the row from 0.132 (in-situ) to
    0.114 (bitop/v1).
- *Class: accounting.* No algorithm changed in this study. It confirms the row
  and the genus-4 exponent; it does not improve index calculus.

= Scope

- Toy hyperelliptic Jacobians, with no deployed system.
- Counts only.
- One curve per row; genus 4 has five sizes, and the winning row is the
  largest.
- Coverage limits as stated above.
- Deviations and the owner's amendment are in `PROTOCOL.md`.
