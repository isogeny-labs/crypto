#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.6pt)
#set par(justify: true, leading: 0.5em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.8pt)
#show heading.where(level: 3): set text(size: 9pt)
#show heading: set block(above: 0.8em, below: 0.4em)
#show table: set text(size: 7.2pt)
#show raw: set text(size: 7.6pt)
#show link: set text(fill: rgb("#2a78d6"))

= Lead 1: the n \= 3 Tian route. Plane-quartic Jacobians in the class of Res E, and Diem index calculus against rho
<lead-1-the-n-3-tian-route.-plane-quartic-jacobians-in-the-class-of-res-e-and-diem-index-calculus-against-rho>
#strong[How this was produced.] A Sonnet lead agent built the tools and ran every measurement under `PROTOCOL.md` (frozen before the runs; Amendments 1–4 dated before the runs they govern). The parent session wrote this report from the agent’s findings: the subagent could not write report files. Every number cites a run directory under `results/`.

The parent session re-checked the following:

- both tools pass rustfmt, clippy `-D warnings` and their unit tests (8 and 5);
- the committed protocol matches the file;
- each run’s `env.txt` records the protocol version it ran under.

No independent validator has replayed these runs yet.

== Answer
<answer>
- #strong[(a) Jacobian census \[exact, sampled\].] Theorem 4 (round 2) flags certain prime-order `E/F_{q³}`. For every such class at `q = 7, 11, 13, 17`, the isogeny class of `Res_{F_{q³}/F_q} E` contains the Jacobian of a smooth plane quartic over `F_q`: #strong[26 of 26].
  - This confirms round 2’s untested "the quotient is a Jacobian" step, at the level of isogeny classes. No isogeny was computed.
  - Unflagged prime-order classes are also hit, so a quartic in the class is #strong[not] evidence of a Tian kernel. The flag only changes the hit rate, to about 2×, with caveats below.
- #strong[(b) Diem index calculus on generic quartic Jacobians \[measured\].] A complete double-large-prime index calculus solved every planted instance up to `q = 8191`, and every answer was verified.
  - With native work priced at 0 it costs 180–340 group additions, against rho’s 5,000–300,000.
  - With native work priced at 0.1 and 1 GAE it loses: its dense linear algebra grows like `q^1.4`.
  - #strong[The ranking is unresolved across the §5a band.] This is not a speedup claim, and item (b) is not run on the item (a) classes.

== Item (a): census (runs `a-census-run-20261024T2150Z`, `a2-summarize-run-20261024T2228Z`)
<item-a-census-runs-a-census-run-20261024t2150z-a2-summarize-run-20261024t2228z>
#strong[Weil polynomial.] For `E/F_{q³}` with trace `t3`, Frobenius on `Res E` has characteristic polynomial `P(T) = T⁶ − t3·T³ + q³`.

#strong[Matching condition.] A smooth quartic matches exactly when all three counts agree, and then `#Jac(F_q) = #E`:

- `N_1 = q + 1`;
- `N_2 = q² + 1`;
- `N_3 = q³ + 1 − 3·t3`.

In that case `c1 = c2 = 0` and `c3 = −t3`.

#strong[Irreducibility.] `P` is irreducible unless `t3 = s³ − 3qs` for some integer `|s| ≤ 2√q`. Prime `#E` excludes this; the reducible classes occur only for composite order (11 of them at `q = 13`).

#strong[Theorem 4, re-implemented independently.] The agent’s version agrees with round 2’s `curves.csv` on class and flag in all 23,182 odd-`ℓ` rows, with 0 conflicts at the `t3` level.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,right,right,right,).at(col),
  inset: 6pt,
  [`q`], [flagged prime-order classes], [hit], [samples],
  [7],
  [3],
  [3],
  [3.0·10⁶],
  [11],
  [5],
  [5],
  [3.0·10⁶],
  [13],
  [8],
  [8 (fewest hits: 3, at `t3 = 87`)],
  [3.0·10⁶],
  [17],
  [10],
  [10],
  [2.5·10⁶],
)
]

At `q = 19` all 11 certain-or-ambiguous prime-order classes are hit. They are not counted as positives, because round 2 has no curves there to settle scalar against Jordan.

#strong[Negative control (unflagged prime-order classes).] These are hit too. Hits by `q`:

