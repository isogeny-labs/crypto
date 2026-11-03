#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[The weak-class cover row re-priced in `bitop/v1`, both arms executed]
  #v(2pt)
  `research/ic_acct1_cover_bitop_20261021` · runs 2026-10-21 · report 2026-10-26 · class: accounting
]

= Question

The scoreboard row "Whole method from a curve that is not weak (section 18):
$p = 1511$ (subgroup $2^61.4$), 4 seeds" reads `0.010×`. Its denominator was
never executed: `run_end_to_end` divides by a pooled rho $S = 1.3609$ from
smaller sizes, in the legacy "$FF_p$ multiplication equivalent" unit (additions
free, flat Wiedemann and inversion prices, probes free).

*Does the row stay below Pollard rho when both arms run on the same challenge
curve and seed, and both are counted in `bitop/v1` primitives?*

This is a re-measurement of an existing algorithm against an executed
reference. It is not an improvement of the index-calculus baseline.

= Protocol and decision rule

`PROTOCOL.md`, frozen in commit `9a6e3de35` before any measured run.

- *Instances.* Ledger §18.4: the weak instance of `generate_spec(p, seed)`
  moved `moves` isogeny steps off the weak locus to the challenge curve $C$.
  Primary: $p = 1511$ ($ell approx 2^61.4$), seeds 1–4, `moves = 8`.
  Secondary: $p = 1511$, `moves = 64`; $p = 1009$ ($ell approx 2^57.9$),
  `moves` $in {8, 64}$.
- *IC arm.* The §18.4 route on $C$, every phase charged (walk, transport,
  model and cover, factor base, relations, descent, linear algebra mod $ell$,
  verification), one thread per run, cold F4 trace cache.
- *Rho arms on $C$ itself.* R3 (primary reference): negation map with
  window-8 cycle detection, batched inversion $k = 16$; 16 runs per $p$.
  R0: no negation, one inversion per step; 8 runs per $p$.
- *Measure.* IC/rho as a ratio of mean `bitop/v1` per solve, 95% percentile
  bootstrap ($10^4$ resamples, seed 20261021). The §5a band re-prices every
  arm's `word_op` and `hash_probe` work at $w = 0, 0.1, 1$ GAE per operation
  (1 GAE $= 5.11 dot 10^5$ bitops at $p = 1511$, $4.26 dot 10^5$ at $p = 1009$).
- *Decision rule.* $U$ and $L$ are the largest and smallest 95% bounds over
  the `bitop/v1` interval and the three band intervals. Stays below rho if
  $U < 1$; does not if $L > 1$; otherwise, or on any failed run, unresolved.
  If a control fails the verdict is withheld. Only the primary row ($p = 1511$,
  `moves = 8`, R3) answers the question.

= Results

*Primary row: stays below rho.* IC/rho $= 0.0098$, 95% CI $[0.0069, 0.0144]$;
$U = 0.0751$ (the $w = 1$ band). All 4 IC runs and all rho runs solved and
verified $[d]G = Q$ on $C$; all controls passed.

*Distance remaining to rho.* On this weak class (full rational 2-torsion and a
genus-3 cover; it contains no prime-order curves) the IC arm costs about 1% of
the strong rho at $ell = 2^61.4$ in `bitop/v1` ($8.16 dot 10^12$ against
$8.34 dot 10^14$ bitops), and at most 7.5% at the upper end of the §5a band.
That gap is the measured position, not a pass mark; it says nothing about
curves outside the weak class.

#figure(image("figures/ratio_vs_rho.svg", width: 100%),
  caption: [IC/rho per row (`analysis.json`, `rows`). Blue: `bitop/v1` ratio of
  means with its 95% bootstrap CI. Orange: the §5a band, with the ratio at each
  weight and a shaded span from the smallest lower to the largest upper 95%
  bound over the three weights. 4 IC instances per row; 16 R3 and 8 R0 rho runs
  per $p$. Dashed: IC = rho.])

