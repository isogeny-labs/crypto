# RESULTS L3-genus2-measured-arm

Frozen protocol: `PREREGISTRATION.md` (sha256 in `PREREG_SHA256.txt`, unedited). Code frozen before the confirmatory run: `FROZEN_SHA256.txt`
(`sha256sum -c` passes after the run). Deviations: `DEVIATIONS.md`. Raw lines: `out/*.txt`; machine-readable results: `results.json`;
all tables: `out/summary.txt` (copied verbatim at the end of this file); graph: `results_plot.png` / `results_plot.svg` (drawn from `results.json` only).
Every number below is copied from `out/summary.txt`, `out/posthoc.txt` or `results.json`, which are written by the code. Primary metric: operation counts
(Eop for the matched rho R_E; Jop for every Jacobian arm; native work priced at w = 0, 0.1, 1 GAE per operation, kappa_J = 1). Wall times are in the raw lines only and are
contention-affected (the host had load average 5 to 10 on 4 cores); the main run took 1204 s elapsed, the reduced replication 50 s.

## What was measured

The measured arm of the twist idea is: glue E and its quadratic twist E' into a genus-2 curve C (Jac(C) is (2,2)-isogenous to E x E'), move the instance with the
transfer map T: E(F_q) -> Jac(C)(F_q) (a pullback of divisors, O(1) field operations), and solve the ECDLP with index calculus in Jac(C) (three arms: IC-G, IC-S, IC-D), against the
matched Pollard rho with negation map on E (R_E) and the same rho run inside the transferred Jacobian subgroup (R_J). Nine sizes q = largest prime <= 2^k, k = 8..16, six admissible curves
per size, four instances per curve (216 instances, 1080 instance-arm runs); all five arms on the same instances. Everything is PARI/GP 2.15.4; Jacobian arithmetic is explicit-formula
Mumford arithmetic with counted field operations (a Jop = 28.0 M + 5.0 S + 2.0 I; an Eop = 2.0 M + 1.6 S + 1.0 I measured).

## Headline (the gap to matched rho, as distance remaining)

No Jacobian index-calculus arm beat the matched rho at any size or any native-work price. F1 (any IC arm with the upper 95% CI of IC/R_E below 1 at every w, at any size) did not fire.
The IC-to-rho cost ratio at w = 0 (native work free), geometric mean over 24 paired instances, grows with q:

| arm / R_E | k=8 (q=251) | k=12 (q=4093) | k=16 (q=65521) | growth |
|---|---|---|---|---|
| IC-D | 1.127 [1.001, 1.265] | 2.962 [2.675, 3.358] | 20.414 [17.754, 22.897] | log-log slope 0.604 [0.579, 0.628] on k = 10..16, 0.510 on k = 8..16 |
| IC-G | 1.369 | 5.239 | 41.440 | slope 0.676 on k = 10..16 |
| IC-S | 1.413 | 8.818 | 176.056 | slope 0.958 on k = 10..16 |
| R_J (rho inside Jac(C)) | 0.965 | 1.018 | 0.953 | no trend; unresolved against R_E at every size |

At w = 1 GAE per native operation the IC-D/R_E ratio is 2.805, 18.698 and 149.993 at k = 8, 12, 16. Ranking by the 5a rule (95% CI of the paired ratio on one side of 1 at all three w):
every IC arm is above R_E at every size (27 of 27 size-arm cells); IC-D is above R_J at 8 of 9 sizes (unresolved at k = 8); IC-G and IC-S are above R_J at all 9; R_J versus R_E is unresolved at all 9 sizes
(ratio 0.953 to 1.044 at w = 0). Moving the instance into the Jacobian therefore costs a rho nothing measurable and gains nothing: the transfer is a verified O(1) injective homomorphism onto the subgroup, and the only
thing that changes the cost is replacing rho by index calculus, which is q^1 against q^(1/2) in decomposition attempts.

