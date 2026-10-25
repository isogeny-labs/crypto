= Results: toy falsifier for the `θ₂` Coppersmith ceiling on `S₃`
#strong[Date.] 2026-10-23. #strong[Protocol.] #link("PROTOCOL.md")[`PROTOCOL.md`], frozen and committed (`c785a0e23`) before the run. #strong[Run.] #link("reports/run-20261023T2122Z/run.md")[`reports/run-20261023T2122Z/`] (`run.md`, `run.json`, `trials.jsonl`, `recovery.svg`, `report.pdf`). #strong[Instrument.] `examples/coppersmith_s3_theta2_falsifier.rs`. #strong[Question.] §7 item 1 of #link("../notes/ecdlp-general/RESEARCH_LATTICE_ECDLP_ROUTES.md")[`RESEARCH_LATTICE_ECDLP_ROUTES.md`]: does a Coppersmith-type lattice on `S₃` recover a planted two-summand decomposition with `x(P_i) ≤ p^β` beyond `β = 1/4`?

== Verdict (decision rule of the protocol, §1 and §5)
#strong[Closed at measured level, for `ℓ ≤ 3`.] Recovery was 0 of 10 in every cell with `β ≥ 0.26` (60 cells, 600 trials), in fact in every cell with `β ≥ 0.20` (96 cells, 960 trials), at all four sizes and all three multiplicities, and every positive control passed. The refutation criterion (≥ 5/10 in any cell with `β > 1/4`) was not met in any cell. The §6 ceiling of the note stands at the measured level, and its finite-`ℓ` rows are confirmed by the Howgrave-Graham column (below).

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [cell family], [result], [positive control (same instrument)], [verdict wording],
  [all `β ≥ 0.26`, 4 sizes × ℓ ∈ {1,2,3}],
  [0/10 in 60 of 60 cells],
  [passed in every `(bits, ℓ)` row that has one (see below)],
  [no recovery above the stop line],
  [all `β > 1/4`],
  [max recovered 0/10],
  [—],
  [refutation criterion not met],
  [negative control, `β = 0.30`],
  [0 genuine pairs in 120 trials],
  [—],
  [no false positives on random targets],
)
]

#strong[Positive controls] (smallest grid `β` with `B ≥ 4`, provided `β ≤ β_det(ℓ) + 0.02`): 24-bit `β = 0.10`: ℓ \= 2 10/10, ℓ \= 3 10/10; 32-bit `β = 0.08`: ℓ \= 2 10/10, ℓ \= 3 10/10; 40-bit `β = 0.06`: ℓ \= 1, 2, 3 all 10/10; 56-bit `β = 0.04`: ℓ \= 1, 2, 3 all 10/10. Rows `(24, ℓ = 1)` and `(32, ℓ = 1)` have no positive control under the protocol\'s clause (their `β_det = 0.056` gives `B < 4`) and are excluded from the verdict; they are reported in full and behave like the others.

== Where recovery actually stops, against the derivation
#align(center)[#table(
  columns: 6,
  align: (col, row) => (right,right,right,left,left,left,).at(col),
  inset: 6pt,
  [ℓ], [dim], [derived `β_det(ℓ)`], [HG holds 10/10 up to (24 / 32 / 40 / 56 bits)], [recovered ≥ 5/10 up to], [last nonzero recovery],
  [1],
  [9],
  [1/18 \= 0.056],
  [0.06 / 0.06 / 0.06 / 0.06],
  [0.12 / 0.08 / 0.08 / 0.08],
  [0.12 / 0.12 / 0.12 / 0.08],
  [2],
  [25],
  [1/10 \= 0.100],
  [0.08 / 0.08 / 0.10 / 0.10],
  [0.14 / 0.12 / 0.12 / 0.12],
  [0.18 / 0.14 / 0.14 / 0.12],
  [3],
  [49],
  [5/42 \= 0.119],
  [0.10 / 0.10 / 0.10 / 0.10],
  [0.14 / 0.14 / 0.14 / 0.12],
  [0.16 / 0.14 / 0.14 / 0.14],
)
]

