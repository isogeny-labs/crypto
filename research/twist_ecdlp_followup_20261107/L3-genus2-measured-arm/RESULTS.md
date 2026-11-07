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

## GAE-band table: ratio to the matched rho R_E (geometric mean over paired instances, 95% CI), kappa_J = 1, native work priced at w = 0 / 0.1 / 1 GAE per operation

| k | pair | w=0 | w=0.1 | w=1 | ranking across the band | gm w=0 excl. censored |
|---|---|---|---|---|---|---|
| 8 | IC-G/R_E | 1.369 [1.211, 1.547] | 1.546 [1.351, 1.785] | 2.753 [2.117, 3.603] | first arm above | 1.426 (n=20) |
| 8 | IC-S/R_E | 1.413 [1.247, 1.591] | 1.642 [1.426, 1.865] | 3.179 [2.409, 4.067] | first arm above | 1.451 (n=20) |
| 8 | IC-D/R_E | 1.127 [1.001, 1.265] | 1.347 [1.155, 1.557] | 2.805 [2.085, 3.685] | first arm above | 1.182 (n=20) |
| 8 | R_J/R_E | 0.965 [0.821, 1.132] | 0.951 [0.783, 1.167] | 0.877 [0.594, 1.342] | unresolved | 0.977 (n=18) |
| 9 | IC-G/R_E | 1.923 [1.680, 2.142] | 2.345 [2.005, 2.643] | 5.673 [4.505, 6.791] | first arm above | 1.923 (n=24) |
| 9 | IC-S/R_E | 2.125 [1.878, 2.385] | 2.662 [2.275, 3.048] | 6.872 [5.284, 8.335] | first arm above | 2.125 (n=24) |
| 9 | IC-D/R_E | 1.450 [1.302, 1.597] | 1.906 [1.666, 2.145] | 5.480 [4.457, 6.566] | first arm above | 1.450 (n=24) |
| 9 | R_J/R_E | 1.040 [0.954, 1.148] | 1.052 [0.939, 1.193] | 1.128 [0.898, 1.467] | unresolved | 1.022 (n=23) |
| 10 | IC-G/R_E | 2.608 [2.389, 2.815] | 3.338 [3.043, 3.634] | 8.960 [7.620, 10.235] | first arm above | 2.608 (n=24) |
| 10 | IC-S/R_E | 3.378 [2.963, 3.827] | 4.396 [3.829, 5.072] | 12.219 [9.851, 14.515] | first arm above | 3.378 (n=24) |
| 10 | IC-D/R_E | 1.685 [1.531, 1.844] | 2.358 [2.080, 2.664] | 7.434 [6.054, 8.912] | first arm above | 1.685 (n=24) |
| 10 | R_J/R_E | 0.966 [0.923, 1.002] | 0.959 [0.906, 1.002] | 0.926 [0.795, 1.008] | unresolved | 0.966 (n=24) |
| 11 | IC-G/R_E | 3.492 [3.140, 3.910] | 4.636 [4.146, 5.176] | 13.632 [11.730, 15.762] | first arm above | 3.492 (n=24) |
| 11 | IC-S/R_E | 5.315 [4.785, 5.949] | 7.046 [6.313, 7.976] | 20.665 [17.696, 23.682] | first arm above | 5.315 (n=24) |
| 11 | IC-D/R_E | 2.101 [1.856, 2.389] | 3.148 [2.680, 3.709] | 11.316 [9.133, 14.341] | first arm above | 2.101 (n=24) |
| 11 | R_J/R_E | 1.034 [0.964, 1.156] | 1.040 [0.958, 1.173] | 1.074 [0.925, 1.335] | unresolved | 1.005 (n=23) |
| 12 | IC-G/R_E | 5.239 [4.721, 5.909] | 7.103 [6.397, 8.081] | 21.579 [18.960, 25.178] | first arm above | 5.239 (n=24) |
| 12 | IC-S/R_E | 8.818 [7.499, 10.540] | 11.752 [9.871, 14.147] | 34.534 [28.246, 42.741] | first arm above | 8.818 (n=24) |
| 12 | IC-D/R_E | 2.962 [2.675, 3.358] | 4.762 [4.228, 5.476] | 18.698 [15.936, 22.455] | first arm above | 2.962 (n=24) |
| 12 | R_J/R_E | 1.018 [0.956, 1.072] | 1.019 [0.952, 1.080] | 1.026 [0.945, 1.121] | unresolved | 1.018 (n=24) |
| 13 | IC-G/R_E | 6.947 [6.045, 8.280] | 9.569 [8.243, 11.531] | 29.611 [24.800, 36.741] | first arm above | 6.947 (n=24) |
| 13 | IC-S/R_E | 14.195 [12.119, 17.251] | 18.916 [15.821, 22.804] | 55.009 [45.634, 68.511] | first arm above | 14.195 (n=24) |
| 13 | IC-D/R_E | 3.757 [3.129, 4.632] | 6.275 [5.118, 7.672] | 25.479 [19.829, 32.979] | first arm above | 3.757 (n=24) |
| 13 | R_J/R_E | 0.975 [0.878, 1.078] | 0.975 [0.873, 1.081] | 0.970 [0.853, 1.098] | unresolved | 0.975 (n=24) |
| 14 | IC-G/R_E | 14.014 [12.769, 15.392] | 19.567 [17.782, 21.307] | 62.037 [55.510, 68.898] | first arm above | 14.014 (n=24) |
| 14 | IC-S/R_E | 34.441 [31.413, 37.943] | 45.975 [41.816, 50.723] | 134.231 [119.514, 149.556] | first arm above | 34.441 (n=24) |
| 14 | IC-D/R_E | 7.527 [6.735, 8.377] | 13.111 [11.593, 14.835] | 55.795 [48.476, 64.505] | first arm above | 7.527 (n=24) |
| 14 | R_J/R_E | 1.044 [0.945, 1.151] | 1.045 [0.943, 1.150] | 1.050 [0.903, 1.204] | unresolved | 1.044 (n=24) |
| 15 | IC-G/R_E | 24.060 [21.464, 26.770] | 33.791 [29.882, 37.666] | 106.784 [90.944, 125.049] | first arm above | 24.060 (n=24) |
| 15 | IC-S/R_E | 78.792 [69.039, 89.018] | 104.856 [91.336, 119.264] | 300.465 [252.370, 357.347] | first arm above | 78.792 (n=24) |
| 15 | IC-D/R_E | 11.057 [9.156, 12.511] | 19.079 [15.773, 22.045] | 78.993 [62.762, 93.844] | first arm above | 11.057 (n=24) |
| 15 | R_J/R_E | 0.978 [0.882, 1.077] | 0.979 [0.882, 1.083] | 0.986 [0.864, 1.108] | unresolved | 0.978 (n=24) |
| 16 | IC-G/R_E | 41.440 [37.748, 44.990] | 58.207 [53.218, 63.276] | 180.692 [162.297, 202.589] | first arm above | 41.440 (n=24) |
| 16 | IC-S/R_E | 176.056 [160.519, 190.618] | 233.510 [212.460, 253.716] | 653.445 [587.614, 724.655] | first arm above | 176.056 (n=24) |
| 16 | IC-D/R_E | 20.414 [17.754, 22.897] | 36.058 [30.652, 41.021] | 149.993 [123.599, 176.809] | first arm above | 20.414 (n=24) |
| 16 | R_J/R_E | 0.953 [0.861, 1.055] | 0.953 [0.854, 1.058] | 0.954 [0.815, 1.108] | unresolved | 0.953 (n=24) |

## Distance remaining (geometric-mean ratio to R_E; the gap, not a pass/fail)


| k | IC-D/R_E w=0 | w=0.1 | w=1 | R_J/R_E w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|
| 8 | 1.127 | 1.347 | 2.805 | 0.965 | 0.951 | 0.877 |
| 9 | 1.450 | 1.906 | 5.480 | 1.040 | 1.052 | 1.128 |
| 10 | 1.685 | 2.358 | 7.434 | 0.966 | 0.959 | 0.926 |
| 11 | 2.101 | 3.148 | 11.316 | 1.034 | 1.040 | 1.074 |
| 12 | 2.962 | 4.762 | 18.698 | 1.018 | 1.019 | 1.026 |
| 13 | 3.757 | 6.275 | 25.479 | 0.975 | 0.975 | 0.970 |
| 14 | 7.527 | 13.111 | 55.795 | 1.044 | 1.045 | 1.050 |
| 15 | 11.057 | 19.079 | 78.993 | 0.978 | 0.979 | 0.986 |
| 16 | 20.414 | 36.058 | 149.993 | 0.953 | 0.953 | 0.954 |

## Absolute costs per verified solve (geometric mean over instances; Eop for R_E, Jop for the Jacobian arms; cost at w = 0, 0.1, 1)


