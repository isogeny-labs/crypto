#set page(margin: 1.8cm)
#set text(size: 9.5pt)
= E3: a Semaev oracle that uses the base's algebra does not change the picture
<e3-a-semaev-oracle-that-uses-the-bases-algebra-does-not-change-the-picture>
#strong[Experiment E3 of #link("PLAN.md")[`PLAN.md`], run under the
protocol frozen in #link("E3-PROTOCOL.md")[`E3-PROTOCOL.md`] (commit
`ede431274`) before any measured run.] No security claim. Nothing here
concerns P-192 or P-224 beyond PLAN §7's fixed wording. The curves are a
restricted prime family (`2^t | p − 1`).

== Question and answer
<question-and-answer>
PR \#2297 found that, with the MITM oracle, a random-scalar known-log
base (`known-log:scalars=random`: multiples of `G`, logarithms free)
matches the smallest-abscissa base's yield. The reason is that MITM
never uses the abscissa structure. E3 asks whether an oracle that does
use the base's algebraic description changes that comparison.

The oracle is new in this PR: `pkm-semaev`, in `ic.pipeline`. For every
target it writes Semaev's `S₄(x₁, x₂, x₃, x_R) = 0` with each `x_i` on
the `prime-kummer` base's tower `(x/g)^{2^t} = 1`
(Petit--Kosters--Messeng), and F4 over the tower quotient ring decides
it. The algebraic base is `prime-kummer`, because `prime-abscissa`'s
bound has no low-degree `F_p` description. The answer, on three 24-bit
prime-order curves for `t = 2…5` (B = 1 to 19):

+ #strong[The algebra buys no yield.] At equal B the Kummer base and the
  random-scalar known-log base have identical exact counts `N₃` of
  points that are a sum of three base points, at all 12 (curve, t)
  cells. The measured hit rates cover `N₃ / r` in every cell.
+ #strong[What the algebra costs is F4's linear algebra.] It is unpriced
  native work, so §5a prices it in the band. At w = 0 the Semaev arm
  costs less than known-log MITM (S ratio 0.30--0.72, point estimates).
  At w = 0.1 and w = 1 it costs 6.5·10³ to 1.8·10¹¹ times more. The
  order flips inside the band at every t, so under AGENTS.md §5a
  #strong[the two are unresolved, and there is no ranking for either
  side.]
+ #strong[No crossover in B.] The break-even weight `w*` at which the
  arms tie falls 49× to 880× per tower level: about 10⁻⁶ at t = 2, 10⁻⁸
  at t = 3, 10⁻¹⁰ at t = 4 and 10⁻¹³ at t = 5. That is because F4's cost
  per target grows 67--406× per level while MITM's grows 2×. No `B*`
  exists in the band at w ≥ 0.1, and the trend points away from one.
+ #strong[F4 is far above the trivial algorithm on the same system.] The
  `V³` grid reference decides every one of these systems with 1.6·10³ (t
  \= 2) to 6.2·10⁷ (t = 5) times fewer operations than F4, with
  identical verdicts.

So for E3's question: replacing MITM by an oracle that uses the base's
polynomial description does not make a structured base worth more than a
random known-log set of the same size. It adds a cost that §5a prices
and that grows much faster than the table's.

#page(flipped: true, margin: 1.2cm)[
#figure(image("e3/e3.svg", width: 100%, alt: "E3: S against B at each band weight (top) and the ratio to known-log MITM (bottom)"),
  caption: [
    E3: S against B at each band weight (top) and the ratio to known-log
    MITM (bottom)
  ]
)

#emph[Figure 1. S = GAE/√r of a full solve against B, at w = 0, 0.1 and
1 GAE per unpriced native operation. Markers o/s/^ are e3-c1/c2/c3.
Solid lines are measured: the grid reference, known-log MITM, and
rho-neg (dotted, per curve). Dashed lines are the F4 arms,
#strong[substituted, not run] (see Method). At w = 0 the F4 lines
coincide with the grid line, because they have the same group work.
Error bars are 90% bootstrap intervals over 8 targets, paired by target
in the ratios. Sessions ECBS1h642a1a44f9f9, ECBS1hd4736f2baf84,
ECBS1h2a56f971e518.]
]

#figure(image("e3/e3_f4.svg", width: 100%, alt: "F4 multiply-adds per call against t"),
  caption: [
    F4 multiply-adds per call against t
  ]
)

