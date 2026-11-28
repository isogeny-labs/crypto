#set document(title: "Genus-5 cover slope test")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Genus-5 cover against a SIMD rho, q = 16001…65521:\ IC stays above rho (3.9× → 1.4×); slopes 1.2 vs 2.0, crossover extrapolated near q ≈ 9·10#super[4]]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/slope_20261120` · 2026-11-20 · follows `next_leads_20261120` (PR \#4848)
]

#v(6pt)
*Question.* In `next_leads_20261120` the best IC arm was at 1.68, 1.19 and 0.77 of a 4-thread rho at
$q = 16001$, 24001 and 32003, over a factor of 2 in $q$. Three levers followed, each checked by an
independent verifier agent (workflow `wf_6b058b58-659`): the crate now accepts $q < 2^(16)$; a
lane-parallel SIMD kernel for the matched rho (`HUMBERT_RHO_SIMD`); and Montgomery mod-$r$
arithmetic in the linear algebra (`HUMBERT_LA_FASTMOD`). The question is whether IC's lead holds
against the stronger rho, and how the gap moves over a factor of 4 in $q$. [pcc:unverified]

*Scope and standing.* Toy fields only. 4 values of $q$ × 5 seeds × 8 arms = 160 runs under
`PROTOCOL.md`, frozen before data, with one binary (`bin/humbert-v8`, sha256 prefix `08d43a4e`),
serial runs in rotated order, and no addendum. The host is the 4-vCPU shared VM of v7 (L0;
load1 median 2.4). Every wall time is exploratory and an unverified assertion [pcc:unverified].
No leaderboard row: there is no replay certificate or `VALIDATION.json`.

= Correctness and counted identity

- 160/160 runs solved (IC: solved and $x P = Q$; rho: the recovered log), with abandoned_walks = 0.
- *C0:* `best7` reproduces the v7 `d35` counted fields on 10/10 instances ($q ≤ 32003$). `best8` and
  `best8_aa` (FASTMOD) equal `best7` on 40/40 runs: same relations, matrix, `la_mulmods` and $x$.
- *M4:* steps / expected median 0.98 for the SIMD rho and 1.09 for the v7 rho. The SIMD kernel
  changes speed, not the walk. [pcc:unverified]

= Predictions against the data

#let rt = csv("results/ratios_v8.csv").slice(1)
#figure(
  text(size: 8pt, table(
    columns: 6, align: right,
    [*q*], [*metric*], [*median*], [*min*], [*max*], [*n*],
    ..rt.map(r => (r.at(0), r.at(1), str(calc.round(float(r.at(2)), digits: 3)), str(calc.round(float(r.at(3)), digits: 3)), str(calc.round(float(r.at(4)), digits: 3)), r.at(5))).flatten()
  )),
  caption: [Paired per-instance ratios (`results/ratios_v8.csv`, L0). L2 = t_LA best8/best7, TIC = t_ic best8/best7, AA = best8_aa/best8, dNN/lp50 = arm/best8, M3 = rho_m8/rho_m7 steps per second.],
)

- *L2: holds.* t_LA FASTMOD/old: median 0.214, 0.240, 0.239, 0.243 (bound 0.40). IC total falls to
  0.69, 0.61, 0.51 and 0.45 as LA's share grows with $q$. [pcc:unverified]
- *M3: holds.* SIMD rho/v7 rho steps per second: median 3.57, 4.17, 4.46, 4.51 (bound 2.5), at 86–133 M
  steps/s on 4 threads.
- *F3 (two-sided).* With LA cheap, the factor-base optimum moves back up: dlpf:0.5 is best at 16001
  (0.88× of 0.35) and level at 32003 (0.96). 0.35 is best at 48017 and 65521, where d50 is
  1.06 and 1.09. The 0.25 base and lp:0.5 are worse everywhere (1.18–1.38).

= Distance remaining to the matched rho

#figure(image("figures/v8_ic_vs_rho.svg", width: 74%), caption: [Median IC time per arm and median expected time of the v7 (rho_m7) and SIMD (rho_m8) matched rho, same instances (L0).])

#align(center, table(
  columns: 5, align: right,
  [*q*], [*best IC arm*], [*IC / SIMD rho (expected)*], [*IC / SIMD rho (actual)*], [*IC / v7 rho (expected)*],
  [16001], [d50_8], [3.89], [3.89], [1.09],
  [32003], [d50_8], [1.97], [2.74], [0.45],
  [48017], [best8], [2.00], [1.92], [0.46],
  [65521], [best8], [1.37], [1.79], [0.31],
))

- *G (prediction on slopes): holds.* The log–log slope of the best IC arm's median time is 1.21
  (best8 alone 1.09). The SIMD rho's expected time has slope 2.00, as $sqrt(r) prop q^2$ predicts.
  [pcc:unverified]
- *The gap is a factor, not a sign.* IC is above the strongest rho at every $q$ measured: 3.9× at
  16001, 1.4× at 65521. Against the v7 rho, the same IC arms are at 0.31–0.46 above 16001. The
  previous study's "0.77× at 32003" was again a statement about the rho, not about IC.
- *Exploratory crossover.* Extrapolating both fits gives $q^* approx 92 000$, beyond this crate's
  $q < 2^(16)$. This is a 4-point extrapolation over a factor of 4 in $q$, with LA growing faster
  than relations, so it is not a measured result. It is the expected crossover of a known attack
  on toy fields, not a break. [pcc:unverified]
- *§5a.* Priced at $w$ GAE per native mulmod, the best IC arm costs 0, 1.8 and 18 times the SIMD
  rho's group operations at $w = 0, 0.1, 1$ ($q = 65521$). The order flips across the band, so
  there is no counted-unit ranking, and the table above is a same-host wall-time comparison.

#figure(image("figures/v8_gap_to_rho.svg", width: 70%), caption: [Median per-instance IC time / expected rho time. Solid: SIMD rho. Dotted: v7 rho.])

= Where the time is now

#figure(image("figures/v8_phases.svg", width: 100%), caption: [Phase medians (L0). At $q = 65521$, best8: relations 4.5 s, LA 3.2 s, descent 0.4 s, setup 0.25 s.])

At $q = 65521$, relation collection (4.5 s, 53 %) is again the largest phase. It grows roughly
linearly in $q$, about 1.05 per log unit of $q$ here. LA grows faster: 0.22 s → 3.2 s from 16001
to 65521, a slope of about 1.9. LA will dominate again past the crossover unless the base
shrinks, which is why the optimum alpha drifts. [pcc:unverified]

= Leads this opens

+ *Relation collection* is the largest phase near the crossover: the norm/splitting test (REL_FAST)
  has had one optimisation pass. A SIMD or batched candidate test could plausibly cut it 2–4×.
+ *Past $2^(16)$:* the crossover estimate lies beyond the crate's field range. Testing it needs
  $q < 2^(17)$ or more, which means re-auditing the lazy-reduction bounds.
+ *Rho is not finished either:* a cross-instance precomputed table, signed lazy reduction, and the
  verifier's other notes. Every rho speed-up moves $q^*$ outward [pcc:unverified].
+ *L2 rerun with a replay certificate* via isolab-eks before any leaderboard row.

= Reproduce

```
cargo build --release && cargo test --release          # 31 tests (1 ignored)
BIN=slope_20261120/bin/humbert-v8 BIN_SHA256_PREFIX=08d43a4e PRIMES=16001 \
  ./slope_20261120/run_v8.sh slope_20261120/campaign/q16001   # likewise 32003, 48017, 65521
python3 slope_20261120/analyze_v8.py
```