#align(center)[#table(
  columns: 2,
  align: (col, row) => (right,right,).at(col),
  inset: 6pt,
  [`q`], [unflagged classes hit],
  [7],
  [7 / 8],
  [11],
  [10 / 10],
  [13],
  [14 / 14],
  [17],
  [22 / 23],
  [19],
  [26 / 26],
)
]

The two zeros are `q = 7, t3 = 37` (the Hasse-range edge) and `q = 17, t3 = 127` (local expectation 3.7). Both mean "not found within N", not "absent". Theorem 4 is about Tian kernels, not about Jacobians in the class.

#strong[Rate gap.] Measured as the mean of hits over local expectation, flagged divided by unflagged; intervals are a bootstrap over `t3`:

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,right,left,).at(col),
  inset: 6pt,
  [`q`], [flagged / unflagged], [95% interval],
  [7],
  [2.12],
  [0.90–4.69],
  [11],
  [1.47],
  [1.00–2.12],
  [13],
  [2.07],
  [1.38–3.09],
  [17],
  [1.95],
  [1.26–2.92],
)
]

The bootstrap treats the `t3` as independent, and they are not. The local-factor explanation is heuristic and untested.

#strong[Class masses.]

- Smooth quartics: 118,091 at `q = 7` (against `q⁶` \= 117,649), up to 47.1 M at `q = 19`, reproducing `q⁶` to about 0.5%.
- `P(c1 = c2 = 0 and smooth)`: 1.2·10⁻² at `q = 7` down to 3.1·10⁻³ at `q = 19`.
- Jacobian mass per flagged class: 24, 74, 117 and 272 at `q = 7, 11, 13, 17`, roughly `q^2.7`.

#strong[Controls.]

- #emph[Positive control.] A constructed quartic `C*` at `q = 7, 11, 13` has brute-force counts `(6, 52, 378)`, `(12, 136, 1323)` and `(11, 189, 2261)`. The fast counters agree, and a fresh search of 10⁶ samples finds the same triple 520, 54 and 57 times.
- #emph[Recount.] An independent brute-force recount confirms 35 of 35 stored examples.
- #emph[A/A.] A second seed at `q = 7` gives χ² \= 69.1 on 55 cells, `z = 1.35`.
- #emph[Smoothness certificate.] Tested against brute-force singular-point search over `F_5`, `F_25` and `F_125`.

#image("figures/fig_a_hits.svg") #image("figures/fig_a_ratio.svg")

== Item (b): index calculus against rho (run `b-ic-vs-rho-run-20261024T2246Z`; medians in `b2-median-summary-run-20261024T2258Z`)
<item-b-index-calculus-against-rho-run-b-ic-vs-rho-run-20261024t2246z-medians-in-b2-median-summary-run-20261024t2258z>
#strong[Tool.] `examples/quartic_ic.rs` provides:

- a group law on the Jacobian of a plane quartic (Mumford-like pairs and two-cubic addition), with negation and doubling;
- orders found by baby-step giant-step;
- double-large-prime index calculus, with the factor base of degree-1 points, relations from lines through two points, a contracted large-prime graph and dense linear algebra mod `ℓ`;
- Pollard rho, plain and with the negation map.

All arms run in one process and use the same counters.

#strong[Instances and verification.] There were 12/12/10/10/8/6/4 instances at `q = 127 … 8191`. Every answer was verified: 52/52 for the index calculus, 52/52 for rho and 42/42 for rho with negation. Rho with negation stops at `q = 2039`. One group addition costs about 5,550 M + 229 I.

Mean cost in GAE at `I = 3M`, with native work priced at 0 / 0.1 / 1 GAE (`I = 2·log2 q·M` gives the same table to three digits):

#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,left,left,left,).at(col),
  inset: 6pt,
  [`q`], [index calculus], [rho], [IC/rho],
  [127],
  [232 / 3,842 / 36,331],
  [5,230 / 5,347 / 6,397],
  [0.044 / 0.72 / 5.7],
  [251],
  [210 / 8,532 / 83,425],
  [4,796 / 5,088 / 7,719],
  [0.044 / 1.68 / 10.8],
  [509],
  [148 / 22,086 / 219,530],
  [9,736 / 11,080 / 23,172],
  [0.015 / 1.99 / 9.5],
  [1021],
  [212 / 66,009 / 658,178],
  [17,918 / 21,086 / 49,596],
  [0.012 / 3.13 / 13.3],
  [2039],
  [177 / 180,870 / 1,807,109],
  [40,744 / 48,529 / 118,592],
  [0.004 / 3.73 / 15.2],
  [4093],
  [202 / 489,622 / 4,894,400],
  [50,927 / 60,902 / 150,681],
  [0.004 / 8.04 / 32.5],
  [8191],
  [338 / 1,411,234 / 14,109,296],
  [302,417 / 363,186 / 910,108],
  [0.001 / 3.89 / 15.5],
)
]

