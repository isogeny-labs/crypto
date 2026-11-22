# GAE band: every twist_ecdlp arm priced at 0, 0.1 and 1 GAE per unpriced native operation

Rule applied: `/home/user/crypto/AGENTS.md` section 5a (owner rule, 2026-10-15). List every unpriced counter of every arm, the rho references included; price the unpriced work at w = 0, 0.1 and 1 group-addition equivalents (GAE) per native operation, the same way on every arm; rank two arms only where the order holds at every w in the band; where it changes, report "unresolved", never a win; never set a calibrated cost against an uncalibrated one; an arm that names a counter but does not emit its count is a lower bound and cannot be ranked.

Written 2026-11-07 from the recorded results of E0 to E6 and G1 to G3 under `research/twist_ecdlp_20261027`. Machine-readable twin: `gae_band.json` (same numbers, all band points, all per-b and per-cell values). Nothing was re-run and no repository crate was built: the arithmetic is PARI/GP 2.15.4 over counts already recorded in each experiment's `RESULTS.md`, `results.json`, `results.csv`, `summary.csv`, `opcounts.*`, `out/ratios.csv` (scripts: `gae_band_work/*.gp`, outputs `gae_band_work/*.md|json`). No number here is an estimate; every derived number says what it was derived from.

## Pricing convention

- cost(w) = (priced work) + w x (unpriced native operations). Priced work is a group addition (1 GAE) or, for the field-operation arms of E5 and G1, M + 0.8 S + (I/M) I divided by the GAE of an affine addition, 2.8 + I/M (one affine addition = 2 M + 1 S + 1 I, S = 0.8 M; G1 preregistration section 7).
- Native sets for the walk arms (G1 section 7): set B (primary) = selector hash SEL + distinguished-point test DPT (1 each per step, all arms) + sign canonicalisation CAN + cycle comparisons CYC (N1 only) + ladder branches LAD (Kummer arms only) + DP lookups LOOK + trail starts START; set A = LOOK + START only. For these arms the band is a grid over I/M in {3, 10, 30, 100} and w in {0, 0.1, 1} (0.1 and 1 shown for set B and for set A). The point w = 0 is the same for both sets.
- Intervals. G1, E5 and E2 record their own intervals on the step ratio (G1, E5) or per-run spread (E2). Because the per-step cost of every walk arm is a deterministic function of counted M, S, I and native counters (K arms within 0.03% of 21.5 M + 12 S + 1 I per step), a band ratio is: recorded step-ratio interval x per-step cost ratio. This reproduces G1's recorded K-mix/N1 and K-pure/N1 values to 0.114% in all 40 band cells and E5's recorded K/A0 cost ratios to 0.063%, so the same method is used for the arms where nothing was recorded (A0/N1 in G1, E5 band cells at w > 0). Where no interval exists, the cell says so and no ranking is made.
- Order across the band: "dearer" or "cheaper" only if the 95% interval is on one side of 1 at every one of the five band points (w = 0; w = 0.1 and 1 for set B; w = 0.1 and 1 for set A). Otherwise unresolved.
- Degenerate w = 0 point: when an arm records no priced (group-addition) count, its cost at w = 0 is 0 and so is its baseline's, and the w = 0 point carries no order. This is stated as such; it is an implementation gap (an arm must emit its group-operation count), not a tie in the science. Where the arms emit only one native counter, the ratio is the same at every w > 0.

## Result at a glance