Raw operation counts at k = 16 (geometric mean per verified solve, w = 0 / 0.1 / 1): R_E 1603 / 1640 / 1975, R_J 1527 / 1563 / 1884, IC-G 66,419 / 95,477 / 356,901, IC-S 282,179 / 383,029 / 1,290,678,
IC-D 32,719 / 59,146 / 296,265. With the Jop priced at the measured constant instead of one GAE (kappa_J = 3.91 at I/M = 10, 6.03 at I/M = 3, 2.76 at I/M = 30; Table 7) the IC-D/R_E ratio at k = 16 is 79.8, 123.2 and 56.4.
The kappa_J = 1 pricing used in the ranking is the generous one for the Jacobian arms.

Two context facts that matter for reading the gap (both from `out/posthoc.txt`, post hoc and not preregistered):
(i) the matched rho is dominated by its table construction and trail starts at these sizes: 95.6% to 97.2% of its Eops at k >= 9 are outside the walk steps and escapes (32 table entries, two scalar multiplications each),
so its cost grows like q^0.25 (slope 0.246 of core Eops), and the walk phase alone is only 66.7 group operations at k = 16 (steps plus escapes); the phase-only ratio IC-D trials / R_E steps is 6.3 at k = 8 and 476 at k = 16;
(ii) the relation-phase exponent of IC-D is q^0.97 to q^0.99 as predicted, but its total core Jops grow like q^0.756 because its setup (16 table entries, a start element) is 69.9% of its Jops at k = 8 and 2.9% at k = 16.
A cheaper rho set-up would only widen the gap, so the ratios above are lower bounds for the distance to rho.

No improvement over the index-calculus baseline is claimed or implied by this study. Within the index-calculus arms, IC-D costs 2.03x fewer Jops than IC-G at k = 16 (32,719 vs 66,419) because it starts solving at 0.2 n_est relations instead of n_est and
accepts every split divisor: that is a schedule difference of the preregistered arms, not a new method, and IC-S (single large prime) is 8.6x worse than IC-D at k = 16, as predicted (cheap graph elimination makes a small factor base useless).

## Hypotheses

- H1 (transfer exact, harness sound): held. #C'(F_q) = q + 1 on 54/54 curves; N = #E #E' annihilates 432/432 random Jacobian elements; the explicit Jacobian addition agrees with the PARI-polynomial Cantor reference on 18,036/18,036 pairs
  (special cases included; about 0.14% of the counted Jops of the IC arms went through the Cantor fallback); associativity 0/1080 failures; T(P1+P2) = T(P1)+T(P2) 1618/1618; l T(G) = l T(H) = 0, T(G) != 0, u | f - v^2 on 216/216 instances.
  All 648/648 IC solves verified with PARI `ellmul` on E. Verified solves overall: 1071/1080; the 9 unverified are rho arms at tiny l (R_E 4 at k = 8; R_J 3 at k = 8, 1 at k = 9, 1 at k = 11).
- H2 (relation phase, exponent and constants): held. IC-D: T_D/q median 0.509 to 0.578 per size (band [0.40, 0.95], and [0.45, 0.80] for k >= 10), slope of log T_D on log q 0.967 [0.951, 0.983] (k = 8..16; F2 band [0.85, 1.15]) and 0.993 [0.964, 1.019] (k = 10..16).
  IC-G: T_G/q 0.990 to 1.004 (band [0.95, 1.25]). IC-S: measured/formula 1.06 to 1.41 (band [0.6, 1.7]), but rising with q, slope 1.244 (the asymptotic 6/5 is approached from above); IC-F (PC-A): measured/formula 0.952 to 1.396 (band [0.5, 2]).
