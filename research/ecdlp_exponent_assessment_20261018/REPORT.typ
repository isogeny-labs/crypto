#set page(margin: 1.6cm)
#set text(size: 9.5pt)
#show table: set text(size: 8pt)

= Where the ECDLP exponent can move, and where it cannot
<where-the-ecdlp-exponent-can-move-and-where-it-cannot>
#strong[Status (assessment, 2026-10-18).] Assessment from committed
evidence; no new speedup is claimed. Every number below cites a frozen
artefact in this repository. Fitted exponents are quoted with their
intervals and the sizes they were fitted on; a `toy`-tier figure is
never carried to another tier (`docs/bounds/README.md`). The companion
sweep (`research/ecbench_comprehensive_sweep_20261018/`) refits the
generic methods on prime-field and Koblitz curves at 2 targets × 3
rounds over nine and six sizes, and a fixed-base index calculus over
six; its three sessions are sealed and replayed, and §5 reads them.

Cost is written `gae = C · r^α` in ecbench\'s counted unit
(group-addition equivalents), `r` the prime subgroup order. \"Moving the
exponent\" means `α < 1/2` on a domain, with the 95 % interval of the
fit excluding `1/2`, over at least four sizes, every answer verified,
and the whole process charged (`docs/bounds/README.md`, acceptance
rule).

== 1. What the frontier says today
<1-what-the-frontier-says-today>
From `docs/bounds/FRONTIER.md` (frontier `ECFR1h8b45b672c283`, 46
bounds):

#align(center)[#table(
  columns: 6,
  align: (col, row) => (left,left,right,left,right,left,).at(col),
  inset: 6pt,
  [domain], [method], [α (95 %)], [sizes (log₂ r)], [ops × floor (95
  %)], [bound],
  [koblitz · medium],
  [`rho.signed_frobenius_strong`],
  [0.451 \[0.399, 0.512\]],
  [32.1 … 47.2 (6)],
  [1.089 \[0.927, 1.261\]],
  [`ECBND1h05a2ea918f3c`],
  [koblitz · medium],
  [`bsgs.negation`],
  [0.503 \[0.474, 0.531\]],
  [32.1 … 47.2 (6)],
  [7.669 \[6.981, 8.375\]],
  [`ECBND1he836821517fb`],
  [koblitz · medium],
  [`claw.pair_table`],
  [0.467 \[0.423, 0.499\]],
  [32.1 … 47.2 (6)],
  [2.390 \[2.160, 2.639\]],
  [`ECBND1h25ac58612850`],
  [koblitz · medium],
  [`rho.negation`],
  [0.473 \[0.125, 0.553\]],
  [27.8, 32.1, 39.0],
  [6.516 \[5.780, 7.245\]],
  [`ECBND1h83091af19aee`],
  [koblitz · medium],
  [`kangaroo.vow`],
  [0.486 \[−0.312, 0.571\]],
  [27.8, 32.1, 39.0],
  [15.558 \[13.742, 17.597\]],
  [`ECBND1h7f1d4d59799f`],
  [koblitz · toy],
  [every generic method],
  [intervals contain 0.5 and are wide],
  [15–22],
  [3.3 – 30],
  [table 1 of the page],
  [prime · toy],
  [`ic.pipeline` prime-abscissa 32, MITM (negation-folded)],
  [0.841 \[0.500, 1.024\]],
  [17.7, 19.1, 21.2, 24.0],
  [22.686 \[11.421, 40.107\]],
  [`ECBND1hbab6d6dc37fc`],
  [prime · toy],
  [`ic.pipeline` prime-abscissa 32, subtract oracle],
  [1.005 \[0.786, 1.099\]],
  [17.7, 19.1, 21.2, 24.0],
  [1258.2 \[395.6, 2482.9\]],
  [`ECBND1h9d3beb6852c6`],
)
]

Reading. Every generic method fits `α ≈ 1/2`, as it must (§2). The
`claw.pair_table` interval `[0.423, 0.499]` sits just under `1/2` on six
sizes; it is a `√r`-time, `r^{1/4}`-memory meet-in-the-middle whose
fitted exponent drifts because its table share changes with `r`. Its ops
are 2.4× the floor, so it is not a candidate: an exponent claim on it
would be an artefact of the constant shifting across the range, the
pattern `docs/bounds/README.md` warns of. The two prime-field
index-calculus rows fit `α = 0.84 [0.50, 1.02]` and `1.01 [0.79, 1.10]`
on four sizes at a fixed 32-point factor base: with the base held
constant the decomposition step grows like `r/|F|^{m-1}`, so the fitted
exponent is #emph[above] `1/2`, the opposite direction from a win, and
23× and 1258× the floor in ops.

