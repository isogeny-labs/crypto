#set document(title: "Quartic-extension genus-5 cover: toy reproduction and end-to-end measurement")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Quartic-extension genus-5 cover:\ toy reproduction and end-to-end measurement]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021` · runs 2026-10-21 to 2026-10-26 · PR \#2580
]

#v(6pt)
*Scope.* Toy fields only: $q$ prime with $17 <= q <= 8009$, $E$ over $FF_(q^4)$, prime subgroup order
$r < 2^62$ with cofactor at most 128. All computation is native Rust in `src/` with no
dependencies; `summarize.py` and `plots.py` only lay out recorded values. Each statement is
labelled *verified* (an exact check that runs on every instance), *measured* (a timed or
counted run; records in `results/`) or *derived* (argued in the code comments of
`src/cover.rs`, not separately proved here).

*Host.* 4-vCPU Intel Xeon @ 2.10 GHz shared cloud VM, no CPU isolation, no hardware counters.
Index calculus and rho for an instance run back to back in the same process, so each
ratio is paired; absolute times carry host noise.

= Lead 1: reproduction of the family

The descended curve $C$ is built as an explicit model over $FF_q$ (a twisted Humbert curve,
`src/cover.rs`). `humbert verify` counts $N_k(C)$ for $k = 1..5$ on that model alone, forms
the L-polynomial, and compares it with $L_E(T^4) dot L_D(T)$, where $\#E(FF_(q^4))$ comes from
baby-step giant-step on $E$ and $\#D(FF_q)$ from the extra factor's own equation
$D: y^2 = N(eta) m_alpha (x)$.

#figure(
  table(
    columns: 6,
    align: center,
    [*instances*], [*L-poly match*], [*twisted-E control rejected*], [*$\#"Jac"(C)(FF_q) = \#E dot \#D$*], [*$h_1$-linear form = Möbius cubic*], [*q*],
    [12], [12/12], [12/12], [12/12], [12/12], [17, 19, 23; both twists; 2 seeds],
  ),
  caption: [Verification grid (`results/verify.jsonl`). All ten L-polynomial coefficients agree on every instance: genus 5, defined over $FF_q$, $"Jac"(C) tilde "Res" E times D$.],
)

#figure(image("figures/construction_diagram.svg", width: 80%), caption: [Objects and maps of the construction (`src/cover.rs`); no data.])

*Transfer (verified on every attack run).* The pushforward of the transferred divisor
$T(P)$ to $E$ is computed point by point and equals $8P$; hence $T(P) != 0$ for the
generator of the order-$r$ subgroup. The discrete log is then recovered by index
calculus on $C$ through $T$: *90/90 runs correct.*

= Lead 2: binary-pattern classifier

`humbert classify` enumerates Frobenius-orbit root patterns for $d in {2,3,4}$ and source
genus $1..3$ (11,200 patterns, `results/classify.jsonl`). On every pattern the
Riemann–Hurwitz genus equals the independent Kani–Rosen sum. The quoted family appears in
both forms ($deg h_1 in {0, 1}$) with $m = 4$, $S = 5$, $g = 5$, $e = 1$, both twists allowed,
non-hyperelliptic, one extra genus-1 factor over $FF_q$. For $d = 4$ the minimum excess
genus is $e = 1$ for every source genus ($g = 5, 9, 13$). For $d = 2$ and $d = 3$ the classifier
reproduces the known $e = 0$ covers (genus 2 hyperelliptic; genus 3, partly non-hyperelliptic).
*Not done:* the remaining Appendix A rows were not compared, because the paper text was not
available in this session.

#figure(image("figures/classifier_d4_g1.svg", width: 78%), caption: [Patterns for $d = 4$, source genus 1, by excess genus and module rank (`results/classify.jsonl`).])

= Lead 3: complete attack against matched Pollard rho

Grid: $q in {61, ..., 8009}$ (15 primes), 3 seeds, two arms (full factor base; factor base
halved by the involution $iota$). Phases recorded per run: cover set-up, relation search,
linear algebra (dense elimination up to 1200 unknowns, scalar Wiedemann above), individual-log
descent for $P$ and $Q$, peak RSS. Rho is a 32-adding walk with Brent cycle detection on the
same subgroup. Two rho references are shown: the measured single run, and the expected-steps
time $sqrt(pi r slash 2)$ times that run's measured seconds per step (lower variance).