- H3 (rho constants): pooled over k >= 10, R = steps / sqrt(l/2) is 1.612 for R_E (all instances = verified only) and 1.642 (all) / 1.412 (verified only) for R_J, inside [1.2, 1.9]. Per size it is inside the band for R_E at k = 11, 12 and 16 and for R_J at k = 10, 12 and 14,
  below 1.2 at k = 13, 14, 15 (R_E) and 13, 15, 16 (R_J) by up to 0.12, and above 1.9 for R_E at k = 10 (3.55) and for R_J at k = 11 (3.33) (heavy tails; geometric-mean l is 110 at k = 10 and 316 at k = 11).
- H4 (the gap): half held. Held: every IC arm costs more than R_E at every size and w (smallest geometric-mean ratio per size 1.127 at k = 8 up to 20.414 at k = 16). Missed: the fitted slope of IC-D/R_E on q for k = 10..16 is 0.604 [0.579, 0.628] against the band [0.35, 0.60]
  (the point estimate is 0.004 outside and the CI contains 0.60; reported as a miss under the preregistered rule), and the point values (ratio >= 5 at k = 8, >= 30 at k = 16) are 1.127 and 20.414. The derivation assumed a rho cost of 1.45 sqrt(l/2) group operations;
  the counted rho cost is table-construction dominated at these sizes (context fact (i)), so the preregistered constant 0.69 sqrt(q) sqrt(h) was too high by the ratio of those two costs. This falsifies the stated cost model for H4, not the "loses" conclusion.
- H5 (no structure exploited): not rejected, not confirmed at the preregistered resolution. Per size glued/random ratio of IC-D trials: 1.123 [0.930, 1.372] (k = 8), 0.907 [0.764, 1.079] (10), 1.141 [0.995, 1.316] (12), 0.982 [0.898, 1.071] (13); none of the four CIs lies inside [0.9, 1.1].
  Pooled over the four sizes (post hoc, `posthoc.gp`): 1.034 [0.958, 1.118]. The failure condition of NC-A read literally (CI disjoint from [0.85, 1.15]) fires at 0 sizes; read strictly (CI not contained in [0.85, 1.15]) it fires at 3 of 4 sizes. No evidence that the IC arms use the E x E' structure; the sample (20 vs 24 instances) is too small to show equality within 10%.
- H6 (Jop constant): kappa_J = 3.91 to 3.99 at I/M = 10 over the nine sizes (band [2.0, 4.5]); 6.03 to 6.20 at I/M = 3 and 2.76 to 2.80 at I/M = 30 (reported, not banded).
- H7 (relation yield): p_split = 0.494 to 0.500 at every size (band [0.46, 0.54]); 9.1 million classified trials at k = 16.

## Controls (section 6) and falsifiers (section 7)

- PC-A (can the harness see a q^(1/2) gap in operation counts): passed. IC-F / IC-D trials ratio 8.22, 21.29, 41.72, 61.09 at k = 8, 10, 12, 14 against the formula 7.92, 15.98, 31.99, 63.99 (measured/formula 0.955 to 1.333, band [0.5, 2]); fitted slope 0.481 (band [0.35, 0.65]); 20/20 solves for both arms at every size.
- PC-B (planted-answer chain): the pooled rho verified-solve rate is 423/432 = 0.979 (>= 0.95: passed as written, "all sizes"); read per size, R_E 20/24 and R_J 21/24 at k = 8 fall below 0.95 (all other sizes 23/24 or 24/24). 100% of the transfer and Jacobian cross-checks passed.
  x'+1 was rejected on 1071/1071 verified solves.
- PC-C (can the pipeline rank a genus-2 IC arm as the winner, IC-D against plain rho in random Jacobians): FAILED at k = 10 and 12. IC-D/plain at w = 0 is 0.452, 0.388, 0.389 at k = 8, 10, 12 (band [0.2, 0.8] holds), the upper CI is below 1 at all three w only at k = 8 (w = 1: 0.746 [0.619, 0.949]);
  at k = 10 and 12 the w = 1 ratio is 0.997 [0.732, 1.352] and 1.409 [1.062, 1.836]. The upper CI of IC-D/plain is below 1 at w = 0 and 0.1 at all three sizes, but IC-D's native work per trial removes the margin at 1 GAE per native operation, so a winner is not ranked as a winner across the whole band at k >= 10.
  Under the preregistration (section 6) a failed PC-C means the negative main result "carries no evidential weight beyond the raw counts and the exponent measurements". The raw counts and exponents above are therefore the evidence; the ranking sentence in the headline is scoped by this control.