| k | q | arm | verified/attempts | Wilson 95% | core ops geomean | median | q1 | q3 | cost w=0 | cost w=0.1 | cost w=1 | censored |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 251 | R_E | 20/24 | [0.641, 0.933] | 449.8 | 477.0 | 344.8 | 571.0 | 449.8 | 470.9 | 629.7 | 4 |
| 8 | 251 | R_J | 21/24 | [0.690, 0.957] | 434.2 | 447.0 | 323.0 | 560.0 | 434.2 | 447.8 | 552.2 | 3 |
| 8 | 251 | IC-G | 24/24 | [0.862, 1.000] | 615.9 | 609.5 | 550 | 680.5 | 615.9 | 728.1 | 1734 | 0 |
| 8 | 251 | IC-S | 24/24 | [0.862, 1.000] | 635.6 | 621.0 | 571.0 | 680.3 | 635.6 | 773.2 | 2002 | 0 |
| 8 | 251 | IC-D | 24/24 | [0.862, 1.000] | 507.1 | 504.5 | 453.3 | 562.3 | 507.1 | 634.2 | 1767 | 0 |
| 9 | 509 | R_E | 24/24 | [0.862, 1.000] | 495.8 | 503.5 | 400.5 | 571.3 | 495.8 | 503.9 | 569.3 | 0 |
| 9 | 509 | R_J | 23/24 | [0.798, 0.993] | 515.8 | 560.0 | 411.0 | 603.8 | 515.8 | 530.2 | 641.9 | 1 |
| 9 | 509 | IC-G | 24/24 | [0.862, 1.000] | 953.5 | 953.0 | 889.5 | 1022 | 953.5 | 1182 | 3229 | 0 |
| 9 | 509 | IC-S | 24/24 | [0.862, 1.000] | 1053 | 1046 | 940.8 | 1178 | 1053 | 1341 | 3912 | 0 |
| 9 | 509 | IC-D | 24/24 | [0.862, 1.000] | 718.9 | 714.5 | 648.3 | 813.5 | 718.9 | 960.5 | 3120 | 0 |
| 10 | 1021 | R_E | 24/24 | [0.862, 1.000] | 569.2 | 558.0 | 497.3 | 628.0 | 569.2 | 580.6 | 671.4 | 0 |
| 10 | 1021 | R_J | 24/24 | [0.862, 1.000] | 549.8 | 567.0 | 487.5 | 598.5 | 549.8 | 557.0 | 621.4 | 0 |
| 10 | 1021 | IC-G | 24/24 | [0.862, 1.000] | 1485 | 1487 | 1424 | 1513 | 1485 | 1938 | 6016 | 0 |
| 10 | 1021 | IC-S | 24/24 | [0.862, 1.000] | 1923 | 2055 | 1684 | 2205 | 1923 | 2552 | 8203 | 0 |
| 10 | 1021 | IC-D | 24/24 | [0.862, 1.000] | 959.2 | 995.5 | 912.3 | 1082 | 959.2 | 1369 | 4991 | 0 |
| 11 | 2039 | R_E | 24/24 | [0.862, 1.000] | 751.4 | 782.0 | 640.5 | 865.5 | 751.4 | 762.6 | 860.8 | 0 |
| 11 | 2039 | R_J | 23/24 | [0.798, 0.993] | 776.9 | 765.0 | 675.0 | 854.3 | 776.9 | 793.0 | 924.9 | 1 |
| 11 | 2039 | IC-G | 24/24 | [0.862, 1.000] | 2624 | 2633 | 2569 | 2678 | 2624 | 3535 | 11734 | 0 |
| 11 | 2039 | IC-S | 24/24 | [0.862, 1.000] | 3994 | 3996 | 3830 | 4299 | 3994 | 5374 | 17789 | 0 |
| 11 | 2039 | IC-D | 24/24 | [0.862, 1.000] | 1579 | 1619 | 1474 | 1795 | 1579 | 2401 | 9741 | 0 |
| 12 | 4093 | R_E | 24/24 | [0.862, 1.000] | 913.0 | 947.0 | 818.5 | 1040 | 913.0 | 927.7 | 1058 | 0 |
| 12 | 4093 | R_J | 24/24 | [0.862, 1.000] | 929.8 | 937.5 | 865.3 | 999.3 | 929.8 | 945.6 | 1086 | 0 |
| 12 | 4093 | IC-G | 24/24 | [0.862, 1.000] | 4784 | 4775 | 4697 | 4874 | 4784 | 6590 | 22838 | 0 |
| 12 | 4093 | IC-S | 24/24 | [0.862, 1.000] | 8051 | 8368 | 6728 | 9234 | 8051 | 10902 | 36549 | 0 |
| 12 | 4093 | IC-D | 24/24 | [0.862, 1.000] | 2705 | 2757 | 2565 | 2883 | 2705 | 4418 | 19789 | 0 |
| 13 | 8191 | R_E | 24/24 | [0.862, 1.000] | 1305 | 1381 | 1208 | 1525 | 1305 | 1328 | 1536 | 0 |
| 13 | 8191 | R_J | 24/24 | [0.862, 1.000] | 1273 | 1351 | 1185 | 1474 | 1273 | 1295 | 1490 | 0 |
| 13 | 8191 | IC-G | 24/24 | [0.862, 1.000] | 9062 | 9116 | 8992 | 9156 | 9062 | 12707 | 45493 | 0 |
| 13 | 8191 | IC-S | 24/24 | [0.862, 1.000] | 18518 | 18491 | 17140 | 20035 | 18518 | 25118 | 84513 | 0 |
| 13 | 8191 | IC-D | 24/24 | [0.862, 1.000] | 4901 | 5020 | 4720 | 5236 | 4901 | 8333 | 39145 | 0 |
| 14 | 16381 | R_E | 24/24 | [0.862, 1.000] | 1227 | 1244 | 1100 | 1320 | 1227 | 1249 | 1445 | 0 |
| 14 | 16381 | R_J | 24/24 | [0.862, 1.000] | 1281 | 1245 | 1139 | 1433 | 1281 | 1305 | 1517 | 0 |
| 14 | 16381 | IC-G | 24/24 | [0.862, 1.000] | 17201 | 17187 | 17104 | 17254 | 17201 | 24444 | 89616 | 0 |
| 14 | 16381 | IC-S | 24/24 | [0.862, 1.000] | 42271 | 41964 | 40500 | 43778 | 42271 | 57434 | 193902 | 0 |
| 14 | 16381 | IC-D | 24/24 | [0.862, 1.000] | 9238 | 9416 | 9060 | 10036 | 9238 | 16379 | 80598 | 0 |
| 15 | 32749 | R_E | 24/24 | [0.862, 1.000] | 1396 | 1413 | 1259 | 1499 | 1396 | 1424 | 1677 | 0 |
| 15 | 32749 | R_J | 24/24 | [0.862, 1.000] | 1365 | 1409 | 1270 | 1500 | 1365 | 1394 | 1653 | 0 |
| 15 | 32749 | IC-G | 24/24 | [0.862, 1.000] | 33583 | 33550 | 33487 | 33725 | 33583 | 48133 | 179032 | 0 |
| 15 | 32749 | IC-S | 24/24 | [0.862, 1.000] | 109981 | 110509 | 106968 | 111952 | 109981 | 149358 | 503754 | 0 |
| 15 | 32749 | IC-D | 24/24 | [0.862, 1.000] | 15434 | 17811 | 14419 | 17962 | 15434 | 27176 | 132439 | 0 |
| 16 | 65521 | R_E | 24/24 | [0.862, 1.000] | 1603 | 1613 | 1451 | 1763 | 1603 | 1640 | 1975 | 0 |
| 16 | 65521 | R_J | 24/24 | [0.862, 1.000] | 1527 | 1505 | 1403 | 1693 | 1527 | 1563 | 1884 | 0 |
| 16 | 65521 | IC-G | 24/24 | [0.862, 1.000] | 66419 | 66413 | 66275 | 66557 | 66419 | 95477 | 356901 | 0 |
| 16 | 65521 | IC-S | 24/24 | [0.862, 1.000] | 282179 | 283513 | 275230 | 287063 | 282179 | 383029 | 1290678 | 0 |
| 16 | 65521 | IC-D | 24/24 | [0.862, 1.000] | 32719 | 34889 | 33992 | 35173 | 32719 | 59146 | 296265 | 0 |

## Table 2. Paired cost ratios (geometric mean, 95% bootstrap CI over curves then instances, B = 2000); primary kappa_J = 1; ranking by the 5a rule
## Table 10. PC-B (planted-answer chain): verified-solve rates of the rho arms, per size

| k | R_E verified | R_J verified | IC-G | IC-S | IC-D | x'+1 rejected / verified solves (all arms) |
|---|---|---|---|---|---|---|
| 8 | 20/24 | 21/24 | 24/24 | 24/24 | 24/24 | 113/113 |
| 9 | 24/24 | 23/24 | 24/24 | 24/24 | 24/24 | 119/119 |
| 10 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 11 | 24/24 | 23/24 | 24/24 | 24/24 | 24/24 | 119/119 |
| 12 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 13 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 14 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 15 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 16 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |

## Table 11. PC-C (can the pipeline rank a genus-2 index-calculus arm as the winner? IC-D against plain rho in a random Jacobian)

| k | q | instances | IC-D/plain w=0 | w=0.1 | w=1 | upper CI < 1 at all w | IC-D solves | plain solves |
|---|---|---|---|---|---|---|---|---|
| 8 | 251 | 20 | 0.452 [0.394, 0.528] | 0.487 [0.422, 0.584] | 0.746 [0.619, 0.949] | yes | 20/20 | 20/20 |
| 10 | 1021 | 20 | 0.388 [0.311, 0.492] | 0.467 [0.369, 0.599] | 0.997 [0.732, 1.352] | no | 20/20 | 20/20 |
| 12 | 4093 | 20 | 0.389 [0.310, 0.485] | 0.539 [0.417, 0.689] | 1.409 [1.062, 1.836] | no | 20/20 | 20/20 |

## Table 12. NC-A (null population: does IC-D profit from the gluing? glued / random-Jacobian ratio of IC-D relation-phase trials)

| k | n glued | n random | glued trials gm | random trials gm | ratio glued/random | 95% CI (bootstrap) | CI inside [0.9,1.1] | CI disjoint from [0.85,1.15] | CI not inside [0.85,1.15] | random IC-D solves |
|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 24 | 20 | 147.97 | 131.76 | 1.123 | [0.930, 1.372] | no | no | yes | 20/20 |
| 10 | 24 | 20 | 493.13 | 543.68 | 0.907 | [0.764, 1.079] | no | no | yes | 20/20 |
| 12 | 24 | 20 | 2042.23 | 1789.09 | 1.141 | [0.995, 1.316] | no | no | yes | 20/20 |
| 13 | 24 | 20 | 4072.70 | 4146.54 | 0.982 | [0.898, 1.071] | no | no | no | 20/20 |

## Table 13. NC-B (wrong-answer detector) and NC-C (sabotaged transfer)

| control | k | arm | instances | returned a value | verified (must be 0) |
|---|---|---|---|---|---|
| NC-B | 8 | IC-D | 20 | 20 | 0 |
| NC-B | 8 | R_E | 20 | 19 | 0 |
| NC-B | 10 | IC-D | 20 | 20 | 0 |
| NC-B | 10 | R_E | 20 | 20 | 0 |
| NC-B | 12 | IC-D | 20 | 20 | 0 |
| NC-B | 12 | R_E | 20 | 20 | 0 |
| NC-C | 8 | IC-D with flipped T(H) | 20 | 20 | 0 |
| NC-C | 10 | IC-D with flipped T(H) | 20 | 20 | 0 |

## Table 14. Falsifiers and preregistered band checks (PASS = inside the band / not fired; MISS = outside / fired)

| item | measured | rule | status |
|---|---|---|---|
| F1 (any IC arm with upper CI of IC/R_E < 1 at every w, any size) | no hit | must not fire (a hit triggers replication on seed + 10^10) | PASS |
| F2 IC-D T_D slope (k all) | 0.967 [0.951, 0.983] | [0.85, 1.15] | PASS |
| F2 R_E steps slope (k all) | 0.297 [0.224, 0.367] | [0.4, 0.6] | MISS |
| H4 slope of IC-D/R_E on q (k >= 10, w=0) | 0.604 [0.579, 0.628] | [0.35, 0.60] | MISS |
| T_D/q median, k=8 | 0.578 | [0.40, 0.95] | PASS |
| T_G/q median, k=8 | 0.990 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=8 | 1.130 | [0.6, 1.7] | PASS |
| PC-B rho verified-solve rate R_E, k=8 | 20/24 | >= 0.95 | MISS |
| PC-B rho verified-solve rate R_J, k=8 | 21/24 | >= 0.95 | MISS |
| p_split pooled, k=8 | 0.497 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=8 | 3.986 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=8 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=9 | 0.572 | [0.40, 0.95] | PASS |
| T_G/q median, k=9 | 1.004 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=9 | 1.095 | [0.6, 1.7] | PASS |
| PC-B rho verified-solve rate R_E, k=9 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=9 | 23/24 | >= 0.95 | PASS |
| p_split pooled, k=9 | 0.494 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=9 | 3.955 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=9 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=10 | 0.546 | [0.45, 0.80] | PASS |
| T_G/q median, k=10 | 1.002 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=10 | 1.125 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=10 | 3.550 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=10 | 1.827 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=10 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=10 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=10 | 0.498 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=10 | 3.945 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=10 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=11 | 0.521 | [0.45, 0.80] | PASS |
| T_G/q median, k=11 | 0.997 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=11 | 1.139 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=11 | 1.642 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=11 | 3.327 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=11 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=11 | 23/24 | >= 0.95 | PASS |
| p_split pooled, k=11 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=11 | 3.930 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=11 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=12 | 0.515 | [0.45, 0.80] | PASS |
| T_G/q median, k=12 | 1.002 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=12 | 1.061 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=12 | 1.473 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=12 | 1.652 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=12 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=12 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=12 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=12 | 3.922 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=12 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=13 | 0.509 | [0.45, 0.80] | PASS |
| T_G/q median, k=13 | 1.004 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=13 | 1.099 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=13 | 1.136 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=13 | 1.080 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=13 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=13 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=13 | 0.499 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=13 | 3.917 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=13 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=14 | 0.523 | [0.45, 0.80] | PASS |
| T_G/q median, k=14 | 0.998 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=14 | 1.113 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=14 | 1.140 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=14 | 1.286 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=14 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=14 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=14 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=14 | 3.913 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=14 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=15 | 0.517 | [0.45, 0.80] | PASS |
| T_G/q median, k=15 | 0.998 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=15 | 1.265 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=15 | 1.094 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=15 | 1.144 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=15 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=15 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=15 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=15 | 3.911 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=15 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=16 | 0.518 | [0.45, 0.80] | PASS |
| T_G/q median, k=16 | 0.999 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=16 | 1.409 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=16 | 1.248 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=16 | 1.181 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=16 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=16 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=16 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=16 | 3.911 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=16 | 1.000 | >= 0.95 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=8 | 1.127 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=9 | 1.450 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=10 | 1.685 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=11 | 2.101 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=12 | 2.962 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=13 | 3.757 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=14 | 7.527 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=15 | 11.057 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=16 | 20.414 | > 1 | PASS |
| H4 point value IC-D/R_E at k=8 (w=0) | 1.127 | >= 5 | MISS |
| H4 point value IC-D/R_E at k=16 (w=0) | 20.414 | >= 30 | MISS |
| R_E pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only) | 1.612 / 1.612 | [1.2, 1.9] | PASS |
| R_J pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only) | 1.642 / 1.412 | [1.2, 1.9] | PASS |
| PC-A measured/formula in [0.5, 2] at every size | all in band | [0.5, 2] | PASS |
| PC-A fitted slope | 0.481 | [0.35, 0.65] | PASS |
| T_F measured/formula in [0.5, 2] at every PC-A size | all in band | [0.5, 2] | PASS |
| PC-B rho verified-solve rate (R_E + R_J, all sizes) | 423/432 | >= 0.95 | PASS |
| PC-B transfer checks, Jacobian cross-checks | 0 failures | 0 (100%) | PASS |
| PC-C IC-D below plain rho (upper CI < 1, all w), k=10 | 0 | yes | MISS |
| PC-C IC-D/plain at w=0, k=10 | 0.388 | [0.2, 0.8] | PASS |
| PC-C IC-D below plain rho (upper CI < 1, all w), k=12 | 0 | yes | MISS |
| PC-C IC-D/plain at w=0, k=12 | 0.389 | [0.2, 0.8] | PASS |
| NC-A (literal: CI disjoint from [0.85,1.15] at any size) | 0 sizes | 0 sizes | PASS |
| NC-A (strict: CI not inside [0.85,1.15] at any size) | 3 sizes | 0 sizes | MISS |
| NC-B verified returned values | 0 of 119 returned | 0 (100% rejected) | PASS |
| NC-B x'+1 rejected for every verified solve (main) | 1071/1071 | 100% | PASS |
| NC-C verified solves with sabotaged T(H) | 0 | 0 | PASS |
## Exploratory post-freeze diagnostics (posthoc.gp; not preregistered)


## A. NC-A pooled glued/random ratio of IC-D relation-phase trials (equal weight per size k in 8,10,12,13); bootstrap B = 2000 resampling instances within each group and size
pooled ratio glued/random = 1.034, 95% CI [0.958, 1.118]; H5 band [0.9, 1.1]; reject band [0.85, 1.15]

## B. Rho baseline diagnostics (R_E): cofactor, l, setup share, steps against sqrt(l/2)