At `q = 8191` one rho instance of 1.1·10⁶ steps inflates the mean. With medians the IC/rho ratios there are 0.007 / 27 / 109.

#strong[Fitted slopes in `q`] (finite range, 7 values of `q`, intervals 95%):

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,right,left,left,).at(col),
  inset: 6pt,
  [quantity], [slope], [interval], [reading],
  [IC cost, native at 0],
  [0.06],
  [−0.02–0.16],
  [about 120–180 target additions, independent of `q`],
  [IC cost, native at 0.1 / 1],
  [1.43 / 1.45],
  [1.41–1.47],
  [dense linear algebra, about `27·q^1.5` multiply-adds],
  [rho cost, native at 0],
  [0.94],
  [0.48–1.13],
  [pulled down by a set-up of about 4,000 additions],
  [rho steps],
  [1.42],
  [—],
  [theory: 1.5],
  [IC relation stage only],
  [0.69],
  [0.68–0.70],
  [a stage diagnostic, not a speedup],
)
]

#strong[Ranking (AGENTS.md §5a).] The index calculus wins with native work priced at 0 and loses at 0.1 and 1, for every `q ≥ 251`, so every pair is #strong[unresolved].

#strong[Predictions.]

- P-b1: holds.
- P-b2 (relation-stage slope 1.0–1.3): not supported. The stage measures 0.69 in GAE units and about 0.9 in raw M.
- P-b3 (no resolved ranking): holds.
- P-b4: mixed. Rho with negation costs 1.7–4× plain rho at `q ≤ 1021`, and about the same at `q = 2039`.

#image("figures/fig_b_cost.svg") #image("figures/fig_b_ratio.svg")

== Distance to rho
<distance-to-rho>
Item (a) has no cost rows. For item (b), IC/rho is:

- 0.001–0.044 with native work at 0;
- 0.7–8.0 at 0.1 GAE;
- 5.7–32.5 at 1 GAE.

No row puts the index calculus below rho at every weight. The relation stage alone sits below rho at every weight, but that is a stage diagnostic.

== What is not established
<what-is-not-established>
- #strong[No end-to-end attack.] No Tian isogeny `Res E → Jac(C)` was computed, so no attack on `E/F_{q³}` was run. Only the existence of a quartic in the isogeny class is shown, and only for `q ≤ 19`.
- #strong[Item (b) uses generic quartics, not the item (a) classes.]
- #strong[No cost exponent.] Seven primes, a 65× range in `q`, a rho set-up constant and a varying cofactor allow finite-range slopes only.
- #strong[Dense linear algebra.] A sparse solver (about `B²·d` instead of `B³`) was not measured, and is the likeliest change to the 0.1 and 1 GAE cells.
- #strong[`B = 3√q` is not optimised.]
- #strong[The rate-gap explanation is untested.]
- #strong[Instance filter.] A planted pair is accepted only if small multiples are computable (Amendment 4). How many pairs were rejected was not recorded.
- #strong[Not yet admitted.] No independent replay has been run, so this study is not in the results table.

== Records kept
<records-kept>
- `b-ic-vs-rho-run-20261024T2228Z`: the first run. Its rho-with-negation rows have no two-cycle escape.
- `b-ic-vs-rho-run-20261024T2234Z`: aborted at `q = 509` on an unusable `Q`.

Both are superseded by the `…T2246Z` run.

== Next improvements
<next-improvements>
+ Sparse linear algebra mod `ℓ` (block Wiedemann or Lanczos), counted.
+ A cheaper target phase, since the target additions are now the whole cost with native work at 0.
+ Connect (a) and (b): compute `E/F_{q³} → Jac(C)` at `q ≤ 19` with a matched rho in `E`, so the Tian route is priced end to end.
+ Item (a) at larger `q`, with a rank test of the rate gap.

== Reproduce
<reproduce>
`run_item_a.sh`, `run_item_b.sh`; `sh build.sh` builds this PDF.
