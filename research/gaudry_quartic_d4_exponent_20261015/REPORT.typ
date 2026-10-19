#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 14pt, weight: "bold")[`k = 4` index calculus on $E(FF_(p^4))$, measured]\
  #v(2pt)
  relations $prop p^(0.96)$, Wiedemann $prop p^(2.04)$, rho $prop p^(1.93)$;
  crossover from the fits $2^(98)$ ($h = 4$), sensitivity $2^(82)$–$2^(127)$\
  #v(2pt)
  `research/gaudry_quartic_d4_exponent_20261015` · 2026-10-15 · class: measurement
]

*Summary.* Gaudry/Diem index calculus with FGHR's $D_4$ oracle (PR \#1863) and rho
with the negation map were run end to end in ecbench on `fp4_torsion` curves for
12 primes $p = 29 dots 1009$: all 72 planned jobs, 312 records, all verified ($[d]G = Q$), `S` a count.
Fitted over the rungs, the relation phase grows like $p^(0.96)$ (95 %: 0.65–1.27),
the linear algebra (Wiedemann, new here) like $p^(2.04)$ (1.85–2.22), and rho like
$p^(1.93)$ (1.78–2.09), as the analysis says ($q^1$, $q^2$, $q^2$). Index calculus
still loses to rho at every measured size ($9.1 dot 10^5 times$ at $p = 29$,
$6.7 dot 10^4 times$ at $p = 1009$). From the measured fits, it meets rho at a group
of $2^(97.9)$ ($h = 4$; bootstrap $2^(94.6)$–$2^(102.5)$); with the relations slope
at its interval ends, $2^(82.1)$–$2^(127.4)$. The §11.16 model gave
$2^(105.4)$–$2^(106.5)$. *This extrapolates 14.5 bits in $p$, 58 bits in group
size, beyond the top rung.* At that point the relation phase is still 97 % of the
cost. Double large primes lower the fitted linear-algebra slope to 1.70 (interval
1.25–2.15, consistent with both the theory's 1.5 and the plain 2.04) but cost
1.09–2.31× more on paired walks at $p >= 61$, and move the fitted crossover
later ($2^(128)$). The first 60 jobs gave $2^(100.4)$; the last 12 jobs were run
afterwards at the owner's request (§Last 12 jobs).

= Method

Protocol `PROTOCOL.md`, registered at `08722a2b7` before any session; amendments:
(1) queue reordered and stop extended to 16 h (runtime, before any IC record);
(2) audit scope by size; (3) relaunch of 20 jobs after the container's CPU set
shrank; (4) the 12 jobs that missed the stop run at the owner's request, with a
corrected count. Code `0bea75038`. Arms (all `fghr-y-line:zero=1`, oracle
`gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1,charge_unsolved=1`):
`ic-d4-w` (`linalg=wiedemann`), `ic-d4-lp2`/`ic-d4-lp1` (`large_primes=2`/`1`,
small base the first $ceil(N^theta)$ columns, $theta = 3/4$, $7/9$), `rho-neg`.
One spec per (arm, $p$, target); the IC arms on a target share the walk. 60 of 72
jobs ran before the 16 h stop; the other 12 (third plain targets at five small
rungs, second LP2 targets at seven) were run afterwards at the owner's request
(amendment 4, same specs and binary). Earlier versions of this report said 59 and
13: that was a miscount.

*New code.* `linalg=wiedemann` solves for the target through a left-kernel vector
of the relation matrix, $O(m dot "nnz" + m^2)$ multiplications mod $r$, priced as
`row_ops`; the default dense elimination ran on the same rows as an unpriced
shadow and *gave the same logarithm in all 60 IC runs*. The large-prime wrapper
works on any summand oracle; the existing large-prime code is binary-field only.

*Fits.* OLS of $ln("rung mean")$ on $ln p$; interval = union of the Student-$t$
95 % interval and a bootstrap (10,000, targets resampled within rungs). The
crossover solves $sum_k A_k p^(beta_k) = overline(S)_"rho" sqrt(p^4 slash h)$.

#figure(image("figures/phases_vs_p.svg", width: 92%),
  caption: [Cost by phase and rho against $p$ (rung means; light dots: single
  targets). The dense elimination is derived from its `row_ops` at the same
  price and is not part of `S`.])