#emph[Figure 2. Native operations per decomposition attempt against t,
on e3-c1. F4's count per refutation is identical on all three curves at
each t. Shaded bars show the planted (consistent) systems from the
`f4cost` runs.]

== What ran
<what-ran>
#figure(
  align(center)[#table(
    columns: (50%, 50%),
    align: (left,left,),
    table.header([item], [value],),
    table.hline(),
    [oracle], [`pkm-semaev`
    (`src/cryptanalysis/ic_framework/pkm_semaev.rs`), registered in
    `solve_ic_prime` (only on `prime-kummer`, whose `t` and `g` it
    reads) and in `ecbench::claim`],
    [curves], [e3-c1 `p = 9061313`, e3-c2 `p = 13040257`, e3-c3
    `p = 11223809`: 24-bit, `a = −3`, prime order, `v₂(p − 1) ≥ 6`
    (table in the protocol); control `p = 2393` (12-bit)],
    [arms], [per t: `sem-chain` and `sem-full` (F4, capped at
    `K_t = 400/100/10/1` trials, `crosscheck=1`), `sem-grid` (grid
    reference, full solve), `kl-mitm3`
    (`known-log:size=B,scalars=random` + `mitm:m=3,negation_folded=1`,
    full solve), and `rho-neg`],
    [targets], [8 `planted_shared` per curve, target seed 20261019, 1
    round, no warm-up, host L0 (shared, no pinning); operation counts
    only],
    [sessions], [e3-c1 `ECBS1h642a1a44f9f9`, e3-c2 `ECBS1hd4736f2baf84`,
    e3-c3 `ECBS1h2a56f971e518`, control `ECBS1h28077f2dbe1c`
    (#link("e3/sessions/")[`e3/sessions/`])],
    [outcome], [416 records. Full-solve arms: 248/248 verified
    (`[d]G = Q`). Capped F4 arms: 168/168 `exhausted` by design],
    [aggregation], [#link("../../examples/generator_effect_e3_summary.rs")[`examples/generator_effect_e3_summary.rs`]
    → #link("e3/summary.json")[`e3/summary.json`]\; pricing through
    `ecbench::band::{unpriced_total, calibrated_native}`\; 10 000
    resamples, seed 20261019],
    [extra inputs], [#link("e3/sizes.jsonl")[`e3/sizes.jsonl`] (B,
    `N₃`\; `generator_effect_e3`),
    #link("e3/f4cost.jsonl")[`e3/f4cost.jsonl`] (146 F4 systems, planted
    and random; `generator_effect_e3 f4cost`)],
    [graphs], [#link("e3/e3.svg")[`e3/e3.svg`],
    #link("e3/e3_f4.svg")[`e3/e3_f4.svg`] (PNG beside them), from
    #link("e3/plot_e3.py")[`e3/plot_e3.py`], which plots `summary.json`
    only],
    [run
    report], [#link("reports/run-20261019T2334Z/README.md")[`reports/run-20261019T2334Z/`]],
  )]
  , kind: table
  )

== Method: what is measured and what is substituted
<method-what-is-measured-and-what-is-substituted>
- #strong[Measured:] every full-solve arm (`sem-grid`, `kl-mitm3`,
  `rho-neg`). F4's multiply-adds per call are measured on the capped
  runs: 3,200 / 800 / 80 / 8 calls per curve at t = 2 / 3 / 4 / 5. The
  planted-system spread is measured by `f4cost`.
- #strong[Substituted (not run):] the F4 arms' full-solve cost. A full
  F4 solve needs about `(B + 1)/h` trials, about 10⁵ at t = 4 at about 1
  s each, which is out of reach. `sem-chain`, `sem-full` and `sem-grid`
  share base and seed, so they draw the same trial stream and reach the
  same verdicts. So each grid record is re-priced per target: the grid
  multiplications on refuted trials are replaced by F4's mean
  multiply-adds per call plus the system-specialisation multiplications
  per call. The grid work on consistent trials stays in, because the F4
  arm reads solutions off the grid too. Group work is the grid record's.
  The two facts the substitution rests on were checked:
  - #strong[Same stream:] the chain and full capped records have
    identical group work and identical trial counts on every curve and
    t. All 168 capped records report 0 F4/grid disagreements over 24,502
    F4 calls, all refutations and every one grid-checked
    (`pkm_crosscheck_disagreements = 0`).

  - #strong[Cost by shape:] every F4 refutation at a given (t,
    presentation) costs the same multiply-adds. The count is identical
    across targets and across the three curves:

    #figure(
      align(center)[#table(
        columns: 3,
        align: (right,right,right,),
        table.header([t], [chain], [full],),
        table.hline(),
        [2], [3.908·10⁵], [2.243·10⁵],
        [3], [1.587·10⁸], [1.260·10⁷],
        [4], [1.060·10¹⁰], [1.683·10¹⁰],
        [5], [3.186·10¹²], [---],
      )]
      , kind: table
      )

    The capped mean equals the `f4cost` refutation count to 4 digits.
    Consistent (planted) systems cost 0.53--1.52× a refutation, at most
    1.52× (P2: ≤ 2×). Solving degree is 5 / 6 / 6 / 7 for chain and 7
    for full.
