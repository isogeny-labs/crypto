#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 9.5pt)
#set par(justify: true)
#show heading: set text(size: 12pt)
#set table(stroke: 0.4pt + luma(160), inset: 3.5pt)
#align(center)[#text(size: 15pt, weight: "bold")[Decomposition index calculus on genus-2 Jacobians over F\_(q^2): local development check]]
#align(center)[Status: \[pcc:unverified\]. Slug `extfield-jacobian-decomp`, 2026-11-15.]

= Question

Can Gaudry/Nagao decomposition index calculus, run directly on the Jacobian J(F\_(q^2)) of a genus-2 curve y^2 = f(x) over F\_(q^2) (an abelian variety of dimension 4 over F\_q, no Weil-descent cover), beat Pollard rho with the negation map on the same subgroup, same Mumford arithmetic, in the repository's counted unit? Textbook exponents: 2 - 2/4 = 1.5 for the decomposition attack with large primes against 2 for rho. This is a small-parameter local development check (q from 67 to 2039, group order about q^4). It claims no improvement; it reports the distance to the matched rho at 0, 0.1 and 1 GAE per unpriced native operation. All numbers are \[pcc:unverified\]: no replay certificate, no validator report, not an isolab or ecbench run.

= Method

- Curves: y^2 = f(x), f a monic quintic with uniformly random coefficients in F\_(q^2), not defined over F\_q. Cantor/Mumford arithmetic over F\_(q^2) written for this study (`src/fq2.rs`, `src/jac.rs`); group law, order and field arithmetic are unit-tested.
- Group order: \#C(F\_(q^2)) by direct count, then a baby-step giant-step search in the Weil window for the second L-polynomial coefficient, filtered by six random elements. A curve is kept when \#J = h \* ell, ell prime, h \<= 16. The order is given to both arms and not charged.
- Factor base: all points with x in F\_q (about q points, about q/2 classes modulo negation). Decomposition of R = eD + gQ into R + P1 + P2 + P3 + P4 = 6 infinity with x(P\_i) in F\_q: writing h = v + u(x+b)/a, the quartic (f - h^2)/u = -(1/a^2)(x^4 + c3 x^3 + c2 x^2 + c1 x + c0) has c\_k quadratic in (a, b) over F\_(q^2). Requiring all c\_k in F\_q gives four real quadrics in four unknowns (Bezout number 16). They are solved exactly by a degree-5 Macaulay matrix (escalating to 6 and 7), multiplication matrices on the 16-dimensional quotient, a characteristic polynomial, a gcd with x^q - x and eigenvectors; the quartic must then split over F\_q. Every relation is verified with the group law. R is moved by a 16-entry random-walk pool, one group addition per trial.
- Correctness reference: a pair-hash collinearity oracle (a decomposition is four factor-base points on one line t = alpha x + beta over F\_(q^2), t = (y - v)/u). The solver must reproduce its decomposition set exactly.
- Linear algebra: dense Gaussian elimination modulo ell (unknowns: the homomorphism value of each class, plus the logarithm k); counted in mulmods. The block-Wiedemann count 2\*m\*nnz is reported as a model only; Wiedemann is not implemented.
- Rho: same group and arithmetic, 64-entry additive walk, negation map (canonical representative of {X, -X}), distinguished points, 8-step history with doubling to escape fruitless cycles.
- Counted unit: group additions (GAE). Unpriced native work is priced at w = 0, 0.1, 1 and w\_cal per operation on every arm: IC counts the F\_q multiply-adds of the solver (Macaulay elimination counted nominally as nonzero row operations times row length, an upper bound), factor-base construction, root finding and the mod-ell mulmods of linear algebra; rho counts partition hash, canonicalisation, distinguished-point test, history check and table operations. w\_cal = scalar F\_q mulmod time / group-addition time, best of 3 on this shared 4-CPU VM (about 0.003, noisy).
- Design: q in {67, 131, 251, 509, 1021, 2039}, 3 instance seeds per cell, one IC run and one rho run per instance; A/A (second internal seed on the same instance) at q = 131 and 251. Preregistered in `PLAN.md` before the first registered run (Amendment 1 added the q = 2039 cell after development smoke runs on a separate seed).

= Results

Counted cost S in group-addition equivalents, mean of 3 instance seeds (every run solved the DLP and passed kD = Q):