#{
set text(size: 7.5pt)
set par(justify: false)
table(columns: 9, stroke: 0.4pt, inset: 3pt, align: (right, right, left, right, right, left, left, left, left),
  [*p*], [*moves*], [*rho*], [*IC mean*], [*rho mean*], [*IC/rho [95%]*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [*1511*], [*8*], [*R3*], [*8.16e12*], [*8.34e14*], [*0.0098 [0.0069, 0.0144]*], [0.0096 [0.0068, 0.0142]], [0.0422 [0.0298, 0.0625]], [0.0510 [0.0357, 0.0751]],
  [1511], [8], [R0], [8.16e12], [9.98e14], [0.0082 [0.0058, 0.0120]], [0.0080 [0.0057, 0.0115]], [0.0647 [0.0461, 0.0939]], [0.1013 [0.0719, 0.1487]],
  [1511], [64], [R3], [7.61e12], [8.34e14], [0.0091 [0.0064, 0.0136]], [0.0090 [0.0062, 0.0133]], [0.0390 [0.0271, 0.0584]], [0.0471 [0.0325, 0.0708]],
  [1511], [64], [R0], [7.61e12], [9.98e14], [0.0076 [0.0054, 0.0111]], [0.0075 [0.0053, 0.0110]], [0.0598 [0.0419, 0.0882]], [0.0935 [0.0657, 0.1391]],
  [1009], [8], [R3], [6.64e12], [2.31e14], [0.0288 [0.0201, 0.0421]], [0.0283 [0.0199, 0.0413]], [0.1180 [0.0817, 0.1725]], [0.1423 [0.0986, 0.2093]],
  [1009], [8], [R0], [6.64e12], [2.45e14], [0.0271 [0.0172, 0.0492]], [0.0265 [0.0169, 0.0476]], [0.2038 [0.1285, 0.3657]], [0.3181 [0.2020, 0.5837]],
  [1009], [64], [R3], [9.73e12], [2.31e14], [0.0422 [0.0274, 0.0650]], [0.0414 [0.0268, 0.0639]], [0.1757 [0.1141, 0.2680]], [0.2120 [0.1376, 0.3268]],
  [1009], [64], [R0], [9.73e12], [2.45e14], [0.0397 [0.0236, 0.0747]], [0.0388 [0.0230, 0.0729]], [0.3033 [0.1807, 0.5685]], [0.4739 [0.2801, 0.8863]],
)
}

Every row's verdict under the same rule is "below rho". Means are bitops per
solve. The legacy column (descriptive, against the pooled 1.3609) reads 0.0049
for the primary row on these cold, one-thread runs; against the executed R3
($S = 0.896$) it reads $approx 0.0075$.

#figure(image("figures/ic_phases_primary.svg", width: 100%),
  caption: [Where the IC arm's bitops go at the primary row, mean of 4 instances
  (`ic_phase_bitops_mean`). Unattributed: 0. For comparison, rho R3's walk alone
  is $8.33 dot 10^14$ bitops at $p = 1511$.])

The sieve dominates (66%), then the descent (23%) and linear algebra modulo the
62-bit $ell$ (10%). The sieve's per-$x$ factor-base tests and counter updates
are its main native work, which is why the §5a band raises the ratios 4–12×.

= Controls (all pass)

+ *Positive control, rho.* Mean steps / $sqrt(ell)$: R0 1.120 (16 runs, window
  $[0.9, 1.7]$); R3 0.946 (32 runs, window $[0.55, 1.25]$).
+ *A/A, rho.* R3 run index 0 vs 1: 1.07 $[0.57, 1.83]$ at $p = 1009$, 1.30
  $[0.75, 2.19]$ at $p = 1511$; both contain 1.
+ *A/A, IC.* A second process re-ran $p = 1511$, seed 1, `moves = 8`:
  identical per-phase counts and legacy total.
+ *IC reproduction.* Walk, transport and model legacy counts equal
  `experiments/40_jv_cover_e2e_fast.json` in 16 of 16 runs.
+ *Completeness.* Zero hook calls on unclaimed threads and zero operations on
  an unclassified modulus in all 16 IC runs.
+ *Instrument unit tests.* `Fq3` price vectors 55/44 and 150/104/1; counted rho
  equals uncounted rho and verifies; rho on $C$ recovers $d$; hooks classify by
  modulus.

= Deviations

+ *Host kill, card 2.* The container restarted during card 2's second command
  (R0 at $p = 1511$). Its first command (R3, 16 complete runs) is kept; the one
  completed R0 run of the killed command is set aside in `raw/killed_card2/`
  and equals its re-run exactly. All 8 R0 runs were re-run by card 2b with
  identical seeds.
+ *Concurrency.* Cards 1–4 ran together (16 workers on a shared 4-core host).
  Counts do not depend on the schedule; wall times are descriptive only.

= Limitations

- *No instruction audit.* `ecbench audit-ir` was not run, so neither arm is an
  admitted `bitop/v1` bound in the sense of `BITOPS.md`. The result is a
  measured ratio under the convention.
- *Known uncounted:* memory; control flow and allocation; RNG internals beyond
  one word per draw; the standard-monomial search of `solve_zero_dim`'s linear
  stage. F4's symbolic preprocessing and pair criteria are charged
  structurally (in `word_op`/`hash_probe`, inside the §5a band), not
  instruction by instruction.
- *Rho-favouring price (descriptive).* Tuned $FF_(q^3)$ prices would cut rho's
  product, squaring and inversion counts to 0.33–0.44 of the harness prices;
  the primary upper bound 0.0144 would become at most about 0.044.
- *Scope.* Weak class only; two primes; 4 IC instances per row. The cold,
  one-thread runs differ from the committed §18.4 runs (trace cache, threads,
  round plan), so the legacy figure is 0.0049 here rather than 0.010.
- *Validation gate.* No `VALIDATION.json`, replay certificate or validator
  report exists for this directory, so the row is not shown on the IC
  leaderboard or in `benchmarks/README.md`.

= Evidence

#set par(justify: false)
`raw/` (IC and rho JSON lines), `cards/` (task cards and receipts),
`analysis.json` (`examples/jv_cover_bitop_analyse.rs`), `PROTOCOL.md`,
`figures/plots.py` (plots recorded values only).