= Results

#table(columns: (auto, auto, auto, auto), stroke: 0.3pt, inset: 4pt,
  [*quantity*], [*$beta$*], [*95 % interval*], [*expected*],
  [relations phase (= total)], [*0.96*], [0.65–1.27 (bootstrap 0.90–1.03)], [$q^1$],
  [trials], [0.85], [0.54–1.17], [],
  [linear algebra, Wiedemann], [*2.04*], [1.85–2.22], [$q^2$],
  [dense elimination, same rows (derived)], [2.34], [2.12–2.56], [$q^3$ worst case],
  [rho (224 distinct runs)], [*1.93*], [1.78–2.09], [$q^2$],
)

The relations interval is wide because trials per relation vary between curves
(rung mean trials over mean relations: 56 at $p = 29$, 75 at 61, 383 at 53, 496
at 37, 81 at 1009), which the within-rung bootstrap does not see. One $D_4$ solve costs $6.1 dot 10^5$ GAE at
$p = 29$ and $8.3 dot 10^5$ at $p = 1009$; 171 of 220,652 systems (0.08 %) were
degenerate and are charged. $overline(S)_"rho" = 1.08$ (1.02–1.15), drifting from
1.6 at $p = 29$ to 0.9–1.0 at the top.

#table(columns: (auto, auto, auto, auto), stroke: 0.3pt, inset: 4pt,
  [*crossover (group size)*], [*$h = 2$*], [*$h = 4$*], [*$h = 16$*],
  [measured fits, point], [$2^(96.0)$], [*$2^(97.9)$* ($p^* = 2^(24.5)$)], [$2^(102.0)$],
  [bootstrap within rungs], [$2^(92.8)$–$2^(100.4)$], [$2^(94.6)$–$2^(102.5)$], [$2^(98.4)$–$2^(106.8)$],
  [relations slope at 0.65 / 1.27], [$2^(80.6)$–$2^(124.6)$], [*$2^(82.1)$–$2^(127.4)$*], [$2^(85.2)$–$2^(133.2)$],
  [§11.16 model (\#1863)], [$2^(103.0)$–$2^(103.7)$], [$2^(105.4)$–$2^(106.5)$], [—],
)

With rho's own fitted power law in $p$: $2^(105.4)$ ($2^(99.9)$–$2^(113.7)$). The
top rung is a $2^(39.9)$ group, so the crossover is *58 bits of group size (14.5
bits of $p$) beyond the measurements*, 7.5–8.6 bits of group size earlier than the
model. The honest interval contains the model's figure; the bootstrap interval
does not see between-curve scatter and understates the uncertainty. With the
dense elimination priced instead, the cost meets rho at $2^(98.9)$ ($h = 4$) and
rises above it again at $2^(128)$; at $h = 16$ it meets it at $2^(106.3)$ and is
above again from $2^(114)$. With Wiedemann the fitted cost stays below rho through
the extrapolated range ($p <= 2^(34)$).

#figure(image("figures/crossover.svg", width: 92%),
  caption: [Extrapolation of the fitted phases to the crossover with rho. Grey:
  the measured range. Blue band: bootstrap; hatched: relations slope at its
  interval ends; orange: the §11.16 model at $h = 4$.])

= Large primes

#table(columns: (auto, auto, auto, auto, auto), stroke: 0.3pt, inset: 4pt,
  [*arm*], [*runs (rungs)*], [*lin. alg. $beta$*], [*relations $beta$*], [*crossover $h = 4$*],
  [none], [32 (12)], [2.04 (1.85–2.22)], [0.96 (0.65–1.27)], [$2^(97.9)$],
  [two, $theta = 3/4$], [16 (8, $p <= 293$)], [*1.70* (1.25–2.15)], [1.26 (0.65–1.86)], [$2^(127.7)$],
  [one, $theta = 7/9$], [12 (6, $p <= 149$)], [1.62 (1.11–2.14)], [1.49 (0.43–2.55)], [$2^(178)$ (153–214)],
)

