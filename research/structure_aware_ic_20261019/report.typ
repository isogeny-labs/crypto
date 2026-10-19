#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)
#show heading: set block(above: 1em, below: 0.5em)
= Structure-aware index calculus: two constant-factor tests

2026-10-19. Branch `research/structure-aware-ic-e11b-j0-20261019`, PR \#2274 (stacked on \#2239).
Protocol, frozen before timing, with four dated amendments: `PROTOCOL.md`.
Every number below is an ecbench operation count $S$ = GAE / $sqrt(r)$ priced across the §5a band
($w in {0, 0.1, 1}$ GAE per unpriced native operation, applied alike to every arm, the rho references included).
Host: cloud VM (`virtual`), below L2, so *no wall-time claim*. All sessions audited (`ecbench verify --replay 12`, 0 problems).
Both tests are constants, not exponents; neither moves a row of the leaderboard.

== Test A: coset-restricted binary factor bases (E11b)

*Setup.* Three new random binary curves, $a = 1$, cofactor 2 (seed 7001, n = 19, 21, 23), 8 targets each.
Half base = `dimension=8,trace_kernel=1` (degree $d-1$) against default `dimension=9`; MITM primary, F4 secondary.

*Headline (tuned against tuned, Amendment 3): not falsified, about 1.45x.* Both families were tuned on fresh a=1 curves (seed 8001), then tested on these curves.
The half base is below the tuned default across the whole band on 3 of 3 curves:

#table(columns: 6, align: (left, left, right, right, right, right),
[*curve*], [*arms (default / half)*], [*S ratio*], [*95% (cluster)*], [*band $w=0, 0.1, 1$ half < default*], [*half S at $w=0.1$ / rho.neg*],
[n=19], [d7 / half d6], [0.690], [0.594 -- 0.810], [yes, yes, yes], [8.8x],
[n=21], [d7 / half d7], [0.653], [0.559 -- 0.766], [yes, yes, yes], [9.0x],
[n=23], [d8 / half d8], [0.725], [0.651 -- 0.819], [yes, yes, yes], [12.4x],
)
Pooled $Sigma S$(half)/$Sigma S$(default) = 0.690, 0.685, 0.666 at $w = 0, 0.1, 1$ (derived from the band report): about 1.45x, below the 2x ceiling.
The chosen dimensions came from 4 targets of one curve per n, and the n=19 pair differs in degree (d7 against half d6), as the rule dictated.

*Secondary (frozen comparison, against a mistuned default): 0.282 [0.271, 0.299].* The pre-registered pair was half at degree 8 against default degree 9; half is below default on 3 of 3 curves (0.261, 0.273, 0.385).
*Caveat, stated with the number:* default degree 9 is past the MITM optimum, so this ratio (2.6--3.8x, beyond the ~2x ceiling) overstates the gain over the best default; it is not an improvement over the baseline.
At equal point count (default 8 against half 9) the ratio is 0.98, 0.83, 0.72 (post hoc session `e11b_x_decomp`).
Controls: the repeated default arm is identical to the count; `rho.plain`/`rho.negation` at $w=0$ is 1.30 pooled (theory 1.41, intervals overlap).

F4 pair (secondary): 1.014 [0.942, 1.090] pooled; 0.89, 1.04, 1.10 per curve; the order holds across the band only at n=19. *Null for F4.*

*Class: engineering* (a verified constant-factor gain of about 1.45x in whole-method S of the MITM pipeline over a tuned default; every answer verified).
The 3.5x of the frozen comparison is a baseline-tuning artefact and is not an improvement.
Distance to rho unchanged in kind: the tuned half-base MITM stays 8.8--12.4x the matched `rho.negation`.
Limits: 3 curves, a = 1 cofactor 2 only; dimension chosen on 4 targets of one curve per n; F4 not tuned.

#figure(image("reports/testA_mitm_S.svg", width: 100%), caption: [Test A. Sessions `e11b_a1` (frozen) and `e11b_tuned`; bar $w=0.1$, whisker $w=0$ to $1$.])

