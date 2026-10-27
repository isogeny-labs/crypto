#set page(margin: 1.6cm)
#set text(size: 9.5pt)
= Prime-field volcano layers, ecbench: RESULT
Two depth-8 ℓ=2 volcanoes over F\_p (m0: p=40327618377097, m5: p=57628051254530177), 2 curves per interior level, 6 targets each, solvers rho.negation and rho.plain, counted unit ecbench.gae. Both sessions audited: 864 records, 864 verified, 8 exact replays reproduced, 0 problems. Host busy, isolation L0 (counts only; no wall time). Neighbour/vertex curves are not in the curve registry.

== Measured
- m0: every ratio interval contains 1 (rho-neg floor/crater 0.889 \[0.661, 1.255\]; rho-plain 1.103 \[0.822, 1.526\]); level slope intervals contain 0.

- m5: rho-neg intervals all contain 1 (floor/crater 0.698 \[0.498, 1.037\]; slope -3.36% \[-7.08, 2.09\]). rho-plain middle/crater 0.688 \[0.562, 0.891\] and floor/crater 0.671 \[0.524, 0.883\] exclude 1; floor/middle 0.975 \[0.793, 1.168\] and the slope \[-3.71, 5.53\] do not.

== Reading
Of the 16 tested intervals (4 ratios plus a slope, for each of 2 solvers, on 2 volcanoes), 14 contain the null value; the 2 that exclude it are both rho.plain on m5 and both involve the crater, which is one curve with 6 targets. This is 2 volcanoes; the crater is a single curve , so a crater-specific effect cannot be separated from a curve-specific one. Nothing here supports or refutes any transfer attack, since no degree-ℓ isogeny map was computed. The non-excluding intervals are wide (about ±30%) and do not show equality to within a small margin.

== Tables
== \`m0\`: p = 40327618377097, depth 8
| rho-neg | middle / crater | 0.784 | \[0.591, 1.102\] | interval contains 1 | \
| rho-neg | floor / crater | 0.889 | \[0.661, 1.255\] | interval contains 1 | \
| rho-neg | floor / middle | 1.135 | \[0.977, 1.336\] | interval contains 1 | \
| rho-plain | middle / crater | 0.972 | \[0.748, 1.325\] | interval contains 1 | \
| rho-plain | floor / crater | 1.103 | \[0.822, 1.526\] | interval contains 1 | \
| rho-plain | floor / middle | 1.134 | \[0.934, 1.375\] | interval contains 1 | \
== \`m5\`: p = 57628051254530177, depth 8
| rho-neg | middle / crater | 0.824 | \[0.597, 1.209\] | interval contains 1 | \
| rho-neg | floor / crater | 0.698 | \[0.498, 1.037\] | interval contains 1 | \
| rho-neg | floor / middle | 0.847 | \[0.668, 1.072\] | interval contains 1 | \
| rho-plain | middle / crater | 0.688 | \[0.562, 0.891\] | interval excludes 1 | \
| rho-plain | floor / crater | 0.671 | \[0.524, 0.883\] | interval excludes 1 | \
| rho-plain | floor / middle | 0.975 | \[0.793, 1.168\] | interval contains 1 | \
Full per-level means: TABLES.md / tables.json. Figure: fig\_levels.svg.

= Koblitz n=83, ℓ=6473 (volcano\_koblitz\_n83\_20261018)
Curve y²+xy=x³+1 over F\_(2^83): t=-6151469093347, D\_K=-7, f=6473·53676929, \#E=2²·2417851639230796216685689. Native koblitz-step built Φ\_6473(X,j₀) from 9864 CRT primes (606365-bit coefficients, 8590 s total on 3 threads), reduced mod 2, factored into 78 Frobenius orbits of size 83, and found 6474 distinct level-1 j-invariants in F\_(2^83) (expected 6474). 48 of 48 sampled neighbours had exactly one twist of order N; 0 of 8 random j passed the control. ALL CHECKS PASS. j-list and log hashes: SHA256SUMS. Not done: ecbench measurements at n=83 (the group is far outside rho reach) and no isogeny maps; the level-1 neighbours share N, so this verifies the volcano structure at ℓ=6473 only.


#figure(image("fig_levels.svg", width: 100%), caption: [Mean counted GAE per level.])