#let rows = csv("results/summary.csv").slice(1)
#figure(
  text(size: 7.5pt, table(
    columns: 12,
    align: right,
    [*q*], [*log#sub[2] r*], [*arm*], [*unknowns*], [*relations*], [*rel. s*], [*LA s*], [*descent s*], [*IC s*], [*rho meas. s*], [*rho exp. s*], [*IC / rho exp. (range)*],
    ..rows.map(r => (r.at(0), r.at(1), r.at(2), r.at(4), r.at(5), r.at(6), r.at(7), r.at(8), r.at(9), r.at(10), r.at(11), [#r.at(13) [#r.at(14), #r.at(15)]])).flatten()
  )),
  caption: [Medians of 3 seeds per row (`results/summary.csv`, from `results/attack.jsonl`). All 90 runs solved; rho solved all 90.],
)

#figure(image("figures/ic_vs_rho_time.svg", width: 82%), caption: [Wall time per DLP against $q$ (log-log); points are single runs, lines medians.])
#figure(image("figures/ic_over_rho_ratio.svg", width: 82%), caption: [IC time over rho time on the same instance; above 1, rho is faster.])
#figure(image("figures/phase_breakdown.svg", width: 82%), caption: [Median phase times, iota arm.])

*Observations (measured).* Rho is faster at every $q$ tested, by the median of each row and
against the expected-steps reference in every single run. Three single runs fell below 1
against the *measured* rho run (iota arm: $q = 3001$ seed 3, 0.51×; $q = 4001$ seed 1, 0.64×;
$q = 8009$ seed 1, 0.73×); in each, that rho walk took 1.8–2.7× its expected steps, so these
are rho variance, not an IC advantage. Against the expected-steps rho,
the iota arm is about 780× slower at $q = 61$ and between 1.14× ($q = 4001$) and 6.1×
($q = 3001$) slower for $q >= 1511$, with no monotone trend in that range. The descent phase is
roughly constant in $q$; relation search grows about linearly; sparse linear algebra grows
faster and dominates the full arm at $q >= 3001$. Peak RSS stayed below 14 MB.

*Reading (not a claim beyond toy scale).* Rho on $E(FF_(q^4))[r]$ costs about $q^2$ steps; this
index calculus uses relations of weight 8 or 16 over a factor base of about $q$ points, so its
linear algebra is also about $q^2$. The remaining distance is therefore a constant factor in
this regime. Large-prime variants that shrink the factor base are the standard route to a
smaller exponent and were not implemented.

*Pricing note (AGENTS.md §5a).* Index calculus here performs no group operations on $E$; all of
its work (norm evaluations, root finding, linear algebra) is unpriced native work. Priced at
0 GAE per operation it would count as zero group operations, and at 0.1 or 1 GAE it would
count far more than rho's steps, so the order flips across the §5a band and the two arms are
*not ranked on operation counts*. The only comparison reported is paired wall time on the same
host, where rho is faster at every $q$.

= Lead 4: the extra factor and covering symmetries

*Derived (code comments in `src/cover.rs`).* The involution $iota: w -> -w$ is defined over
$FF_q$, acts as $-1$ on the $"Res" E$ part and $+1$ on $D$. In the $r$-part this forces
$log[iota P] = c - log[P]$ and $log[D_infinity] = 4c$.

*Verified on every run:* both invariants hold for every factor-base pair (90/90).

#figure(image("figures/iota_halving.svg", width: 82%), caption: [Unknowns and relation candidates, full vs iota arm (medians).])

*Measured, paired by instance (full / iota):* unknowns 2.00× at every $q >= 1009$; relation
candidates about 2× (1.90–2.26× for $q >= 401$); total IC time 1.95–10.2× for $q >= 1009$
(10.2× at $q = 8009$, where linear algebra dominates). At small $q$ the gain vanishes: at
$q = 101$ the iota arm was slower (0.86×), because the descent phase, which iota does not
shrink, dominates there (`results/summary.md`).

*Other findings.* Rational points of $C$ lie over only about half of $x in FF_q$, namely where
$N(eta) m_alpha (x)$ is a square — the defining condition of $D$ — so each residual root
costs a density factor of about 1/2. No quotient of $C$ over $FF_q$ other than the $x$-line
has genus 0, so the cover yields no free relations. Hyperplane relations run out when
$q^3 lt.tilde 8!$; a quadric relation family is the fallback. Degree-8 places in the descent are
avoided by descending $T(m P)$ for the smallest suitable $m$ and dividing by $m$.

= Failures and corrections

A first sweep (kept as `results/attack_v1_superseded.*`) returned one wrong logarithm
($q = 1511$, full arm): scalar Wiedemann gives a factor-base column that appears in no relation
the value 0 under every column scaling, so an undetermined unknown went undetected. The
fix adds an empty-column check and random column scaling to the corank test; the whole sweep
was rerun with the fixed binary (90/90 correct). About 30 s of unit tests overlapped the rerun
on the same host.

= Limits

- Toy sizes only; no statement is made about cryptographic sizes.
- Wall time on a shared VM with no isolation; ratios are paired but not tier-qualified.
- Rho without the negation map; with it, rho would be about $sqrt(2)$ faster.
- Appendix A cross-check limited to the quoted family.
- Conductor was not installed in this session.

= Records

`results/verify.jsonl`, `results/classify.jsonl`, `results/attack.jsonl` (+ `.stderr`),
`results/summary.csv`, `results/summary.md`, `results/host.txt`, `results/attack_v1_superseded.*`.
Reproduce: `cargo test --release`; `./run_sweep.sh classify | verify | attack`;
`python3 summarize.py`; `python3 plots.py`.