#block(breakable: false)[
#set text(size: 7.5pt)
#table(columns: 9, align: left, [*q*], [*IC S(0)*], [*IC S(0.1)*], [*IC S(1)*], [*IC S(w\_cal)*], [*rho S(0)*], [*rho S(0.1)*], [*rho S(1)*], [*rho S(w\_cal)*], [67], [2.48e+03], [7.31e+07], [7.31e+08], [2.47e+06], [8.7e+03], [9.35e+03], [1.52e+04], [8.72e+03], [131], [3.42e+03], [9.27e+07], [9.27e+08], [2.92e+06], [1.68e+04], [2e+04], [4.97e+04], [1.69e+04], [251], [5.3e+03], [1.62e+08], [1.62e+09], [5.34e+06], [4.4e+04], [5.67e+04], [1.71e+05], [4.44e+04], [509], [8.8e+03], [2.86e+08], [2.86e+09], [8.02e+06], [8.54e+04], [1.15e+05], [3.83e+05], [8.64e+04], [1021], [1.64e+04], [5.75e+08], [5.75e+09], [1.96e+07], [4.18e+05], [5.8e+05], [2.03e+06], [4.24e+05], [2039], [3.04e+04], [1.13e+09], [1.13e+10], [3.88e+07], [8.11e+05], [1.13e+06], [3.99e+06], [8.22e+05])
]

Distance to matched rho, S(IC) / S(rho), with 95% bootstrap interval over instance seeds (below 1 means IC is cheaper):

#block(breakable: false)[
#set text(size: 8pt)
#table(columns: 5, align: left, [*q*], [*w = 0*], [*w = 0.1*], [*w = 1*], [*w = w\_cal*], [67], [0.285 \[0.272, 0.319\]], [7.82e+03 \[6.56e+03, 8.7e+03\]], [4.8e+04 \[4.57e+04, 5.07e+04\]], [284 \[232, 327\]], [131], [0.204 \[0.147, 0.307\]], [4.63e+03 \[3.06e+03, 7.37e+03\]], [1.87e+04 \[1.13e+04, 3.94e+04\]], [173 \[128, 213\]], [251], [0.121 \[0.091, 0.208\]], [2.86e+03 \[2.22e+03, 4.51e+03\]], [9.49e+03 \[7.27e+03, 1.68e+04\]], [120 \[91.3, 181\]], [509], [0.103 \[0.0641, 0.292\]], [2.48e+03 \[1.49e+03, 7.67e+03\]], [7.47e+03 \[4.37e+03, 2.49e+04\]], [92.8 \[67.6, 183\]], [1021], [0.0391 \[0.0351, 0.0478\]], [991 \[903, 1.21e+03\]], [2.83e+03 \[2.57e+03, 3.45e+03\]], [46.2 \[41.2, 54.6\]], [2039], [0.0375 \[0.0288, 0.0537\]], [1e+03 \[757, 1.44e+03\]], [2.83e+03 \[2.14e+03, 4.09e+03\]], [47.2 \[34.4, 72.3\]])
]

At q = 2039 IC costs 0.0375 of rho at w = 0, but 47.2 times rho at w\_cal, 1e+03 times at w = 0.1 and 2.83e+03 times at w = 1. The order of the arms changes inside the band, so by the repository's rule the two arms are unresolved at the band boundary: this is the known zero-priced-work win that disappears once the solver is priced. The reason is the decomposition solver: about 4.58e+05 counted F\_q mulmods (about 0.45 ms) per trial, constant in q, and about 24.9 trials per relation.

Decomposition probability per trial (degenerate R counted as failures). Pooled for q \>= 131: 5856/145839 = 0.0402, Wilson 95% \[0.0392, 0.0412\], i.e. 24 p = 0.96: inside the preregistered 1/24 +-30% band, about 4% below 1/24 (degenerate R, repeated roots and finite q are candidate causes; not isolated here).

#block(breakable: false)[
#set text(size: 7.5pt)
#table(columns: 7, align: left, [*q*], [*trials*], [*R with relation*], [*p*], [*24 p*], [*degenerate-R rate*], [*solver mulmods per trial*], [67], [4017], [123], [0.0306], [0.73], [0.0162], [5.5e+05], [131], [5771], [220], [0.0381], [0.91], [0.0064], [4.83e+05], [251], [10312], [395], [0.0383], [0.92], [0.0043], [4.7e+05], [509], [18707], [773], [0.0413], [0.99], [0.0019], [4.56e+05], [1021], [37705], [1488], [0.0395], [0.95], [0.0011], [4.52e+05], [2039], [73344], [2980], [0.0406], [0.98], [0.0006], [4.52e+05])
]