| experiment | arms priced | matched baseline | priced counts recorded? | order holds across the band? |
|---|---|---|---|---|
| G1 | N1 (negation-map rho), A0, K-mix, K-pure | N1 (sound negation-map rho, measured) | yes (M, S, I, native set A and B) | K-pure dearer than N1 at every I/M in {3,10,30,100}: yes. K-mix dearer than N1 at I/M = 3 and 10: yes. K-mix vs N1 at I/M = 30 and 100: UNRESOLVED (interval straddles 1 at w = 1, set B). A0 vs N1: UNRESOLVED (N1 cheaper at w = 0, A0 cheaper at w = 1: A0/N1 = 0.67 [0.61, 0.73] at w = 1) |
| E5 | A0, K-pure, K-mix, K-pure on twist | A0 (additive rho, measured) | M, S, I yes; native counters not emitted by E5 (taken from G1, same code) | K-pure dearer than A0 at every I/M: yes. K-mix dearer than A0 at I/M = 3, 10, 30: yes; at I/M = 100 UNRESOLVED (interval straddles 1 at w = 0). Twist arm: no interval recorded, not ranked |
| E2 | E alone, E' alone, joint table, E-only user of joint table, rho on E | rho on E (measured, comparisons priced at w); within method, E alone and E + E' separate | yes (A + K priced, native priced at w; rho priced) | relation phase dearer than rho: yes, in all 63 runs (minimum ratio 8.04, 20.2, 80.8 at w = 0, 0.1, 1). Joint never cheaper than two separate runs: tie at w = 0 (|saving| < 3e-6), joint dearer by 3.3% to 6.2% at w = 0.1 and 1. rho cost is a lower bound (see gaps) |
| G2 | D_m, P class S, PF(m,1), PF(m,m), D_2m, X coupled | D_m (E-only Semaev decomposition) | projection group operations only (textbook model count) | D_m cheaper than P and PF at every w incl. 0: yes (projection 280,000 or 96,000 GAE vs 0). D_2m and X: dearer per relation at every w > 0 (w = 0 degenerate) |
| E3 | D (direct Semaev), X (F_p^2 route) | D | none (root-solves and table group sums only) | X dearer than D at every w > 0 (28.6 per relation pooled; 53.3 to 2812 per powered cell); w = 0 degenerate |
| E4 | A_E (E-only base), A_S (shared base) | A_E | none (counted solves only) | A_S dearer than A_E at every w > 0 (2.0246; same 773,404 relations, A_S does A_E's solves plus 21,887,240 more); w = 0 degenerate |
| E6 | none measured | model rho only | none (model columns only) | not rankable, model distance only |
| E0, E1, G3 | verification and census tiers (not ECDLP arms) | none | native counters (E0 T1 also explicit curve additions) | not applicable, no matched baseline, listed for completeness |

Gap to the matched rho, the distance remaining and not a pass/fail gate (AGENTS.md success criterion). Against the sound negation-map rho N1 (G1), the best Kummer arm, K-mix, costs 1.29x at I/M = 100 with native work at 0 and 0.98x at w = 1, set B (interval [0.91, 1.07], unresolved); at I/M = 3 it costs 5.9x (w = 0), 4.3x (w = 0.1), 1.77x (w = 1). Against rho on E (E2), the relation phase alone of the twin-table index calculus costs at least 8.04x (smallest of 63 runs, w = 0), with per-cell medians of 8.7 to 56.0 at w = 0 and 88 to 1215 at w = 1. No arm of any experiment is a verified improvement over its matched baseline anywhere in the band.

## G1 (N1 matched baseline; N1, A0, K-mix, K-pure)

Source: `G1-sound-negation-map-rho-baseline/out/opcounts.csv` (per-arm, per-b totals of steps, M, S, I and every native counter) and `out/ratios.csv` (recorded step-ratio intervals, b = 28 to 40, four instances). All arms solved every trial except where noted in G1; the sabotage and r = 64 control arms (n1sab, n1c2, n1r64) are controls, not ranked arms, and are in `gae_band.json` only through the recorded counts they share (not priced here). Unpriced counters listed for every arm: SEL, DPT (all arms), CAN and CYC (N1), LAD (K arms), LOOK, START (all arms).

| arm | steps (b>=28) | native B per step (SEL+DPT+CAN+CYC+LAD+LOOK+START) | native A per step (LOOK+START) | M/step | S/step | I/step |
|---|---|---|---|---|---|---|
| N1 negation-map rho (matched baseline) | 131915962 | 5.00025 | 0.000254655 | 2.04671 | 1.03114 | 1.01557 |
| A0 additive rho (no negation) | 96454351 | 2.00035 | 0.000354012 | 2.00000 | 1.00000 | 1.00000 |
| K-mix Kummer walk | 76188339 | 4.50021 | 0.000345788 | 21.4991 | 11.9995 | 1.00000 |
| K-pure Kummer walk | 103007317 | 4.50087 | 0.000358722 | 21.5036 | 12.0021 | 1.00000 |

Per-step cost in GAE (one GAE = one affine addition = I/M + 2.8 M, S = 0.8 M), native set B priced at w, pooled b >= 28:

| arm | I/M | w=0 | w=0.1 (B) | w=1 (B) | w=0.1 (A) | w=1 (A) |
|---|---|---|---|---|---|---|
| N1 negation-map rho (matched baseline) | 3 | 1.02040 | 1.52043 | 6.02066 | 1.02043 | 1.02066 |
| N1 negation-map rho (matched baseline) | 10 | 1.01776 | 1.51779 | 6.01802 | 1.01779 | 1.01802 |
| N1 negation-map rho (matched baseline) | 30 | 1.01643 | 1.51645 | 6.01668 | 1.01645 | 1.01668 |
| N1 negation-map rho (matched baseline) | 100 | 1.01584 | 1.51587 | 6.01610 | 1.01587 | 1.01610 |
| A0 additive rho (no negation) | 3 | 1.00000 | 1.20004 | 3.00035 | 1.00004 | 1.00035 |
| A0 additive rho (no negation) | 10 | 1.00000 | 1.20004 | 3.00035 | 1.00004 | 1.00035 |
| A0 additive rho (no negation) | 30 | 1.00000 | 1.20004 | 3.00035 | 1.00004 | 1.00035 |
| A0 additive rho (no negation) | 100 | 1.00000 | 1.20004 | 3.00035 | 1.00004 | 1.00035 |
| K-mix Kummer walk | 3 | 5.87907 | 6.32910 | 10.3793 | 5.87911 | 5.87942 |
| K-mix Kummer walk | 10 | 3.21083 | 3.66085 | 7.71104 | 3.21087 | 3.21118 |
| K-mix Kummer walk | 30 | 1.86276 | 2.31278 | 6.36297 | 1.86280 | 1.86311 |
| K-mix Kummer walk | 100 | 1.27528 | 1.72530 | 5.77549 | 1.27531 | 1.27562 |
| K-pure Kummer walk | 3 | 5.88022 | 6.33030 | 10.3811 | 5.88025 | 5.88058 |
| K-pure Kummer walk | 10 | 3.21135 | 3.66144 | 7.71222 | 3.21138 | 3.21171 |
| K-pure Kummer walk | 30 | 1.86297 | 2.31305 | 6.36384 | 1.86300 | 1.86332 |
| K-pure Kummer walk | 100 | 1.27534 | 1.72543 | 5.77622 | 1.27538 | 1.27570 |

Cost per walk (trial) at b = 40 in GAE (mean over the recorded trials of that cell, failures none; set B):

| arm | trials | mean steps | I/M | w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|
| N1 negation-map rho (matched baseline) | 104 | 8.80487e5 | 3 | 8.98429e5 | 1.33868e6 | 5.30092e6 |
| N1 negation-map rho (matched baseline) | 104 | 8.80487e5 | 100 | 8.94419e5 | 1.33467e6 | 5.29691e6 |
| A0 additive rho (no negation) | 48 | 1.19951e6 | 3 | 1.19951e6 | 1.43942e6 | 3.59860e6 |
| A0 additive rho (no negation) | 48 | 1.19951e6 | 100 | 1.19951e6 | 1.43942e6 | 3.59860e6 |
| K-mix Kummer walk | 48 | 9.67830e5 | 3 | 5.69120e6 | 6.12679e6 | 1.00471e7 |
| K-mix Kummer walk | 48 | 9.67830e5 | 100 | 1.23432e6 | 1.66991e6 | 5.59020e6 |
| K-pure Kummer walk | 48 | 1.26417e6 | 3 | 7.43514e6 | 8.00418e6 | 1.31255e7 |
| K-pure Kummer walk | 48 | 1.26417e6 | 100 | 1.61234e6 | 2.18138e6 | 7.30271e6 |

Ratio to the matched baseline N1 (cost per solve; > 1 means the arm is dearer). point [lo, hi]: step-ratio interval recorded by G1 times the deterministic per-step cost ratio from counted M, S, I and native counters. `rec` = value recorded in G1 `out/summary.txt` where one exists.

| arm | I/M | w=0 | w=0.1 (B) | w=1 (B) | w=0.1 (A) | w=1 (A) | order across band |
|---|---|---|---|---|---|---|---|
| A0 additive rho (no negation) | 3 | 1.31038 [1.20328, 1.42834] | 1.05535 [0.969098, 1.15035] | 0.666344 [0.611883, 0.726324] | 1.31040 [1.20330, 1.42835] | 1.31052 [1.20341, 1.42848] | UNRESOLVED |
| A0 additive rho (no negation) | 10 | 1.31379 [1.20641, 1.43204] | 1.05719 [0.970786, 1.15235] | 0.666637 [0.612152, 0.726643] | 1.31380 [1.20642, 1.43206] | 1.31392 [1.20653, 1.43219] | UNRESOLVED |
| A0 additive rho (no negation) | 30 | 1.31551 [1.20799, 1.43393] | 1.05812 [0.971640, 1.15337] | 0.666785 [0.612287, 0.726805] | 1.31552 [1.20801, 1.43394] | 1.31565 [1.20812, 1.43407] | UNRESOLVED |
| A0 additive rho (no negation) | 100 | 1.31626 [1.20868, 1.43475] | 1.05853 [0.972013, 1.15381] | 0.666849 [0.612347, 0.726875] | 1.31628 [1.20870, 1.43476] | 1.31640 [1.20881, 1.43489] | UNRESOLVED |
| K-mix Kummer walk | 3 | 5.90810 [5.43848, 6.43873] rec 5.90745 | 4.26861 [3.92931, 4.65199] rec 4.26796 | 1.76781 [1.62729, 1.92658] rec 1.76753 | 5.90799 [5.43838, 6.43860] rec 5.90688 | 5.90698 [5.43744, 6.43750] rec 5.90170 | X_dearer_across_band |
| K-mix Kummer walk | 10 | 3.23506 [2.97791, 3.52561] rec 3.23477 | 2.47333 [2.27673, 2.69547] rec 2.47303 | 1.31393 [1.20948, 1.43193] rec 1.31378 | 3.23502 [2.97787, 3.52556] rec 3.23451 | 3.23460 [2.97749, 3.52511] rec 3.23221 | X_dearer_across_band |
| K-mix Kummer walk | 30 | 1.87929 [1.72991, 2.04807] rec 1.87916 | 1.56393 [1.43962, 1.70439] rec 1.56380 | 1.08446 [0.998260, 1.18186] rec 1.08438 | 1.87928 [1.72990, 2.04806] rec 1.87907 | 1.87917 [1.72979, 2.04794] rec 1.87824 | UNRESOLVED |
| K-mix Kummer walk | 100 | 1.28733 [1.18500, 1.40295] rec 1.28728 | 1.16712 [1.07434, 1.27194] rec 1.16706 | 0.984430 [0.906180, 1.07284] rec 0.984376 | 1.28733 [1.18500, 1.40295] rec 1.28726 | 1.28735 [1.18503, 1.40298] rec 1.28707 | UNRESOLVED |
| K-pure Kummer walk | 3 | 8.19365 [7.41244, 9.03453] rec 8.19260 | 5.91990 [5.35548, 6.52744] rec 5.91889 | 2.45163 [2.21788, 2.70323] rec 2.45121 | 8.19350 [7.41230, 9.03436] rec 8.19180 | 8.19211 [7.41104, 9.03283] rec 8.18463 | X_dearer_across_band |
| K-pure Kummer walk | 10 | 4.48640 [4.05865, 4.94682] rec 4.48592 | 3.43002 [3.10299, 3.78203] rec 3.42954 | 1.82214 [1.64841, 2.00914] rec 1.82191 | 4.48634 [4.05859, 4.94675] rec 4.48556 | 4.48578 [4.05809, 4.94613] rec 4.48238 | X_dearer_across_band |
| K-pure Kummer walk | 30 | 2.60607 [2.35759, 2.87351] rec 2.60587 | 2.16877 [1.96199, 2.39134] rec 2.16856 | 1.50390 [1.36051, 1.65824] rec 1.50376 | 2.60605 [2.35758, 2.87350] rec 2.60574 | 2.60591 [2.35746, 2.87335] rec 2.60459 | X_dearer_across_band |
| K-pure Kummer walk | 100 | 1.78507 [1.61488, 1.96827] rec 1.78500 | 1.61842 [1.46412, 1.78451] rec 1.61833 | 1.36516 [1.23500, 1.50526] rec 1.36507 | 1.78508 [1.61488, 1.96827] rec 1.78497 | 1.78513 [1.61493, 1.96833] rec 1.78471 | X_dearer_across_band |

Check: largest relative difference between this recomputation and G1's recorded K-mix/N1 and K-pure/N1 values (point, lo, hi, all 40 band cells): 0.00114012.

K arms against A0 (the unmatched E5 baseline), same method with G1's recorded K/A0 step ratios:

| arm | I/M | w=0 | w=0.1 (B) | w=1 (B) | w=0.1 (A) | w=1 (A) | order across band |
|---|---|---|---|---|---|---|---|
| K-mix Kummer walk | 3 | 4.50869 [4.12482, 4.94760] | 4.04472 [3.70036, 4.43847] | 2.65299 [2.42712, 2.91126] | 4.50855 [4.12470, 4.94746] | 4.50736 [4.12360, 4.94614] | X_dearer_across_band |
| K-mix Kummer walk | 10 | 2.46240 [2.25275, 2.70211] | 2.33953 [2.14035, 2.56728] | 1.97098 [1.80317, 2.16285] | 2.46234 [2.25270, 2.70204] | 2.46179 [2.25220, 2.70145] | X_dearer_across_band |
| K-mix Kummer walk | 30 | 1.42856 [1.30693, 1.56763] | 1.47803 [1.35219, 1.62191] | 1.62641 [1.48793, 1.78473] | 1.42854 [1.30691, 1.56760] | 1.42832 [1.30671, 1.56737] | X_dearer_across_band |
| K-mix Kummer walk | 100 | 0.978016 [0.894748, 1.07323] | 1.10258 [1.00871, 1.20992] | 1.47624 [1.35055, 1.61995] | 0.978008 [0.894741, 1.07322] | 0.977935 [0.894674, 1.07314] | UNRESOLVED |
| K-pure Kummer walk | 3 | 6.25287 [5.72351, 6.86476] | 5.60940 [5.13452, 6.15832] | 3.67923 [3.36775, 4.03927] | 6.25269 [5.72334, 6.86456] | 6.25104 [5.72183, 6.86275] | X_dearer_across_band |
| K-pure Kummer walk | 10 | 3.41487 [3.12577, 3.74903] | 3.24447 [2.96980, 3.56196] | 2.73334 [2.50194, 3.00081] | 3.41478 [3.12569, 3.74894] | 3.41404 [3.12501, 3.74813] | X_dearer_across_band |
| K-pure Kummer walk | 30 | 1.98103 [1.81332, 2.17489] | 2.04964 [1.87612, 2.25021] | 2.25545 [2.06450, 2.47616] | 1.98100 [1.81329, 2.17485] | 1.98071 [1.81302, 2.17454] | X_dearer_across_band |
| K-pure Kummer walk | 100 | 1.35617 [1.24136, 1.48888] | 1.52894 [1.39950, 1.67855] | 2.04719 [1.87387, 2.24752] | 1.35616 [1.24135, 1.48887] | 1.35607 [1.24127, 1.48877] | X_dearer_across_band |

## E5 (A0 matched reference; K-pure, K-mix, K-pure on twist)

Source: `E5-kummer-multiplicative-walk/out/opcounts_pooled.txt` (b >= 28, instances 1 and 2, 272 trials per arm, trial-weighted mix 60/40/24/12) and `out/costs.csv` (recorded normalised step ratios vs A0: K-pure 1.058 [0.959, 1.170], K-mix 0.787 [0.709, 0.872]; twist 1.002 [0.896, 1.114] vs K-pure on E). E5 emitted M, S and I only. It names no native counter, so by 5a its own cost is a lower bound and could not be ranked on its own record. The same code and the same raw E5 cells (instances 1 and 2) feed G1, which does emit the native counters, so the per-step counters below are G1's pooled values for the same arms (K-pure on twist uses K-pure's, same code); the E5 row is therefore priced from two records and says so. E5's negation-map references A1 and a1c carry no per-arm M, S, I or native counts in the headline opcount record (A1 is not a sound reference for b >= 28 and a1c is not a sound reference at b = 36 and 40 per E5), so they are not priced; the sound negation-map arm is N1 in G1. K-pure on twist: step ratio to A0 derived as 1.058 x 1.002 (point only, no interval recorded).

