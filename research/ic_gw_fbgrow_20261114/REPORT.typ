#set page(margin: 1.8cm)
#set text(size: 10pt)
= Incremental factor-base growth vs the charged restart
_Plane-quartic double-large-prime IC, GHS-weak family E0/F\_(q^3), k = 18..22. Every number is an unverified assertion \[pcc:unverified\]; local L0 run (Iso Lab EKS budget exceeded, owner's choice); wall and CPU are L0 diagnostics._

*Verdict under the pre-registered rule (frozen at 0402ffb5c1): NULL for both growth arms.* Growth is an improvement only if the DL counted cost is below the REF (v3, c_D = 1.7, charged restart) at all 15 (q, seed) cells for additions priced 0, 0.5, 1 M-eq and hash priced 0 and 0.1 M-eq. G160 (start c_D 1.6, heavy-row trigger W = 50, +5% steps) violates 20 of 30 (size, price) grid points, G155 (c_D 1.55, W = 60) 22 of 30. Worst cells: 1.077 and 1.045.

#figure(image("fbgrow.png", width: 100%), caption: [DL counted cost G / REF (geometric mean over seeds, dots: seeds), hash at 0.1 M-eq; right: total CPU ratio (L0).])

== Gates
All passed: counted output of the study binary identical to the scratch candidate (6 of 6 records); REF identical to the committed combo B170 units (45 arm records, 0 mismatches); 45 of 45 units rc 0, last record ok with k_found = k; load1 at the lock at most 1.69. REF never retried.

== DL counted ratio G / REF (geometric mean over seeds; hash 0.1)
#table(columns: 6, align: (left, right, right, right, right, right),
  [arm, w], [k=18], [19], [20], [21], [22],
  [G160, 0], [0.981], [0.988], [0.988], [0.982], [1.044],
  [G160, 0.5], [0.994], [1.001], [1.001], [0.996], [1.018],
  [G160, 1], [0.999], [1.006], [1.007], [1.002], [1.008],
  [G155, 0], [0.934], [0.952], [0.933], [0.955], [0.953],
  [G155, 0.5], [0.987], [1.008], [0.998], [0.998], [0.997],
  [G155, 1], [1.011], [1.031], [1.026], [1.016], [1.015])

== What the data say
- With additions free G155 is 0.93-0.95 of REF in the geometric mean (one seed at k = 21 and 22 above 1); additions at 0.5 M-eq erase it.
- The saving is in the fb, relations and linear-algebra phases (0.92-0.98); the descent phase is 1.3-2.7 times dearer with the larger final factor base.
- Hash work rises 1.5-2.0 times; pricing it at 0.1 M-eq moves ratios by at most 0.002, at 1 M-eq by at most 0.02.
- Total DL CPU is 0.95-1.45 of REF per cell (median 1.13 and 1.11), as in the pilot: no time gain.
- The charged restart cost nothing at c_D = 1.7: no D, DL1 or DL record at c_D >= 1.7 and q >= 262111 in the earlier committed records has a retry (`retries.json`).

== Distance to rho (DL M-eq, additions and hash free, over matched plain rho at w = 0)
#table(columns: 6, align: (left, right, right, right, right, right),
  [arm], [k=18], [19], [20], [21], [22],
  [REF], [0.01059], [0.00708], [0.00546], [0.00324], [0.00243],
  [G160], [0.01038], [0.00699], [0.00539], [0.00318], [0.00253],
  [G155], [0.00988], [0.00673], [0.00509], [0.00309], [0.00231])
No robust change in the distance remaining is claimed.

== Binaries
REF v3 de500bc154f3074359badf434255605ae103b974f4ac2b00159fc84bd3a76bb6; growth v3g 575ee3baec40f71ce3bc519dd5ed0d8273aa8fa9c1fa65cc5cb0d023ea68fe5e.