- #strong[Pricing:] `S(w) = (G − C + w·(U + n_C)) / √r` with `U` every
  unpriced counter of the record. For F4 that is the solver block's
  multiply-adds; for the grid, `pkm_grid_fp_mults_uncharged`\; for MITM,
  lookups and target-guard probes; for the base build, square roots and
  Legendre symbols. `C = n_C = 0`: these fresh curves have no pinned
  calibration.

== Results
<results>
=== 1. Yield at equal B (P1 holds)
<yield-at-equal-b-p1-holds>
`N₃` (exact) is identical for the two bases in all 12 cells, from
`N₃ = 4` (e3-c3, t = 2, B = 1) to `N₃ = 9196` (t = 5, B = 19). The
measured hit rates (Wilson 90%) of `sem-grid` and `kl-mitm3` cover
`N₃ / r` in all 24 (curve, t, arm) cells. Examples: e3-c1 t = 5, h =
1.015·10⁻³; grid 1.110·10⁻³ \[0.971, 1.27\]·10⁻³; known-log 1.505·10⁻³
\[0.848, 2.67\]·10⁻³. The one miss is on the 12-bit control curve,
outside the population: `kl-mitm3` found 8 relations in 170 trials
against `h = 0.0185`, and its interval \[0.027, 0.082\] misses `h` (see
"Controls").

=== 2. S at the band weights, and the flip (P3 holds)
<s-at-the-band-weights-and-the-flip-p3-holds>
S ratio of the Semaev arm (F4 chain, substituted) to `kl-mitm3`, paired
over 8 targets, with 90% intervals:

#figure(
  align(center)[#table(
    columns: (14.29%, 14.29%, 14.29%, 14.29%, 14.29%, 14.29%, 14.29%),
    align: (left,right,right,left,left,left,right,),
    table.header([curve], [t], [B], [w = 0], [w = 0.1], [w = 1], [w\*
      (F4 chain)],),
    table.hline(),
    [e3-c1], [2], [2], [0.35 \[0.18, 0.62\]], [1.13·10⁴ \[0.61,
    1.99\]·10⁴], [6.2·10⁴], [5.2·10⁻⁶],
    [e3-c1], [3], [4], [0.52 \[0.36, 0.78\]], [7.4·10⁶ \[5.1,
    11\]·10⁶], [4.1·10⁷], [5.9·10⁻⁹],
    [e3-c1], [4], [9], [0.65 \[0.36, 1.32\]], [6.1·10⁸ \[3.4,
    12\]·10⁸], [3.4·10⁹], [5.2·10⁻¹¹],
    [e3-c1], [5], [19], [0.68 \[0.42, 1.38\]], [1.8·10¹¹ \[1.1,
    3.5\]·10¹¹], [1.0·10¹²], [1.6·10⁻¹³],
    [e3-c2], [2], [3], [0.68 \[0.42, 1.17\]], [2.3·10⁴ \[1.4,
    3.9\]·10⁴], [1.3·10⁵], [1.3·10⁻⁶],
    [e3-c2], [3], [5], [0.55 \[0.37, 0.79\]], [7.8·10⁶ \[5.2,
    11\]·10⁶], [4.3·10⁷], [5.3·10⁻⁹],
    [e3-c2], [4], [10], [0.47 \[0.29, 1.01\]], [4.5·10⁸ \[2.7,
    9.5\]·10⁸], [2.5·10⁹], [1.1·10⁻¹⁰],
    [e3-c2], [5], [19], [0.44 \[0.24, 1.19\]], [1.2·10¹¹ \[0.63,
    3.1\]·10¹¹], [6.4·10¹¹], [4.5·10⁻¹³],
    [e3-c3], [2], [1], [0.72 \[0.43, 1.34\]], [1.9·10⁴ \[1.2,
    3.4\]·10⁴], [1.1·10⁵], [1.3·10⁻⁶],
    [e3-c3], [3], [3], [0.48 \[0.25, 1.21\]], [6.6·10⁶ \[3.4,
    17\]·10⁶], [3.7·10⁷], [7.1·10⁻⁹],
    [e3-c3], [4], [9], [0.63 \[0.45, 0.96\]], [5.9·10⁸ \[4.2,
    9.0\]·10⁸], [3.3·10⁹], [5.7·10⁻¹¹],
    [e3-c3], [5], [18], [0.30 \[0.19, 0.60\]], [8.0·10¹⁰ \[5.0,
    16\]·10¹⁰], [4.5·10¹¹], [7.9·10⁻¹³],
  )]
  , kind: table
  )