| arm | mean steps/trial | M/trial | S/trial | I/trial | native B per step (from G1) | native A per step (from G1) |
|---|---|---|---|---|---|---|
| A0 additive rho (matched reference in E5) | 1.89621e5 | 3.79242e5 | 1.89621e5 | 1.89621e5 | 2.00035 | 0.000354012 |
| K-pure | 2.00625e5 | 4.31445e6 | 2.40808e6 | 2.00625e5 | 4.50087 | 0.000358722 |
| K-mix | 1.52664e5 | 3.28201e6 | 1.83182e6 | 1.52664e5 | 4.50021 | 0.000345788 |
| K-pure on twist E' | 2.12539e5 | 4.56861e6 | 2.54991e6 | 2.12539e5 | 4.50087 | 0.000358722 |

Ratio to A0 (cost per solve; > 1 means dearer). Step-ratio interval recorded by E5 (normalised by sqrt n, b >= 28) times the deterministic per-step cost ratio from E5's counted M, S, I and G1's native counters. `rec` = E5's own w=0 ratio (M + 0.8 S + (I/M) I, no native work).

| arm | I/M | w=0 | w=0.1 (B) | w=1 (B) | w=0.1 (A) | w=1 (A) | order across band |
|---|---|---|---|---|---|---|---|
| K-pure | 3 | 6.22165 [5.63947, 6.88027] rec 6.22100 | 5.58137 [5.05910, 6.17221] | 3.66076 [3.31821, 4.04829] | 6.22146 [5.63930, 6.88007] | 6.21982 [5.63782, 6.87826] | X_dearer_across_band |
| K-pure | 10 | 3.39778 [3.07984, 3.75747] rec 3.39700 | 3.22821 [2.92614, 3.56995] | 2.71958 [2.46510, 3.00747] | 3.39770 [3.07976, 3.75738] | 3.39695 [3.07909, 3.75656] | X_dearer_across_band |
| K-pure | 30 | 1.97108 [1.78664, 2.17974] rec 1.97100 | 2.03934 [1.84851, 2.25522] | 2.24407 [2.03409, 2.48163] | 1.97105 [1.78662, 2.17971] | 1.97077 [1.78636, 2.17939] | X_dearer_across_band |
| K-pure | 100 | 1.34933 [1.22307, 1.49217] rec 1.34900 | 1.52123 [1.37888, 1.68226] | 2.03685 [1.84625, 2.25247] | 1.34932 [1.22306, 1.49216] | 1.34924 [1.22298, 1.49207] | X_dearer_across_band |
| K-mix | 3 | 4.62668 [4.16813, 5.12639] rec 4.62500 | 4.15059 [3.73922, 4.59887] | 2.72246 [2.45264, 3.01650] | 4.62655 [4.16801, 5.12624] | 4.62532 [4.16690, 5.12488] | X_dearer_across_band |
| K-mix | 10 | 2.52686 [2.27642, 2.79977] rec 2.52600 | 2.40078 [2.16284, 2.66008] | 2.02260 [1.82214, 2.24105] | 2.52679 [2.27636, 2.79970] | 2.52624 [2.27586, 2.79908] | X_dearer_across_band |
| K-mix | 30 | 1.46597 [1.32068, 1.62430] rec 1.46600 | 1.51673 [1.36641, 1.68055] | 1.66901 [1.50360, 1.84928] | 1.46594 [1.32065, 1.62427] | 1.46572 [1.32045, 1.62403] | X_dearer_across_band |
| K-mix | 100 | 1.00364 [0.904165, 1.11203] rec 1.00300 | 1.13147 [1.01933, 1.25367] | 1.51492 [1.36478, 1.67854] | 1.00363 [0.904158, 1.11202] | 1.00355 [0.904090, 1.11194] | UNRESOLVED |
| K-pure on twist E' | 3 | 6.23152 (point only) | 5.59039 (point only) | 3.66722 (point only) | 6.23134 (point only) | 6.22969 (point only) | no_interval_recorded |
| K-pure on twist E' | 10 | 3.40341 (point only) | 3.23370 (point only) | 2.72463 (point only) | 3.40333 (point only) | 3.40258 (point only) | no_interval_recorded |
| K-pure on twist E' | 30 | 1.97457 (point only) | 2.04304 (point only) | 2.24841 (point only) | 1.97454 (point only) | 1.97425 (point only) | no_interval_recorded |
| K-pure on twist E' | 100 | 1.35189 (point only) | 1.52415 (point only) | 2.04087 (point only) | 1.35188 (point only) | 1.35179 (point only) | no_interval_recorded |