Paired $S$ ratios (same target and walk), two large primes, two targets per
rung: 1.00/1.00, 1.00/1.00, 0.91/1.12, 1.09/1.36, 1.09/1.32, 1.22/1.80, 1.80/2.31,
1.44/2.28 for $p = 29 dots 293$; one: 0.89–1.12 at $p <= 53$, 1.78–3.45 at
$p >= 61$. The matrix shrinks as the analysis says (Wiedemann 3.1–4.7× cheaper at
$p = 293$) and the fitted slope falls from 2.04 to 1.70, but the intervals
(1.25–2.15, 1.85–2.22) overlap, so it is consistent with the theory's 1.5 and with
2.04. The matrix is $10^(-6)$ of the cost here and 3 % at the crossover; the extra
trials land on the relations phase. Large primes would pay only where the linear
algebra dominates, past about $2^(117)$. With the relations slope at 1.86 the
double-large-prime cost never meets rho below $p = 2^(64)$.

#figure(image("figures/large_primes.svg", width: 92%),
  caption: [Linear algebra and total cost with and without large primes.])

#figure(image("figures/slopes.svg", width: 80%),
  caption: [All fitted exponents with their 95 % intervals.])

= Last 12 jobs: the fit before and after

The 60-job fit (`raw/summary-60-jobs.json`, commit `9fb5c60e6`) was computed and
read before the decision to run the remaining 12; it is kept. The 72-job fit is the
headline because it uses the full planned set.

#table(columns: (auto, auto, auto), stroke: 0.3pt, inset: 4pt,
  [*quantity*], [*60 jobs*], [*72 jobs*],
  [relations $beta$ (plain)], [1.00 (0.74–1.26)], [0.96 (0.65–1.27)],
  [Wiedemann $beta$], [2.04 (1.85–2.22)], [2.04 (1.85–2.22)],
  [rho GAE $beta$], [1.93 (1.77–2.08)], [1.93 (1.78–2.09)],
  [LP2 linear algebra $beta$], [1.57 (1.12–2.01)], [1.70 (1.25–2.15)],
  [crossover $h = 4$ (point; bootstrap; slope band)], [$2^(100.4)$; $2^(98.0)$–$2^(103.6)$; $2^(85.9)$–$2^(124.9)$], [$2^(97.9)$; $2^(94.6)$–$2^(102.5)$; $2^(82.1)$–$2^(127.4)$],
  [crossover, LP2, $h = 4$], [$2^(123.9)$], [$2^(127.7)$],
)

= Predictions

P1 held: 312/312 verified, 290/290 replays reproduced. P2 held (0.96 ∈ [0.9, 1.3]). P3 held (2.04).
*P4 not held* (dense elimination 2.34 < 2.4). P5 held (1.93; $S_"rho"$ slope
−0.15). P6: crossover $2^(97.9) in [2^(88), 2^(115)]$ held; *"no dense crossover
below $2^(200)$" not held* ($2^(98.9)$, recrossing at $2^(128)$). P7 held (1.70;
1.26 > 0.96; every paired ratio at $p >= 101$ above 1: 1.09–2.31). P8 held.

= Audit

`ecbench verify` on all 72 sessions (receipts `audits/<job>.audit.json`; scope
fixed by amendment 2): *audit OK on all 72, 312 records, 0 problems.* Full replay
(`--replay-all`) on 61 sessions, every session with $p <= 197$, every rho-only
session and the spot checks `main-p293-t0` and `main-p401-t0`: *290 replays
reproduced exactly*. Integrity only (stored $[d]G = Q$ checks, every planned run
recorded, hashes) on the other 11: `main-p1009-t0/t1`, `main-p701-t0/t1`,
`main-p509-t0/t1`, `main-p401-t1`, `main-p293-t1/t2`, `lp2-p293-t0`,
`lp2-p293-t1`. The first 60 sessions were audited on 2026-10-16 and the last 12 on
2026-10-19, with the same `ecbench` binary.

= Limits

The exponents are fitted over $p = 29 dots 1009$ (groups $2^(19)$–$2^(40)$); the
crossover is an extrapolation of those fits, not a measurement, and its width is
set by curve-to-curve scatter in the relation rate. The large-prime fits rest on
two targets per rung over $p <= 293$. $S$ is a count on one host class;
wall time is not admitted. No `docs/bounds` domain exists for $FF_(p^4)$ curves,
so there is no challenge verdict. Study files: `README.md`, `raw/summary.json`,
`sessions/`, run reports `reports/run-20261015T0457Z/` and `reports/run-20261019T1612Z/`.