- #strong[At w = 0] F4 is free, and the Semaev arm pays only for target
  draws and lifts. It collects `B + 1` relations at about one addition
  per trial, where known-log MITM pays 2B additions per trial for one
  relation. Every point estimate is below 1. The interval excludes 1 in
  5 of 12 cells and contains it in 7. `sem-full` gives the same w = 0
  ratios, because its group work is the same.
- #strong[At w = 0.1 and w = 1] every cell is above 6·10³. At t ≥ 3 the
  chain cells are above 6·10⁶ and the full cells above 5·10⁵: P3's 10⁴
  bar with room to spare. The full presentation is 12.6× cheaper than
  chain at t = 3 and 1.6× dearer at t = 4. Neither changes the picture.
- #strong[§5a verdict:] the order of the Semaev arm and known-log MITM
  changes inside the band in every cell. #strong[Unresolved. Not a win
  for either.]

=== 3. Crossover in B (P4 holds)
<crossover-in-b-p4-holds>
- #strong[At w = 0.1 and w = 1:] the Semaev arm is above known-log at
  every t on every curve, so no `B*` exists.
- #strong[At w = 0:] it is below at every t, so again there is no
  crossover.
- #strong[`w*`:] it falls monotonically along t on every curve, by 49×
  to 880× per level, and is below 10⁻⁸ for every t ≥ 3.
- #strong[Why:] F4's multiply-adds per call grow 406× (t = 2→3), 67×
  (3→4) and 301× (4→5), while MITM's per-trial cost doubles. The yield
  is identical at equal B, so the gap between the arms is the per-trial
  decision cost alone. At w = 0.1 it grows from about 10⁴ to 10¹¹ across
  the band.
- #strong[Extrapolation:] `t = 6` (N = 18) was not run, and the PKM note
  §16 put one system there at 44 minutes and its host's 14 GB. Extending
  the measured per-level ratios gives no sign of a turn. This is an
  extrapolation.

=== 4. The grid reference (P5 holds)
<the-grid-reference-p5-holds>
The grid decides the same systems with 244 / 1,224 / 7,440 / 51,232
`F_p` multiplications per call at t = 2…5, identical on every curve. F4
takes 1.6·10³, 1.3·10⁵ (full: 1.0·10⁴), 1.4·10⁶ and 6.2·10⁷ times more.
The grid arm is itself measured end to end:

- #strong[At w = 0:] below known-log, with S ratio 0.30--0.72.
- #strong[At w \> 0:] above it, from 7.4× at t = 2 (e3-c1, w = 0.1) to
  1.6·10⁴× at t = 5 (w = 1).

Its `O(|V|³)` cost per target grows faster than MITM's `O(B)`.

=== 5. Distance to rho (P6 holds; reported as distance, not a gate)
<distance-to-rho-p6-holds-reported-as-distance-not-a-gate>
Every IC arm is above `rho-neg` in every cell and at every weight. The
closest is at t = 5:

- #strong[grid at w = 0:] 5.5--7.0× rho;
- #strong[known-log MITM:] 8.6--23× rho at every weight.

F4 at w = 0.1 is 10¹¹--10¹² times above rho at t = 5. Rho's S is 1.08 /
1.18 / 2.04 (e3-c1), 1.33 / 1.45 / 2.56 (e3-c2) and 1.17 / 1.28 / 2.24
(e3-c3) at w = 0 / 0.1 / 1.