== Test B: automorphism asymmetry on j = 0 prime curves

*Setup.* $p equiv 1 (mod 3)$ of 20, 24, 28, 32 bits, prime $\#E$. Six $j=0$ curves and twelve random-curve controls per size (two disjoint sets of six for the A/A control), 8 targets each.
Reference: `rho.cubic` (six-fold walk with look-ahead and cycle escape, the in-repo equivalent of aburan28/cryptanalysis \#953).
Equivalence check (frozen, before the run): pooled $S$(rho.cubic)/floor at 32 bits and $w=0$ must be $<= 1.45$: *observed 0.993* ($w=0.1$: 1.09; $w=1$: 1.97 because canonicalisations are then priced).
IC arms: `glv-orbit` $A=6$ fold on $j=0$ and $A=2$ on both families, MITM oracle, base size tuned per (size, family) on separate curves (chosen $s$: A6 16/32/64/256, A2 on $j=0$ 32/128/256/1024, A2 on random 64/128/256/512).
$Q = (Sigma "IC"_(A 6) / Sigma "rho"_6)_(j=0) \/ (Sigma "IC"_(A 2) / Sigma "rho"_"neg")_"random"$; $Q<1$ means IC/rho is better on $j=0$.

#table(columns: 8, align: (right, right, left, left, left, left, left, left), inset: 3pt, stroke: 0.3pt,
[*bits*], [*w*], [*Q*], [*IC/rho, j=0 (A6)*], [*IC/rho, random (A2)*], [*IC gain A2→A6 on j=0*], [*rho gain neg→six*], [*A/A (randA/randB)*],
[20], [0], [1.09 [0.97, 1.23]], [8.00 [7.48, 8.64]], [7.32 [6.69, 8.09]], [2.07 [1.89, 2.30]], [1.67 [1.49, 1.84]], [0.98 [0.80, 1.18]],
[20], [0.1], [1.10 [0.97, 1.25]], [8.01 [7.47, 8.70]], [7.27 [6.64, 8.05]], [2.22 [2.01, 2.46]], [1.68 [1.50, 1.85]], [0.97 [0.79, 1.17]],
[20], [1], [1.14 [0.97, 1.32]], [8.04 [7.37, 8.84]], [7.08 [6.34, 8.01]], [2.96 [2.62, 3.31]], [1.76 [1.56, 1.95]], [0.93 [0.73, 1.17]],
[24], [0], [0.91 [0.80, 1.01]], [10.47 [9.52, 11.26]], [11.54 [10.71, 12.51]], [1.85 [1.75, 1.94]], [1.66 [1.46, 1.91]], [1.10 [0.95, 1.28]],
[24], [0.1], [0.90 [0.79, 1.00]], [10.66 [9.67, 11.50]], [11.85 [11.01, 12.86]], [1.86 [1.76, 1.96]], [1.67 [1.46, 1.93]], [1.11 [0.96, 1.29]],
[24], [1], [0.87 [0.76, 0.99]], [11.62 [10.37, 12.73]], [13.33 [12.33, 14.58]], [1.92 [1.77, 2.06]], [1.73 [1.50, 2.01]], [1.11 [0.95, 1.31]],
[28], [0], [0.95 [0.86, 1.06]], [18.35 [17.26, 19.69]], [19.22 [17.87, 20.81]], [1.87 [1.74, 1.99]], [1.88 [1.64, 2.14]], [1.16 [1.01, 1.31]],
[28], [0.1], [0.95 [0.86, 1.05]], [19.20 [18.03, 20.66]], [20.12 [18.76, 21.81]], [1.88 [1.74, 2.00]], [1.89 [1.64, 2.15]], [1.15 [1.01, 1.31]],
[28], [1], [0.96 [0.86, 1.06]], [23.13 [21.49, 25.05]], [24.22 [22.62, 26.19]], [1.90 [1.74, 2.05]], [1.90 [1.65, 2.17]], [1.14 [1.00, 1.29]],
[32], [0], [1.06 [0.92, 1.23]], [34.54 [31.12, 38.71]], [32.61 [29.33, 35.83]], [1.86 [1.82, 1.91]], [1.91 [1.57, 2.25]], [0.93 [0.77, 1.12]],
[32], [0.1], [0.97 [0.85, 1.14]], [33.86 [30.45, 37.83]], [34.78 [31.25, 38.28]], [1.87 [1.82, 1.92]], [1.91 [1.57, 2.25]], [0.93 [0.76, 1.12]],
[32], [1], [0.69 [0.60, 0.80]], [30.76 [27.84, 33.92]], [44.56 [40.01, 49.08]], [1.92 [1.85, 2.00]], [1.92 [1.56, 2.25]], [0.93 [0.76, 1.13]],
)