Fitted exponents b in S = c q^b over the six cell means, 95% seed-bootstrap interval:

#block(breakable: false)[
#set text(size: 8pt)
#table(columns: 5, align: left, [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*], [*w = w\_cal*], [IC], [0.74 \[0.684, 0.793\]], [0.83 \[0.732, 0.917\]], [0.83 \[0.732, 0.917\]], [0.83 \[0.733, 0.934\]], [rho], [1.38 \[1.27, 1.49\]], [1.45 \[1.34, 1.57\]], [1.66 \[1.53, 1.81\]], [1.38 \[1.27, 1.49\]])
]

Preregistered predictions: P1 confirmed; P2 confirmed; P3 refuted; P4 confirmed; P5 confirmed; P6 refuted; P7 confirmed; P8 confirmed; P9 confirmed. Detail:

#block(breakable: false)[
#table(columns: 3, align: left, [*id*], [*outcome*], [*evidence*], [P1], [confirmed], [per-q p\*24 for q\>=131: 0.91, 0.92, 0.99, 0.95, 0.98; band 0.70-1.30], [P2], [confirmed], [IC exponents w0/w0.1/w1/wcal: 0.74, 0.83, 0.83, 0.83], [P3], [refuted], [rho exponents w0/w1: 1.38, 1.66; walk-steps-only 1.82; S\_w0 for q\>=251 1.48], [P4], [confirmed], [registered oracle runs agree 380/380; supplemental 976/976, 2881/2881], [P5], [confirmed], [ratio IC/rho at w=0 by q: \[0.285, 0.204, 0.121, 0.103, 0.039, 0.038\]; w=0.1: \[7816, 4625, 2861, 2483, 991, 1001\]; w\_cal: \[283.7, 172.9, 120.3, 92.8, 46.2, 47.2\]; q\* (fit) w0=9.94311429668717, w0.1=95328429.81180932, w1=19364488.19501907, wcal=1661607.4942709918], [P6], [refuted], [solver per-trial exponent -0.050], [P7], [confirmed], [max dense-LA share of S\_IC(0.1) = 0.0039], [P8], [confirmed], [degenerate-R rate q\>=131: 0.0064, 0.0043, 0.0019, 0.0011, 0.0006], [P9], [confirmed], [IC A/A ratios \[0.89, 0.82, 0.82, 0.9, 0.81, 0.81\]; rho A/A ratios \[0.5, 0.33, 0.46, 0.4\]])
]

- P3 refuted: the rho exponent over this ladder is below 2 because S(rho) carries a fixed overhead of about 9511 group additions (the 64-entry walk pool and start points; OLS intercept over 18 runs). The walk itself behaves as expected: log-log slope of walk steps on ell is 0.48 (ideal 0.5) and steps/sqrt(ell) pools to 0.99. Over the ladder the fixed term and 3-seed noise bias the exponent downward; this is a measurement of the instrument, not of asymptotic rho.
- P6 refuted at the band edge: the solver cost per trial has exponent -0.050, i.e. constant; it is slightly negative because the q = 67 cell has more non-generic Macaulay cases. The prediction expected a small positive slope from O(q) root finding.
- The two refuted predictions concern instrument behaviour, not the decomposition mechanism: the solver/oracle agreement (P4: every non-degenerate R, 380/380 registered plus 3857/3857 supplemental) and the 1/24 probability (P1) hold.
- IC exponents 0.7-0.9 (P2) are a small-q artifact: over q \<= 2039 the cost is the solver term, linear in q. Fitted crossovers from the two power laws (w = 0.1: q\* 9.53e+07, w = 1: 1.94e+07, w\_cal: 1.66e+06) inherit rho's biased low exponent and should not be used.

= Honest crossover projection

A component model (constants fitted from the 18 runs, see `RESULTS.json`) separates the terms: trials = (classes + 10) / p, solver cost per trial constant, linear algebra as the block-Wiedemann model 2\*m\*nnz (about 2.5 q^2 mulmods), rho = c sqrt(ell) steps with the measured intercept. This is an extrapolation over many orders of magnitude, not a measurement.

