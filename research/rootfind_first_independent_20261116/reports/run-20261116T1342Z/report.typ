#set page(paper: "us-letter", margin: 0.62in)
#set text(font: "Helvetica Neue", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= First-independent relation collection
_Fixed-stream operation result · 2026-11-16 · run-20261116T1342Z_

The batch oracle's first verified triple can yield a dependent relation.
This screen continued the same residual only until it found a row that
advanced modular rank. First-hit and all-hit controls used the same curves,
factor bases, pair table, collision rows and residual streams. The source
harness re-added every triple and asserted scalar 641 at full rank.

#image("diagram.svg", width: 95%)

== Full-rank stopping point

#table(
  columns: (0.52fr, 1.2fr, 1.68fr, 0.75fr),
  inset: 4pt,
  stroke: 0.5pt + rgb("c5d1d8"),
  [Cell], [First / candidate / all queries], [First / candidate / all group proxy], [Candidate minus first],
  [K13], [37 / 37 / 37], [963 / 991 / 1,267], [+28],
  [K19], [182 / 156 / 103], [14,203 / 15,337 / 14,565], [+1,134],
  [K23], [610 / 610 / 503], [128,211 / 144,219 / 138,914], [+16,008],
)

The candidate's frozen operation prediction failed in all three cells.
It reduced K19 target queries by 26 but added 1,134 modeled group additions;
K13 and K23 stopped at the same query as first-hit and spent more work.
The all-hit arm reached full rank sooner on K19/K23, but also cost more
modeled additions than first-hit.

#pagebreak()

== The charged tradeoff

#image("tradeoff.svg", width: 100%)

The proxy charges one group addition for each computed batch subtraction
and pair re-addition. It excludes native field work, lookup latency,
row-reduction time, setup, target generation, memory and verification.
The observed 0.28-second process wall covered the diagnostic harness as a
whole and cannot rank the policies. This was one deterministic execution
on a shared macOS arm64 host (L0), with no interval or independent replay.

== Scope and next gate

K13 enumerated its order-2,003 subgroup. K19/K23 each used 1,024 distinct
coefficients `105000+3t`, disjoint from prior samples. The fixed protocol
is `../../PROTOCOL.md`; raw output is `../../run.stdout`; the checked
machine result is `../../RESULTS.json`. Protocol commit `0e413634e8`,
source commit `83407456e0`, executable SHA-256 starts `f58ad80e4402`.

First-hit remains the operation winner for these fixed streams. A next
candidate must lower per-query or reusable setup cost, then face the
frozen same-target complete-call F4/F5/F6 panel with isolation and
independent answer replay. The current screen has no admitted wall-time
or complete-call performance conclusion.
