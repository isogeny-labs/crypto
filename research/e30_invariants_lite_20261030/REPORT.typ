#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E30 ("invariants", lite): an accounting audit of trivial $FF_(p^4)$ products

*Run report and results, 2026-10-30.* Note §9.28 (pre-registration, `b3ddccb13`) and §9.32 (results). Code and binary `0e4fccce1`. Records `experiments/35_invariants_lite/`. Host: the 4-vCPU container; no isolation tier; operation counts only. *No Stage B.*

*What E30 is.* The lost draft's counter change could not be reconstructed (the note records neither the counter nor the draft). Registered and run instead: a separate counter that re-prices products whose operand is 0 or 1 (to 0) or lies in $FF_p$ (to 4, a scaling). Recorded counts do not change.

== Measured
- IC side (E27's Karatsuba solver-A runs plus the audit, 20 runs, 0 failures): every non-count field equal to E27's records; trivial products 0.3087–0.3131 % of $S$.
- Walks (E25's seeds, 320 walks): saving 0–1.1 $FF_p$ multiplications per walk ($<= 0.0002$ %); the GLS multipliers are not in $FF_p$.
- Distance to the matched $chevron.l plus.minus psi chevron.r$ walk (E27 (iii)): −0.309 to −0.313 % at 0 GAE, −0.084 to −0.086 % at 0.1, −0.011 % at 1 GAE.

#figure(image("figures/distance_change.svg", width: 100%), caption: [Change of $S\/"rho"$ per row under the audit at three weights. Source: `summary.json`.])

== Verdicts (§9.28)
G2.110, G2.111, G2.112, G2.113 hold (largest change 0.313 %; the red team's estimate for the lost draft was ≈ 0.6 %).

== What it says
*Accounting, not an improvement*: no algorithm changed. Recorded so that a counter change of this size is not read as progress. Main's 2026-10-24 change (charging the walks' $[d]G$ check, §9.29) moved the same distances by 0.13–0.78 % the other way.