Reading (classes per the controlled-experiment skill):

- #strong[Howgrave-Graham (the rigorous sufficient condition) tracks `β_det(ℓ)` to within the grid step at every size.] Measured: holds through 0.06 / 0.08–0.10 / 0.10 for ℓ \= 1 / 2 / 3, against derived 0.056 / 0.100 / 0.119. This is the determinant condition of §6 of the note, measured rather than taken on faith. #emph[Comparative, established by this run on these cells.]
- #strong[Heuristic recovery runs 0.02–0.06 above the rigorous bound, and the excess shrinks as the prime grows] (ℓ \= 1: last recovery 0.12 at 24 bits, 0.08 at 56 bits; ℓ \= 2: 0.18 → 0.12; ℓ \= 3: 0.16 → 0.14). LLL finds shorter vectors than the worst-case bound promises, and the margin measured in `β` is a constant number of bits that is worth less as `bits` grows. #emph[Comparative; the trend is in the direction the `p → ∞` analysis predicts and is not a claim about the limit.]
- #strong[Nothing approaches rho parity.] The stop line `β = 1/4` is 0.07–0.17 above the last recovery of any cell; the `ℓ → ∞` limit 1/6 ≈ 0.167 was crossed by a single 1/10 cell (24 bits, ℓ \= 2, β \= 0.18) and by nothing at 32 bits or more. #emph[Exact count on the recorded cells.]
- #strong[Multiplicity buys little.] Going from ℓ \= 1 (dim 9, ≈ 1–7 ms per reduction) to ℓ \= 3 (dim 49, 0.1–7 s) moves the recovery threshold by about 0.04–0.06 in `β`, as `β_det` predicts (0.056 → 0.119); the asymptote is 0.167. #emph[Comparative on the recorded cells.]

Effect size in the unit that matters: at 56 bits the largest recovered bound is `B = 105` (ℓ \= 3, β \= 0.12, 10/10) against the `B = p^{1/4} ≈ 1.6·10⁴` that rho parity would need and the `B = 539027` (β \= 0.34) that was tested and failed 0/10.

== Scope: what this does not show
- Only the Jochemsz–May basic box lattice of the single polynomial `S₃`, at `ℓ ≤ 3`, with the repository\'s LLL (no BKZ, no extended shifts, no symmetric rewriting; Probe C and W10 cover those and found the same threshold). It is a measured ceiling at these multiplicities, not a proof of the `ℓ → ∞` limit and not a statement about other lattice families.
- Four sizes, one curve each, `N = 10` per cell: a 0/10 cell bounds the recovery probability by 0.31 (95% one-sided). The verdict rests on 60 such cells agreeing and on the monotone shape of every row, not on any one cell.
- Not an attack, a speedup or a security statement. Under the success criterion this is a boundary measurement of a derivation: it changes no baseline and reaches no leaderboard.

== Prior measurements this agrees with
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §3.2 (Probe B, 40 bits: \"≥ 2 rows vanish\" to `δ = 0.12`–`0.14`, Howgrave-Graham to 0.10), §3.3 (Probe C, extended and symmetric lattices, same threshold), and `research/lit2026_w10_s5_coppersmith_20261011/` §5.4 (32 bits, ℓ \= 1, 2). This run adds genuine root extraction as the metric (the resultant step, candidates checked against `S₃`), two further sizes, ℓ \= 3, the sweep past 1/4, and the two control arms, under a frozen protocol.

== Checklist (controlled-experiment skill §6)
- ☒ every published sentence classified (heuristic estimate tested; comparative statements marked; exact counts marked)
- ☒ negative-control population stated (random targets, 120 trials)
- ☒ every \"nothing found\" has a passing positive control on the same path
- ☒ one instrument, identical budgets across arms, seeds recorded
- ☒ resolution arithmetic in the protocol (per-cell count; family false-refutation probability below `4·10⁻³`)

#page(flipped: true, margin: 1.2cm)[#figure(image("reports/run-20261023T2122Z/recovery.svg", width: 100%), caption: [Recovered fraction per cell; the run report has the table])]
