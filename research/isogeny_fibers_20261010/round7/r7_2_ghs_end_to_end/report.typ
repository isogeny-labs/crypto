#set page(margin: 1.6cm)
#set text(size: 9pt)
#set table(inset: 2pt, stroke: 0.4pt)
#show table: set text(size: 7.5pt)
= R7.2: is any toy binary elliptic curve weak under GHS, end to end?
2026-11-01. Branch `research/r7-2-ghs-end-to-end`. Source: `RESULTS.md` (same directory); raw records in `raw/`; commands in `commands.sh`.
Labels: measured = printed by `examples/r7_2_ghs_end_to_end.rs`; derived = arithmetic on measured values; extrapolated = a fitted line read outside the measured range.

*Question.* At a toy instance with N <= about 18, does Gaudry-style index calculus (IC) on the real GHS-descended Jacobian solve a planted ECDLP on $E(F_(q^n))$ with fewer counted group operations than Pollard rho, and than the best generic method, with relation collection, linear algebra mod ell and extraction all counted?

*Result.* No. 30 applicable instances, N = 12 to 28, 600 planted logs, every answer verified on E by all three methods. In bitop/v1 gate equivalents, expressed in E additions of the same curve (gae), the IC total divided by the best generic method (baby-step giant-step on 28 instances, tuned table rho on 2) lies between 1.91 \[1.72, 2.11\] (N = 28, q = 128, g = 4, n = 4) and 131 \[123, 141\] (95% bootstrap over the 20 paired logs); at N <= 18 the smallest is 9.18 \[8.49, 9.96\]. Against rho alone the smallest is 1.60 \[1.25, 2.12\]. For genus 4 over $F_(q^4)$ the ratio halves with every doubling of q (11.5, 6.4, 3.3, 1.9 at q = 16, 32, 64, 128); that fit crosses 1 at N about 32 (31 to 33), *extrapolated*. For genus 4 over $F_(q^3)$ the ratio stays at 18 to 30 (q = 16 to 256) and the linear algebra (q^2.1) outgrows the generic cost (q^1.4): no crossover is supported. Counting only Jacobian additions (one per E addition) would put IC below the generic method from N = 20 for n = 4, but that is a stage diagnostic: the library's Cantor addition costs 5.4 to 10.4 E additions.

*The (l,n) = (3,4), g = 8, q = 8 run.* Not hung and not a bug: R7.1's own example finishes it in 1175 s (point counts up to $N_9$ over $F_(2^27)$, about 9 min, and a 400000-element closure of 8 x 10^6 validated Cantor additions, about 14 min); IC on it costs 122 \[104, 143\] times BSGS.

#figure(image("fig1_ic_vs_generic.svg", width: 70%), caption: [Online cost per planted log, IC against the best generic method on the same curve (E-add equivalents, mean of 20 paired logs, 95% bootstrap bars). Source: `raw/run_*.json`, `figure_data.tsv`.])
#figure(image("fig2_ratio_vs_N.svg", width: 74%), caption: [Ratio IC / best generic against N. Filled: bitop/v1 gates with the library's Cantor arithmetic; open: Jacobian additions counted as E additions (stage diagnostic); x: all other priced phases with Jacobian additions free (derived). Dashed: log-log fits of the two genus-4 families.])
#figure(image("fig3_phase_breakdown.svg", width: 76%), caption: [Where the IC cost goes (mean of 20 logs), as a multiple of the best generic cost on the same curve.])
#figure(image("fig6_extrapolation.svg", width: 72%), caption: [Fitted trend of the two genus-4 families extended past the measured sizes (extrapolation; dashed and dotted lines are derived variants). Fit and bootstrap: `raw/fit_genus4.json`.])
#figure(image("fig4_factor_base_scan.svg", width: 70%), caption: [Factor-base scan on the 10 tuning logs (first curve of each q, n = 3, g = 4): the cost falls as the base grows to all $F_q$-points; sparse relations beat dense ones.])
#figure(image("fig5_hang_profile.svg", width: 96%), caption: [The (l,n) = (3,4), g = 8, q = 8 run of R5.2/R7.1: point-counting time by extension degree (two profile runs) and the phases of R7.1's run_curve against its measured total. Source: `raw/profile_*.json`, `raw/r71_rerun.log`.])

