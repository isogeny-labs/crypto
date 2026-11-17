#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8.5pt)

= R7.4: King §5 and Eagle–Galbraith–Ong, and a binary-Edwards check of Corollary 9.2

Date 2026-10-31. Items 1 and 2 of the three small open items of §19.17. Toy instances only
(n = 9 … 14). No sub-rho claim and no claim about a deployed curve. Source of every number:
`binary_edwards.json` (measured, native Rust `examples/r7_4_binary_edwards.rs`) or the cited page. Full text
and the paste-ready note text are in `RESULTS.md`.

== Verdicts

*Item 1 (literature).* The primary texts of King (PKC 2004) and of Eagle–Galbraith–Ong (EGO) could not be read:
Springer redirects to a login and the IACR PDFs return a Cloudflare 403. Read: the ePrint 2009/086 abstract page,
the publisher page of EGO (abstract, 13-entry reference list with King as [4]), Eagle's 2008 PhD thesis (a
different document; Lemma 4.1.15, $op("Tr")(x) = op("Tr")(a)$ on an odd-prime-order subgroup; no King, no
halving, no cofactor-4 test), and citation metadata. None of it contradicts or pre-empts a statement of the note,
and none of it bears on the kernel-saturation law, symmetrisation or summation polynomials. *King §5 and the EGO
cofactor-4 test remain unread.*

*Item 2a (algebraic).* Proved: Corollary 9.2 holds for every rational subgroup of 2-power order on every
ordinary binary curve in every model, so for binary Edwards curves and for Z/4 (Proposition 9.3 in
`RESULTS.md`). Tested on 46 toy binary Edwards curves (151 034 points): 11 identity families, 0 exceptions.
Binary Edwards has no torsion that Weierstrass lacks: $4 | N$ iff $op("Tr")(d_1) = 1$, as $4 | N$ iff
$op("Tr")(a) = 0$ for Weierstrass (1 600 random curves each, 0 exceptions). Its coordinate $w = x + y$ has the
orbits of $chevron.l T, -1 chevron.r$ as fibres (37 792 fibres), so the natural Edwards frame is T-symmetrised by
construction.

*Item 2b (pair-table yield).* Hypothesis: the yield-ratio interval contains 1. Falsifier as worded: an interval
excluding 1 on some family. *The falsifier fires at n ≤ 12* (primary metric: 39 of 40 pooled cells), with a sign
that depends on whether the relation matrix is charged $c$ or $c + 1$ rows; I do not confirm the hypothesis
at toy size and I do not claim a gain. The naive full sweep reproduces the law $1 slash mu$ (n = 13, 14), and the
quotient sweep finds the same relations with $1 slash mu$ lookups (268 000 of 268 000 checks).

== Method (item 2b)

Complete binary Edwards curves $d_1 (x + y) + d_2 (x^2 + y^2) = x y + x y (x + y) + x^2 y^2$, $op("Tr")(d_2) = 1$,
over $FF_(2^n)$: 40 curves with n = 9 … 12 and 6 with n = 13, 14 (scale check). Cyclic group of order $N$,
$r$ the largest prime factor ($r >= 100$, $r^2 divides.not N$). Pair-table oracle exact over every target of
$G$; randomness only in 1 000 bases per arm per cell. Arms at equal $|F|$: plain, s2 (saturated under $chevron.l T chevron.r$,
the fibres of $w$), s4 (under $chevron.l Q_4 chevron.r$, if $4 | N$), each swept over all of $F$ or over one point per
coset (quotient). Yield $= sum lambda slash sum (U W)$, $lambda$ the mean number of relations modulo $M$ per
target, $W$ lookups, $U = c$ or $c + 1$ for $c$ distinct signed projected columns; the primary metric is
$lambda slash c$. Interval: 2 000 bootstrap resamples over bases, stratified by curve.

#figure(
  image("fig_edwards_yield.svg", width: 100%),
  caption: [Pooled yield ratio to the plain base and 95% bootstrap interval against n, plain hit fraction target
  0.25 (`binary_edwards.json`, `pooled_ratios`; n = 13, 14: 3 curves each, 2 and 3 with $4 | N$). Left: quotient
  sweep, $U = c$ (solid) and $U = c + 1$ (dashed). Right: full sweep with the law $1 slash mu$ (dotted).],
)

== Results

Pooled over the 40 curves with n ≤ 12 (ratio, 95% interval).

#table(
  columns: (auto, 1fr, 1fr, 1fr, 1fr),
  align: (left, center, center, center, center),
  [*arm, metric*], [*H 0.05*], [*0.1*], [*0.25*], [*0.5*],
  [s2 quot, λ/c], [1.119 \[1.110, 1.128\]], [1.084 \[1.077, 1.091\]], [1.061 \[1.056, 1.067\]], [1.048 \[1.044, 1.050\]],
  [s2 quot, λ/(c+1)], [0.934 \[0.926, 0.941\]], [0.942 \[0.936, 0.948\]], [0.940 \[0.934, 0.945\]], [0.963 \[0.960, 0.966\]],
  [s4 quot, λ/c], [1.481 \[1.464, 1.500\]], [1.342 \[1.329, 1.354\]], [1.212 \[1.198, 1.228\]], [1.109 \[1.103, 1.116\]],
  [s4 quot, λ/(c+1)], [0.928 \[0.918, 0.940\]], [0.924 \[0.915, 0.933\]], [0.880 \[0.870, 0.891\]], [0.884 \[0.878, 0.889\]],
  [s2 full, λ/c (law 1/2)], [0.559 \[0.555, 0.564\]], [0.542 \[0.538, 0.545\]], [0.530 \[0.528, 0.533\]], [0.524 \[0.522, 0.525\]],
  [s4 full, λ/c (law 1/4)], [0.370 \[0.366, 0.374\]], [0.336 \[0.333, 0.338\]], [0.303 \[0.299, 0.307\]], [0.277 \[0.276, 0.279\]],
)

Scale check, n = 13 and 14, primary metric: s2 quot 0.992–1.038, s4 quot 0.994–1.146; the interval contains 1 in
10 of 16 cells; s2 full 0.496–0.519 and s4 full 0.248–0.286, the law value inside the interval in 12 of 16 cells.
The hit-fraction metric $H slash c$ keeps an excess (1.14 at n = 14, H target 0.05, s2) that decreases with n and
that I cannot attribute; larger instances would be needed.

== Limits

Toy $n lt.eq 14$ with 1–20 projected columns (R5.1 found the law unreached in this regime). The pair-table yield
depends on the abstract group and the base, not on the model; no Gröbner/F4 oracle was run on Edwards summation
polynomials, and the Galbraith–Gebregiyorgis and Huang–Petit–Shinohara–Takagi setups were not read. The primary
metric was fixed after a smoke run; all four are in the JSON. The finite-size reading of the $c$ versus $c + 1$
split is derived, not proved.

== Unread sources

King PKC 2004 (§5, Table 2); the EGO body; Pornin ePrint 2022/1164 and 2022/1325; Galbraith–Gebregiyorgis 2014;
Huang–Petit–Shinohara–Takagi 2015. A search-engine summary says EGO refers to King §5 for the cofactor-4 case;
that is snippet-level and unverified.