| k | q | mean h | gm l | gm steps | gm sqrt(l/2) | gm steps / gm sqrt(l/2) | mean share of Eops outside walk steps+escapes (table + starts) | IC-D: mean share of Jops in setup (jsetup/jops) |
|---|---|---|---|---|---|---|---|---|
| 8 | 251 | 7.667 | 42.3 | 19.8 | 4.60 | 4.297 | 0.882 | 0.699 |
| 9 | 509 | 8.000 | 74.3 | 9.8 | 6.09 | 1.609 | 0.963 | 0.591 |
| 10 | 1021 | 10.667 | 110.3 | 12.5 | 7.43 | 1.683 | 0.960 | 0.475 |
| 11 | 2039 | 7.667 | 316.1 | 17.2 | 12.57 | 1.369 | 0.972 | 0.368 |
| 12 | 4093 | 7.333 | 650.8 | 24.5 | 18.04 | 1.359 | 0.970 | 0.241 |
| 13 | 8191 | 4.333 | 2916.8 | 39.6 | 38.19 | 1.036 | 0.967 | 0.168 |
| 14 | 16381 | 6.667 | 2709.8 | 37.9 | 36.81 | 1.031 | 0.967 | 0.090 |
| 15 | 32749 | 8.000 | 4725.7 | 44.2 | 48.61 | 0.910 | 0.962 | 0.058 |
| 16 | 65521 | 10.000 | 6684.3 | 65.6 | 57.81 | 1.135 | 0.956 | 0.029 |

slope of log(gm steps) on log(gm sqrt(l/2)) over k = 8..16: 0.600 (a sqrt(l/2) law gives 1)

## C. Phase-only diagnostic: gm(IC-D trials) / gm(R_E steps + escape doublings) per size (excludes table construction, starts and IC setup; not an arm cost, not used for ranking)

| k | gm IC-D trials | gm R_E steps+escapes | ratio |
|---|---|---|---|
| 8 | 148.0 | 23.5 | 6.31 |
| 9 | 287.8 | 10.3 | 27.84 |
| 10 | 493.1 | 13.0 | 37.92 |
| 11 | 990.0 | 17.6 | 56.37 |
| 12 | 2042.2 | 25.1 | 81.39 |
| 13 | 4072.7 | 40.2 | 101.22 |
| 14 | 8402.5 | 38.2 | 220.14 |
| 15 | 14538.3 | 44.6 | 325.69 |
| 16 | 31768.9 | 66.7 | 476.19 |

slope of the phase-only ratio on log q (k = 8..16): 0.691
## All preregistered tables (verbatim from out/summary.txt)

sizes with data: [8, 9, 10, 11, 12, 13, 14, 15, 16]; complete (both halves DONE): [8, 9, 10, 11, 12, 13, 14, 15, 16]

## Table 1. Verified solves, core operation counts per solve (geometric mean over instances), cost at w = 0, 0.1, 1 GAE per native op (kappa_J = 1)

| k | q | arm | verified/attempts | Wilson 95% | core ops geomean | median | q1 | q3 | cost w=0 | cost w=0.1 | cost w=1 | censored |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 251 | R_E | 20/24 | [0.641, 0.933] | 449.8 | 477.0 | 344.8 | 571.0 | 449.8 | 470.9 | 629.7 | 4 |
| 8 | 251 | R_J | 21/24 | [0.690, 0.957] | 434.2 | 447.0 | 323.0 | 560.0 | 434.2 | 447.8 | 552.2 | 3 |
| 8 | 251 | IC-G | 24/24 | [0.862, 1.000] | 615.9 | 609.5 | 550 | 680.5 | 615.9 | 728.1 | 1734 | 0 |
| 8 | 251 | IC-S | 24/24 | [0.862, 1.000] | 635.6 | 621.0 | 571.0 | 680.3 | 635.6 | 773.2 | 2002 | 0 |
| 8 | 251 | IC-D | 24/24 | [0.862, 1.000] | 507.1 | 504.5 | 453.3 | 562.3 | 507.1 | 634.2 | 1767 | 0 |
| 9 | 509 | R_E | 24/24 | [0.862, 1.000] | 495.8 | 503.5 | 400.5 | 571.3 | 495.8 | 503.9 | 569.3 | 0 |
| 9 | 509 | R_J | 23/24 | [0.798, 0.993] | 515.8 | 560.0 | 411.0 | 603.8 | 515.8 | 530.2 | 641.9 | 1 |
| 9 | 509 | IC-G | 24/24 | [0.862, 1.000] | 953.5 | 953.0 | 889.5 | 1022 | 953.5 | 1182 | 3229 | 0 |
| 9 | 509 | IC-S | 24/24 | [0.862, 1.000] | 1053 | 1046 | 940.8 | 1178 | 1053 | 1341 | 3912 | 0 |
| 9 | 509 | IC-D | 24/24 | [0.862, 1.000] | 718.9 | 714.5 | 648.3 | 813.5 | 718.9 | 960.5 | 3120 | 0 |
| 10 | 1021 | R_E | 24/24 | [0.862, 1.000] | 569.2 | 558.0 | 497.3 | 628.0 | 569.2 | 580.6 | 671.4 | 0 |
| 10 | 1021 | R_J | 24/24 | [0.862, 1.000] | 549.8 | 567.0 | 487.5 | 598.5 | 549.8 | 557.0 | 621.4 | 0 |
| 10 | 1021 | IC-G | 24/24 | [0.862, 1.000] | 1485 | 1487 | 1424 | 1513 | 1485 | 1938 | 6016 | 0 |
| 10 | 1021 | IC-S | 24/24 | [0.862, 1.000] | 1923 | 2055 | 1684 | 2205 | 1923 | 2552 | 8203 | 0 |
| 10 | 1021 | IC-D | 24/24 | [0.862, 1.000] | 959.2 | 995.5 | 912.3 | 1082 | 959.2 | 1369 | 4991 | 0 |
| 11 | 2039 | R_E | 24/24 | [0.862, 1.000] | 751.4 | 782.0 | 640.5 | 865.5 | 751.4 | 762.6 | 860.8 | 0 |
| 11 | 2039 | R_J | 23/24 | [0.798, 0.993] | 776.9 | 765.0 | 675.0 | 854.3 | 776.9 | 793.0 | 924.9 | 1 |
| 11 | 2039 | IC-G | 24/24 | [0.862, 1.000] | 2624 | 2633 | 2569 | 2678 | 2624 | 3535 | 11734 | 0 |
| 11 | 2039 | IC-S | 24/24 | [0.862, 1.000] | 3994 | 3996 | 3830 | 4299 | 3994 | 5374 | 17789 | 0 |
| 11 | 2039 | IC-D | 24/24 | [0.862, 1.000] | 1579 | 1619 | 1474 | 1795 | 1579 | 2401 | 9741 | 0 |
| 12 | 4093 | R_E | 24/24 | [0.862, 1.000] | 913.0 | 947.0 | 818.5 | 1040 | 913.0 | 927.7 | 1058 | 0 |
| 12 | 4093 | R_J | 24/24 | [0.862, 1.000] | 929.8 | 937.5 | 865.3 | 999.3 | 929.8 | 945.6 | 1086 | 0 |
| 12 | 4093 | IC-G | 24/24 | [0.862, 1.000] | 4784 | 4775 | 4697 | 4874 | 4784 | 6590 | 22838 | 0 |
| 12 | 4093 | IC-S | 24/24 | [0.862, 1.000] | 8051 | 8368 | 6728 | 9234 | 8051 | 10902 | 36549 | 0 |
| 12 | 4093 | IC-D | 24/24 | [0.862, 1.000] | 2705 | 2757 | 2565 | 2883 | 2705 | 4418 | 19789 | 0 |
| 13 | 8191 | R_E | 24/24 | [0.862, 1.000] | 1305 | 1381 | 1208 | 1525 | 1305 | 1328 | 1536 | 0 |
| 13 | 8191 | R_J | 24/24 | [0.862, 1.000] | 1273 | 1351 | 1185 | 1474 | 1273 | 1295 | 1490 | 0 |
| 13 | 8191 | IC-G | 24/24 | [0.862, 1.000] | 9062 | 9116 | 8992 | 9156 | 9062 | 12707 | 45493 | 0 |
| 13 | 8191 | IC-S | 24/24 | [0.862, 1.000] | 18518 | 18491 | 17140 | 20035 | 18518 | 25118 | 84513 | 0 |
| 13 | 8191 | IC-D | 24/24 | [0.862, 1.000] | 4901 | 5020 | 4720 | 5236 | 4901 | 8333 | 39145 | 0 |
| 14 | 16381 | R_E | 24/24 | [0.862, 1.000] | 1227 | 1244 | 1100 | 1320 | 1227 | 1249 | 1445 | 0 |
| 14 | 16381 | R_J | 24/24 | [0.862, 1.000] | 1281 | 1245 | 1139 | 1433 | 1281 | 1305 | 1517 | 0 |
| 14 | 16381 | IC-G | 24/24 | [0.862, 1.000] | 17201 | 17187 | 17104 | 17254 | 17201 | 24444 | 89616 | 0 |
| 14 | 16381 | IC-S | 24/24 | [0.862, 1.000] | 42271 | 41964 | 40500 | 43778 | 42271 | 57434 | 193902 | 0 |
| 14 | 16381 | IC-D | 24/24 | [0.862, 1.000] | 9238 | 9416 | 9060 | 10036 | 9238 | 16379 | 80598 | 0 |
| 15 | 32749 | R_E | 24/24 | [0.862, 1.000] | 1396 | 1413 | 1259 | 1499 | 1396 | 1424 | 1677 | 0 |
| 15 | 32749 | R_J | 24/24 | [0.862, 1.000] | 1365 | 1409 | 1270 | 1500 | 1365 | 1394 | 1653 | 0 |
| 15 | 32749 | IC-G | 24/24 | [0.862, 1.000] | 33583 | 33550 | 33487 | 33725 | 33583 | 48133 | 179032 | 0 |
| 15 | 32749 | IC-S | 24/24 | [0.862, 1.000] | 109981 | 110509 | 106968 | 111952 | 109981 | 149358 | 503754 | 0 |
| 15 | 32749 | IC-D | 24/24 | [0.862, 1.000] | 15434 | 17811 | 14419 | 17962 | 15434 | 27176 | 132439 | 0 |
| 16 | 65521 | R_E | 24/24 | [0.862, 1.000] | 1603 | 1613 | 1451 | 1763 | 1603 | 1640 | 1975 | 0 |
| 16 | 65521 | R_J | 24/24 | [0.862, 1.000] | 1527 | 1505 | 1403 | 1693 | 1527 | 1563 | 1884 | 0 |
| 16 | 65521 | IC-G | 24/24 | [0.862, 1.000] | 66419 | 66413 | 66275 | 66557 | 66419 | 95477 | 356901 | 0 |
| 16 | 65521 | IC-S | 24/24 | [0.862, 1.000] | 282179 | 283513 | 275230 | 287063 | 282179 | 383029 | 1290678 | 0 |
| 16 | 65521 | IC-D | 24/24 | [0.862, 1.000] | 32719 | 34889 | 33992 | 35173 | 32719 | 59146 | 296265 | 0 |