== 2. Why the generic exponent cannot move
<2-why-the-generic-exponent-cannot-move>
For any algorithm that uses `E(F_p)` only through the group law and
equality tests, Shoup (1997) gives `T ≥ Ω(√r)` online, and with `S`
words of target-independent advice Corrigan-Gibbs–Kogan (2018) give
`S·T² ≥ Ω̃(r)`. `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1 prices
this repository\'s own methods against that line: rho `S = 0`,
`T = √(πr/4)`; preprocessing rho `S = r^{1/3}`, `T ≈ 2r^{1/3}` (tight);
every index-calculus variant whose decomposition is a table look-up
(`IC-2`, `IC-3`, `IC-m`) lands on or above `S·T² ≥ r/4`. So on
prime-field curves every method in ecbench\'s registry is bound to
`α = 1/2` on a single target, and the only movable quantities are:

- #strong[the constant] through usable automorphisms: `√2` from negation
  (`rho.negation`, `bsgs.negation`), `√3` and `√2` from the order-6 and
  order-4 CM automorphisms (`rho.cubic`, `rho.quartic`: measured 0.70
  and 0.71 of `rho.negation`, `benchmarks/README.md`, 2026-10-17),
  `√(2n)` from signed Frobenius on Koblitz curves
  (`rho.signed_frobenius_strong` at 1.089× its own floor, medium tier);
- #strong[the walk\'s distance to its floor]: the medium-tier Koblitz
  rho is within 9 % of `√(π r / 4n)`, the toy-tier generic rows 3–10×
  above theirs (start-up and distinguished-point overheads at
  `r < 2^22`);
- #strong[accounting across targets]: Kuhn–Struik\'s `L(k)` for `k`
  targets (table C of the leaderboard) and Bernstein–Lange
  precomputation (`r^{1/3}` online after `r^{2/3}` setup) lower the
  per-target cost without lowering `S·T²`.

None of these is an exponent move, and the success criterion does not
ask for one: each is a measured improvement over the standing baseline
when it lowers end-to-end `S` on the same instances (AGENTS.md,
\"Success criterion\").

== 3. Where the exponent is movable in principle, and what the evidence
says
<3-where-the-exponent-is-movable-in-principle-and-what-the-evidence-says>
=== 3.1 Koblitz curves over F\_{2^n}: Frobenius-invariant index calculus
<31-koblitz-curves-over-f_2n-frobenius-invariant-index-calculus>
The lever is real (`RESEARCH_KOBLITZ_INDEX_CALCULUS.md`, \"Where the
speed-up lives\"): a Frobenius-invariant factor base collapses `|F|`
unknowns to `|F|/n` orbit unknowns, saving `n` relations and `n²` in the
matrix, and `m!` in the decomposition. The measured record
(`docs/ic/BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md`, `vs_rho`
rows):

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,right,).at(col),
  inset: 6pt,
  [round], [best cold row], [fitted total α (R², sizes)], [× counted
  signed-Frobenius rho],
  [1 (2026-09-21)],
  [`n = 41` signed-orbit fold, `S = 58.8`],
  [0.54 (0.71, 10 rungs); 0.51 (0.98, cofactor ≤ 4)],
  [300×],
  [2],
  [folded pair table, walked targets, `S = 5.12`],
  [0.43 (0.82, 4 rungs)],
  [26.2×],
  [3–5],
  [restart at one addition, lazy offsets, `S = 5.11`],
  [unchanged family],
  [26.1×],
  [leaderboard table A (rule v3)],
  [`n = 47`, `S = 9.51` vs reference 1.82],
  [–],
  [5.23×],
  [§20, `k = 32` batch (`research/ic_exponent_20260926`)],
  [thread / batch rho 4.86× \[4.21, 5.55\] at 2^39],
  [ratio exponent β \= 0.137 \[−0.10, 0.37\]],
  [13.6× at 2^47.2],
)
]

Reading. The best fitted total exponent, `0.43` over four rungs with
`R² = 0.82`, has no interval narrow enough to exclude `1/2`, and the
family\'s derived optimum is itself ≈ 30× the asymptotic
signed-Frobenius rho (round 3). The §20 ratio exponent `β` contains `0`,
so \"IC/rho shrinks with `r`\" is not established either. What stops the
exponent from moving is named precisely in the Koblitz note: every
oracle works at `m = 2, 3`, a useful attack needs `m ≈ n/ℓ`, and the
chaining representation needs `unknowns(n, ℓ, m) = m·ℓ + (m − 2)·n`
variables (633 at `n = 63, ℓ = 6` against a 64-variable budget), while
the measured first fall degree never exceeds 3. The algebra is benign;
the variable count is the wall. #strong[This is the one place in the
program where the exponent is movable with known mathematics
(`Õ(2^{(2−2/m)·n/…})` for `m` summands) and where the blocker is a
representation problem, not a lower bound.]

=== 3.2 Extension-field curves: Gaudry\'s k \= 4 and the F\_(2^m)^4
analogue
<32-extension-field-curves-gaudrys-k--4-and-the-f_2m4-analogue>
`ic.k4_gaudry` and `ic.pipeline` on `fp4_translation` and `F2m4`
workloads implement Gaudry–Diem index calculus with `m = n = 4`
summands. Its heuristic cost `Õ(p^{2−2/n}) = Õ(p^{1.5})` against rho\'s
`√r ≈ p^2` is `α = 3/8` in `r`: an exponent move on paper, at constants
(the `S₅` summation polynomial and its Gröbner/root-finding step) that
this repository has measured only on toy fields. These are the correct
domains for an exponent claim because the sub-`1/2` exponent is a
theorem-level heuristic, not a hope; the work is to drive the constant
down and run four sizes with the whole process charged. The
`research/ecbench_rootfinding_*` studies (2026-10-12…14) and
`research/gaudry_quartic_d4_ecbench_20261013` are the standing
baselines.

=== 3.3 Prime-field curves: closed on paper, probes negative
<33-prime-field-curves-closed-on-paper-probes-negative>
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4 lists the only levers
with a falsifier each: small roots beyond Coppersmith for the `S_{m+1}`
shape (milestone `δ ≥ 1/4` at `m = 3`, target `δ ≈ 1/5` at `m = 4`;
Probes A–C negative), a factor base with sub-`B`-degree membership (none
exists in `F_p`), transfer to a structured setting (the isogeny volcano
studies of 2026-10-16/17 find no level effect:
`floor/crater 1.022 [0.983, 1.062]`), and the closed-on-paper items of
§4.6 (symmetric `S₃`, subgroup bases, xedni, embedding into an
extension, which costs `p^{2−2/n}` against `√p` and enlarges the
problem). The leaderboard\'s prime-field IC rows (table B) sit 5.5–13.5×
rho with their family optimum `Θ(r^{1/6})` above a flat reference: no
crossover at any size. #strong[No exponent move is available here; the
honest program is constants (§2) and accounting.]

== 4. Recommendations, ranked by expected movement per unit of work
<4-recommendations-ranked-by-expected-movement-per-unit-of-work>
+ #strong[Koblitz chaining at `m ≥ 4` with a variable budget that
  scales.] The wall is `unknowns = m·ℓ + (m − 2)·n`. Replace the full
  chain with the Frobenius-orbit-reduced chain (unknowns per orbit, not
  per point) and measure the first fall degree and solve time at
  `m = 4, 5` on `n = 23, 29, 31, 37`; a four-size fit with the interval
  of `α` excluding `1/2` is the acceptance test. Protocol:
  `ecbench challenge` against `ECBND1h05a2ea918f3c` (the medium-tier rho
  holder).
+ #strong[Gaudry `k = 4` end to end at four field sizes.] `ic.k4_gaudry`
  on `fp4_translation` with the root-finding oracle from the 2026-10-13
  tuning; whole process charged; aim for the `α = 3/8` fit to exclude
  `1/2`. Even at constants far above rho this is the first row the
  program can label #emph[exponent-level] honestly.
+ #strong[Generic constants to the floor.] The toy-tier generic rows sit
  3–10× above `√(π r / 2A)`; the medium-tier Koblitz rho is at 1.09×.
  The companion sweep quantifies the prime-field gap at `2^16–2^32`
  (§5). Start-up (two scalar multiplications per walk), distinguished
  point density and the jump-table build are the whole of that gap; none
  is an exponent, all are improvements under the success criterion.
+ #strong[Kernel engineering with identical counts] (the two companion
  passes of this date on the Rust and C kernels): wall-clock only, no
  change to any `S`, recorded as engineering.

== 5. Companion sweep (filled from the sealed sessions)
<5-companion-sweep-filled-from-the-sealed-sessions>
`research/ecbench_comprehensive_sweep_20261018/README.md` holds three
sealed sessions of this date, every run verified and every measured run
replayed identically (`ECBS1ha345d9b7f6b2`, `ECBS1h88c58e3e8c89`,
`ECBS1hec2206c6baf4`; 702 measured runs, 702 replays). Descriptive OLS
exponents, `α ± se`:

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,right,left,).at(col),
  inset: 6pt,
  [domain], [method], [`α` ± se (sizes)], [mean `S` range across sizes],
  [prime, `2^16 … 2^32`],
  [`rho.plain`],
  [0.474 ± 0.013 (9)],
  [2.07 → 1.35 (floor 0.886)],
  [prime],
  [`rho.negation`],
  [0.447 ± 0.012 (9)],
  [1.91 → 0.91],
  [prime],
  [`bsgs.textbook`],
  [0.499 ± 0.014 (9)],
  [1.14 … 1.82],
  [prime],
  [`bsgs.interleaved`],
  [0.501 ± 0.023 (9)],
  [0.85 … 1.84],
  [prime],
  [`bsgs.negation`],
  [0.498 ± 0.022 (9)],
  [0.64 … 1.32],
  [prime],
  [`kangaroo.vow`],
  [0.487 ± 0.032 (9)],
  [1.07 … 3.00],
  [Koblitz `K_1`, `2^15 … 2^37`],
  [`rho.signed_frobenius_strong`],
  [0.248 ± 0.015 (6)],
  [4.54 → 0.11 (floor 0.215 → 0.129)],
  [Koblitz],
  [`rho.negation`],
  [0.430 ± 0.029 (6)],
  [1.97 → 1.08],
  [Koblitz],
  [`bsgs.negation`],
  [0.511 ± 0.015 (6)],
  [0.84 … 1.35],
  [Koblitz],
  [`kangaroo.vow`],
  [0.562 ± 0.021 (6)],
  [0.79 … 2.97],
  [prime, `2^16 … 2^25`],
  [`ic.pipeline`, prime-abscissa 32, MITM],
  [0.757 ± 0.051 (6)],
  [10.9 → 59.0; 5.7× → 43× `rho.negation`],
  [prime],
  [`rho.frozen_reference`],
  [0.404 ± 0.037 (6)],
  [5.24 → 2.17],
)
]

Reading. The BSGS and kangaroo fits contain `1/2`. The two rho fits
below `1/2` are the start-up artefact of §1: `S` falls across the range
while the floor is constant, because the two scalar multiplications per
walk and the jump-table build are a larger share at `2^16` than at
`2^32`. The Koblitz strong reference shows it at scale (32 lanes of
start-up before a step; 20× the floor at `2^16`, at the floor by
`2^32`), so its `0.248` is not an exponent and the medium-tier frontier
fit `0.451 [0.399, 0.512]` on `2^32 … 2^47` stands as the admitted
value. The fixed-base index-calculus row fits above `1/2`, consistent
with the frontier\'s `0.841 [0.500, 1.024]` on the same configuration,
and its gap to rho widens with `r`. Nothing in the sweep moves an
exponent; it quantifies the generic constants recommendation 3 names: at
`2^28 … 2^32` the tuned rho arms are within their own model\'s
expectation (1.25 and 0.89) and the gap to the floor is 1.0 – 1.5× in
the counted unit.

== 6. Graphs
<6-graphs>
`exponent_map.svg` (beside this file, from `exponent_map.py`, with the
plotted values in `exponent_map.diagram.json`): the exponent `α` per
domain and method with its interval against the `1/2` line, by evidence
class (frontier bound with its bootstrap interval, this sweep with
`± 1.96 se`, fitted without a published interval, on paper).
`REPORT.pdf` embeds it. The sweep\'s own graphs (mean `S` per arm and
size, distributions, phases, scaling fits) are under
`research/ecbench_comprehensive_sweep_20261018/reports/*/`.

== References inside this repository
<references-inside-this-repository>
- `docs/bounds/FRONTIER.md`, `docs/bounds/README.md`
- `docs/ic/LEADERBOARD.md`,
  `docs/ic/BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md`
- `research/ic_exponent_20260926/README.md`, `analysis.json`
- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`,
  `RESEARCH_KOBLITZ_INDEX_CALCULUS.md`
- `benchmarks/README.md` rows of 2026-10-16/17 (volcano studies,
  `rho.cubic`/`rho.quartic`)

#pagebreak()
#figure(
  image("exponent_map.svg", width: 100%),
  caption: [The ECDLP cost exponent α by domain and method, with intervals, against α = 1/2. Frontier bounds carry their 95 % cluster-bootstrap interval; the companion sweep's fits carry ± 1.96 standard errors; the Koblitz index-calculus rounds and the on-paper values have none. Values in `exponent_map.diagram.json`.],
)
