#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Rule v8: the index calculus against a matched rho

Two runs of one binary (`ic` from commit 370e1919), six sizes of ledger §23, 64 targets a size, 408 rows each, Emerald Rapids container, isolated one-core runs. *v8*: rho's signed-Frobenius canonicalisation batched 16 lanes at a time. *v8c*: the same binary with `KIC_RHO_CANON=scalar` (rho as at rule v7). Source: `matched.json`, `analysis.json`, `PROTOCOL.md` (declared before any row). Class: matched rho = engineering (rho side); larger distance = accounting; slope = measurement. No advance; no index-calculus improvement.

== Checks
408 of 408 rows pass in both runs; every logarithm is §23's; on 64 of 64 paired targets at every size the two rho arms take the same steps and recover the same logarithms. The pin failed once before any timing (a counter, `row_ops`, added to main after §23): Amendment 1. The matched rho is admissible against the strong walk at all six sizes (0.57--0.82 of its time a step); the control is not at four of six (1.05, 1.20, 1.04, 1.03).

== Cold IC/rho, with 95% bootstrap intervals over targets
#table(columns: 7, align: (right, right, left, right, right, right, left),
[log2 r],[n],[v8 matched],[v8c control],[v7],[v3],[matched / control],
[36.64],[47],[4.67 [4.34, 5.07]],[3.59],[3.43],[5.23],[1.30 [1.29, 1.31]],
[38.0],[57],[5.70 [5.33, 6.13]],[4.25],[3.97],[6.12],[1.34 [1.31, 1.37]],
[39.0],[41],[7.07 [6.43, 7.84]],[5.00],[4.74],[7.66],[1.41 [1.38, 1.45]],
[44.26],[53],[11.75 [10.49, 13.25]],[7.65],[7.03],[13.24],[1.54 [1.49, 1.58]],
[44.52],[59],[10.65 [9.38, 12.21]],[6.74],[6.37],[12.03],[1.58 [1.53, 1.64]],
[47.21],[61],[18.07 [15.98, 20.58]],[11.56],[11.51],[20.97],[1.56 [1.51, 1.62]])
All six values lie inside the pre-declared prediction ranges. The index calculus's own online time, v8 over v8c, is 0.976--1.033: the measurement's drift is about 3%.

#image("cold-ratio.svg", width: 100%)

== Slope of ln(cold ratio) on ln r
#table(columns: 4, [arm],[slope],[95% t (six sizes)],[bootstrap over targets],
[v3],[0.1756],[0.137, 0.214],[],
[v7],[0.1440],[0.092, 0.196],[],
[v8c control],[0.1409],[0.096, 0.186],[0.125, 0.157],
[v8 matched],[0.1679],[0.128, 0.208],[0.152, 0.184])
Kill condition 2 (slope not below v3's 0.1756) is not triggered: 0.1679 is below, "within the interval" (difference from v3 −0.008). Matching rho moved the slope by +0.027; the fall from v3 to v7 was mostly an unmatched-rho artefact. n and log2 r are not independent in these six curves. Condition 1 (gap at 50 bits or more) was not testable: no Koblitz curve with n ≤ 63 reaches 50 bits (largest 47.21) and `ic price` has no two-word path; the fit extrapolates 23.0× [18.3, 29.0] at 50 bits for the matched arm (a prediction).

#image("paired.svg", width: 100%)