## Table 2. Paired cost ratios (geometric mean, 95% bootstrap CI over curves then instances, B = 2000); primary kappa_J = 1; ranking by the 5a rule

| k | pair | w=0 | w=0.1 | w=1 | ranking across the band | gm w=0 excl. censored |
|---|---|---|---|---|---|---|
| 8 | IC-G/R_E | 1.369 [1.211, 1.547] | 1.546 [1.351, 1.785] | 2.753 [2.117, 3.603] | first arm above | 1.426 (n=20) |
| 8 | IC-S/R_E | 1.413 [1.247, 1.591] | 1.642 [1.426, 1.865] | 3.179 [2.409, 4.067] | first arm above | 1.451 (n=20) |
| 8 | IC-D/R_E | 1.127 [1.001, 1.265] | 1.347 [1.155, 1.557] | 2.805 [2.085, 3.685] | first arm above | 1.182 (n=20) |
| 8 | R_J/R_E | 0.965 [0.821, 1.132] | 0.951 [0.783, 1.167] | 0.877 [0.594, 1.342] | unresolved | 0.977 (n=18) |
| 8 | IC-G/R_J | 1.419 [1.201, 1.651] | 1.626 [1.332, 1.931] | 3.140 [2.187, 4.216] | first arm above | 1.508 (n=21) |
| 8 | IC-S/R_J | 1.464 [1.219, 1.727] | 1.727 [1.399, 2.098] | 3.625 [2.448, 4.866] | first arm above | 1.566 (n=21) |
| 8 | IC-D/R_J | 1.168 [0.996, 1.345] | 1.416 [1.173, 1.666] | 3.199 [2.251, 4.285] | unresolved | 1.247 (n=21) |
| 9 | IC-G/R_E | 1.923 [1.680, 2.142] | 2.345 [2.005, 2.643] | 5.673 [4.505, 6.791] | first arm above | 1.923 (n=24) |
| 9 | IC-S/R_E | 2.125 [1.878, 2.385] | 2.662 [2.275, 3.048] | 6.872 [5.284, 8.335] | first arm above | 2.125 (n=24) |
| 9 | IC-D/R_E | 1.450 [1.302, 1.597] | 1.906 [1.666, 2.145] | 5.480 [4.457, 6.566] | first arm above | 1.450 (n=24) |
| 9 | R_J/R_E | 1.040 [0.954, 1.148] | 1.052 [0.939, 1.193] | 1.128 [0.898, 1.467] | unresolved | 1.022 (n=23) |
| 9 | IC-G/R_J | 1.849 [1.700, 2.020] | 2.229 [2.029, 2.468] | 5.031 [4.217, 5.843] | first arm above | 1.869 (n=23) |
| 9 | IC-S/R_J | 2.042 [1.840, 2.260] | 2.530 [2.239, 2.847] | 6.094 [4.967, 7.370] | first arm above | 2.046 (n=23) |
| 9 | IC-D/R_J | 1.394 [1.294, 1.500] | 1.812 [1.658, 1.986] | 4.860 [4.083, 5.659] | first arm above | 1.408 (n=23) |
| 10 | IC-G/R_E | 2.608 [2.389, 2.815] | 3.338 [3.043, 3.634] | 8.960 [7.620, 10.235] | first arm above | 2.608 (n=24) |
| 10 | IC-S/R_E | 3.378 [2.963, 3.827] | 4.396 [3.829, 5.072] | 12.219 [9.851, 14.515] | first arm above | 3.378 (n=24) |
| 10 | IC-D/R_E | 1.685 [1.531, 1.844] | 2.358 [2.080, 2.664] | 7.434 [6.054, 8.912] | first arm above | 1.685 (n=24) |
| 10 | R_J/R_E | 0.966 [0.923, 1.002] | 0.959 [0.906, 1.002] | 0.926 [0.795, 1.008] | unresolved | 0.966 (n=24) |
| 10 | IC-G/R_J | 2.700 [2.512, 2.910] | 3.479 [3.200, 3.779] | 9.681 [8.817, 10.659] | first arm above | 2.700 (n=24) |
| 10 | IC-S/R_J | 3.497 [3.086, 3.970] | 4.582 [4.025, 5.286] | 13.201 [11.404, 15.331] | first arm above | 3.497 (n=24) |
| 10 | IC-D/R_J | 1.745 [1.580, 1.920] | 2.458 [2.168, 2.771] | 8.032 [6.663, 9.576] | first arm above | 1.745 (n=24) |
| 11 | IC-G/R_E | 3.492 [3.140, 3.910] | 4.636 [4.146, 5.176] | 13.632 [11.730, 15.762] | first arm above | 3.492 (n=24) |
| 11 | IC-S/R_E | 5.315 [4.785, 5.949] | 7.046 [6.313, 7.976] | 20.665 [17.696, 23.682] | first arm above | 5.315 (n=24) |
| 11 | IC-D/R_E | 2.101 [1.856, 2.389] | 3.148 [2.680, 3.709] | 11.316 [9.133, 14.341] | first arm above | 2.101 (n=24) |
| 11 | R_J/R_E | 1.034 [0.964, 1.156] | 1.040 [0.958, 1.173] | 1.074 [0.925, 1.335] | unresolved | 1.005 (n=23) |
| 11 | IC-G/R_J | 3.378 [3.065, 3.727] | 4.458 [4.011, 4.955] | 12.687 [10.648, 14.570] | first arm above | 3.452 (n=23) |
| 11 | IC-S/R_J | 5.141 [4.678, 5.644] | 6.776 [6.117, 7.543] | 19.233 [16.145, 21.923] | first arm above | 5.251 (n=23) |
| 11 | IC-D/R_J | 2.032 [1.799, 2.312] | 3.028 [2.605, 3.530] | 10.532 [8.390, 13.131] | first arm above | 2.064 (n=23) |
| 12 | IC-G/R_E | 5.239 [4.721, 5.909] | 7.103 [6.397, 8.081] | 21.579 [18.960, 25.178] | first arm above | 5.239 (n=24) |
| 12 | IC-S/R_E | 8.818 [7.499, 10.540] | 11.752 [9.871, 14.147] | 34.534 [28.246, 42.741] | first arm above | 8.818 (n=24) |
| 12 | IC-D/R_E | 2.962 [2.675, 3.358] | 4.762 [4.228, 5.476] | 18.698 [15.936, 22.455] | first arm above | 2.962 (n=24) |
| 12 | R_J/R_E | 1.018 [0.956, 1.072] | 1.019 [0.952, 1.080] | 1.026 [0.945, 1.121] | unresolved | 1.018 (n=24) |
| 12 | IC-G/R_J | 5.144 [4.760, 5.662] | 6.969 [6.444, 7.656] | 21.034 [19.409, 23.096] | first arm above | 5.144 (n=24) |
| 12 | IC-S/R_J | 8.658 [7.331, 10.135] | 11.530 [9.695, 13.651] | 33.662 [28.191, 39.559] | first arm above | 8.658 (n=24) |
| 12 | IC-D/R_J | 2.909 [2.656, 3.213] | 4.672 [4.201, 5.275] | 18.226 [15.964, 20.972] | first arm above | 2.909 (n=24) |
| 13 | IC-G/R_E | 6.947 [6.045, 8.280] | 9.569 [8.243, 11.531] | 29.611 [24.800, 36.741] | first arm above | 6.947 (n=24) |
| 13 | IC-S/R_E | 14.195 [12.119, 17.251] | 18.916 [15.821, 22.804] | 55.009 [45.634, 68.511] | first arm above | 14.195 (n=24) |
| 13 | IC-D/R_E | 3.757 [3.129, 4.632] | 6.275 [5.118, 7.672] | 25.479 [19.829, 32.979] | first arm above | 3.757 (n=24) |
| 13 | R_J/R_E | 0.975 [0.878, 1.078] | 0.975 [0.873, 1.081] | 0.970 [0.853, 1.098] | unresolved | 0.975 (n=24) |
| 13 | IC-G/R_J | 7.122 [6.131, 8.634] | 9.815 [8.411, 11.928] | 30.527 [25.390, 38.643] | first arm above | 7.122 (n=24) |
| 13 | IC-S/R_J | 14.553 [12.527, 17.540] | 19.402 [16.611, 23.643] | 56.710 [46.867, 71.036] | first arm above | 14.553 (n=24) |
| 13 | IC-D/R_J | 3.851 [3.329, 4.630] | 6.437 [5.463, 7.927] | 26.267 [21.199, 33.895] | first arm above | 3.851 (n=24) |
| 14 | IC-G/R_E | 14.014 [12.769, 15.392] | 19.567 [17.782, 21.307] | 62.037 [55.510, 68.898] | first arm above | 14.014 (n=24) |
| 14 | IC-S/R_E | 34.441 [31.413, 37.943] | 45.975 [41.816, 50.723] | 134.231 [119.514, 149.556] | first arm above | 34.441 (n=24) |
| 14 | IC-D/R_E | 7.527 [6.735, 8.377] | 13.111 [11.593, 14.835] | 55.795 [48.476, 64.505] | first arm above | 7.527 (n=24) |
| 14 | R_J/R_E | 1.044 [0.945, 1.151] | 1.045 [0.943, 1.150] | 1.050 [0.903, 1.204] | unresolved | 1.044 (n=24) |
| 14 | IC-G/R_J | 13.427 [12.012, 14.747] | 18.732 [16.848, 20.587] | 59.064 [51.478, 66.313] | first arm above | 13.427 (n=24) |
| 14 | IC-S/R_J | 32.998 [29.403, 36.436] | 44.012 [38.983, 48.573] | 127.797 [110.869, 144.479] | first arm above | 32.998 (n=24) |
| 14 | IC-D/R_J | 7.211 [6.223, 8.140] | 12.551 [10.814, 14.417] | 53.120 [44.649, 63.002] | first arm above | 7.211 (n=24) |
| 15 | IC-G/R_E | 24.060 [21.464, 26.770] | 33.791 [29.882, 37.666] | 106.784 [90.944, 125.049] | first arm above | 24.060 (n=24) |
| 15 | IC-S/R_E | 78.792 [69.039, 89.018] | 104.856 [91.336, 119.264] | 300.465 [252.370, 357.347] | first arm above | 78.792 (n=24) |
| 15 | IC-D/R_E | 11.057 [9.156, 12.511] | 19.079 [15.773, 22.045] | 78.993 [62.762, 93.844] | first arm above | 11.057 (n=24) |
| 15 | R_J/R_E | 0.978 [0.882, 1.077] | 0.979 [0.882, 1.083] | 0.986 [0.864, 1.108] | unresolved | 0.978 (n=24) |
| 15 | IC-G/R_J | 24.598 [22.759, 26.739] | 34.520 [31.578, 37.997] | 108.296 [96.330, 121.935] | first arm above | 24.598 (n=24) |
| 15 | IC-S/R_J | 80.557 [73.577, 88.217] | 107.117 [97.698, 118.252] | 304.719 [272.645, 342.020] | first arm above | 80.557 (n=24) |
| 15 | IC-D/R_J | 11.305 [10.152, 12.297] | 19.490 [17.162, 21.615] | 80.112 [69.077, 90.240] | first arm above | 11.305 (n=24) |
| 16 | IC-G/R_E | 41.440 [37.748, 44.990] | 58.207 [53.218, 63.276] | 180.692 [162.297, 202.589] | first arm above | 41.440 (n=24) |
| 16 | IC-S/R_E | 176.056 [160.519, 190.618] | 233.510 [212.460, 253.716] | 653.445 [587.614, 724.655] | first arm above | 176.056 (n=24) |
| 16 | IC-D/R_E | 20.414 [17.754, 22.897] | 36.058 [30.652, 41.021] | 149.993 [123.599, 176.809] | first arm above | 20.414 (n=24) |
| 16 | R_J/R_E | 0.953 [0.861, 1.055] | 0.953 [0.854, 1.058] | 0.954 [0.815, 1.108] | unresolved | 0.953 (n=24) |
| 16 | IC-G/R_J | 43.495 [41.295, 45.725] | 61.078 [57.906, 64.131] | 189.473 [174.968, 205.037] | first arm above | 43.495 (n=24) |
| 16 | IC-S/R_J | 184.787 [173.615, 195.770] | 245.028 [230.518, 258.757] | 685.200 [635.516, 740.495] | first arm above | 184.787 (n=24) |
| 16 | IC-D/R_J | 21.426 [18.297, 24.199] | 37.837 [31.339, 43.486] | 157.282 [126.425, 188.427] | first arm above | 21.426 (n=24) |