== All instances
Online cost in E-add equivalents, mean of 20 holdout logs; ratio = IC / best generic with 95% bootstrap interval; J/E = Jacobian additions over the best generic method's E operations (stage diagnostic).
#table(columns: (auto, auto, auto, auto, auto, auto, auto, auto, auto), align: (left, right, right, right, right, right, right, right, right),
  [instance (l,n,a,beta)], [N], [q], [g], [ell], [IC], [generic], [ratio (95% CI)], [J/E],
  [ghs-l2-n6-a0-b0x228], [12], [4], [8], [29], [1852], [14.1], [131.10 (122.81, 141.08)], [5.66],
  [ghs-l2-n6-a0-b0x221], [12], [4], [8], [67], [1822], [21.5], [84.66 (78.07, 92.34)], [3.88],
  [ghs-l2-n6-a0-b0xe2], [12], [4], [8], [83], [2502], [23.1], [108.11 (90.58, 129.32)], [8.48],
  [ghs-l2-n6-a0-b0x28], [12], [4], [8], [131], [2332], [27.8], [83.95 (72.53, 99.16)], [5.87],
  [ghs-l2-n6-a0-b0x220], [12], [4], [8], [149], [2835], [30.1], [94.20 (83.96, 105.23)], [7.65],
  [ghs-l2-n6-a0-b0x229], [12], [4], [8], [1049], [6379], [61.0], [104.62 (82.13, 129.95)], [12.41],
  [ghs-l3-n4-a0-b0xa2], [12], [8], [2], [7], [117], [6.4], [18.22 (16.35, 20.58)], [4.77],
  [ghs-l3-n4-a0-b0x3], [12], [8], [8], [37], [2139], [17.5], [122.14 (104.47, 142.82)], [5.01],
  [ghs-l3-n4-a0-b0x8], [12], [8], [4], [37], [426], [17.5], [24.35 (22.79, 26.12)], [3.51],
  [ghs-l3-n4-a0-b0x16], [12], [8], [4], [83], [524], [23.1], [22.63 (20.65, 24.73)], [3.73],
  [ghs-l4-n3-a0-b0x36f], [12], [16], [4], [1013], [1816], [66.7], [27.24 (24.31, 30.56)], [2.58],
  [ghs-l4-n3-a0-b0x74b], [12], [16], [4], [1039], [1902], [63.2], [30.07 (26.65, 34.36)], [2.89],
  [ghs-l2-n7-a0-b0x4d], [14], [4], [7], [79], [2483], [22.0], [112.67 (103.69, 122.53)], [8.22],
  [ghs-l2-n7-a0-b0x4c], [14], [4], [8], [157], [2519], [30.9], [81.62 (75.11, 89.46)], [4.25],
  [ghs-l2-n7-a0-b0x58], [14], [4], [8], [677], [3085], [58.3], [52.91 (47.54, 59.36)], [4.57],
  [ghs-l5-n3-a1-b0x31a6], [15], [32], [4], [16349], [4298], [229.2], [18.76 (16.00, 21.59)], [1.62],
  [ghs-l5-n3-a1-b0x1df9], [15], [32], [4], [16451], [5451], [207.0], [26.34 (23.29, 29.62)], [2.31],
  [ghs-l4-n4-a1-b0x3d2c], [16], [16], [4], [8179], [1725], [149.4], [11.54 (10.57, 12.63)], [1.83],
  [ghs-l4-n4-a1-b0x2944], [16], [16], [4], [8191], [1585], [172.6], [9.18 (8.49, 9.96)], [1.44],
  [ghs-l6-n3-a0-b0x37711], [18], [64], [4], [32801], [7291], [305.0], [23.91 (20.42, 27.56)], [1.69],
  [ghs-l6-n3-a1-b0x1e702], [18], [64], [4], [65563], [9652], [408.7], [23.61 (21.11, 26.40)], [1.73],
  [ghs-l5-n4-a0-b0x48eb3], [20], [32], [4], [131023], [3528], [553.9], [6.37 (5.60, 7.18)], [0.93],
  [ghs-l5-n4-a0-b0x90f2d], [20], [32], [4], [131129], [3382], [585.9], [5.77 (5.18, 6.50)], [0.83],
  [ghs-l7-n3-a0-b0x11ceab], [21], [128], [4], [524221], [22375], [1063.9], [21.03 (17.54, 26.04)], [1.40],
  [ghs-l7-n3-a1-b0x1dd060], [21], [128], [4], [1047199], [31714], [1641.5], [19.32 (16.70, 21.76)], [1.23],
  [ghs-l6-n4-a0-b0x5716d4], [24], [64], [4], [2097383], [7064], [2280.3], [3.10 (2.66, 3.67)], [0.41],
  [ghs-l6-n4-a1-b0xb1261a], [24], [64], [4], [2097803], [7484], [2264.7], [3.30 (3.02, 3.62)], [0.41],
  [ghs-l8-n3-a1-b0xc136f2], [24], [256], [4], [4194973], [65568], [3122.2], [21.00 (18.37, 23.55)], [1.15],
  [ghs-l8-n3-a0-b0x2a521b], [24], [256], [4], [4195349], [63833], [3483.9], [18.32 (16.84, 19.92)], [0.90],
  [ghs-l7-n4-a1-b0x19e70cc], [28], [128], [4], [33553901], [17129], [8969.6], [1.91 (1.72, 2.11)], [0.21],
)

== Cost model, protocol and limits
Unit: bitop/v1 (`docs/units/BITOPS.md`); Jacobian-side field operations are recorded by the library's operation ledger while the Cantor arithmetic runs (`add_unchecked`, no re-validation), E-side group operations by formula (ledger check: 1.000 over 400 additions); arithmetic mod ell is counted for the non-zero entries. Per instance: 10 tuning logs (relation mode, factor-base size, rho multiplier width) and 20 fresh paired holdout logs. Sparse relations chosen in 30 of 30 instances; the whole $F_q$-point set as factor base in 26 of 30.
Limits: toy sizes; generic Cantor arithmetic; no large-prime variation; table-based generic arms without the negation map; ecbench not used (no replay certificate, no scoreboard entry); cover construction unpriced; a selected class of curves (beta of magic number 3 for n = 4); N = 32 not buildable with the library's irreducible-polynomial table (degree <= 31); wall times contended and secondary. No claim about any deployed curve.