== Controls
<controls>
- #strong[End to end, F4 actually run:] on the 12-bit control curve,
  `sem-chain-t2` ran F4 on every trial to full solves: 8/8 verified,
  1,228 F4 calls, 0 disagreements. Its S equals `sem-grid-t2`'s at w = 0
  (18.38), as the substitution assumes. Its measured F4 count, 3.927·10⁵
  per call, matches the 24-bit refutation count to 0.5%. Its
  `ecbench band` price, measured, is 18.383 / 123,668 / 1,236,514 at w =
  0 / 0.1 / 1. The substitution formula applied to the control's grid
  records gives the same three numbers. This checks the substitution's
  accounting. It does not check the cost-by-shape step, because there
  the mean comes from the same calls.
- #strong[The control's `kl-mitm3` hit rate:] 8 hits in 170 trials
  against `h = 0.0185`\; the Wilson interval misses `h`. The stream
  stops at the first relation, so the 8 samples are geometric waiting
  times and a fixed-n interval is only approximate for them. The walk
  also revisits points on a group of 2,377 elements, and the target
  guard skips the repeats. It is reported as measured. On this curve the
  Semaev arm is above known-log even at w = 0 (1.28 \[1.06, 1.55\]).
- #strong[Unit and harness tests (this PR):]
  - planted `m = 3` decompositions found in both presentations and by
    the grid;
  - F4 against the grid and the exact decomposable set on random and
    planted targets;
  - refusal of a foreign base;
  - a verified `solve_ic_prime` run whose query stream and relations
    match the grid.

== Predictions against outcome
<predictions-against-outcome>
#figure(
  align(center)[#table(
    columns: (33.33%, 33.33%, 33.33%),
    align: (left,left,left,),
    table.header([], [prediction], [outcome],),
    table.hline(),
    [P1], [`N₃` equal within 1%; hit rates cover `N₃/r`], [holds:
    identical `N₃` in 12/12 cells; 24/24 intervals cover],
    [P2], [refutations identical per shape; consistent ≤ 2×], [holds:
    identical to the last digit across targets and curves; consistent ≤
    1.52×],
    [P3], [below at w = 0, above ≥ 10⁴ at w ≥ 0.1 and t ≥ 3; flips,
    unresolved], [holds in point estimates; at w = 0 the interval
    excludes 1 in 5/12 cells],
    [P4], [`w*` falls with B, \< 10⁻⁴ for t ≥ 3, no `B*`], [holds: `w*`
    \< 10⁻⁸ for t ≥ 3, no `B*` at any weight],
    [P5], [grid ≥ 10³ cheaper than F4 per trial for t ≥ 3], [holds:
    1.0·10⁴ to 6.2·10⁷ (1.6·10³ at t = 2)],
    [P6], [every IC arm above rho], [holds],
  )]
  , kind: table
  )

== Scope and limits
<scope-and-limits>
- #strong[Population:] a restricted prime family (`2^t | p − 1`), three
  curves, `m = 3`, `t ≤ 5`, `B ≤ 19`. These B are far below E1's MITM
  optimum (`B ≈ r^{1/3} ≈ 210`), because F4 does not reach larger towers
  on this host.
- #strong[Engine:] one engine (`f4_fp_tower`, normal strategy, degree
  bound 12) and two presentations. A different engine or a different
  system (for example symmetrised variables, or an isogeny tower) could
  change F4's constants; the measured growth per level makes a turn
  inside this band unlikely.
- #strong[Substitution:] the F4 arms' full-solve S is a substitution on
  measured records, not a run. It is checked by identical streams and
  verdicts, by shape-determined per-call counts, and by the control
  curve, where F4 ran to full solves.
- #strong[No security statement.] These are generic precomputation
  shapes and no exponent moves. This is not an improvement over the IC
  baseline: the Semaev arm is above known-log at every w \> 0, and
  unresolved across the band.

== Files
<files>
- `src/cryptanalysis/ic_framework/pkm_semaev.rs`: the oracle, the grid
  reference and `decomposable_points`, with unit tests.
- `src/cryptanalysis/ecbench/methods.rs`: registration and the
  end-to-end test.
- `src/cryptanalysis/ecbench/claim.rs`: claim entry.
- `examples/generator_effect_e3.rs`: sizes, `N₃` and `f4cost`.
- `examples/generator_effect_e3_summary.rs`: aggregation.
- `examples/a_minus_3_prime_order_curves.rs`: gains the optional
  `MIN_V2` filter.
- `e3/specs/`, `e3/sessions/`, `e3/summary.json`, `e3/plot_e3.py`,
  figures.
- `E3.pdf` (this report), built by `build_e3_pdf.sh`.