Check: largest relative difference between the w=0 recomputation and E5's recorded K-pure/A0 and K-mix/A0 cost ratios (4 I/M values each): 0.000633614.

## E2 (twin-target amortization: E and E' on one table; rho on E)

Source: `E2-twin-target-amortization/results.csv` (63 runs; cost columns CE, CEp, Cboth, CA4, rho at w = 0, 0.1, 1) and `summary.csv`. E2 already prices on the band: cost(w) = A + K + w x (native ops), A = group additions, K = pair-step inversions (1 GAE each), native = Legendre scan, square roots, sort comparisons (nominal n ceil(log2 n)), lookup probes, scaling multiplications. Rho = measured r-adding walk with 16 branches and Brent cycle detection, priced as steps + init + setup + w x comparisons. Table below: cell medians over the runs of the cell (9 runs at 22 and 26 bits, 4 at 30 bits, 1 at 32 bits; mu in {0.75, 1.5, 3}); ratios are medians of the per-run ratios. Column "rec dist_med" is E2's own recorded median of C_E / rho, reproduced here from the raw cost columns.

Arms: A1 = E alone, A2 = E' alone, joint = one shared table serving both, A4 = a user who wants only E but is served from the joint table, rho = measured Pollard rho on E (matched reference). The relation phase only is priced (W verified relations, no linear algebra, no DLP solved), so C_E / rho is a lower bound on the distance of a complete index calculus.