- NC-B (wrong-answer detector): 0 of 119 returned values verified (IC-D 60 returned, R_E 59 returned, 120 runs); NC-C (sabotaged transfer): 0/40 verified.
- F1: not fired. F2: IC-D slope inside the band; the R_E steps slope is 0.297 [0.224, 0.367] (k = 8..16) and 0.376 [0.288, 0.458] (k = 10..16), outside [0.4, 0.6], so the preregistered F2 flag "the baseline is broken" fires. Post hoc: the number of steps depends on l, not q (l = q/h with the cofactor h of each of the six curves, mean h from 4.3 to 10.7 across sizes), and the geometric-mean R (steps over sqrt(l/2)) falls with size from 4.3 (k = 8) to 0.91 to 1.14
  (k >= 13) because tiny subgroups trap the walk; the slope of log gm(steps) on log gm sqrt(l/2) is 0.600. The rho runs are 100% verified for k >= 12 with mean R between 1.08 and 1.65. The flag is reported as fired; the baseline is the preregistered G1 design applied to a geometric-mean l of 42 (k = 8) to 6684 (k = 16).
  F3: see "Preregistered checks that missed". F5: no transfer failure. F6: IC-D solve rate 24/24 at every size.

## Preregistered checks that missed (from `out/summary.txt`, Table 14)

H4 slope (0.604 vs [0.35, 0.60]); H4 point values at k = 8 and 16; F2 R_E slope; R per size at k = 10, 11, 13, 14, 15, 16 (listed above); PC-B per size at k = 8; PC-C at k = 10 and 12 (all-w ranking); NC-A strict reading (3 sizes), H5 CI containment.
Everything else in Table 14 passed (T_D, T_G, T_S, p_split, kappa_J at I/M = 10, IC-D solve rate, the PC-A bands, pooled R, pooled PC-B, NC-B, NC-C, the transfer checks).

## Limits

Prime fields, E with rational 2-torsion and cofactor h in {2..16} (so l is about q/h; geometric-mean l is 42 at k = 8 and 6684 at k = 16); the one-parameter gluing family of the prior study; the matched rho is the G1 design rewritten in GP with its table construction counted;
the Jacobian arithmetic is a GP explicit-formula implementation, not native code; the exponent fits cover 8 bits of q; six curves per size; sizes up to 2^16. The study says nothing about production sizes, about other gluings, about pattern [3] (no rational Weierstrass point), or about methods that exploit E x E' structure
other than through the transfer (the H5 test is a test of obliviousness of these three arms only). A same-code, same-seed reduced replication reproduced all raw lines for k = 8..12 and the k = 8, 10 controls exactly (DEVIATIONS P3); the independent Haiku replication and the Sonnet adversarial reviews of the preregistration are not done.
Promotion rules of the main repository (benchmarks row, IC leaderboard, validation gate) are not triggered: this is a scratchpad study and no repository file was touched.

## Reproduction

Full (about 20 min on the loaded 4-core host, 1204 s measured): `bash run.sh`. Reduced subset (k = 8..12, controls at k = 8, 10; 50 s): `KMAX=12 CTRLSET=reduced OUT=$D/out_reduced RES=$D/results_reduced.json bash run.sh`.
Post hoc: `DATA=$D/out OUTF=$D/out/posthoc.txt gp -q posthoc.gp </dev/null`; then `bash make_results.sh`. Disk used: under 3 MB; no Rust crate or `target/` directory was created.