## Table 3. Jacobian group operation constant kappa_J (counted field operations of the explicit formulas over those of E additions, S = 0.8 M) and p_split

| k | kappa_J (I/M=3) | kappa_J (I/M=10) | kappa_J (I/M=30) | J op (M,S,I per op) | E op (M,S,I per op) | p_split pooled | splits | classified trials |
|---|---|---|---|---|---|---|---|---|
| 8 | 6.197 | 3.986 | 2.796 | 28.35, 5.37, 2.00 | 1.99, 1.57, 1.00 | 0.497 | 8001 | 16093 |
| 9 | 6.127 | 3.955 | 2.782 | 28.28, 5.28, 2.00 | 2.00, 1.61, 1.00 | 0.494 | 16858 | 34100 |
| 10 | 6.110 | 3.945 | 2.778 | 28.20, 5.20, 2.00 | 2.00, 1.60, 1.00 | 0.498 | 35973 | 72211 |
| 11 | 6.076 | 3.930 | 2.771 | 28.13, 5.13, 2.00 | 2.00, 1.62, 1.00 | 0.500 | 77522 | 155179 |
| 12 | 6.059 | 3.922 | 2.768 | 28.08, 5.08, 2.00 | 2.00, 1.62, 1.00 | 0.500 | 163492 | 327161 |
| 13 | 6.047 | 3.917 | 2.766 | 28.05, 5.05, 2.00 | 2.00, 1.62, 1.00 | 0.499 | 359429 | 720669 |
| 14 | 6.037 | 3.913 | 2.764 | 28.02, 5.02, 2.00 | 2.00, 1.63, 1.00 | 0.500 | 795362 | 1589376 |
| 15 | 6.035 | 3.911 | 2.763 | 28.01, 5.01, 2.00 | 2.00, 1.63, 1.00 | 0.500 | 1880608 | 3758626 |
| 16 | 6.034 | 3.911 | 2.763 | 28.01, 5.01, 2.00 | 2.00, 1.62, 1.00 | 0.500 | 4547881 | 9094177 |

## Table 4. Preregistered band predictions (section 4) against measurements, per size

| k | q | IC-D T_D/q median [min,max] | frac in [0.40,0.95] | IC-G T_G/q median | IC-S T_S meas/formula (gm) | R_E steps/sqrt(l/2) mean (gm) | R_J mean (gm) | IC-D solve rate |
|---|---|---|---|---|---|---|---|---|
| 8 | 251 | 0.578 [0.434, 0.805] | 1.000 | 0.990 | 1.130 | 12.165 (4.297) | 7.572 (3.032) | 1.000 |
| 9 | 509 | 0.572 [0.308, 0.827] | 0.958 | 1.004 | 1.095 | 2.930 (1.609) | 5.924 (2.536) | 1.000 |
| 10 | 1021 | 0.546 [0.202, 0.690] | 0.750 | 1.002 | 1.125 | 3.550 (1.683) | 1.827 (1.531) | 1.000 |
| 11 | 2039 | 0.521 [0.276, 0.658] | 0.792 | 0.997 | 1.139 | 1.642 (1.369) | 3.327 (1.689) | 1.000 |
| 12 | 4093 | 0.515 [0.298, 0.630] | 0.917 | 1.002 | 1.061 | 1.473 (1.359) | 1.652 (1.447) | 1.000 |
| 13 | 8191 | 0.509 [0.271, 0.584] | 0.958 | 1.004 | 1.099 | 1.136 (1.036) | 1.080 (0.902) | 1.000 |
| 14 | 16381 | 0.523 [0.354, 0.624] | 0.917 | 0.998 | 1.113 | 1.140 (1.031) | 1.286 (1.054) | 1.000 |
| 15 | 32749 | 0.517 [0.201, 0.581] | 0.750 | 0.998 | 1.265 | 1.094 (0.910) | 1.144 (1.026) | 1.000 |
| 16 | 65521 | 0.518 [0.201, 0.573] | 0.875 | 0.999 | 1.409 | 1.248 (1.135) | 1.181 (1.068) | 1.000 |