| bits | mu | runs | w | C_E (A1) | C_E' (A2) | C_joint | C_E-only user in joint table (A4) | rho (matched, measured) | joint / (C_E + C_E') | joint / C_E | C_E / rho [min, max over runs] | rec dist_med |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 22 | 0.750000 | 9 | 0 | 47984.0 | 46266.0 | 94692.0 | 55365.0 | 5540.80 | 1.00000 | 1.99786 | 8.73989 [8.04236, 11.1688] | 8.73989 |
| 22 | 0.750000 | 9 | 0.1 | 1.29652e5 | 1.29225e5 | 2.70877e5 | 1.65231e5 | 5980.98 | 1.04195 | 2.11387 | 21.8603 [20.1783, 28.1108] | 21.8603 |
| 22 | 0.750000 | 9 | 1 | 8.64663e5 | 8.75852e5 | 1.85655e6 | 1.15402e6 | 9942.60 | 1.06237 | 2.17154 | 87.6657 [80.7877, 115.999] | 87.6657 |
| 22 | 1.50000 | 9 | 0 | 54018.0 | 52874.0 | 1.06815e5 | 83421.0 | 5540.80 | 1.00000 | 1.97801 | 9.74913 [9.28565, 12.5837] | 9.74913 |
| 22 | 1.50000 | 9 | 0.1 | 1.92256e5 | 1.91544e5 | 4.00110e5 | 3.32462e5 | 5980.98 | 1.04291 | 2.08124 | 32.1446 [30.6782, 41.7131] | 32.1445 |
| 22 | 1.50000 | 9 | 1 | 1.43639e6 | 1.43958e6 | 3.03976e6 | 2.57383e6 | 9942.60 | 1.05727 | 2.11619 | 144.468 [137.518, 192.826] | 144.469 |
| 22 | 3.00000 | 9 | 0 | 1.30862e5 | 1.30633e5 | 2.61495e5 | 2.48232e5 | 5540.80 | 1.00000 | 1.99825 | 23.6663 [22.7392, 30.3703] | 23.6663 |
| 22 | 3.00000 | 9 | 0.1 | 5.89894e5 | 5.90553e5 | 1.22988e6 | 1.18881e6 | 5980.98 | 1.04188 | 2.08492 | 98.7556 [94.8915, 127.661] | 98.7555 |
| 22 | 3.00000 | 9 | 1 | 4.72118e6 | 4.72985e6 | 9.94535e6 | 9.65406e6 | 9942.60 | 1.05230 | 2.10654 | 475.367 [455.236, 632.367] | 475.366 |
| 26 | 0.750000 | 9 | 0 | 3.62631e5 | 3.59303e5 | 7.23164e5 | 4.09296e5 | 15039.9 | 1.00000 | 1.97454 | 23.0984 [21.1113, 31.7627] | 23.0984 |
| 26 | 0.750000 | 9 | 0.1 | 1.08128e6 | 1.10337e6 | 2.26971e6 | 1.34095e6 | 16409.6 | 1.03725 | 2.07964 | 63.3318 [57.5593, 87.0287] | 63.3320 |
| 26 | 0.750000 | 9 | 1 | 7.54909e6 | 7.79994e6 | 1.61886e7 | 9.72588e6 | 28736.8 | 1.05303 | 2.12488 | 252.844 [228.125, 351.268] | 252.845 |
| 26 | 1.50000 | 9 | 0 | 3.66962e5 | 3.59372e5 | 7.26334e5 | 5.53317e5 | 15039.9 | 1.00000 | 1.98387 | 23.9673 [20.9853, 32.2483] | 23.9673 |
| 26 | 1.50000 | 9 | 0.1 | 1.44660e6 | 1.44151e6 | 2.99772e6 | 2.45104e6 | 16409.6 | 1.03795 | 2.07588 | 86.9844 [75.7343, 116.725] | 86.9846 |
| 26 | 1.50000 | 9 | 1 | 1.11633e7 | 1.11808e7 | 2.34402e7 | 1.95305e7 | 28736.8 | 1.04906 | 2.10314 | 383.814 [331.943, 520.623] | 383.815 |
| 26 | 3.00000 | 9 | 0 | 8.41803e5 | 8.38487e5 | 1.68063e6 | 1.58539e6 | 15039.9 | 1.00000 | 1.99606 | 55.9595 [48.1399, 73.7622] | 55.9595 |
| 26 | 3.00000 | 9 | 0.1 | 4.24921e6 | 4.24822e6 | 8.81479e6 | 8.49461e6 | 16409.6 | 1.03722 | 2.07418 | 258.916 [222.460, 342.092] | 258.916 |
| 26 | 3.00000 | 9 | 1 | 3.49158e7 | 3.49359e7 | 7.30223e7 | 7.06776e7 | 28736.8 | 1.04528 | 2.09113 | 1214.90 [1038.23, 1625.03] | 1214.90 |
| 30 | 0.750000 | 4 | 0 | 2.60251e6 | 2.62113e6 | 5.22363e6 | 2.89780e6 | 68354.1 | 0.999999 | 2.00715 | 38.0760 [37.7799, 38.3720] | 38.0759 |
| 30 | 0.750000 | 4 | 0.1 | 8.47763e6 | 8.76638e6 | 1.78250e7 | 1.03305e7 | 75034.8 | 1.03369 | 2.10259 | 112.989 [112.113, 113.865] | 112.989 |
| 30 | 0.750000 | 4 | 1 | 6.13537e7 | 6.40737e7 | 1.31237e8 | 7.72250e7 | 1.35161e5 | 1.04632 | 2.13902 | 453.956 [450.414, 457.497] | 453.955 |
| 30 | 1.50000 | 4 | 0 | 2.47353e6 | 2.45912e6 | 4.93265e6 | 3.65395e6 | 68354.1 | 1.00000 | 1.99419 | 36.1882 [36.0162, 36.3603] | 36.1882 |
| 30 | 1.50000 | 4 | 0.1 | 1.06563e7 | 1.07377e7 | 2.21228e7 | 1.77462e7 | 75034.8 | 1.03407 | 2.07605 | 142.023 [141.265, 142.780] | 142.023 |
| 30 | 1.50000 | 4 | 1 | 8.43010e7 | 8.52451e7 | 1.76833e8 | 1.44577e8 | 1.35161e5 | 1.04298 | 2.09765 | 623.732 [620.272, 627.193] | 623.732 |
| 32 | 1.50000 | 1 | 0 | 6.42538e6 | 6.38854e6 | 1.28139e7 | 9.39852e6 | 1.41659e5 | 0.999998 | 1.99426 | 45.3581 [45.3581, 45.3581] | 45.3582 |
| 32 | 1.50000 | 1 | 0.1 | 2.85380e7 | 2.87560e7 | 5.91690e7 | 4.70132e7 | 1.55660e5 | 1.03273 | 2.07334 | 183.335 [183.335, 183.335] | 183.336 |
| 32 | 1.50000 | 1 | 1 | 2.27552e8 | 2.30063e8 | 4.76365e8 | 3.85545e8 | 2.81668e5 | 1.04097 | 2.09343 | 807.873 [807.873, 807.873] | 807.872 |

