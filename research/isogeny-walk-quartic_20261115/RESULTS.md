# RESULTS (all `[pcc:unverified]`)

## S1a one-step Smith full_success, random octics, n = 3000 per seed

| p | seed 1 | seed 2 | seed 3 | pooled | Wilson 95% |
|---|---|---|---|---|---|
| 101 | 0.1907 | 0.1873 | 0.2050 | 0.1943 | [0.1863, 0.2026] |
| 211 | 0.1950 | 0.1887 | 0.1743 | 0.1860 | [0.1781, 0.1942] |
| 521 | 0.1740 | 0.1893 | 0.1900 | 0.1844 | [0.1766, 0.1926] |
| all | | | | 0.1883 | [0.1836, 0.1930] |

Prediction P1 band [0.14, 0.24]; point prediction 0.186.

## S1b exhaustive class-level ceilings (existence of a Frobenius-stable kernel, no isogeny constructed)

| p | curves (with twists) | direct pair kernel | direct any Lagrangian | class contains pair-kernel curve | class any Lagrangian |
|---|---|---|---|---|---|
| 7 | 1411788 | 0.4897 | 0.6563 | 0.4897 | 0.6563 |

## S2 projection of X by the Mumford coefficients (b0 : b1 : b2)

- preregistered cells (d = 2..5): 60 instances (p = 37, 101; 3 seeds x 10), 36 to 200 points each; nonzero kernel at d <= 5 in 0 instances.
- exploratory extension (p = 101, d up to 12): minimal degree counts {'9': 1, '10': 11}; example kernel dims d=2..12: [0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 6].
- fraction of H(F_p) points with an F_p-rational R-pair: 0.491; image errors: 0.

p = 11 exhaustive coverage: started, killed unfinished after 28 CPU-minutes (not a result). Cited, not re-measured (research/smith_iso_granularity_20261015): D / C / U = 0.1758 / 0.4304 / 0.4897 at p = 7 and 0.1805 / 0.475 / 0.4943 at p = 11.


## S2b canonical map by the trace of the H differentials (amendment A1)

60 instances (p = 37, 101; 3 seeds x 10): kernel-dim vectors (d = 2..6) and counts: {'(0, 0, 1, 3, 6)': 58, '(1, 3, 6, 10, 15)': 2}
per p: {'37': {'instances': 30, 'unique_quartic': 29, 'on_conic': 1}, '101': {'instances': 30, 'unique_quartic': 29, 'on_conic': 1}}; min points 33, image errors 0, chart-skipped points 35. [0,0,1,3,6] = unique plane quartic; [1,3,6,10,15] = image on a conic (X hyperelliptic).

## S3a push-forward into Pic^0 of the quartic (amendment A1, P7)

60 instances: 58 with a smooth canonical quartic (PlaneQuartic::new smoothness certificate), 2 on a conic (no quartic).
Principal-divisor test Phi(P)+Phi(iota P) independent of x: 1538 pairs tested, instances with tests 57, single value in 57, several values in 0; Phi None 44; library panics caught 254 (plane_quartic_pic `complete` limit, counted as not tested).
Negative control (Phi(P)+Phi(P') with P, P' not related by iota): several values in 57 of 57 instances.

## S3b/S4 toy DLP: BSGS on Pic^0(C), log pulled back and checked on H (amendment A2, A2b)

| p | inst | targets | log found on C | verified on H | found, not verified | of which diff in 8-torsion | ord(G_H)/ord(G_C) |
|---|---|---|---|---|---|---|---|
| 101 | 10 | 60 | 50 | 44 | 6 | 6 | {'1': 10} |
| 211 | 12 | 72 | 60 | 54 | 6 | 6 | {'1': 10, '2': 2} |
| 521 | 12 | 72 | 50 | 43 | 7 | 7 | {'1': 7, '2': 5} |

Cost per target (median over instances, F_p mul+inv counted by FqCounter) and matched-rho reference (analytic, see PLAN A2), S/rho median [min, max] at native price w = 0, 0.1, 1 per unpriced op:

| p | median ord(G_H) | IC-route priced ops/target | rho priced ops (analytic steps x Cantor field ops) | S/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|
| 101 | 134294 | 1.95e+06 | 1.95e+05 | 10.5 [8.9, 13.6] | 10.5 [8.9, 13.6] | 10.5 [8.9, 13.5] |
| 211 | 1166668 | 4.93e+06 | 5.05e+05 | 11.0 [7.4, 13.3] | 11.0 [7.4, 13.3] | 11.0 [7.4, 13.3] |
| 521 | 16948893 | 1.83e+07 | 2.07e+06 | 10.6 [6.2, 15.2] | 10.6 [6.2, 15.2] | 10.6 [6.2, 15.2] |

Log-log exponent fit over p (3 points, no interval): {'ic_priced_per_target': 1.3687494502309632, 'rho_priced': 1.4460634949766418, 'note': 'slope of ln(cost) vs ln(p) over p = 101, 211, 521 (three points, no interval); BSGS on C expects 1.5, rho on a group of size p^3 expects 1.5'}
