#set document(title: "S2b: dense step-column block in the native hyperelliptic IC solve")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)
#show table: set text(size: 8pt)

= S2b: a dense step-column block in the native solve (genus 2–4)

*Study:* `research/hec_ic_speedup_20261113/` (PLAN.md §8, S2B-RESULTS.md). \
*Protocol* frozen at `9e66fa70ce`; *code* `7ef8c38315`; *scripts* `507c1dc511`
(branch `research/hec-s2b-20261114`). \
*Record:* `benchmarks/records/20261114T2233Z-hec-s2b-dense-solve.json`. \
*Command:* `run_s2b.sh s2b-20261114` (`HEC_SOLVER=fast,dense`, 5 repeats), then `summarise_s2b.sh` and
`pairs_s2b.sh`. Raw output: `s2b-20261114/rep1–5.txt`, `binaries.sha256`, `host.txt`. \
*Host:* one arm64 macOS laptop, single thread, *shared*: load average 81–95 during the run
(`host.txt`), so not the L0 condition of S1 and S2. \
*Class:* engineering on the solve stage. Same relation search and group operations (gate G6); more
solve multiplications; wall clock only. No `S` claim, no advance, not validated.

== What changed

`Solver::Dense` splits each row into two parts:

- a *sparse part* over the factor-base columns that are not step places;
- a *dense array* over the $approx 64$ step columns and $k$.

It then eliminates in two phases:

- *Phase 1:* Markowitz over the sparse columns only. Each row operation merges the sparse parts,
  then runs a contiguous $"row" -= c dot "pivot"$ over the dense array, at one REDC per product.
- *Phase 2:* plain elimination of the remaining dense block. A row left with non-zero $k$ gives $k$.

Exact elimination returns $k$ exactly when the rows determine it. So the gate is the same
`Option<k>` on every row set, not the same pivots.

== Pre-registered verdicts

#table(columns: (auto, 1fr, auto), align: left,
  [*id*], [*expectation (PLAN.md §8)*], [*verdict*],
  [E5], [G5: same `Option<k>` as `Fast` on 90 random dense-block systems, with and without a determined $k$. G6: on all 55 runs, the same $k$, trials, relations and solve rounds.], [*holds*: G5 passes; G6 275/275 over 5 repeats; 550/550 $k dot D_1 = D_2$],
  [E6], [genus 2, $p >= 503$: median paired solve ratio Fast/Dense $>= 2 times$ on every row], [*holds*: 4.21, 4.04, 5.49, 5.75×],
  [E7], [genus 2, $p >= 503$: median Dense solve share $< 35%$ on every row], [*holds*: 26, 18, 25, 20 %],
)

== Results

Medians are over 25 (repeat, seed) pairs per row. "Pairs $>= 2 times$" counts the pairs at or above
the E6 bar. Mulmods and row operations are medians for Fast / Dense.

#table(columns: 11, align: right,
  [*g*], [*p*], [*m*], [*Fast ms*], [*Dense ms*], [*ratio*], [*pairs $>= 2 times$*], [*Fast share*], [*Dense share*], [*mulmods F / D*], [*row ops F / D*],
  [2], [101], [46], [0.090], [0.023], [3.90×], [24/25], [40 %], [15 %], [3238 / 7524], [327 / 712],
  [2], [251], [123], [0.368], [0.091], [4.03×], [24/25], [56 %], [24 %], [21008 / 36143], [1233 / 2100],
  [2], [503], [259], [0.944], [0.213], [4.21×], [20/25], [59 %], [26 %], [76422 / 116320], [3370 / 3884],
  [2], [1009], [478], [1.764], [0.425], [4.04×], [21/25], [45 %], [18 %], [216935 / 254084], [6654 / 6877],
  [2], [2003], [1000], [6.766], [0.884], [5.49×], [20/25], [68 %], [25 %], [448339 / 483199], [12587 / 12737],
  [2], [4001], [2005], [13.714], [1.761], [5.75×], [20/25], [61 %], [20 %], [892957 / 915581], [24174 / 24251],
  [3], [61], [33], [0.082], [0.020], [4.10×], [23/25], [18 %], [5 %], [4704 / 7445], [413 / 631],
  [3], [211], [104], [0.508], [0.141], [3.82×], [23/25], [27 %], [9 %], [57978 / 90518], [1966 / 2535],
  [3], [401], [196], [0.874], [0.231], [3.64×], [22/25], [23 %], [8 %], [128227 / 136469], [3345 / 3213],
  [4], [31], [16], [0.034], [0.010], [3.23×], [20/25], [3 %], [1 %], [1953 / 2444], [209 / 231],
  [4], [61], [36], [0.126], [0.033], [4.55×], [24/25], [8 %], [2 %], [12819 / 14107], [708 / 813],
)

#figure(image("ratios.svg", width: 100%),
  caption: [Paired solve ratio Fast/Dense per (repeat, seed); the bar is the median. The spread is
  preemption on a shared host. Record 20261114T2233Z-hec-s2b-dense-solve.])

#figure(image("shares.svg", width: 100%),
  caption: [Median over runs of the solve's share of native wall time, both solvers, same runs.
  Same record.])

#figure(image("scaling.svg", width: 80%),
  caption: [Median solve time against factor-base size $m$. Same record.])

== Reading

- *Wall clock, not counted work.* Dense does 2.5–132 % more multiplications mod $ell$ and about as
  many row operations or more. The gain is layout: the hot columns become a contiguous array with
  one REDC per product, instead of tracked sparse entries. Under accounting that charges
  linear-algebra multiplications (the BigUint engine's `S`), this is a small increase.
- *Scaling.* The genus-2 Dense solve grows about linearly in $m$ (0.21 → 1.76 ms for $m$ = 259 →
  2005); Fast grows faster (0.94 → 13.7 ms). This holds on round eight's sizes only.
- *What is left.* With Dense the solve is 18–26 % of native time at genus 2 ($p >= 503$) and 1–9 %
  at genus 3 and 4. There the oracle and walk dominate, so per-trial cost (plan track T2) is the next
  lever.

== Noise, deviations, limits

- The run happened under load average 81–95, not on a quiet host. Each pair runs both solvers back
  to back in one process, so contention hits both arms. Even so, 4–5 of the 25 pairs in each gated row
  (genus 2, $p >= 503$) fall below 2× and single pairs range from 0.05× to 244×. The pre-registered medians are at least twice
  the bar on every gated row, but they are not a quiet-host measurement. A quiet-host rerun has not
  been done.
- A smoke run after the protocol froze (one process, G6 55/55, ratios 2.7–7.7×) checked the gate.
  It is not part of the verdict.
- No external comparison (§15): FLINT and CADO-NFS `bwc` on exported matrices are planned under T3.
  Not validated: no `VALIDATION.json` or matched rho.