Check against E2's recorded summary.csv (eps_med, sav_med, dist_med, all 27 cell x w rows): largest discrepancy 6.84514e-6.
Minimum over all 63 runs of C_E / rho at w = 0, 0.1, 1: 8.04236, 20.1783, 80.7877.
Minimum over all 63 runs of C_A4 / C_E (the user who wants only E, served from the joint table) at w = 0, 0.1, 1: 1.11338, 1.21840, 1.25850.
Largest saving of the joint arm over separate runs, over all 63 runs (1 - min joint/(C_E+C_E')) at w = 0, 0.1, 1: 2.38005e-6, -0.0327259, -0.0409733 (negative = joint dearer in every run).

## G2 (decoupled A/B projection arm; same relations as D_m)

Source: `G2-decoupled-AB-projection-arm/results.json` key `operation_counts` (totals over the primary curves c = 1, 2, all 22 (family, p, N) cells, 200 targets per curve-cell) and `RESULTS.md`. Priced work: projection group operations (2 elladd + 1 ellneg + 2 double-and-add scalar multiplications by 1/2 mod #E x #E' per target, a textbook count and not a PARI-internal or measured one; 1 GAE each); unpriced: Semaev root-solves. P, PF(j=m,k=1) and PF(j=m,k=m) recover the same independent E-relations as D_m (E-relation counts are equal, so the ratio of totals is the ratio per relation). No rho arm and no matched rho exist in G2.

Family A (m=2, 32 curve-cells): priced = projection group operations (textbook model count, 1 GAE each); unpriced = Semaev root-solves.

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| D_m (baseline) | 0 | 192400 | 0 | 19240.0 | 1.92400e5 | n/a | 1.00000 | 1.00000 |
| P class S (j=m): D_m + projection | 280000 | 192400 | 2.80000e5 | 2.99240e5 | 4.72400e5 | n/a | 15.5530 | 2.45530 |
| PF (j=m,k=1) | 280000 | 198800 | 2.80000e5 | 2.99880e5 | 4.78800e5 | n/a | 15.5863 | 2.48857 |
| PF (j=m,k=m) | 280000 | 448000 | 2.80000e5 | 3.24800e5 | 7.28000e5 | n/a | 16.8815 | 3.78378 |

Family B (m=3, 12 curve-cells): priced = projection group operations (textbook model count, 1 GAE each); unpriced = Semaev root-solves.

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| D_m (baseline) | 0 | 148800 | 0 | 14880.0 | 1.48800e5 | n/a | 1.00000 | 1.00000 |
| P class S (j=m): D_m + projection | 96000 | 148800 | 96000.0 | 1.10880e5 | 2.44800e5 | n/a | 7.45161 | 1.64516 |
| PF (j=m,k=1) | 96000 | 151200 | 96000.0 | 1.11120e5 | 2.47200e5 | n/a | 7.46774 | 1.66129 |
| PF (j=m,k=m) | 96000 | 643200 | 96000.0 | 1.60320e5 | 7.39200e5 | n/a | 10.7742 | 4.96774 |

Arms whose relations differ from D_m's (per-relation ratio recorded by G2, root-solve work units; no priced counter exists, so the ratio is the same at every w > 0 and both costs are 0 at w = 0): D_2m (same term count) 2.03 to 19.7 over 22 powered primary cells; X coupled (distinct E-sets) 209.8 to 916.9 (family A), 390.2 to 2973.8 (family B); PF (j=m,k=m) without projection 2.087 to 3.556 (A), 2.528 to 8.333 (B). Total root-solves: D_2m 43,713,600 (A) and 2,067,200 (B); X 167,270,400 (A, 24 curve-cells) and 528,998,400 (B, 12 curve-cells).

## E3 (eigen-decomposition over F_{q^2}; two index-calculus arms)

Source: `E3-eigen-decomposition-Fq2/RESULTS.md` "Operation counts" and `results.json` key `op_counts`. Arms: D (direct Semaev decomposition in E(F_p), baseline) and X (the F_{p^2} eigen-decomposition route). Counters recorded: univariate root-solves (native, the Semaev step) and, for the table arm, sign-vector group sums (native). No priced (group-addition) count is recorded, so w = 0 gives 0 for both arms. Field multiplications and polynomial evaluations were not instrumented. No rho arm and no matched rho exist in E3.

Semaev arm, all 36 cells (priced count: none recorded; root-solves are the native unit):

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| D direct Semaev, E(F_p) (baseline), root-solves | 0 | 94400 | 0 | 9440.00 | 94400.0 | n/a | 1.00000 | 1.00000 |
| X F_p^2 eigen-decomposition route, root-solves | 0 | 37440000 | 0 | 3.74400e6 | 3.74400e7 | n/a | 396.610 | 396.610 |

Table arm, all 36 cells (group sums as native op):

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| D table arm, group sums | 0 | 10044 | 0 | 1004.40 | 10044.0 | n/a | 1.00000 | 1.00000 |
| X table arm, group sums | 0 | 25629568 | 0 | 2.56296e6 | 2.56296e7 | n/a | 2551.73 | 2551.73 |

Per E-relation (same 36 cells): D 6.85747 root-solves/relation, X 195.992 root-solves/relation, X/D = 28.5808 (derived from the recorded totals; relations D = 9021 + 4745 = 13,766, X(j=1..m) = 77,900 + 113,128 = 191,028). E3's recorded per-cell values over the 33 powered cells: work ratio per relation 53.3 to 2812 (= rho_gen), root-solves per relation ratio 6.66 to 703, table-arm group-sum ratio 93.3 to 2031.

## E4 (Koblitz twist pair, shared x-only base)

Source: `E4-koblitz-twist-pair/RESULTS.md` "Operation counts" (`out/opcounts.json`). Arms: A_E (E-only base, baseline) and A_S (shared E + twist base), totals over 345 V draws, K0 targets. Counter recorded: counted solves. Group operations, polynomial evaluations and field multiplications were not instrumented. No rho arm and no matched rho exist in E4. Because both arms produce the identical relation set and A_S's solves are A_E's plus those on the twist-type abscissas, A_E dominates A_S in the only counter that exists, at every weight.

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| A_E E-only base (baseline), counted solves on K0 targets | 0 | 21361840 | 0 | 2.13618e6 | 2.13618e7 | n/a | 1.00000 | 1.00000 |
| A_S shared E+twist base, counted solves | 0 | 43249080 | 0 | 4.32491e6 | 4.32491e7 | n/a | 2.02460 | 2.02460 |

Both arms yield the same 773,404 K0 relations (relation sets equal target by target), so per relation: A_E 27.6205, A_S 55.9204 counted solves, ratio 2.02460 (E4 records 27.62, 55.92, 2.0246). A counted solve is one table inversion + one trace test + one half-trace extraction; E4 does not split them, and solves on wrong-type abscissas end at the trace test, so the per-op mix is not recoverable from the record.

## E6 (genus-2 gluing)

No arm was measured: no Jacobian group law, no rho and no index-calculus arm on Jac(C) were run, and no group operation was counted (E6 RESULTS, "Operation counts"). The only costs are model columns (rho = sqrt(pi l / 2) steps per curve, l = largest prime factor or l = #E; exponent-1 index calculus on Jac(C) = p group operations with constant 1), which model group operations only and no native work for either arm. Under 5a this cannot be priced: both arms would need counted native operations. Recorded model ratio, index calculus ops / rho ops (l = #E), median per prime, reported as model distance at w = 0 only, no band: p = 101: 8.019; 211: 11.56; 409: 16.00; 1009: 25.52; 4001: 50.48; 10007: 79.95; 20011: 112.7 (model rho ops 12.6, 18.25, 25.56, 39.53, 79.27, 125.2, 177.6). The exponent gap 1/2 (p vs sqrt(l)) is the only weight-independent statement, and it is not a measurement. Evaluation counts of the verification itself (degree-3 and degree-6 polynomial evaluations, `hyperellcharpoly` calls) are native counters of the experiment, not of an ECDLP arm.

## E0, E1, G3 (verification and census experiments, no ECDLP arm)

E0 (separation lemma), E1 and G3 (mixed-solution census) state that they make no ECDLP cost claim and run no rho arm. Their tiers or detector arms are priced below only so that every recorded count appears on the band; there is no matched baseline, so no ratio and no ranking is made. E0: priced = explicit curve additions (T1 only), unpriced = the recorded total of counted operations minus the additions (ordinate evaluations and phi(B) candidate tests; E0 records the T1 candidate count 305,030 separately and does not include it in its T1 total). E1 and G3: priced = none; unpriced = S_3 root-set evaluations (pkeys); E1 also records an upper bound on lookups (527,591,600 for m = 2 and 43,135,600 for m = 3), shown as a second row. G3 records no lookup count. Brute-force Oracle B tuples of E1 (7,370,252,744,900) were not run and are a comparator count, not an arm cost.

E0 tiers:

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| T1 full pair loop (8254 classes) | 18788752 | 19081056 | 1.87888e7 | 2.06969e7 | 3.78698e7 | n/a | n/a | n/a |
| T2 B-only linear check (34,498 classes) | 0 | 25393972 | 0 | 2.53940e6 | 2.53940e7 | n/a | n/a | n/a |
| T2S sample m=8,9 (4000 curves) | 0 | 6152000 | 0 | 6.15200e5 | 6.15200e6 | n/a | n/a | n/a |
| T3 unreduced curves (1448) | 0 | 108464 | 0 | 10846.4 | 1.08464e5 | n/a | n/a | n/a |


E1 detector arm:

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| Detector A, pkeys only (592 configs) | 0 | 414817366 | 0 | 4.14817e7 | 4.14817e8 | n/a | n/a | n/a |
| Detector A, pkeys + lookup upper bound | 0 | 985544566 | 0 | 9.85545e7 | 9.85545e8 | n/a | n/a | n/a |


G3 detector arm:

| arm | priced (GAE) count | unpriced native count | cost w=0 | cost w=0.1 | cost w=1 | ratio to baseline w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|---|---|
| Detector A, pkeys, all stages (3,972 configs) | 0 | 4170649121 | 0 | 4.17065e8 | 4.17065e9 | n/a | n/a | n/a |


## Counters named but not emitted (lower bounds and implementation gaps to fix)

| experiment | arm | what is missing | consequence |
|---|---|---|---|
| E2 | rho on E | the 16-branch selector hash per step (E2 prices comparisons only); init and set-up are priced as GAE | rho cost is a lower bound at w > 0, so C_E / rho is an upper bound on the distance at w = 0.1 and 1; at w = 0 the unpriced work is 0 by construction and the 8.04 minimum stands |
| E2 | all IC arms | the about 10 field multiplications of a pair step are listed in raw counts only and priced as 1 GAE per inversion; sqrt cost is one native op | IC cost is priced consistently across E, E', joint and A4, so the within-method ratios are unaffected |
| E3 | D and X | no group-addition count; field multiplications and polynomial evaluations not instrumented | w = 0 point degenerate (0 for both); ratio is w-invariant for w > 0 |
| E4 | A_E and A_S | no group-operation count; the three operations inside a counted solve are not split | w = 0 degenerate; dominance (A_S >= A_E in the single counter) is weight-independent |
| E5 | A0, K arms, K-pure twist | no native counter at all (SEL, DPT, LAD, LOOK, START); A1 and a1c not priced | priced from G1's counters for the same code; E5's own cost ratios are lower-bound comparisons |
| G1 | all four arms | START is priced at 1 although a trail start is one scalar multiplication; SEL and DPT are 1 per step by code structure, not instrumented | native work of every arm is a lower bound; the K arms' LAD is exact from the multiplier histogram |
| G2 | P, PF | projection group operations are a textbook model count, not measured; Semaev root-solve internals and polynomial evaluations not instrumented | D_m, P, PF ordering uses model counts for the priced part |
| E6 | all | nothing measured; model columns only | not rankable |
| E0, E1, G3 | detector and tier arms | field multiplications inside GP not instrumented | not applicable (no ECDLP arm) |

## Caveats

- Derived intervals (A0/N1, E5 at w > 0, every G1 and E5 cell outside G1's recorded set) are the recorded step-ratio interval times the deterministic per-step cost ratio. They share the recorded intervals' instance-level resampling and its limits (G1: four instances per b; E5: two instances per b). A0/N1 was not ranked by G1 (A0 is not its matched baseline); the unresolved verdict here is derived from G1's counted native counters, CAN = 1 and CYC = 2 per N1 step against none for A0.
- E5's cost model prices field operations only as M + 0.8 S + (I/M) I (E5 and G1 both). Neither records a calibrated cost, so no calibrated and uncalibrated cost is compared anywhere.
- No arm in this file is a verified improvement over its matched baseline at any band point: this is a statement about the recorded evidence, with the gap to the matched rho reported above as the distance remaining.
- No validation-gate evidence (replay certificate, validator report, matched rho with the 5a band named by a `VALIDATION.json`) is claimed for any row; nothing here goes to the IC leaderboard or `benchmarks/`.
