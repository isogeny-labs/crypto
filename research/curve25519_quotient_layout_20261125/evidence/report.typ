#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)
#set par(justify: true, leading: 0.62em)

#align(center)[
  #text(size: 18pt, weight: "bold")[Quotient-first grevlex layout for a Curve25519 relation system]
  #v(3pt)
  #text(size: 10pt)[EXP-C25519-QLAYOUT-001 · 2026-11-25 · [pcc:unverified]]
]

#v(8pt)
#block(fill: rgb("f8e8e8"), inset: 9pt, radius: 4pt)[
  *Decision: stop before scaling and m=4.* Primary F uses *1.2934390725x*
  D's counted F4 work, with descriptive 95% interval
  *[1.2934369447, 1.2934413625]*. It improves on trailing T by only 2.69%
  and reaches *2.3766x* D's peak RSS.
]

== Question

The selective-cut B system lowered maximum productive degree from 5 to 4 but
carried too much auxiliary state. Under repository grevlex, variable 0 is
largest. The old layout placed quotient variables last, letting quadratic
residue terms lead over equal-degree quotient terms. This study permutes the
same ideal so quotient variables are first (F), after the elementary symmetric
variables (E), or locally interleaved (I). T reproduces the old trailing
layout; D is direct.

== Counted result

#table(
  columns: (auto, 1.35fr, auto, auto, auto),
  inset: 5pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Arm*], [*Mean field ops*], [*ratio / D*], [*degree*], [*max RSS KiB*],
  [D], [1,643,659,738.625], [1.000000], [5], [134,192],
  [T], [2,184,819,473.750], [1.329241], [4], [299,220],
  [F], [2,125,973,727.875], [1.293439], [4], [318,316],
  [E], [2,153,049,208.375], [1.309912], [4], [318,368],
  [I], [1,950,163,543.000], [1.186476], [4], [301,352],
)

#figure(
  image("figure.svg", width: 100%),
  caption: [Paired geometric-mean F4 work over eight fresh random d=19 targets. Only F was eligible to qualify.],
)

#pagebreak()

== Frozen gates

#table(
  columns: (2.55fr, 1fr),
  inset: 6pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Gate*], [*Outcome*],
  [Self-test, planted exactness, and paired targets], [Pass],
  [Inverse-permutation identity and identical verdicts], [Pass: 0 mismatches],
  [T exact canonical-B reproduction], [Pass: 0 / 12 mismatches],
  [F censoring no worse; at least six triples], [Pass: 0 / 0 / 0; 8 triples],
  [Upper 95% F/D work endpoint below 0.75], [*Fail: 1.293441*],
  [Upper 95% F/T work endpoint below 0.60], [*Fail: 0.973068*],
  [F productive degree at most 4 on at least 6/8], [Pass: 8 / 8],
  [Every F/D peak-RSS ratio at most 1.25], [*Fail: 2.376557*],
  [Zero layout build overhead; F/D build at most 1.25], [Pass: 0; 0.986357],
)

Variable layout matters, but not nearly enough. Moving quotients first saves
2.69% of T's work while increasing peak memory. Local interleaving saves more,
10.74% versus T, yet remains 18.65% slower than D and over twice as large in
memory. Generic grevlex renaming does not consume the local auxiliaries early
enough to repay them.

== Correctness and flow

All 60 planned qualification rows completed: 8 random and 4 planted targets
for five arms. No row timed out, hit the degree ceiling, was killed, or was
excluded. All 20 planted rows evaluated exactly and group-verified. The native
self-test inverse-permuted every quotient system back to one canonical ideal
and checked the exhaustive distinct-root and tiny truth panels.

A separate 12-row run used the predecessor selective-cut binary at its pinned
hash with the fresh seed. T matched it exactly on targets, counts, degrees,
matrix extrema, steps, and verdicts.

== Scope and next direction

This is a variable-permutation experiment under ordinary grevlex, not a true
block order or incremental local-elimination solver. It covers distinct roots
on one small prime-field family instance. Complete relation collection and
linear algebra are unpriced, so no exponent or subexponential conclusion
follows.

The negative result closes simple auxiliary placement. A stronger successor
must change the algebraic split, not just rename variables: split three roots
as a symmetric pair plus one root, impose quadratic-divisor and scalar
root-of-unity membership separately, and replace S4 by two S3 constraints
through an intermediate pair-sum abscissa. That lead needs prior-art review and
a separately frozen protocol.

== Provenance

Native Rust instrument and analyzer in Isogeny Labs `crypto`; hash-pinned
measurement and reproduction binaries; disclosed PR #4650 compile-repair
overlay because the task branch has unrelated build failures. Raw rows,
protocol, analyzer, CSV, JSON, SVG/PNG, Typst/PDF, and export are retained
beside this report. No JavaScript was used.
