# Genus131: exact five-class complementary-factor screen

Scope: E/F2 of trace t=-2,-1,0,1,2; hypothetical J(C) isogenous to A131(E0)*E.
No curve, timing comparison, full gluing enumeration or DLP solve is planned.

Use native C++17/GMP. Reconstruct P_A(X) from
(X^262-s131 X^131+2^131)/(X²+X+2), where s is the trace recurrence for E0.
Convert exactly to P_A(X)=X^130 h_A(X+2/X) by integer triangular subtraction.
For every t, evaluate h_A(t), and check the independent quadratic-ring norm
of P_A(gamma), gamma²-t gamma+2=0, equals2^130*h_A(t)^2.

Compute N_m for the hypothetical curve and closed-place counts through262
from exact trace recurrences and divisor subtraction. Retain every row,
including negative values and the first failure. Predictions: t=-2 fails
at degree2; parent identity h_A(-1)=131*U131 must agree. No prediction of
curve existence is made for classes passing these necessary conditions.

Artifacts: screen.cpp, summary.txt, places.tsv; factorization output from
GNU factor, to be replayed with native certificates before any prime claim.
Use the complete linear-factor reduced resultant |h_A(t)| when applying
Howe2022 Theorem4.15, not the much larger full Weil resultant.

All computations are exact arithmetic proof checks, not performance runs.
Mathematical uses of the output remain written and unsigned until independently
reviewed; polarization, actual curve equations and subgroup maps are separate.
