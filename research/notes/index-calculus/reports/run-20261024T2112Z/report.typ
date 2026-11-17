#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.55in))
#set text(font: "Helvetica", size: 9pt)
#set heading(numbering: none)
= N=51 honest-prior GHS genus distribution
_24 October 2026 · phase 1 structural diagnostic_

The frozen protocol sampled 16,384 curves from public seed `2026102420261024202610242026102420261024202610242026102420261024` in GF(2^51), polynomial $x^51+x^6+x^3+x+1$. The BLAKE3 XOF generator and complete GHS audits are in the source repository. The statistic is the smallest genus with a descending row over a proper intermediate subfield ($l=3$ or $l=17$); the $l=1$ audit remains in raw records but is excluded from this statistic.

#image("genus-histogram.svg", width: 100%)

== Observed frequencies

#table(
  columns: (2fr, 1fr, 1fr),
  inset: 5pt,
  table.header([*Threshold*], [*Hits / 16,384*], [*Empirical p*]),
  [$g <= 0,1,2,3$ (each)], [0], [0],
  [$g <= 4,8$ (each)], [16,384], [1],
)

For each zero-hit threshold, modeling the indexed BLAKE3 outputs as independent uniform draws, the exact one-sided 95% binomial upper bound is $1 - 0.05^(1/16384) = 0.0001828282722735608$. No finite empirical grind-bit estimate is assigned to a zero-hit event. The observed $g=4$ row uses $l=17$ for every index.

== Scope and replay

This is a sampled structural observation. Hidden-route secrecy, detector error, destination solving, complete cost, and matched rho were not measured. The run used Darwin arm64 kernel 25.6.0 without CPU isolation; it makes no speed claim. Reproduce from base revision `0200e2b2af7c608b100930be9397124441c2710b` plus this PR with:

#text(size: 8pt)[
`cargo run --release --example seed_grind_honest_prior -- 16384 <seed-hex> <new-output-dir>` \
`cargo run --release --example seed_grind_honest_prior -- --verify <output-dir>`
]

The scoreboard records the panel; the existing IC progress timeline and method-bound frontier remain unchanged because this phase adds no priced pipeline or method bound.

The verifier rederived and compared all 16,384 records and their aggregate successfully. The run directory `seed_grind_trapdoor_20261024_run/observations.jsonl` is 4,970,702 bytes, BLAKE3 `034878466ddb0598ded498d54fb252599f0366d4abd63ae8993a52932930f00c`. See the run directory's `summary.json`, `observations.jsonl`, this report's `REPORT.md` and the frozen section 11 of `RESEARCH_SEED_GRIND_TRAPDOOR_20261002.md` for the full seed, command, derivation and limits. Replay used the same implementation and host; independent cross-implementation verification remains open.

== Next discriminating phase

Freeze seed freedom and a detector rule before selecting curves, then measure success and detector errors with matched controls. Keep any subsequent end-to-end comparison separately accounted.