## Table 5. Fitted exponents (OLS of log geometric mean on log q; 95% bootstrap CI resampling instances within size)

| quantity | k range | slope | 95% CI | preregistered band |
|---|---|---|---|---|
| IC-D trials (T_D) | 8..16 | 0.967 | [0.951, 0.983] | F2: [0.85,1.15]; H2: [0.9,1.1] |
| IC-D trials (T_D) | 10..16 | 0.993 | [0.964, 1.019] | H2: [0.9,1.1] |
| IC-G trials (T_G) | 8..16 | 1.000 | [0.997, 1.003] | ~1 |
| IC-S trials (T_S) | 8..16 | 1.244 | [1.231, 1.257] | 6/5 asymptotically (not banded) |
| R_E steps | 8..16 | 0.297 | [0.224, 0.367] | F2: [0.4,0.6] |
| R_E steps | 10..16 | 0.376 | [0.288, 0.458] | F2: [0.4,0.6] |
| R_J steps | 8..16 | 0.297 | [0.230, 0.359] | [0.4,0.6] |
| IC-D core Jops | 8..16 | 0.756 | [0.742, 0.769] | not banded |
| R_E core Eops | 8..16 | 0.246 | [0.232, 0.262] | not banded |
| ratio IC-D/R_E (w=0, kappa_J=1) | 10..16 | 0.604 | [0.579, 0.628] | H4: [0.35,0.60] |
| ratio IC-D/R_E (w=0, kappa_J=1) | 8..16 | 0.510 | [0.493, 0.526] | not banded |
| ratio IC-D/R_E (w=1, kappa_J=1) | 10..16 | 0.720 | [0.679, 0.758] | not banded |
| ratio IC-G/R_E (w=0, kappa_J=1) | 10..16 | 0.676 | [0.659, 0.694] | not banded |
| ratio IC-S/R_E (w=0, kappa_J=1) | 10..16 | 0.958 | [0.937, 0.980] | not banded |

## Table 6. Distance remaining: geometric-mean ratio to the matched rho R_E (kappa_J = 1)

| k | IC-D/R_E w=0 | w=0.1 | w=1 | R_J/R_E w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|
| 8 | 1.127 | 1.347 | 2.805 | 0.965 | 0.951 | 0.877 |
| 9 | 1.450 | 1.906 | 5.480 | 1.040 | 1.052 | 1.128 |
| 10 | 1.685 | 2.358 | 7.434 | 0.966 | 0.959 | 0.926 |
| 11 | 2.101 | 3.148 | 11.316 | 1.034 | 1.040 | 1.074 |
| 12 | 2.962 | 4.762 | 18.698 | 1.018 | 1.019 | 1.026 |
| 13 | 3.757 | 6.275 | 25.479 | 0.975 | 0.975 | 0.970 |
| 14 | 7.527 | 13.111 | 55.795 | 1.044 | 1.045 | 1.050 |
| 15 | 11.057 | 19.079 | 78.993 | 0.978 | 0.979 | 0.986 |
| 16 | 20.414 | 36.058 | 149.993 | 0.953 | 0.953 | 0.954 |

## Table 7. Secondary pricing: Jop priced at the measured kappa_J (I/M = 3, 10, 30), IC-D/R_E geometric-mean ratio at w = 0 (point estimates)

| k | kappa=1 | kappa(I/M=3) | kappa(I/M=10) | kappa(I/M=30) |
|---|---|---|---|---|
| 8 | 1.127 | 6.986 | 4.493 | 3.152 |
| 9 | 1.450 | 8.884 | 5.735 | 4.034 |
| 10 | 1.685 | 10.297 | 6.649 | 4.681 |
| 11 | 2.101 | 12.764 | 8.256 | 5.821 |
| 12 | 2.962 | 17.948 | 11.619 | 8.199 |
| 13 | 3.757 | 22.718 | 14.715 | 10.390 |
| 14 | 7.527 | 45.441 | 29.449 | 20.803 |
| 15 | 11.057 | 66.727 | 43.248 | 30.556 |
| 16 | 20.414 | 123.184 | 79.835 | 56.407 |

## Table 8. Per-size construction and checks (all curves)

| k | curves | candidate E sampled | rejections by filter step (pattern3, cofactor, l|E', g2=0, singular, noWpt, #C'!=q+1, other) | #C'=q+1 failures | N*D!=0 | ref-Cantor mismatches / pairs | assoc fails | transfer hom fails / pairs | T(G),T(H) checks failed / total |
|---|---|---|---|---|---|---|---|---|---|
| 8 | 6 | 15 (singular 0, dup 0) | [6, 3, 0, 0, 0, 0, 0, 0] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 9 | 6 | 148 (singular 0, dup 0) | [63, 48, 0, 0, 0, 31, 0, 31] | 0 | 0/48 | 0/2004 | 0/120 | 0/179 | 0/24 |
| 10 | 6 | 120 (singular 0, dup 0) | [48, 38, 0, 0, 0, 38, 0, 28] | 0 | 0/48 | 0/2004 | 0/120 | 0/179 | 0/24 |
| 11 | 6 | 22 (singular 0, dup 0) | [9, 7, 0, 0, 0, 0, 0, 0] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 12 | 6 | 222 (singular 0, dup 0) | [91, 94, 0, 0, 0, 43, 0, 31] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 13 | 6 | 36 (singular 0, dup 0) | [12, 18, 0, 0, 0, 0, 0, 0] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 14 | 6 | 105 (singular 0, dup 0) | [37, 48, 0, 0, 0, 16, 0, 14] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 15 | 6 | 240 (singular 0, dup 0) | [82, 121, 0, 0, 0, 36, 0, 31] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |
| 16 | 6 | 261 (singular 0, dup 0) | [83, 136, 0, 0, 0, 49, 0, 36] | 0 | 0/48 | 0/2004 | 0/120 | 0/180 | 0/24 |

## Table 9. PC-A (IC-F against IC-D: can the harness resolve a q^(1/2) gap in operation counts?)

| k | q | instances | IC-F trials gm | IC-D trials gm | measured ratio F/D (paired gm) | formula sqrt(q)/2 | measured/formula | IC-F T_F measured/formula (q^1.5/4) | solves F | solves D |
|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 251 | 20 | 1114 | 135.5 | 8.222 | 7.921 | 1.038 | 1.121 | 20/20 | 20/20 |
| 10 | 1021 | 20 | 11383 | 534.7 | 21.289 | 15.977 | 1.333 | 1.396 | 20/20 | 20/20 |
| 12 | 4093 | 20 | 88681 | 2126 | 41.720 | 31.988 | 1.304 | 1.355 | 20/20 | 20/20 |
| 14 | 16381 | 20 | 498770 | 8164 | 61.093 | 63.994 | 0.955 | 0.952 | 20/20 | 20/20 |

PC-A fitted slope of log(ratio F/D) on log q: 0.481 (pass band [0.35, 0.65]; per-size measured/formula pass band [0.5, 2])

## Table 10. PC-B (planted-answer chain): verified-solve rates of the rho arms, per size

| k | R_E verified | R_J verified | IC-G | IC-S | IC-D | x'+1 rejected / verified solves (all arms) |
|---|---|---|---|---|---|---|
| 8 | 20/24 | 21/24 | 24/24 | 24/24 | 24/24 | 113/113 |
| 9 | 24/24 | 23/24 | 24/24 | 24/24 | 24/24 | 119/119 |
| 10 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 11 | 24/24 | 23/24 | 24/24 | 24/24 | 24/24 | 119/119 |
| 12 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 13 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 14 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 15 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |
| 16 | 24/24 | 24/24 | 24/24 | 24/24 | 24/24 | 120/120 |

## Table 11. PC-C (can the pipeline rank a genus-2 index-calculus arm as the winner? IC-D against plain rho in a random Jacobian)

