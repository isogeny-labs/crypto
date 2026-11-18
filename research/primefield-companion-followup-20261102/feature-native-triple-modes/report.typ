#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Triple-mode coefficient reuse
T-1893 · Prime-field direct-image update · 2026-11-08

The optional update generates three selected modes from one mode1 anchor stream. Adjacent-binomial identities provide both neighbors; their denominators join the stride-transition batch inverse. Mode0's known leading coefficient is restored separately.

#image("flow.svg", width: 100%)

*Construction.* For h=(r−1)/2, m≥3 dividing h, and n=1+jm, derive $c_(n-1) = -n c_n / (h-n+1)$ and $c_(n+1) = -(h-n) c_n / (n+1)$. Both denominators are nonzero. The reader holds at most cap triples and 3·cap denominator entries; no full coefficient vector is retained.

*Quotient reuse.* Compute F0,F1,F2 modulo the old branch polynomial g. After g1=gcd(g,C0), these representatives remain valid because g1 divides g. Applying the same nonlinear C2 gcd therefore preserves the staged support. Source schedule, initialization and final point verification remain unchanged.

*Exact qualification.* validation01 passes 40 reader configurations, 975 coefficient equalities, 1,920 quotient equalities and 3,640 candidate-state equalities. The optional adapter matches outcomes, routes and source calls for 843 public target/cap cases; 735 subsequent updates exercise the new path. Inherited stdout is byte-identical. PCC-20261108-130f40 is unsigned producer evidence; signed review pending.

*Cost tradeoff.* If a branch survives C0, the staged update performs two anchor traversals; the fused update performs one. But it adds neighbor work, a third component and a leading power, and evaluates modes1/2 before pruning. If C0 kills every branch, staged evaluation avoids that quotient work. These correctness checks do not select a faster policy.

*Evidence and next.* Sources, reversible generation, compiler hashes, raw logs and derivation are retained. No cloud/timed or production comparison ran. Next freeze a matched fused/staged update comparison, including zero-survivor cases and full quotient/coefficient accounting. Original cohort and separate transfer research remain intact.