*Verdict: falsified (null).* The frozen rule needs the upper 95% limit of $Q$ below 1 at every $w$ at 3 of 4 sizes; it holds at *none*.
$Q$ is 0.87--1.14 at 20--28 bits, and at 32 bits is 1.06, 0.97 and 0.69 at $w = 0, 0.1, 1$: the order does not hold across the band (§5a), so the 32-bit $w=1$ value cannot be ranked.
The reason is in the two gains: the fold cuts IC by 1.85--2.2x at $w=0, 0.1$ (2.96x at $w=1$, 20 bits; above $sqrt(3)$, below 3) and the six-fold rho by 1.67--1.92x ($approx sqrt(3)$ = 1.73 expected); they match within their intervals, so the asymmetry the idea predicts (IC gains $|A|$, rho only $sqrt(|A|)$) is not there at these sizes. The IC gain is below the 3x column ratio of the fold; this report does not decompose why (table, relation and linear-algebra phases are not separated here).
Prior art on the same fold (RESEARCH_GLV_INVARIANT_FACTOR_BASES §6.1, §6.6) measured the relation count at full rank, against an older rho; this is the end-to-end $S$ against a rho at its floor.

*Controls.* A/A population control contains 1 in 10 of 12 (size, weight) cells; the exceptions are 28 bits at $w=0$ and $w=0.1$ (1.157 [1.015, 1.309] and 1.153 [1.015, 1.308]); at $w=1$ it is 1.137 [0.998, 1.292], which just contains 1. Positive control (`rho.plain`/`rho.negation`, theory 1.41, $w=0$): 1.01 [0.91, 1.11] at 20 bits (*fails*: rho overhead at $r approx 2^19$), 1.28, 1.63, 1.28 at 24, 28, 32 bits (contain 1.41). Under $w>0$ the negation walk's canonicalisations are priced and the ratio falls below 1, as expected.

*Class: null.* IC/rho on $j=0$ is no better than on the random control (point estimates 0.87--1.14, all intervals include 1 except 24 bits at $w=1$, 0.87 [0.76, 0.99], and 32 bits at $w=1$, 0.69 [0.60, 0.80], neither robust across the band).
Distance to rho: tuned IC is 8--35x its matched rho and rises with size (j=0: 8.0, 10.5, 18.3, 34.5x at $w=0$).
Limits: toy curves (p $<= 2^32$), 6 $j=0$ curves per size (curve-level intervals), MITM oracle only, one size grid, 4 sizes; no exponent claim.

#figure(image("reports/testB_Q.svg", width: 100%), caption: [Test B. Sessions `test_*_{20,24,28,32}`; analysis `analysis_B.json`; bootstrap over curves, 10,000 resamples.])

== Deviations and disclosures
- Amendments 1--4 (`PROTOCOL.md`): fresh a=1 curves; tuned-against-tuned baseline; inadmissible exhausted arms and a doubled grid at 32 bits (before any 32-bit test spec).
- The model-tier card / relay (AGENTS.md "Model tiers") was not used; the main session ran the frozen script `run_sessions.sh` directly.
- New curves are not yet in the curve registry (`registered=false`); they are identified by explicit parameters in the specs.
- Pilot runs on other curves (seed 9001) preceded the freeze; discarded.
- Bugbot declined to review the PR (size, from committed session records).