| k | q | instances | IC-D/plain w=0 | w=0.1 | w=1 | upper CI < 1 at all w | IC-D solves | plain solves |
|---|---|---|---|---|---|---|---|---|
| 8 | 251 | 20 | 0.452 [0.394, 0.528] | 0.487 [0.422, 0.584] | 0.746 [0.619, 0.949] | yes | 20/20 | 20/20 |
| 10 | 1021 | 20 | 0.388 [0.311, 0.492] | 0.467 [0.369, 0.599] | 0.997 [0.732, 1.352] | no | 20/20 | 20/20 |
| 12 | 4093 | 20 | 0.389 [0.310, 0.485] | 0.539 [0.417, 0.689] | 1.409 [1.062, 1.836] | no | 20/20 | 20/20 |

## Table 12. NC-A (null population: does IC-D profit from the gluing? glued / random-Jacobian ratio of IC-D relation-phase trials)

| k | n glued | n random | glued trials gm | random trials gm | ratio glued/random | 95% CI (bootstrap) | CI inside [0.9,1.1] | CI disjoint from [0.85,1.15] | CI not inside [0.85,1.15] | random IC-D solves |
|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 24 | 20 | 147.97 | 131.76 | 1.123 | [0.930, 1.372] | no | no | yes | 20/20 |
| 10 | 24 | 20 | 493.13 | 543.68 | 0.907 | [0.764, 1.079] | no | no | yes | 20/20 |
| 12 | 24 | 20 | 2042.23 | 1789.09 | 1.141 | [0.995, 1.316] | no | no | yes | 20/20 |
| 13 | 24 | 20 | 4072.70 | 4146.54 | 0.982 | [0.898, 1.071] | no | no | no | 20/20 |

## Table 13. NC-B (wrong-answer detector) and NC-C (sabotaged transfer)

| control | k | arm | instances | returned a value | verified (must be 0) |
|---|---|---|---|---|---|
| NC-B | 8 | IC-D | 20 | 20 | 0 |
| NC-B | 8 | R_E | 20 | 19 | 0 |
| NC-B | 10 | IC-D | 20 | 20 | 0 |
| NC-B | 10 | R_E | 20 | 20 | 0 |
| NC-B | 12 | IC-D | 20 | 20 | 0 |
| NC-B | 12 | R_E | 20 | 20 | 0 |
| NC-C | 8 | IC-D with flipped T(H) | 20 | 20 | 0 |
| NC-C | 10 | IC-D with flipped T(H) | 20 | 20 | 0 |

## Table 14. Falsifiers and preregistered band checks (PASS = inside the band / not fired; MISS = outside / fired)

| item | measured | rule | status |
|---|---|---|---|
| F1 (any IC arm with upper CI of IC/R_E < 1 at every w, any size) | no hit | must not fire (a hit triggers replication on seed + 10^10) | PASS |
| F2 IC-D T_D slope (k all) | 0.967 [0.951, 0.983] | [0.85, 1.15] | PASS |
| F2 R_E steps slope (k all) | 0.297 [0.224, 0.367] | [0.4, 0.6] | MISS |
| H4 slope of IC-D/R_E on q (k >= 10, w=0) | 0.604 [0.579, 0.628] | [0.35, 0.60] | MISS |
| T_D/q median, k=8 | 0.578 | [0.40, 0.95] | PASS |
| T_G/q median, k=8 | 0.990 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=8 | 1.130 | [0.6, 1.7] | PASS |
| PC-B rho verified-solve rate R_E, k=8 | 20/24 | >= 0.95 | MISS |
| PC-B rho verified-solve rate R_J, k=8 | 21/24 | >= 0.95 | MISS |
| p_split pooled, k=8 | 0.497 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=8 | 3.986 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=8 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=9 | 0.572 | [0.40, 0.95] | PASS |
| T_G/q median, k=9 | 1.004 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=9 | 1.095 | [0.6, 1.7] | PASS |
| PC-B rho verified-solve rate R_E, k=9 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=9 | 23/24 | >= 0.95 | PASS |
| p_split pooled, k=9 | 0.494 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=9 | 3.955 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=9 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=10 | 0.546 | [0.45, 0.80] | PASS |
| T_G/q median, k=10 | 1.002 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=10 | 1.125 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=10 | 3.550 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=10 | 1.827 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=10 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=10 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=10 | 0.498 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=10 | 3.945 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=10 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=11 | 0.521 | [0.45, 0.80] | PASS |
| T_G/q median, k=11 | 0.997 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=11 | 1.139 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=11 | 1.642 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=11 | 3.327 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=11 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=11 | 23/24 | >= 0.95 | PASS |
| p_split pooled, k=11 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=11 | 3.930 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=11 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=12 | 0.515 | [0.45, 0.80] | PASS |
| T_G/q median, k=12 | 1.002 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=12 | 1.061 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=12 | 1.473 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=12 | 1.652 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=12 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=12 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=12 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=12 | 3.922 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=12 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=13 | 0.509 | [0.45, 0.80] | PASS |
| T_G/q median, k=13 | 1.004 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=13 | 1.099 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=13 | 1.136 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=13 | 1.080 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=13 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=13 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=13 | 0.499 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=13 | 3.917 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=13 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=14 | 0.523 | [0.45, 0.80] | PASS |
| T_G/q median, k=14 | 0.998 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=14 | 1.113 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=14 | 1.140 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=14 | 1.286 | [1.2, 1.9] (k >= 10) | PASS |
| PC-B rho verified-solve rate R_E, k=14 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=14 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=14 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=14 | 3.913 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=14 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=15 | 0.517 | [0.45, 0.80] | PASS |
| T_G/q median, k=15 | 0.998 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=15 | 1.265 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=15 | 1.094 | [1.2, 1.9] (k >= 10) | MISS |
| R_J mean steps/sqrt(l/2), k=15 | 1.144 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=15 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=15 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=15 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=15 | 3.911 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=15 | 1.000 | >= 0.95 | PASS |
| T_D/q median, k=16 | 0.518 | [0.45, 0.80] | PASS |
| T_G/q median, k=16 | 0.999 | [0.95, 1.25] | PASS |
| T_S measured/formula (gm), k=16 | 1.409 | [0.6, 1.7] | PASS |
| R_E mean steps/sqrt(l/2), k=16 | 1.248 | [1.2, 1.9] (k >= 10) | PASS |
| R_J mean steps/sqrt(l/2), k=16 | 1.181 | [1.2, 1.9] (k >= 10) | MISS |
| PC-B rho verified-solve rate R_E, k=16 | 24/24 | >= 0.95 | PASS |
| PC-B rho verified-solve rate R_J, k=16 | 24/24 | >= 0.95 | PASS |
| p_split pooled, k=16 | 0.500 | [0.46, 0.54] | PASS |
| kappa_J (I/M=10), k=16 | 3.911 | [2.0, 4.5] | PASS |
| IC-D solve rate (F6), k=16 | 1.000 | >= 0.95 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=8 | 1.127 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=9 | 1.450 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=10 | 1.685 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=11 | 2.101 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=12 | 2.962 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=13 | 3.757 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=14 | 7.527 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=15 | 11.057 | > 1 | PASS |
| H4 every IC arm / R_E > 1 at all w (min gm ratio), k=16 | 20.414 | > 1 | PASS |
| H4 point value IC-D/R_E at k=8 (w=0) | 1.127 | >= 5 | MISS |
| H4 point value IC-D/R_E at k=16 (w=0) | 20.414 | >= 30 | MISS |
| R_E pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only) | 1.612 / 1.612 | [1.2, 1.9] | PASS |
| R_J pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only) | 1.642 / 1.412 | [1.2, 1.9] | PASS |
| PC-A measured/formula in [0.5, 2] at every size | all in band | [0.5, 2] | PASS |
| PC-A fitted slope | 0.481 | [0.35, 0.65] | PASS |
| T_F measured/formula in [0.5, 2] at every PC-A size | all in band | [0.5, 2] | PASS |
| PC-B rho verified-solve rate (R_E + R_J, all sizes) | 423/432 | >= 0.95 | PASS |
| PC-B transfer checks, Jacobian cross-checks | 0 failures | 0 (100%) | PASS |
| PC-C IC-D below plain rho (upper CI < 1, all w), k=10 | 0 | yes | MISS |
| PC-C IC-D/plain at w=0, k=10 | 0.388 | [0.2, 0.8] | PASS |
| PC-C IC-D below plain rho (upper CI < 1, all w), k=12 | 0 | yes | MISS |
| PC-C IC-D/plain at w=0, k=12 | 0.389 | [0.2, 0.8] | PASS |
| NC-A (literal: CI disjoint from [0.85,1.15] at any size) | 0 sizes | 0 sizes | PASS |
| NC-A (strict: CI not inside [0.85,1.15] at any size) | 3 sizes | 0 sizes | MISS |
| NC-B verified returned values | 0 of 119 returned | 0 (100% rejected) | PASS |
| NC-B x'+1 rejected for every verified solve (main) | 1071/1071 | 100% | PASS |
| NC-C verified solves with sabotaged T(H) | 0 | 0 | PASS |