#block(breakable: false)[
#set text(size: 8pt)
#table(columns: 5, align: left, [*w*], [*q\* (IC cheaper above)*], [*ratio at q = 2039*], [*at q = 1e6*], [*at q = 1e9*], [w0], [none], [0.0193], [3.74e-05], [3.74e-08], [w0p1], [1.98e+06], [523], [1.52], [0.466], [w1], [none], [1.47e+03], [4.28], [1.31], [wcal], [5.09e+04], [24.1], [0.0703], [0.0215])
]

The model says: at w = 0 IC is cheaper at every q only because mulmods are free; at w = 1 even the Wiedemann linear algebra alone costs more than all of rho (no crossover); at w = 0.1 and w\_cal the crossover sits above q of about 5e4 (group order above 1e18), far beyond anything measured. More fundamentally, the algorithm built here has a factor base of size q and no large primes: its linear algebra is Theta(q^2), so its exponent is 2, the same as rho. The 1.5 of the literature needs the reduced-factor-base/large-prime balancing that was not implemented; the sub-linear measured IC exponents must not be read as an exponent below rho's.

= Limitations

- Dense linear algebra (block Wiedemann not implemented); dense share of S(IC, 0.1) is at most 0.4% here, the 2\*m\*nnz line is a model.
- The Macaulay elimination is counted nominally (dense row operations); a sparse F4 would charge fewer mulmods, so the IC cost at w \> 0 is an upper bound for this solver design. Wall time per solver call is 0.45 ms; at w\_cal the effective solver rate was not separately calibrated (SIMD inner loop), and calibrating only some counters would break the pricing rule, so the band governs.
- Rho's fixed overhead inflates S(rho) at small q and flattens its fitted exponent; rho cofactor h up to 16 and 3 seeds per cell make single rho runs noisy (A/A ratios 0.33-0.50 for rho).
- w\_cal comes from a shared 4-CPU VM, best of 3, not an isolated host. Group additions use a generic Vec-based Cantor implementation shared by both arms.
- Degenerate R (Macaulay quotient does not close by degree 7, rate about 1/q) are counted as failed trials; relations with repeated roots are kept. Solutions with a = 0 and at infinity are excluded by construction.
- Factor-base classes only; no automorphism quotient, no large primes, no reduced factor base. Only q \<= 2039 (lazy u32 elimination).
- Local development check: no isolab, benchrec, replay certificate or validator report; every number is \[pcc:unverified\] and must not enter a leaderboard row.

= Reproduction

From /home/user/crypto at git HEAD 3c24520f7cdc1c47bac26cc7778d058e655b79a8 with the new files present:

#block(fill: luma(240), inset: 5pt, width: 100%)[#text(size: 7.5pt)[```
CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin extfield_jac_decomp -j2 && ./research/extfield-jacobian-decomp_20261115/run_ladder.sh && python3 research/extfield-jacobian-decomp_20261115/analyze.py && python3 research/extfield-jacobian-decomp_20261115/build_report.py
```]]

Smallest cell: `GIT_HEAD=$(git log -1 --format=%H) ./target/release/extfield_jac_decomp run 67 1 both 0`. Counts are deterministic given the seeds; only wall times and w\_cal vary between runs (the ladder was run twice with identical counts, the second time after lint fixes).

= Sources read

- `docs/ECDLP_ATTACK_MATRIX.md` (rows 9, 10, 12), `AGENTS.md` section 5a (pricing band), `CLAUDE.md` rules.
- `src/cryptanalysis/genus2_fp.rs` (API list only; arithmetic rewritten over F\_(q^2)), `src/bin/quartic_ic.rs` (path-module pattern), `src/cryptanalysis/f4_fp.rs` (header; not used because of crate-internal dependencies).
- `research/nagao_relations/GOAL.md`, `benchmarks/20261018-weil-descent-round4-genus2-ic/validator-report.json` (header), directory listings of `benchmarks/20261018-weil-descent-n2-routes/`.
- Not read in this session: `src/cryptanalysis/jv_cover.rs`, the Nagao 2010 and Gaudry 2009 papers (the decomposition was derived directly and checked against the oracle). The knowledge-base search (research-kb) was not run.

#figure(image("graph.svg", width: 100%), caption: [Counted cost S versus q (left; solid IC, dashed rho) and distance to matched rho (right). \[pcc:unverified\]])
